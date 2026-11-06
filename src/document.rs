//! One open document: its text, the raw text view, the formatted view, the file
//! it came from, and everything that keeps those in step.
//!
//! The buttons and bars around a document (the header bar, formatting bar, find
//! bar and banner) belong to the window, in window.rs. A document can tell its
//! window to update them, but its own work never depends on which window it's
//! in, because the link to the window can be swapped for another one.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;
use webkit6::prelude::*;

use crate::dialog::{Answer, Question};
use crate::file_change::{self, Comparison, Version};
use crate::format::{self, Inline, LineStyle};
use crate::render;
use crate::state::ViewMode;
use crate::typing;
use crate::window::{Window, file_filters};

/// The empty page (styles only) that the formatted view is drawn into.
const PREVIEW_PAGE: &str = include_str!("preview.html");

/// Helpers for click-to-jump and synced scrolling, run inside the formatted view.
const PREVIEW_SCRIPT: &str = include_str!("preview.js");

/// LeafMD's private script "world" in the formatted view. Code and messages here
/// are kept apart from the document itself.
const SCRIPT_WORLD: &str = "leafmd";

/// How long to wait after the last keystroke before updating the formatted view.
const RENDER_DELAY: Duration = Duration::from_millis(150);

/// After LeafMD scrolls one side to match the other, scroll events from that
/// side are ignored for this long, so the two sides don't keep nudging each other.
const SYNC_QUIET: Duration = Duration::from_millis(250);

/// How long to wait after a sign that the open file changed before reading it.
/// One save can produce several signs (deleted, created, changed), so waiting
/// gives one check, and an app that saves by deleting and recreating the file
/// doesn't look like it deleted it.
const FILE_CHECK_DELAY: Duration = Duration::from_millis(500);

/// What a document's find bar was showing when another tab was selected. The
/// window has one find bar, so this is how each tab keeps its own.
#[derive(Default)]
pub struct FindState {
    pub open: bool,
    /// The text last searched for
    pub query: String,
    pub replace_open: bool,
    pub replace_text: String,
    /// The keyboard's focus was in the find bar rather than the document
    pub focus_in_bar: bool,
    /// And it was in the replace box in particular
    pub focus_in_replace: bool,
}

thread_local! {
    /// Every open document in the app. Tabs are looked up here by their widget,
    /// which works whichever window a tab has moved to. (GTK only ever runs on
    /// one thread, so this is simply one list.)
    static OPEN_DOCUMENTS: RefCell<Vec<Rc<Document>>> = const { RefCell::new(Vec::new()) };
}

pub struct Document {
    /// The window showing this document. Weak, so a document never keeps a
    /// closed window alive.
    window: RefCell<Weak<Window>>,

    /// The raw text on the left and the formatted view on the right
    pub paned: gtk::Paned,
    source_scroll: gtk::ScrolledWindow,
    pub source_view: sourceview5::View,
    pub buffer: sourceview5::Buffer,
    pub webview: webkit6::WebView,
    /// What to look for in the raw text, and how (match case, whole words)
    pub search_settings: sourceview5::SearchSettings,
    /// Searches the raw text: highlights every match and counts them
    pub search_context: sourceview5::SearchContext,
    pub find_state: RefCell<FindState>,

    /// The file being shown, or None for a new, unsaved document.
    pub file: RefCell<Option<gio::File>>,
    pub view_mode: Cell<ViewMode>,
    /// True once the empty preview page has loaded and can receive content.
    pub preview_ready: Cell<bool>,
    /// True while LeafMD itself is loading the preview page, so that load is allowed through.
    preview_loading: Cell<bool>,
    render_scheduled: Cell<bool>,
    /// True while an input method (for typing Chinese, Japanese and so on) is
    /// part-way through composing text in the raw text. Enter then belongs
    /// to the input method, so lists leave it alone.
    composing: Cell<bool>,
    ignore_editor_scroll_until: Cell<Instant>,
    ignore_preview_scroll_until: Cell<Instant>,
    /// Briefly highlights the line you jumped to
    flash_tag: gtk::TextTag,
    /// A heading to scroll to once the next file has loaded (from a link like other.md#intro)
    pub pending_heading: RefCell<Option<String>>,
    /// True while a file has loaded but its formatted view is still being drawn.
    /// The document is shown once that's done, so it appears all at once.
    pub reveal_pending: Cell<bool>,
    /// True for a tab restored at startup whose formatted view hasn't been drawn,
    /// which waits until the tab is first selected. This keeps starting LeafMD
    /// quick with many tabs.
    pub preview_waiting: Cell<bool>,

    /// Tells LeafMD when the open file changes. Dropping it stops the watching.
    file_monitor: RefCell<Option<gio::FileMonitor>>,
    /// The version of the open file LeafMD last opened, saved or was told to
    /// ignore. Anything else on disk counts as another app's change.
    known_version: RefCell<Option<Version>>,
    /// The version the banner is telling the user about, or None if it's closed.
    banner_version: RefCell<Option<Version>>,
    /// True while waiting out FILE_CHECK_DELAY.
    file_check_scheduled: Cell<bool>,
    /// Goes up whenever the document's file changes, so a slow read of the previous
    /// file can tell its answer is out of date and throw it away.
    file_generation: Cell<u64>,
    /// How many saves are being written. While any are, the file on disk is about
    /// to change because of LeafMD itself, so checks wait.
    saves_running: Cell<u32>,
    /// True while "Replace the Other Version?" is open, so saving again can't open a second one.
    asking_to_replace: Cell<bool>,
}

impl Document {
    pub fn new(initial_view: ViewMode) -> Rc<Self> {
        // ---- Raw editor ----
        let buffer = sourceview5::Buffer::new(None);
        if let Some(language) = sourceview5::LanguageManager::default().language("markdown") {
            buffer.set_language(Some(&language));
        }
        buffer.set_highlight_syntax(true);
        let flash_tag = gtk::TextTag::new(Some("leafmd-flash"));
        buffer.tag_table().add(&flash_tag);

        let source_view = sourceview5::View::with_buffer(&buffer);
        source_view.set_show_line_numbers(true);
        source_view.set_monospace(true);
        source_view.set_wrap_mode(gtk::WrapMode::WordChar);
        source_view.set_tab_width(4);
        source_view.set_auto_indent(true);
        source_view.set_top_margin(12);
        source_view.set_bottom_margin(12);
        source_view.set_left_margin(8);
        source_view.set_right_margin(12);
        source_view.set_highlight_current_line(true);

        let source_scroll = gtk::ScrolledWindow::builder()
            .child(&source_view)
            .hexpand(true)
            .vexpand(true)
            .build();

        // ---- Formatted view ----
        let webview = webkit6::WebView::new();
        webview.set_hexpand(true);
        webview.set_vexpand(true);
        if let Some(settings) = WebViewExt::settings(&webview) {
            // Scripts inside documents never run. LeafMD can still update the page itself.
            settings.set_enable_javascript_markup(false);
            settings.set_enable_developer_extras(false);
        }
        // Lets LeafMD's own helper script (and only that) send messages back to the app
        if let Some(content_manager) = webview.user_content_manager() {
            content_manager.register_script_message_handler("leafmd", Some(SCRIPT_WORLD));
        }

        // ---- Searching the raw text ----
        let search_settings = sourceview5::SearchSettings::new();
        // After the last match, Enter goes back to the first
        search_settings.set_wrap_around(true);
        let search_context = sourceview5::SearchContext::new(&buffer, Some(&search_settings));
        search_context.set_highlight(true);

        // ---- Layout ----
        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .start_child(&source_scroll)
            .end_child(&webview)
            .shrink_start_child(false)
            .shrink_end_child(false)
            .build();

        let doc = Rc::new(Self {
            window: RefCell::new(Weak::new()),
            paned,
            source_scroll,
            source_view,
            buffer,
            webview,
            search_settings,
            search_context,
            find_state: RefCell::new(FindState::default()),
            file: RefCell::new(None),
            view_mode: Cell::new(initial_view),
            preview_ready: Cell::new(false),
            preview_loading: Cell::new(false),
            render_scheduled: Cell::new(false),
            composing: Cell::new(false),
            ignore_editor_scroll_until: Cell::new(Instant::now()),
            ignore_preview_scroll_until: Cell::new(Instant::now()),
            flash_tag,
            pending_heading: RefCell::new(None),
            reveal_pending: Cell::new(false),
            preview_waiting: Cell::new(false),
            file_monitor: RefCell::new(None),
            known_version: RefCell::new(None),
            banner_version: RefCell::new(None),
            file_check_scheduled: Cell::new(false),
            file_generation: Cell::new(0),
            saves_running: Cell::new(0),
            asking_to_replace: Cell::new(false),
        });
        doc.setup_signals();
        OPEN_DOCUMENTS.with(|open| open.borrow_mut().push(doc.clone()));
        doc
    }

    /// The document shown by a tab's widget.
    pub fn for_widget(widget: &gtk::Widget) -> Option<Rc<Self>> {
        OPEN_DOCUMENTS.with(|open| {
            open.borrow()
                .iter()
                .find(|doc| doc.paned.upcast_ref::<gtk::Widget>() == widget)
                .cloned()
        })
    }

    /// The document has been closed: lets go of it, which also stops it
    /// watching its file.
    pub fn forget(&self) {
        OPEN_DOCUMENTS.with(|open| open.borrow_mut().retain(|doc| !std::ptr::eq(&**doc, self)));
    }

    // =====================================================================
    // The link to the window
    // =====================================================================

    /// Points this document at the window now showing it.
    pub fn set_window(&self, window: Weak<Window>) {
        *self.window.borrow_mut() = window;
    }

    pub fn window(&self) -> Option<Rc<Window>> {
        self.window.borrow().upgrade()
    }

    /// Shows a short message in this document's window.
    pub fn toast(&self, message: &str) {
        if let Some(win) = self.window() {
            win.toast(message);
        }
    }

    /// The name or the dot for unsaved changes changed, so the window updates
    /// its title and this document's tab.
    fn title_changed(&self) {
        if let Some(win) = self.window() {
            win.update_title();
            win.update_tab(self);
        }
    }

    /// Moves a file to the top of the recent list that all windows share.
    fn remember_recent(&self, file: &gio::File) {
        if let Some(win) = self.window() {
            win.remember_recent(file);
        }
    }

    // =====================================================================
    // Signals
    // =====================================================================

    fn setup_signals(self: &Rc<Self>) {
        // Typing updates the formatted view after a short pause
        let weak = Rc::downgrade(self);
        self.buffer.connect_changed(move |_| {
            if let Some(doc) = weak.upgrade() {
                doc.schedule_render();
            }
        });

        // Show a dot in the title when there are unsaved changes
        let weak = Rc::downgrade(self);
        self.buffer.connect_modified_changed(move |_| {
            if let Some(doc) = weak.upgrade() {
                doc.title_changed();
                // The banner's words depend on whether there are unsaved changes
                doc.update_file_banner();
            }
        });

        // Once the preview page has loaded, fill it in
        let weak = Rc::downgrade(self);
        self.webview.connect_load_changed(move |_, event| {
            if event == webkit6::LoadEvent::Finished
                && let Some(doc) = weak.upgrade()
            {
                doc.preview_ready.set(true);
                doc.run_js(PREVIEW_SCRIPT);
                doc.tell_preview_view_mode();
                doc.update_style();
                doc.render_now();
                // A new file opened while the find bar is open: search it too
                if let Some(win) = doc.window()
                    && win.is_current(&doc)
                    && win.find.bar.is_search_mode()
                {
                    win.refresh_find();
                }
                // Opened from a link like other.md#part-two: scroll to that heading
                if let Some(heading) = doc.pending_heading.take() {
                    doc.scroll_to_heading(&heading);
                }
                // A file just opened: show it once the formatted view has its
                // content. Scripts run in order, so when this empty one finishes,
                // the document is already in the page.
                if doc.reveal_pending.get() {
                    let weak = Rc::downgrade(&doc);
                    doc.webview.evaluate_javascript(
                        "true",
                        Some(SCRIPT_WORLD),
                        None,
                        gio::Cancellable::NONE,
                        move |_| {
                            if let Some(win) = weak.upgrade().and_then(|doc| doc.window()) {
                                win.reveal_document();
                            }
                        },
                    );
                }
            }
        });

        // Messages from the formatted view: a double-click (jump) or a scroll
        if let Some(content_manager) = self.webview.user_content_manager() {
            let weak = Rc::downgrade(self);
            content_manager.connect_script_message_received(Some("leafmd"), move |_, value| {
                if let (Some(doc), Some(json)) = (weak.upgrade(), value.to_json(0)) {
                    doc.handle_preview_message(&json);
                }
            });
        }

        // Scrolling the raw text scrolls the formatted view to match (split view only)
        let weak = Rc::downgrade(self);
        self.source_scroll.vadjustment().connect_value_changed(move |_| {
            if let Some(doc) = weak.upgrade() {
                doc.sync_preview_to_editor();
            }
        });

        // Stop the formatted view from ever leaving LeafMD's own page
        let weak = Rc::downgrade(self);
        self.webview
            .connect_decide_policy(move |webview, decision, kind| {
                let Some(doc) = weak.upgrade() else {
                    return false;
                };
                doc.decide_navigation(webview, decision, kind)
            });

        // The find bar's count ("3 of 12") follows the cursor and changes to the text
        let weak = Rc::downgrade(self);
        self.search_context.connect_occurrences_count_notify(move |_| {
            if let Some(doc) = weak.upgrade() {
                doc.find_count_changed();
            }
        });
        let weak = Rc::downgrade(self);
        self.buffer.connect_mark_set(move |buffer, _, mark| {
            if *mark == buffer.get_insert()
                && let Some(doc) = weak.upgrade()
            {
                doc.find_count_changed();
            }
        });

        self.setup_typing_signals();
    }

    /// The number of matches or the selected match changed. Only the selected
    /// document's numbers belong in the find bar.
    fn find_count_changed(&self) {
        if let Some(win) = self.window()
            && win.is_current(self)
        {
            win.update_raw_find_count();
        }
    }

    // =====================================================================
    // Views
    // =====================================================================

    /// True when the formatted view is the only thing showing, so that's what
    /// gets searched. In Raw and Split view the raw text is searched.
    pub fn searching_preview(&self) -> bool {
        self.view_mode.get() == ViewMode::Preview
    }

    /// Shows the raw text, the formatted view, or both.
    pub fn apply_view(&self, mode: ViewMode) {
        self.view_mode.set(mode);
        let (show_source, show_preview) = match mode {
            ViewMode::Preview => (false, true),
            ViewMode::Source => (true, false),
            ViewMode::Split => (true, true),
        };
        self.source_scroll.set_visible(show_source);
        self.webview.set_visible(show_preview);

        if mode == ViewMode::Split {
            // Split down the middle. Before the window is drawn its width is 0,
            // so fall back to the window's planned width.
            let width = match self.paned.width() {
                0 => self.window().map_or(0, |win| match win.window.width() {
                    0 => win.window.default_size().0,
                    width => width,
                }),
                w => w,
            };
            self.paned.set_position(width / 2);
        }
        if show_preview {
            // The preview isn't updated while hidden, so catch it up now
            self.render_now();
        }
        self.tell_preview_view_mode();
    }

    /// Double-click on the formatted side only jumps to the raw text in split view.
    fn tell_preview_view_mode(&self) {
        if self.preview_ready.get() {
            let split = self.view_mode.get() == ViewMode::Split;
            self.run_js(&format!(
                "window.leafmd && window.leafmd.setSplitView({split});"
            ));
        }
    }

    // =====================================================================
    // Opening
    // =====================================================================

    /// Puts a file that has just been read into this document.
    pub fn show_file(self: &Rc<Self>, file: gio::File, text: String) {
        self.set_text(&text);
        *self.known_version.borrow_mut() = Some(Version::Contents(text.into_bytes()));
        self.set_file(Some(file));
    }

    /// True for an empty new document that nothing has been typed in.
    pub fn is_untouched(&self) -> bool {
        self.file.borrow().is_none() && !self.buffer.is_modified() && self.buffer.char_count() == 0
    }

    /// The folder of the open file for the title, with the home folder as `~`.
    /// Empty for a new, never-saved document.
    pub fn folder(&self) -> String {
        self.file
            .borrow()
            .as_ref()
            .and_then(|file| file.parent())
            .and_then(|dir| dir.path())
            .map(|path| match path.strip_prefix(glib::home_dir()) {
                Ok(rest) if rest.as_os_str().is_empty() => "~".to_string(),
                Ok(rest) => format!("~/{}", rest.display()),
                Err(_) => path.display().to_string(),
            })
            .unwrap_or_default()
    }

    // =====================================================================
    // Smarter typing: lists on Enter, pasting a link over text
    // =====================================================================

    /// Lists continue when Enter is pressed and end on an empty item, and
    /// pasting a web address over selected text makes a link. The rules for
    /// what to type are in typing.rs; this wires them to the raw text.
    fn setup_typing_signals(self: &Rc<Self>) {
        // Enter in a list item. This watches key presses rather than text
        // being inserted, so it only reacts to someone pressing Enter: pasted
        // text with new lines in it, and undo and redo, never gain markers.
        // The capture phase means it sees the key before the editor does, so
        // it can make the edit itself and stop the editor's ordinary new line.
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            let enter = matches!(key, gdk::Key::Return | gdk::Key::KP_Enter);
            // Shift+Enter (a plain new line) and Ctrl or Alt combinations go
            // to the editor as usual
            let held = gdk::ModifierType::SHIFT_MASK
                | gdk::ModifierType::CONTROL_MASK
                | gdk::ModifierType::ALT_MASK;
            if enter
                && !modifiers.intersects(held)
                && let Some(doc) = weak.upgrade()
                && doc.enter_in_list()
            {
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        self.source_view.add_controller(keys);

        // While an input method is composing, the editor shows the unfinished
        // text (the "preedit"), and Enter is how the user accepts it
        let weak = Rc::downgrade(self);
        self.source_view.connect_preedit_changed(move |_, preedit| {
            if let Some(doc) = weak.upgrade() {
                doc.composing.set(!preedit.is_empty());
            }
        });

        // Ctrl+V, Shift+Insert and the right-click menu's Paste all come
        // through this signal. Stopping it skips the editor's own paste, for
        // when LeafMD pastes the link itself.
        let weak = Rc::downgrade(self);
        self.source_view.connect_paste_clipboard(move |view| {
            if let Some(doc) = weak.upgrade()
                && doc.paste_over_selection()
            {
                view.stop_signal_emission_by_name("paste-clipboard");
            }
        });
    }

    /// Enter in the raw text. In a list item it starts the next item, or on
    /// an empty item ends the list, as one undo step. Returns false when
    /// Enter should just insert an ordinary new line.
    fn enter_in_list(&self) -> bool {
        let buffer = &self.buffer;
        if self.composing.get() || buffer.has_selection() {
            return false;
        }
        let cursor = buffer.iter_at_mark(&buffer.get_insert());
        let mut line_start = cursor;
        line_start.set_line_offset(0);
        let mut line_end = cursor;
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }
        let line = buffer.text(&line_start, &line_end, true);
        let action = typing::on_enter(&line, cursor.line_offset() as usize);
        if action == typing::Enter::Normal {
            return false;
        }
        // Lines in a code block are code, not markdown. Checked only now,
        // once the line looks like a list item, because it reads the whole
        // document above the cursor.
        let before = buffer.text(&buffer.start_iter(), &line_start, true);
        if typing::in_code_block(&before) {
            return false;
        }

        buffer.begin_user_action();
        match action {
            // Inserting at the cursor moves the cursor along with the text,
            // so it ends up after the new marker, ready to type
            typing::Enter::Continue(text) => buffer.insert_at_cursor(&text),
            // Clearing the line leaves the cursor on the now empty line
            typing::Enter::EndList => buffer.delete(&mut line_start, &mut line_end),
            typing::Enter::Normal => {}
        }
        buffer.end_user_action();
        self.source_view.scroll_mark_onscreen(&buffer.get_insert());
        true
    }

    /// A paste with text on one line selected: if the clipboard holds a web
    /// address, the selection becomes `[selection](address)`; anything else
    /// is pasted as usual. Returns true when LeafMD takes over the paste.
    fn paste_over_selection(self: &Rc<Self>) -> bool {
        let Some((start, end)) = self.buffer.selection_bounds() else {
            return false;
        };
        if start.line() != end.line() {
            return false;
        }
        let selected_range = (start.offset(), end.offset());
        let clipboard = WidgetExt::clipboard(&self.source_view);
        let doc = self.clone();
        // The clipboard may belong to another app, so its text arrives a
        // moment later rather than straight away
        glib::spawn_future_local(async move {
            let pasted = clipboard.read_text_future().await.ok().flatten();
            let buffer = &doc.buffer;
            // Only make the link if the same text is still selected
            let selection = buffer
                .selection_bounds()
                .filter(|(s, e)| (s.offset(), e.offset()) == selected_range);
            match (pasted, selection) {
                (Some(address), Some((mut s, mut e))) if typing::is_web_address(&address) => {
                    let selected = buffer.text(&s, &e, true);
                    let link = format!("[{selected}]({})", address.trim());
                    // Grouped so one Ctrl+Z brings back the selected text
                    buffer.begin_user_action();
                    buffer.delete(&mut s, &mut e);
                    buffer.insert(&mut s, &link);
                    buffer.end_user_action();
                    // After inserting, `s` points just past the link
                    buffer.place_cursor(&s);
                }
                // Not a web address (or the selection changed while
                // waiting): paste exactly as the editor would have
                _ => buffer.paste_clipboard(&clipboard, None, doc.source_view.is_editable()),
            }
            doc.source_view.scroll_mark_onscreen(&buffer.get_insert());
        });
        true
    }

    // =====================================================================
    // Formatting bar
    // =====================================================================

    /// Applies a formatting button (bold, bullet list, ...) to the selection,
    /// or to the word or line the cursor is on. One Ctrl+Z undoes it.
    pub fn apply_format(&self, name: &str) {
        let buffer = &self.buffer;
        let (sel_start, mut sel_end) = buffer.selection_bounds().unwrap_or_else(|| {
            let cursor = buffer.iter_at_mark(&buffer.get_insert());
            (cursor, cursor)
        });
        // Selecting whole lines usually ends at the very start of the next
        // line. Leave that next line out.
        if sel_end.line_offset() == 0 && sel_end.offset() > sel_start.offset() {
            sel_end.backward_char();
        }
        // The formatting code is given the whole lines the selection touches
        let mut block_start = sel_start;
        block_start.set_line_offset(0);
        let mut block_end = sel_end;
        if !block_end.ends_line() {
            block_end.forward_to_line_end();
        }
        let block = buffer.text(&block_start, &block_end, true).to_string();
        let base = block_start.offset();
        let start = (sel_start.offset() - base) as usize;
        let end = (sel_end.offset() - base) as usize;

        let edit = match name {
            "bold" => format::toggle_inline(&block, start, end, Inline::Bold),
            "italic" => format::toggle_inline(&block, start, end, Inline::Italic),
            "strikethrough" => format::toggle_inline(&block, start, end, Inline::Strikethrough),
            "code" => format::toggle_inline(&block, start, end, Inline::Code),
            "link" => format::link(&block, start, end),
            "heading-1" => format::toggle_line_style(&block, start, end, LineStyle::Heading(1)),
            "heading-2" => format::toggle_line_style(&block, start, end, LineStyle::Heading(2)),
            "heading-3" => format::toggle_line_style(&block, start, end, LineStyle::Heading(3)),
            "bullet-list" => format::toggle_line_style(&block, start, end, LineStyle::Bullet),
            "numbered-list" => format::toggle_line_style(&block, start, end, LineStyle::Numbered),
            "task-list" => format::toggle_line_style(&block, start, end, LineStyle::Task),
            "quote" => format::toggle_quote(&block, start, end),
            "code-block" => format::toggle_code_block(&block),
            "table" => {
                let mut before = block_start;
                let before_blank = !before.backward_line() || line_is_blank(&before);
                let mut after = block_end;
                let after_blank = !after.forward_line() || line_is_blank(&after);
                format::table(&block, before_blank, after_blank)
            }
            _ => return,
        };

        if edit.text != block {
            // Grouped so a single undo puts everything back
            buffer.begin_user_action();
            buffer.delete(&mut block_start, &mut block_end);
            buffer.insert(&mut block_start, &edit.text);
            buffer.end_user_action();
        }
        let new_start = buffer.iter_at_offset(base + edit.start as i32);
        let new_end = buffer.iter_at_offset(base + edit.end as i32);
        buffer.select_range(&new_end, &new_start);
        self.source_view.scroll_mark_onscreen(&buffer.get_insert());
        self.source_view.grab_focus();
    }

    // =====================================================================
    // Formatted view
    // =====================================================================

    /// Loads the empty preview page. Its base address is the document's folder,
    /// so relative image paths like `images/photo.png` work.
    pub fn reload_preview(&self) {
        let base = self
            .file
            .borrow()
            .as_ref()
            .and_then(|file| file.parent())
            .map(|dir| {
                let mut uri = dir.uri().to_string();
                if !uri.ends_with('/') {
                    uri.push('/');
                }
                uri
            });
        self.preview_ready.set(false);
        self.preview_loading.set(true);
        self.webview.load_html(PREVIEW_PAGE, base.as_deref());
    }

    /// The tab has been selected: draws the formatted view if it was waiting.
    pub fn draw_preview_if_waiting(&self) {
        if self.preview_waiting.replace(false) {
            self.reload_preview();
        }
    }

    fn schedule_render(self: &Rc<Self>) {
        if self.render_scheduled.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(RENDER_DELAY, move || {
            if let Some(doc) = weak.upgrade() {
                doc.render_scheduled.set(false);
                doc.render_now();
            }
        });
    }

    /// Converts the current text and swaps it into the preview page.
    /// Only the page's contents change, so the scroll position is kept.
    pub fn render_now(&self) {
        if !self.preview_ready.get() || !self.webview.is_visible() {
            return;
        }
        let html = render::markdown_to_safe_html(&self.text());
        let html_as_js_string = serde_json::to_string(&html).unwrap_or_default();
        // The new content replaces any find highlights, so they're redrawn after
        self.run_js(&format!(
            "document.getElementById('content').innerHTML = {html_as_js_string};
             window.leafmd && window.leafmd.findAgain();"
        ));
    }

    pub fn run_js(&self, script: &str) {
        self.webview.evaluate_javascript(
            script,
            Some(SCRIPT_WORLD),
            None,
            gio::Cancellable::NONE,
            |result| {
                if let Err(err) = result {
                    eprintln!("LeafMD: could not update the formatted view: {err}");
                }
            },
        );
    }

    fn handle_preview_message(&self, json: &str) {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        match message["kind"].as_str() {
            Some("jump") => {
                if let (Some(start), Some(end)) = (message["start"].as_i64(), message["end"].as_i64()) {
                    self.jump_to(
                        start as i32,
                        end as i32,
                        message["word"].as_str().unwrap_or(""),
                        message["occurrence"].as_u64().unwrap_or(0) as usize,
                    );
                }
            }
            Some("scroll") => {
                let now = Instant::now();
                if self.view_mode.get() != ViewMode::Split
                    || now < self.ignore_preview_scroll_until.get()
                {
                    return;
                }
                // About to scroll the editor: ignore the scroll events that causes
                self.ignore_editor_scroll_until.set(now + SYNC_QUIET);
                if message["bottom"].as_bool() == Some(true) {
                    let adjustment = self.source_scroll.vadjustment();
                    adjustment.set_value(adjustment.upper() - adjustment.page_size());
                } else if let Some(line) = message["line"].as_f64() {
                    self.scroll_editor_to_line(line);
                }
            }
            // The result of searching the formatted view
            Some("find") if self.searching_preview() => {
                if let Some(win) = self.window()
                    && win.is_current(self)
                {
                    win.find.show_count(
                        message["position"].as_i64().unwrap_or(0) as i32,
                        message["count"].as_i64().unwrap_or(0) as i32,
                    );
                }
            }
            _ => {}
        }
    }

    /// Double-clicking a word on the formatted side of split view: select that same
    /// word in the raw text and briefly highlight its line. In the formatted-only
    /// view, double-click just selects the word like in any reader.
    ///
    /// `start` and `end` are the source lines of the block that was clicked, and
    /// `occurrence` says which copy of the word it was (0 = first) within that block.
    fn jump_to(&self, start: i32, end: i32, word: &str, occurrence: usize) {
        if self.view_mode.get() != ViewMode::Split {
            return;
        }
        self.source_view.grab_focus();
        let Some(block_start) = self.buffer.iter_at_line((start - 1).max(0)) else {
            return;
        };
        let block_end = self
            .buffer
            .iter_at_line(end)
            .unwrap_or_else(|| self.buffer.end_iter());

        // Find the same copy of the word within the block's lines
        let mut found = None;
        if !word.is_empty() {
            let mut from = block_start;
            for _ in 0..=occurrence {
                match from.forward_search(word, gtk::TextSearchFlags::TEXT_ONLY, Some(&block_end)) {
                    Some((match_start, match_end)) => {
                        from = match_end;
                        found = Some((match_start, match_end));
                    }
                    None => break,
                }
            }
        }
        let cursor = match found {
            Some((match_start, match_end)) => {
                self.buffer.select_range(&match_start, &match_end);
                match_start
            }
            None => {
                self.buffer.place_cursor(&block_start);
                block_start
            }
        };
        self.flash_line(&cursor);

        // Wait until the split view has been laid out, then bring the spot into view
        let view = self.source_view.clone();
        let mark = self.buffer.get_insert();
        glib::idle_add_local_once(move || {
            view.scroll_to_mark(&mark, 0.0, true, 0.0, 0.3);
        });
    }

    /// Highlights a line in the accent colour for a moment.
    fn flash_line(&self, iter: &gtk::TextIter) {
        let mut color = adw::StyleManager::default().accent_color_rgba();
        color.set_alpha(0.25);
        self.flash_tag.set_paragraph_background_rgba(Some(&color));

        let (all_start, all_end) = self.buffer.bounds();
        self.buffer.remove_tag(&self.flash_tag, &all_start, &all_end);
        let mut line_start = *iter;
        line_start.set_line_offset(0);
        let mut line_end = line_start;
        line_end.forward_to_line_end();
        self.buffer.apply_tag(&self.flash_tag, &line_start, &line_end);

        let buffer = self.buffer.clone();
        let tag = self.flash_tag.clone();
        glib::timeout_add_local_once(Duration::from_millis(1200), move || {
            let (start, end) = buffer.bounds();
            buffer.remove_tag(&tag, &start, &end);
        });
    }

    /// Scrolls the raw text so the given (possibly fractional) line is at the top.
    fn scroll_editor_to_line(&self, line: f64) {
        let whole = line.floor().max(1.0);
        let Some(iter) = self.buffer.iter_at_line(whole as i32 - 1) else {
            return;
        };
        let (y, height) = self.source_view.line_yrange(&iter);
        let target = y as f64 + (line - whole).clamp(0.0, 1.0) * height as f64;
        let adjustment = self.source_scroll.vadjustment();
        let max = (adjustment.upper() - adjustment.page_size()).max(0.0);
        adjustment.set_value(target.clamp(0.0, max));
    }

    /// Called when the raw text scrolls: moves the formatted view to the same spot.
    fn sync_preview_to_editor(&self) {
        let now = Instant::now();
        if self.view_mode.get() != ViewMode::Split
            || !self.preview_ready.get()
            || now < self.ignore_editor_scroll_until.get()
        {
            return;
        }
        let adjustment = self.source_scroll.vadjustment();
        let top = adjustment.value();
        let at_bottom = top > 0.0 && top >= adjustment.upper() - adjustment.page_size() - 1.0;

        let (iter, line_top) = self.source_view.line_at_y(top as i32);
        let (_, height) = self.source_view.line_yrange(&iter);
        let fraction = if height > 0 {
            ((top - line_top as f64) / height as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let line = (iter.line() + 1) as f64 + fraction;

        // About to scroll the formatted view: ignore the scroll events that causes
        self.ignore_preview_scroll_until.set(now + SYNC_QUIET);
        self.run_js(&format!(
            "window.leafmd && window.leafmd.scrollToLine({line}, {at_bottom});"
        ));
    }

    /// Decides what happens when something tries to change the page in the formatted view.
    /// Returns true when LeafMD has handled it.
    fn decide_navigation(
        self: &Rc<Self>,
        webview: &webkit6::WebView,
        decision: &webkit6::PolicyDecision,
        kind: webkit6::PolicyDecisionType,
    ) -> bool {
        if kind != webkit6::PolicyDecisionType::NavigationAction {
            return false;
        }
        let Some(nav) = decision.downcast_ref::<webkit6::NavigationPolicyDecision>() else {
            return false;
        };
        let Some(action) = nav.navigation_action() else {
            return false;
        };

        // LeafMD loading its own page, or a reload of it: allow
        if self.preview_loading.replace(false)
            || action.navigation_type() == webkit6::NavigationType::Reload
        {
            return false;
        }

        let uri = action
            .request()
            .and_then(|request| request.uri())
            .map(|uri| uri.to_string())
            .unwrap_or_default();

        // A link to a heading on the same page, like [Install](#install): allow
        let current_page = webview.uri().map(|u| u.to_string()).unwrap_or_default();
        let without_heading = |u: &str| u.split('#').next().unwrap_or("").to_string();
        if uri.contains('#') && without_heading(&uri) == without_heading(&current_page) {
            return false;
        }

        // Anything else is blocked. Clicked links open in the right app instead,
        // and a file dropped onto the formatted view is opened in LeafMD.
        decision.ignore();
        if action.navigation_type() == webkit6::NavigationType::LinkClicked
            && uri.starts_with("file://")
        {
            // A link to another file, like [License](LICENSE)
            self.open_local_link(&uri);
        } else if action.navigation_type() == webkit6::NavigationType::LinkClicked {
            // Opens the link in your default browser. Inside Flatpak, GIO asks the
            // desktop to do it through a "portal". (GTK's UriLauncher was tried first,
            // but its requests failed with "The application launch failed", while
            // this route works.)
            // If it fails, say so instead of doing nothing.
            let context = WidgetExt::display(&self.webview).app_launch_context();
            let weak = Rc::downgrade(self);
            glib::spawn_future_local(async move {
                if let Err(err) = gio::AppInfo::launch_default_for_uri_future(&uri, Some(&context)).await
                    && let Some(doc) = weak.upgrade()
                {
                    eprintln!("LeafMD: could not open link: {err}");
                    doc.toast(&format!("Couldn't open the link: {}", err.message()));
                }
            });
        } else if uri.starts_with("file://")
            && let Some(win) = self.window()
        {
            win.open_file_here(gio::File::for_uri(&uri), None);
        }
        true
    }

    /// A clicked link to a file on this computer. Text files (markdown, LICENSE,
    /// .txt and so on) open in a new tab; anything else, like an image, PDF or
    /// folder, opens in its usual app.
    fn open_local_link(self: &Rc<Self>, uri: &str) {
        // A "#heading" part names a spot in the file, used after it opens
        let (file_uri, heading) = match uri.split_once('#') {
            Some((file_uri, heading)) => (file_uri, Some(heading.to_string())),
            None => (uri, None),
        };
        let file = gio::File::for_uri(file_uri);
        let doc = self.clone();
        glib::spawn_future_local(async move {
            let info = file
                .query_info_future(
                    "standard::type,standard::content-type",
                    gio::FileQueryInfoFlags::NONE,
                    glib::Priority::DEFAULT,
                )
                .await;
            let info = match info {
                Ok(info) => info,
                Err(_) => {
                    let name = file
                        .basename()
                        .map(|n| n.display().to_string())
                        .unwrap_or_default();
                    doc.toast(&format!("“{name}” doesn't exist"));
                    return;
                }
            };
            let is_text = info.file_type() == gio::FileType::Regular
                && info
                    .content_type()
                    .is_some_and(|kind| gio::content_type_is_a(&kind, "text/plain"));
            if is_text {
                // "#My%20Heading" in a link means the heading id "My Heading"
                let heading = heading.map(|heading| {
                    glib::Uri::unescape_string(&heading, None::<&str>)
                        .map(|unescaped| unescaped.to_string())
                        .unwrap_or(heading)
                });
                if let Some(win) = doc.window() {
                    win.open_file_here(file, heading);
                }
            } else {
                let context = WidgetExt::display(&doc.webview).app_launch_context();
                if let Err(err) =
                    gio::AppInfo::launch_default_for_uri_future(&file.uri(), Some(&context)).await
                {
                    doc.toast(&format!("Couldn't open the link: {}", err.message()));
                }
            }
        });
    }

    pub fn file_uri(&self) -> Option<String> {
        self.file.borrow().as_ref().map(|file| file.uri().to_string())
    }

    /// Scrolls the formatted view to the heading with this id (like "part-two").
    pub fn scroll_to_heading(&self, heading: &str) {
        let id = serde_json::to_string(heading).unwrap_or_default();
        self.run_js(&format!("document.getElementById({id})?.scrollIntoView();"));
    }

    /// Saves to the current file, or asks where to save a new document.
    /// Returns true if it was saved.
    pub async fn save(self: &Rc<Self>) -> bool {
        let file = self.file.borrow().clone();
        match file {
            Some(file) => {
                if !self.ok_to_replace(&file).await {
                    return false;
                }
                let saved = self.write_to(&file).await;
                // Saving counts as recent use, and puts the file back on the
                // list if Clear Recent Files emptied it while the file was open
                if saved {
                    self.remember_recent(&file);
                }
                saved
            }
            None => self.save_as().await,
        }
    }

    pub async fn save_as(self: &Rc<Self>) -> bool {
        let dialog = gtk::FileDialog::builder()
            .title("Save Markdown File")
            .modal(true)
            .initial_name(self.save_name())
            .filters(&file_filters())
            .build();
        let parent = self.window().map(|win| win.window.clone());
        let Ok(file) = dialog.save_future(parent.as_ref()).await else {
            return false;
        };
        if !self.write_to(&file).await {
            return false;
        }
        self.set_file(Some(file.clone()));
        self.remember_recent(&file);
        true
    }

    async fn write_to(self: &Rc<Self>, file: &gio::File) -> bool {
        let text = self.text();
        self.saves_running.set(self.saves_running.get() + 1);
        // GIO writes to a temporary file first and then swaps it in, so a crash
        // or full disk part-way through can't leave a half-written file.
        let result = file
            .replace_contents_future(
                text.clone().into_bytes(),
                None,
                false,
                gio::FileCreateFlags::NONE,
            )
            .await;
        self.saves_running.set(self.saves_running.get() - 1);
        let saved = match result {
            Ok(_) => {
                // Only mark as saved if nothing was typed while saving
                if self.text() == text {
                    self.buffer.set_modified(false);
                }
                // What was written, not what's in the editor now, which may
                // have gained more typing during the save
                *self.known_version.borrow_mut() = Some(Version::Contents(text.into_bytes()));
                self.close_file_banner();
                self.toast("Saved");
                true
            }
            Err((_, err)) => {
                self.toast(&format!("Could not save: {}", err.message()));
                false
            }
        };
        // Checks wait during a save, so look now in case another app changed
        // the file during it
        self.schedule_file_check();
        saved
    }

    // =====================================================================
    // Noticing when another app changes the open file
    // =====================================================================

    /// Starts watching this document's file, and forgets anything known
    /// about the previous one. Fails quietly: on a drive that can't be watched,
    /// LeafMD still checks when returning to the window and before saving.
    fn watch_file(self: &Rc<Self>) {
        self.file_generation.set(self.file_generation.get() + 1);
        self.close_file_banner();
        let monitor = self.file.borrow().as_ref().and_then(|file| {
            file.monitor_file(gio::FileMonitorFlags::NONE, gio::Cancellable::NONE)
                .ok()
        });
        if let Some(monitor) = &monitor {
            let weak = Rc::downgrade(self);
            monitor.connect_changed(move |_, _, _, _| {
                if let Some(doc) = weak.upgrade() {
                    doc.schedule_file_check();
                }
            });
        }
        *self.file_monitor.borrow_mut() = monitor;
    }

    /// Checks the file after FILE_CHECK_DELAY. Calling this again while waiting
    /// doesn't restart the wait, so an app that writes constantly can't hold the
    /// check off forever.
    pub fn schedule_file_check(self: &Rc<Self>) {
        if self.file_check_scheduled.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(FILE_CHECK_DELAY, move || {
            if let Some(doc) = weak.upgrade() {
                doc.file_check_scheduled.set(false);
                glib::spawn_future_local(async move {
                    doc.check_file().await;
                });
            }
        });
    }

    /// Reads the open file, and opens or closes the banner to match.
    async fn check_file(self: &Rc<Self>) {
        // write_to runs another check when it finishes
        if self.saves_running.get() > 0 {
            return;
        }
        let Some(file) = self.file.borrow().clone() else {
            return;
        };
        let generation = self.file_generation.get();
        let on_disk = read_version(&file).await;
        // While reading, the document may have moved to another file or started a save
        if generation != self.file_generation.get() || self.saves_running.get() > 0 {
            return;
        }
        let comparison = match self.known_version.borrow().as_ref() {
            Some(known) => file_change::compare(known, on_disk.as_ref()),
            None => return,
        };
        match comparison {
            Comparison::Same => self.close_file_banner(),
            Comparison::Changed | Comparison::Deleted => {
                *self.banner_version.borrow_mut() = on_disk;
                self.update_file_banner();
            }
            Comparison::Unknown => {}
        }
    }

    /// Makes the banner match what it's telling the user about and whether there
    /// are unsaved changes.
    pub fn update_file_banner(&self) {
        let Some(win) = self.window() else { return };
        if !win.is_current(self) {
            return;
        }
        match self.banner_version.borrow().as_ref() {
            Some(version) => {
                let (message, action, loses_work) = file_change::banner_text(
                    &self.display_name(),
                    version,
                    self.buffer.is_modified(),
                );
                win.file_banner.show(&message, action, loses_work);
            }
            None => win.file_banner.hide(),
        }
    }

    pub fn close_file_banner(&self) {
        *self.banner_version.borrow_mut() = None;
        self.update_file_banner();
    }

    /// Ignore: the version the banner was about now counts as seen, so it isn't
    /// mentioned again and saving replaces it without asking. Using that version
    /// rather than reading the file again means a newer change still gets a banner.
    pub fn ignore_file_change(self: &Rc<Self>) {
        if let Some(version) = self.banner_version.borrow_mut().take() {
            *self.known_version.borrow_mut() = Some(version);
        }
        self.update_file_banner();
        self.schedule_file_check();
    }

    /// The banner's main button: Reload, Discard and Reload, or Save Again.
    pub async fn banner_action(self: &Rc<Self>) {
        let deleted = self.banner_version.borrow().as_ref() == Some(&Version::Missing);
        if deleted {
            self.save().await;
        } else {
            self.reload_file().await;
        }
    }

    /// Replaces the document with the newest version of the file. If that can't be
    /// done, the document and the banner stay and a message says why.
    async fn reload_file(self: &Rc<Self>) {
        let Some(file) = self.file.borrow().clone() else {
            return;
        };
        let generation = self.file_generation.get();
        let result = file.load_contents_future().await;
        if generation != self.file_generation.get() {
            return;
        }
        let bytes = match result {
            Ok((bytes, _)) => bytes.to_vec(),
            Err(err) => {
                self.toast(&format!("Could not reload the file: {}", err.message()));
                self.schedule_file_check();
                return;
            }
        };
        let Ok(text) = String::from_utf8(bytes) else {
            self.toast("This file isn't plain text, so LeafMD can't open it");
            return;
        };
        self.replace_text_keeping_place(&text);
        *self.known_version.borrow_mut() = Some(Version::Contents(text.into_bytes()));
        self.close_file_banner();
    }

    /// Before saving: if another app changed the file and the user hasn't chosen
    /// to ignore that, asks before replacing it. Returns true if it's fine to save.
    /// A deleted file has nothing to replace, and if the file can't be read, the
    /// save itself will report any problem.
    async fn ok_to_replace(self: &Rc<Self>, file: &gio::File) -> bool {
        if self.asking_to_replace.get() {
            return false;
        }
        let on_disk = read_version(file).await;
        let changed = match self.known_version.borrow().as_ref() {
            Some(known) => file_change::compare(known, on_disk.as_ref()) == Comparison::Changed,
            None => false,
        };
        if !changed {
            return true;
        }
        let question = Question::new(
            "Replace the Other Version?",
            &format!(
                "“{}” was changed by another app since LeafMD last opened or saved it. Saving will replace that version.",
                self.display_name()
            ),
            &[
                Answer { id: "save", label: "_Save Anyway", loses_work: true },
                Answer { id: "cancel", label: "_Cancel", loses_work: false },
            ],
            "cancel",
            "cancel",
        );
        self.asking_to_replace.set(true);
        let parent = self.window().map(|win| win.window.clone());
        let response = question.ask(parent.as_ref()).await;
        self.asking_to_replace.set(false);
        response.as_str() == "save"
    }

    /// Runs `then` straight away if there are no unsaved changes. Otherwise asks
    /// the user to save, discard or cancel, and only runs `then` if they didn't cancel.
    pub fn confirm_discard(self: &Rc<Self>, then: impl FnOnce(&Rc<Self>) + 'static) {
        if !self.buffer.is_modified() {
            then(self);
            return;
        }
        let doc = self.clone();
        glib::spawn_future_local(async move {
            if doc.ask_to_save().await {
                then(&doc);
            }
        });
    }

    /// Asks whether to save before closing, if there are unsaved changes. Returns
    /// true if it's fine to carry on, and false if the user cancelled or the
    /// save failed.
    pub async fn ask_to_save(self: &Rc<Self>) -> bool {
        if !self.buffer.is_modified() {
            return true;
        }
        // Saving a document that has no file yet opens a dialog, and "…" says so
        let save_label = match self.file.borrow().as_ref() {
            Some(_) => "_Save",
            None => "Save _As…",
        };
        let question = save_changes_question(
            &format!(
                "Save changes to document “{}” before closing?",
                self.display_name()
            ),
            "If you don't save, your changes will be permanently lost.",
            save_label,
        );
        let parent = self.window().map(|win| win.window.clone());
        match question.ask(parent.as_ref()).await.as_str() {
            "discard" => true,
            // Only carry on if the save actually worked
            "save" => self.save().await,
            _ => false,
        }
    }

    pub fn set_file(self: &Rc<Self>, file: Option<gio::File>) {
        if file.is_none() {
            *self.known_version.borrow_mut() = None;
        }
        *self.file.borrow_mut() = file;
        self.title_changed();
        if !self.preview_waiting.get() {
            self.reload_preview();
        }
        self.watch_file();
    }

    /// Replaces the editor text without adding it to the undo history.
    pub fn set_text(&self, text: &str) {
        self.buffer.begin_irreversible_action();
        self.buffer.set_text(text);
        self.buffer.end_irreversible_action();
        self.buffer.set_modified(false);
        self.buffer.place_cursor(&self.buffer.start_iter());
        self.source_view
            .scroll_to_iter(&mut self.buffer.start_iter(), 0.0, false, 0.0, 0.0);
    }

    /// Like set_text, but for reloading the same document: the cursor and scroll
    /// position stay where they were instead of going back to the top. The
    /// formatted view redraws through its normal update, which keeps its position.
    fn replace_text_keeping_place(self: &Rc<Self>, text: &str) {
        let cursor = self.buffer.iter_at_mark(&self.buffer.get_insert());
        let (line, column) = (cursor.line(), cursor.line_offset());
        let scroll = self.source_scroll.vadjustment().value();

        self.buffer.begin_irreversible_action();
        self.buffer.set_text(text);
        self.buffer.end_irreversible_action();
        self.buffer.set_modified(false);

        // If the file got shorter, the cursor goes to the end of the document or line
        let place = match self.buffer.iter_at_line(line) {
            Some(mut place) => {
                let mut line_end = place;
                if !line_end.ends_line() {
                    line_end.forward_to_line_end();
                }
                place.set_line_offset(column.min(line_end.line_offset()));
                place
            }
            None => self.buffer.end_iter(),
        };
        self.buffer.place_cursor(&place);

        // GTK works out the new text's height over the next moments, and until
        // it has, the scroll position would be cut short
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(Duration::from_millis(50), move || {
            if let Some(doc) = weak.upgrade() {
                doc.source_scroll.vadjustment().set_value(scroll);
            }
        });
    }

    pub fn text(&self) -> String {
        let (start, end) = self.buffer.bounds();
        self.buffer.text(&start, &end, false).to_string()
    }

    pub fn display_name(&self) -> String {
        self.file
            .borrow()
            .as_ref()
            .and_then(|file| file.basename())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Untitled".into())
    }

    pub fn save_name(&self) -> String {
        match self.file.borrow().as_ref() {
            Some(_) => self.display_name(),
            None => "Untitled.md".into(),
        }
    }

    /// Matches the editor colours and the formatted view to the system's light or dark style.
    pub fn update_style(&self) {
        let dark = adw::StyleManager::default().is_dark();
        let scheme = sourceview5::StyleSchemeManager::default()
            .scheme(if dark { "Adwaita-dark" } else { "Adwaita" });
        self.buffer.set_style_scheme(scheme.as_ref());

        let background = if dark { "#1e1e1e" } else { "#ffffff" };
        if let Ok(rgba) = gdk::RGBA::parse(background) {
            self.webview.set_background_color(&rgba);
        }
        if self.preview_ready.get() {
            let theme = if dark { "dark" } else { "light" };
            self.run_js(&format!(
                "document.documentElement.dataset.theme = '{theme}';"
            ));
        }
    }
}

/// Whether the line starting at this point has nothing but spaces on it.
fn line_is_blank(line_start: &gtk::TextIter) -> bool {
    let mut line_end = *line_start;
    if !line_end.ends_line() {
        line_end.forward_to_line_end();
    }
    line_start
        .buffer()
        .text(line_start, &line_end, true)
        .trim()
        .is_empty()
}

/// Reads a file as it is on disk now. None means it couldn't be read for some
/// reason other than not being there.
async fn read_version(file: &gio::File) -> Option<Version> {
    match file.load_contents_future().await {
        Ok((bytes, _)) => Some(Version::Contents(bytes.to_vec())),
        Err(err) if err.matches(gio::IOErrorEnum::NotFound) => Some(Version::Missing),
        Err(_) => None,
    }
}

/// The question about unsaved changes. Save is the answer Enter gives.
pub fn save_changes_question(heading: &str, body: &str, save_label: &'static str) -> Question {
    Question::new(
        heading,
        body,
        &[
            Answer { id: "discard", label: "Close _without Saving", loses_work: true },
            Answer { id: "cancel", label: "_Cancel", loses_work: false },
            Answer { id: "save", label: save_label, loses_work: false },
        ],
        "save",
        "cancel",
    )
}

//! The LeafMD window: the toolbar, the formatting bar, the raw editor, the
//! formatted view, and opening and saving files.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;
use webkit6::prelude::*;

use crate::find::FindBar;
use crate::format::{self, Inline, LineStyle};
use crate::render;
use crate::typing;
use crate::state::{self, SavedState, ViewMode};

/// The empty page (styles only) that the formatted view is drawn into.
const PREVIEW_PAGE: &str = include_str!("preview.html");

/// Helpers for click-to-jump and synced scrolling, run inside the formatted view.
const PREVIEW_SCRIPT: &str = include_str!("preview.js");

/// LeafMD's private script "world" in the formatted view. Code and messages here
/// are kept apart from the document itself.
const SCRIPT_WORLD: &str = "leafmd";

/// When a file is opening, the longest LeafMD waits for the formatted view to
/// finish drawing before showing the document anyway.
const REVEAL_TIMEOUT: Duration = Duration::from_secs(1);

/// How long to wait after the last keystroke before updating the formatted view.
const RENDER_DELAY: Duration = Duration::from_millis(150);

/// After LeafMD scrolls one side to match the other, scroll events from that
/// side are ignored for this long, so the two sides don't keep nudging each other.
const SYNC_QUIET: Duration = Duration::from_millis(250);

/// The description in the About window.
const ABOUT_TEXT: &str = "LeafMD is a markdown viewer and editor that keeps things simple.

Read a document cleanly formatted, work directly in the raw text, or see both side by side and watch your changes appear as you type.

Rather than trying to do everything, LeafMD focuses on doing the essentials well. There are no plugins to manage, panels to arrange or settings to tune. It is just a fast, dependable app that feels obvious from the moment you open it, so you can spend your time on your writing instead of your tools.

LeafMD was created with the help of AI.";

/// Actions that only make sense while a document is open.
const DOCUMENT_ACTIONS: [&str; 8] = [
    "close-document",
    "save",
    "save-as",
    "view",
    "find",
    "find-replace",
    "find-next",
    "find-previous",
];

pub struct Window {
    pub window: adw::ApplicationWindow,
    title: adw::WindowTitle,
    toasts: adw::ToastOverlay,
    stack: gtk::Stack,
    paned: gtk::Paned,
    view_switcher: adw::ToggleGroup,
    /// The row of formatting buttons above the raw text (Raw and Split view only)
    format_bar: gtk::ScrolledWindow,
    /// The find and replace bar (Ctrl+F, Ctrl+H)
    find: FindBar,
    /// The magnifying glass in the header bar that opens the find bar
    find_button: gtk::ToggleButton,
    /// The list under the arrow next to Open
    recent_menu: gio::Menu,
    /// Recently opened files as file:// addresses, newest first
    recent: RefCell<Vec<String>>,
    source_scroll: gtk::ScrolledWindow,
    source_view: sourceview5::View,
    buffer: sourceview5::Buffer,
    webview: webkit6::WebView,

    /// The file being shown, or None for a new, unsaved document.
    file: RefCell<Option<gio::File>>,
    view_mode: Cell<ViewMode>,
    /// True once the empty preview page has loaded and can receive content.
    preview_ready: Cell<bool>,
    /// True while LeafMD itself is loading the preview page, so that load is allowed through.
    preview_loading: Cell<bool>,
    render_scheduled: Cell<bool>,
    /// Set after the user has answered "discard" or "save" when closing.
    close_confirmed: Cell<bool>,
    /// True while an input method (for typing Chinese, Japanese and so on) is
    /// part-way through composing text in the raw text. Enter then belongs
    /// to the input method, so lists leave it alone.
    composing: Cell<bool>,
    ignore_editor_scroll_until: Cell<Instant>,
    ignore_preview_scroll_until: Cell<Instant>,
    /// Briefly highlights the line you jumped to
    flash_tag: gtk::TextTag,
    /// A heading to scroll to once the next file has loaded (from a link like other.md#intro)
    pending_heading: RefCell<Option<String>>,
    /// True while a file has loaded but its formatted view is still being drawn.
    /// The document is shown once that's done, so it appears all at once.
    reveal_pending: Cell<bool>,
}

impl Window {
    pub fn new(app: &adw::Application) -> Rc<Self> {
        let saved = SavedState::load();

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("LeafMD")
            .default_width(saved.width)
            .default_height(saved.height)
            .width_request(360)
            .height_request(300)
            .build();
        if saved.maximized {
            window.maximize();
        }

        // ---- Header bar ----
        let title = adw::WindowTitle::new("LeafMD", "");
        let header = adw::HeaderBar::new();
        header.set_title_widget(Some(&title));

        // "Open" with a small arrow beside it that lists recent files. A word instead
        // of an icon, because folder icons vary a lot between icon themes.
        let recent_menu = gio::Menu::new();
        header.pack_start(
            &adw::SplitButton::builder()
                .label("Open")
                .tooltip_text("Open a file (Ctrl+O)")
                .dropdown_tooltip("Recent files")
                .action_name("win.open")
                .menu_model(&recent_menu)
                .build(),
        );
        // LeafMD's own floppy disk icon, because the Adwaita theme LeafMD uses
        // (see main.rs) draws Save as a downward arrow.
        header.pack_start(
            &gtk::Button::builder()
                .icon_name("leafmd-save-symbolic")
                .tooltip_text("Save (Ctrl+S)")
                .action_name("win.save")
                .build(),
        );

        // pack_end fills from the right edge inwards: the menu, then the view buttons.
        // The menu is grouped into sections (shown with dividers); shortcuts appear automatically.
        let file_section = gio::Menu::new();
        file_section.append(Some("New"), Some("win.new"));
        file_section.append(Some("Open…"), Some("win.open"));
        file_section.append(Some("Close Document"), Some("win.close-document"));
        let save_section = gio::Menu::new();
        save_section.append(Some("Save"), Some("win.save"));
        save_section.append(Some("Save As…"), Some("win.save-as"));
        let find_section = gio::Menu::new();
        find_section.append(Some("Find…"), Some("win.find"));
        find_section.append(Some("Find and Replace…"), Some("win.find-replace"));
        let app_section = gio::Menu::new();
        app_section.append(Some("Help"), Some("win.help"));
        app_section.append(Some("Markdown Cheatsheet"), Some("win.cheatsheet"));
        app_section.append(Some("Keyboard Shortcuts"), Some("win.shortcuts"));
        app_section.append(Some("About LeafMD"), Some("win.about"));
        let menu = gio::Menu::new();
        menu.append_section(None, &file_section);
        menu.append_section(None, &save_section);
        menu.append_section(None, &find_section);
        menu.append_section(None, &app_section);
        header.pack_end(
            &gtk::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .menu_model(&menu)
                .primary(true)
                .tooltip_text("Main Menu")
                .build(),
        );
        // GNOME's rounded switcher with LeafMD's own icons. Each toggle is named
        // after its view ("preview", "source", "split"); hovering shows the name.
        let view_switcher = adw::ToggleGroup::new();
        for (mode, icon, tooltip) in [
            (ViewMode::Preview, "leafmd-view-formatted-symbolic", "Formatted view (Ctrl+1)"),
            (ViewMode::Source, "leafmd-view-raw-symbolic", "Raw text (Ctrl+2)"),
            (ViewMode::Split, "leafmd-view-split-symbolic", "Split view (Ctrl+3)"),
        ] {
            view_switcher.add(
                adw::Toggle::builder()
                    .name(mode.as_str())
                    .icon_name(icon)
                    .tooltip(tooltip)
                    .build(),
            );
        }
        view_switcher.set_active_name(Some(saved.view.as_str()));
        // A magnifying glass between the views and the menu opens the find bar.
        // It stays pressed in while the bar is open.
        let find_button = gtk::ToggleButton::builder()
            .icon_name("edit-find-symbolic")
            .tooltip_text("Find (Ctrl+F)")
            .build();
        header.pack_end(&find_button);
        header.pack_end(&view_switcher);

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

        // ---- Formatting bar ----
        let format_bar = build_format_bar();

        // ---- Find and replace ----
        let find = FindBar::new(&buffer);

        // ---- Layout ----
        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .start_child(&source_scroll)
            .end_child(&webview)
            .shrink_start_child(false)
            .shrink_end_child(false)
            .build();

        let open_button = gtk::Button::builder()
            .label("Open File…")
            .action_name("win.open")
            .halign(gtk::Align::Center)
            .build();
        open_button.add_css_class("pill");
        open_button.add_css_class("suggested-action");
        let empty_page = adw::StatusPage::builder()
            .icon_name("text-x-generic-symbolic")
            .title("No Document Open")
            .description("Open a markdown file, or drop one here")
            .child(&open_button)
            .build();

        // "loading" is a blank page shown while a file opens, so the
        // "No Document Open" page doesn't flash up first
        let stack = gtk::Stack::builder()
            .transition_type(gtk::StackTransitionType::Crossfade)
            .transition_duration(150)
            .build();
        stack.add_named(&empty_page, Some("empty"));
        stack.add_named(&adw::Bin::new(), Some("loading"));
        stack.add_named(&paned, Some("document"));

        let toasts = adw::ToastOverlay::new();
        toasts.set_child(Some(&stack));

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header);
        toolbar_view.add_top_bar(&format_bar);
        toolbar_view.add_top_bar(&find.bar);
        toolbar_view.set_content(Some(&toasts));
        window.set_content(Some(&toolbar_view));

        let win = Rc::new(Self {
            window,
            title,
            toasts,
            stack,
            paned,
            view_switcher,
            format_bar,
            find,
            find_button,
            recent_menu,
            recent: RefCell::new(state::load_recent()),
            source_scroll,
            source_view,
            buffer,
            webview,
            file: RefCell::new(None),
            view_mode: Cell::new(saved.view),
            preview_ready: Cell::new(false),
            preview_loading: Cell::new(false),
            render_scheduled: Cell::new(false),
            close_confirmed: Cell::new(false),
            composing: Cell::new(false),
            ignore_editor_scroll_until: Cell::new(Instant::now()),
            ignore_preview_scroll_until: Cell::new(Instant::now()),
            flash_tag,
            pending_heading: RefCell::new(None),
            reveal_pending: Cell::new(false),
        });

        win.setup_actions(saved.view);
        win.setup_signals();
        win.set_document_actions_enabled(false);
        win.apply_view(saved.view);
        win.update_style();
        win.update_title();
        win.update_recent_menu();
        win
    }

    // =====================================================================
    // Setup
    // =====================================================================

    fn setup_actions(self: &Rc<Self>, initial_view: ViewMode) {
        let add = |name: &str, run: fn(&Rc<Window>)| {
            let action = gio::SimpleAction::new(name, None);
            let weak = Rc::downgrade(self);
            action.connect_activate(move |_, _| {
                if let Some(win) = weak.upgrade() {
                    run(&win);
                }
            });
            self.window.add_action(&action);
        };
        add("new", |win| win.ask_new());
        add("open", |win| win.ask_open());
        add("close-document", |win| win.ask_close_document());
        add("clear-recent", |win| win.clear_recent());

        // Opening a recent file: the menu item carries the file's address
        let open_recent = gio::SimpleAction::new("open-recent", Some(glib::VariantTy::STRING));
        let weak = Rc::downgrade(self);
        open_recent.connect_activate(move |_, param| {
            if let (Some(win), Some(uri)) = (weak.upgrade(), param.and_then(|p| p.str())) {
                win.ask_open_file(gio::File::for_uri(uri));
            }
        });
        self.window.add_action(&open_recent);
        add("save", |win| {
            let win = win.clone();
            glib::spawn_future_local(async move {
                win.save().await;
            });
        });
        add("save-as", |win| {
            let win = win.clone();
            glib::spawn_future_local(async move {
                win.save_as().await;
            });
        });
        add("help", |win| win.show_help());
        add("cheatsheet", |win| win.show_cheatsheet());

        // The formatting buttons and their shortcuts: the target names the format
        let format = gio::SimpleAction::new("format", Some(glib::VariantTy::STRING));
        let weak = Rc::downgrade(self);
        format.connect_activate(move |_, param| {
            if let (Some(win), Some(name)) = (weak.upgrade(), param.and_then(|p| p.str())) {
                win.apply_format(name);
            }
        });
        self.window.add_action(&format);
        add("shortcuts", |win| win.show_shortcuts());
        add("about", |win| win.show_about());

        // Find and replace
        add("find", |win| win.open_find(false));
        add("find-replace", |win| win.open_find(true));
        add("find-next", |win| win.find_step(1));
        add("find-previous", |win| win.find_step(-1));
        add("replace", |win| win.replace_one());
        add("replace-all", |win| win.replace_all());
        // The two search options: tick boxes in the find bar's menu
        for name in ["find-match-case", "find-whole-words"] {
            let option = gio::SimpleAction::new_stateful(name, None, &false.to_variant());
            let weak = Rc::downgrade(self);
            option.connect_activate(move |option, _| {
                let on = !option.state().and_then(|s| s.get::<bool>()).unwrap_or(false);
                option.set_state(&on.to_variant());
                if let Some(win) = weak.upgrade() {
                    win.find_options_changed();
                }
            });
            self.window.add_action(&option);
        }

        // "view" remembers which of the three views is active
        let view = gio::SimpleAction::new_stateful(
            "view",
            Some(glib::VariantTy::STRING),
            &initial_view.as_str().to_variant(),
        );
        let weak = Rc::downgrade(self);
        view.connect_activate(move |action, param| {
            let Some(win) = weak.upgrade() else { return };
            let Some(mode) = param.and_then(|p| p.str()).and_then(ViewMode::parse) else {
                return;
            };
            action.set_state(&mode.as_str().to_variant());
            win.apply_view(mode);
            // Keep the switcher in step when the view changes by keyboard shortcut
            if win.view_switcher.active_name().as_deref() != Some(mode.as_str()) {
                win.view_switcher.set_active_name(Some(mode.as_str()));
            }
        });
        self.window.add_action(&view);

        // Clicking the switcher changes the view through the same "view" action
        let weak = Rc::downgrade(self);
        self.view_switcher.connect_active_name_notify(move |switcher| {
            let (Some(win), Some(name)) = (weak.upgrade(), switcher.active_name()) else {
                return;
            };
            if Some(win.view_mode.get()) != ViewMode::parse(&name) {
                win.action("view").activate(Some(&name.to_variant()));
            }
        });
    }

    fn setup_signals(self: &Rc<Self>) {
        // Typing updates the formatted view after a short pause
        let weak = Rc::downgrade(self);
        self.buffer.connect_changed(move |_| {
            if let Some(win) = weak.upgrade() {
                win.schedule_render();
            }
        });

        // Show a dot in the title when there are unsaved changes
        let weak = Rc::downgrade(self);
        self.buffer.connect_modified_changed(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_title();
            }
        });

        // Once the preview page has loaded, fill it in
        let weak = Rc::downgrade(self);
        self.webview.connect_load_changed(move |_, event| {
            if event == webkit6::LoadEvent::Finished
                && let Some(win) = weak.upgrade()
            {
                win.preview_ready.set(true);
                win.run_js(PREVIEW_SCRIPT);
                win.tell_preview_view_mode();
                win.update_style();
                win.render_now();
                // A new file opened while the find bar is open: search it too
                if win.find.bar.is_search_mode() {
                    win.refresh_find();
                }
                // Opened from a link like other.md#part-two: scroll to that heading
                if let Some(heading) = win.pending_heading.take() {
                    win.scroll_to_heading(&heading);
                }
                // A file just opened: show it once the formatted view has its
                // content. Scripts run in order, so when this empty one finishes,
                // the document is already in the page.
                if win.reveal_pending.get() {
                    let weak = Rc::downgrade(&win);
                    win.webview.evaluate_javascript(
                        "true",
                        Some(SCRIPT_WORLD),
                        None,
                        gio::Cancellable::NONE,
                        move |_| {
                            if let Some(win) = weak.upgrade() {
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
                if let (Some(win), Some(json)) = (weak.upgrade(), value.to_json(0)) {
                    win.handle_preview_message(&json);
                }
            });
        }

        // Scrolling the raw text scrolls the formatted view to match (split view only)
        let weak = Rc::downgrade(self);
        self.source_scroll.vadjustment().connect_value_changed(move |_| {
            if let Some(win) = weak.upgrade() {
                win.sync_preview_to_editor();
            }
        });

        // Stop the formatted view from ever leaving LeafMD's own page
        let weak = Rc::downgrade(self);
        self.webview
            .connect_decide_policy(move |webview, decision, kind| {
                let Some(win) = weak.upgrade() else {
                    return false;
                };
                win.decide_navigation(webview, decision, kind)
            });

        // Follow the system's light or dark style
        let weak = Rc::downgrade(self);
        adw::StyleManager::default().connect_dark_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_style();
            }
        });

        // Dropping a file anywhere on the window opens it
        let drop = gtk::DropTarget::new(gio::File::static_type(), gdk::DragAction::COPY);
        let weak = Rc::downgrade(self);
        drop.connect_drop(move |_, value, _, _| {
            let (Some(win), Ok(file)) = (weak.upgrade(), value.get::<gio::File>()) else {
                return false;
            };
            win.ask_open_file(file);
            true
        });
        self.window.add_controller(drop);

        // Switching to this window: refresh the recent list, which another
        // LeafMD window may have changed
        let weak = Rc::downgrade(self);
        self.window.connect_is_active_notify(move |window| {
            if window.is_active()
                && let Some(win) = weak.upgrade()
            {
                win.reload_recent();
            }
        });

        self.setup_find_signals();
        self.setup_typing_signals();

        // Ask before closing with unsaved changes
        let weak = Rc::downgrade(self);
        self.window.connect_close_request(move |_| {
            let Some(win) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            if win.buffer.is_modified() && !win.close_confirmed.get() {
                win.confirm_discard(|win| {
                    win.close_confirmed.set(true);
                    win.window.close();
                });
                return glib::Propagation::Stop;
            }
            win.save_state();
            // Each window records its file as it closes, so the window closed
            // last decides what opens next time. A window with no saved file
            // (start page or a new, never-saved document) leaves it unchanged.
            // So does a file that isn't on the recent list: after Clear Recent
            // Files, closing a file that's still open shouldn't bring it back.
            // This reads the saved list because another window may have cleared it.
            if let Some(uri) = win.file_uri()
                && state::load_recent().contains(&uri)
            {
                state::save_last_file(&uri);
            }
            glib::Propagation::Proceed
        });
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
                && let Some(win) = weak.upgrade()
                && win.enter_in_list()
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
            if let Some(win) = weak.upgrade() {
                win.composing.set(!preedit.is_empty());
            }
        });

        // Ctrl+V, Shift+Insert and the right-click menu's Paste all come
        // through this signal. Stopping it skips the editor's own paste, for
        // when LeafMD pastes the link itself.
        let weak = Rc::downgrade(self);
        self.source_view.connect_paste_clipboard(move |view| {
            if let Some(win) = weak.upgrade()
                && win.paste_over_selection()
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
        let win = self.clone();
        // The clipboard may belong to another app, so its text arrives a
        // moment later rather than straight away
        glib::spawn_future_local(async move {
            let pasted = clipboard.read_text_future().await.ok().flatten();
            let buffer = &win.buffer;
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
                _ => buffer.paste_clipboard(&clipboard, None, win.source_view.is_editable()),
            }
            win.source_view.scroll_mark_onscreen(&buffer.get_insert());
        });
        true
    }

    // =====================================================================
    // Views and editing
    // =====================================================================

    fn apply_view(&self, mode: ViewMode) {
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
                0 => self.window.default_size().0,
                w => w,
            };
            self.paned.set_position(width / 2);
        }
        if show_preview {
            // The preview isn't updated while hidden, so catch it up now
            self.render_now();
        }
        self.tell_preview_view_mode();
        self.update_format_bar();
        // An open find bar now searches whichever text is showing
        if self.find.bar.is_search_mode() {
            self.refresh_find();
        }
    }

    /// The formatting bar (and its shortcuts) only work where the raw text can be edited.
    fn update_format_bar(&self) {
        let editable = self.stack.visible_child_name().as_deref() == Some("document")
            && self.view_mode.get() != ViewMode::Preview;
        self.format_bar.set_visible(editable);
        self.action("format").set_enabled(editable);
        // Replacing changes the raw text, so it isn't offered in the formatted view
        self.action("replace").set_enabled(editable);
        self.action("replace-all").set_enabled(editable);
        self.find.replace_toggle.set_visible(editable);
        self.find
            .replace_row
            .set_visible(editable && self.find.replace_toggle.is_active());
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

    fn set_view(&self, mode: ViewMode) {
        WidgetExt::activate_action(&self.window, "win.view", Some(&mode.as_str().to_variant()))
            .ok();
    }

    fn set_document_actions_enabled(&self, enabled: bool) {
        for name in DOCUMENT_ACTIONS {
            self.action(name).set_enabled(enabled);
        }
        self.view_switcher.set_sensitive(enabled);
        self.find_button.set_sensitive(enabled);
        self.update_format_bar();
    }

    // =====================================================================
    // Find and replace
    // =====================================================================

    fn setup_find_signals(self: &Rc<Self>) {
        let find = &self.find;

        // Typing in the find box: jump to the first match from where you are
        let weak = Rc::downgrade(self);
        find.entry.connect_search_changed(move |_| {
            if let Some(win) = weak.upgrade() {
                win.refresh_find();
            }
        });
        // Enter goes to the next match. (Ctrl+G and Shift+Ctrl+G also work,
        // because the find box has them built in.)
        let step = |win: &Rc<Self>, step: i32| {
            let weak = Rc::downgrade(win);
            move |_: &gtk::SearchEntry| {
                if let Some(win) = weak.upgrade() {
                    win.find_step(step);
                }
            }
        };
        find.entry.connect_activate(step(self, 1));
        find.entry.connect_next_match(step(self, 1));
        find.entry.connect_previous_match(step(self, -1));
        // Shift+Enter goes to the previous match
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            let enter = matches!(key, gdk::Key::Return | gdk::Key::KP_Enter);
            if enter
                && modifiers.contains(gdk::ModifierType::SHIFT_MASK)
                && let Some(win) = weak.upgrade()
            {
                win.find_step(-1);
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        find.entry.add_controller(keys);

        // Enter in the replace box replaces the current match
        let weak = Rc::downgrade(self);
        find.replace_entry.connect_activate(move |_| {
            if let Some(win) = weak.upgrade() {
                win.replace_one();
            }
        });
        // Escape in the replace box closes the bar, like it does in the find box
        let keys = gtk::EventControllerKey::new();
        let bar = find.bar.downgrade();
        keys.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape
                && let Some(bar) = bar.upgrade()
            {
                bar.set_search_mode(false);
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        find.replace_entry.add_controller(keys);

        // The header bar button shows whether the bar is open, and opens or closes it
        find.bar
            .bind_property("search-mode-enabled", &self.find_button, "active")
            .sync_create()
            .build();
        let weak = Rc::downgrade(self);
        self.find_button.connect_clicked(move |button| {
            let Some(win) = weak.upgrade() else { return };
            if button.is_active() {
                win.open_find(false);
            } else {
                win.find.bar.set_search_mode(false);
            }
        });

        // Closing the bar (Escape or its close button) removes the highlights
        let weak = Rc::downgrade(self);
        find.bar.connect_search_mode_enabled_notify(move |bar| {
            if !bar.is_search_mode()
                && let Some(win) = weak.upgrade()
            {
                win.close_find();
            }
        });

        // The count ("3 of 12") follows the cursor and changes to the text
        let weak = Rc::downgrade(self);
        find.context.connect_occurrences_count_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_raw_find_count();
            }
        });
        let weak = Rc::downgrade(self);
        self.buffer.connect_mark_set(move |buffer, _, mark| {
            if *mark == buffer.get_insert()
                && let Some(win) = weak.upgrade()
            {
                win.update_raw_find_count();
            }
        });
    }

    /// True when the formatted view is the only thing showing, so that's what
    /// gets searched. In Raw and Split view the raw text is searched.
    fn searching_preview(&self) -> bool {
        self.view_mode.get() == ViewMode::Preview
    }

    /// Ctrl+F (or Ctrl+H with `replace`): shows the find bar and puts the
    /// cursor in it, starting with the selected text if there is some.
    fn open_find(self: &Rc<Self>, replace: bool) {
        let find = &self.find;
        if !self.searching_preview()
            && let Some((start, end)) = self.buffer.selection_bounds()
        {
            let selected = self.buffer.text(&start, &end, false);
            if !selected.contains('\n') && selected.chars().count() <= 200 {
                find.entry.set_text(&selected);
            }
        }
        if replace && !self.searching_preview() {
            find.replace_toggle.set_active(true);
        }
        let was_open = find.bar.is_search_mode();
        find.bar.set_search_mode(true);
        if !was_open {
            // Bring back the highlights for whatever is already in the find box
            self.refresh_find();
        }
        if replace && find.replace_row.is_visible() && find.query().is_some() {
            find.replace_entry.grab_focus();
        } else {
            find.entry.grab_focus();
            find.entry.select_region(0, -1);
        }
    }

    /// The find bar was closed: clear the highlights and go back to the text.
    fn close_find(&self) {
        self.find.settings.set_search_text(None);
        self.run_js("window.leafmd && window.leafmd.clearFind();");
        if self.searching_preview() {
            self.webview.grab_focus();
        } else {
            self.source_view.grab_focus();
        }
    }

    /// The search text, an option or the view changed: search again from the
    /// current spot, so typing more letters keeps the same match where it can.
    fn refresh_find(&self) {
        let find = &self.find;
        let query = find.query();
        find.settings
            .set_case_sensitive(self.find_option("find-match-case"));
        find.settings
            .set_at_word_boundaries(self.find_option("find-whole-words"));
        find.settings.set_search_text(query.as_deref());

        if self.searching_preview() {
            self.preview_find(0);
            return;
        }
        // Only one side shows highlights at a time
        self.run_js("window.leafmd && window.leafmd.clearFind();");
        if query.is_some() {
            let from = match self.buffer.selection_bounds() {
                Some((start, _)) => start,
                None => self.buffer.iter_at_mark(&self.buffer.get_insert()),
            };
            self.raw_find_from(&from, false);
        }
        self.update_raw_find_count();
    }

    fn find_options_changed(&self) {
        if self.find.bar.is_search_mode() {
            self.refresh_find();
        }
    }

    fn find_option(&self, name: &str) -> bool {
        self.action(name)
            .state()
            .and_then(|state| state.get::<bool>())
            .unwrap_or(false)
    }

    /// Goes to the next match (`step` 1) or the previous one (`step` -1).
    /// Opens the find bar first if it's closed (F3 with nothing searched yet).
    fn find_step(self: &Rc<Self>, step: i32) {
        if !self.find.bar.is_search_mode() || self.find.query().is_none() {
            self.open_find(false);
            return;
        }
        if self.searching_preview() {
            self.preview_find(step);
            return;
        }
        let from = match self.buffer.selection_bounds() {
            Some((start, end)) => {
                if step > 0 {
                    end
                } else {
                    start
                }
            }
            None => self.buffer.iter_at_mark(&self.buffer.get_insert()),
        };
        self.raw_find_from(&from, step < 0);
    }

    /// Selects the first match after (or before) `from` in the raw text and
    /// scrolls it into view. Wraps around at the end of the document.
    fn raw_find_from(&self, from: &gtk::TextIter, backward: bool) {
        let context = &self.find.context;
        let found = if backward {
            context.backward(from)
        } else {
            context.forward(from)
        };
        if let Some((start, end, _wrapped)) = found {
            self.buffer.select_range(&start, &end);
            // Leave some room above and below, rather than at the very edge
            self.source_view
                .scroll_to_mark(&self.buffer.get_insert(), 0.15, false, 0.0, 0.0);
        }
        self.update_raw_find_count();
    }

    /// Updates "3 of 12" for the raw text.
    fn update_raw_find_count(&self) {
        if !self.find.bar.is_search_mode() || self.searching_preview() {
            return;
        }
        let context = &self.find.context;
        let position = self
            .buffer
            .selection_bounds()
            .map(|(start, end)| context.occurrence_position(&start, &end))
            .unwrap_or(0);
        self.find.show_count(position.max(0), context.occurrences_count());
    }

    /// Searches the formatted view. `step` 0 starts a new search, 1 and -1
    /// move to the next or previous match. The page answers with a "find"
    /// message carrying the count.
    fn preview_find(&self, step: i32) {
        let Some(query) = self.find.query() else {
            self.run_js("window.leafmd && window.leafmd.clearFind();");
            self.find.show_count(0, 0);
            return;
        };
        let query = serde_json::to_string(&query).unwrap_or_default();
        let match_case = self.find_option("find-match-case");
        let whole_words = self.find_option("find-whole-words");
        self.run_js(&format!(
            "window.leafmd && window.leafmd.find({query}, {match_case}, {whole_words}, {step});"
        ));
    }

    /// Replaces the selected match, then moves to the next one. If no match is
    /// selected yet, it just goes to the next one, so you see what will change.
    fn replace_one(self: &Rc<Self>) {
        if self.searching_preview() || self.find.query().is_none() {
            return;
        }
        let replacement = self.find.replace_entry.text();
        if let Some((mut start, mut end)) = self.buffer.selection_bounds()
            && self.find.context.occurrence_position(&start, &end) > 0
        {
            self.buffer.begin_user_action();
            let replaced = self.find.context.replace(&mut start, &mut end, &replacement);
            self.buffer.end_user_action();
            if replaced.is_ok() {
                // Carry on from just after the new text
                self.raw_find_from(&end, false);
                return;
            }
        }
        self.find_step(1);
    }

    /// Replaces every match. One Ctrl+Z puts them all back.
    fn replace_all(&self) {
        if self.searching_preview() || self.find.query().is_none() {
            return;
        }
        let count = self.find.context.occurrences_count();
        let replacement = self.find.replace_entry.text();
        self.buffer.begin_user_action();
        let result = self.find.context.replace_all(&replacement);
        self.buffer.end_user_action();
        match result {
            Ok(()) if count == 1 => self.toast("Replaced 1 match"),
            Ok(()) if count > 1 => self.toast(&format!("Replaced {count} matches")),
            Ok(()) => {}
            Err(err) => self.toast(&format!("Couldn't replace: {}", err.message())),
        }
        self.update_raw_find_count();
    }

    // =====================================================================
    // Formatting bar
    // =====================================================================

    /// Applies a formatting button (bold, bullet list, ...) to the selection,
    /// or to the word or line the cursor is on. One Ctrl+Z undoes it.
    fn apply_format(&self, name: &str) {
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

    /// Shows the open document. If it has just been loaded into an empty
    /// window and the formatted view is still drawing it, waits for that first
    /// (at most REVEAL_TIMEOUT), so the document appears all at once.
    fn show_document(self: &Rc<Self>) {
        let opening = self.stack.visible_child_name().as_deref() == Some("loading");
        if opening && self.webview.is_visible() && !self.preview_ready.get() {
            self.reveal_pending.set(true);
            let weak = Rc::downgrade(self);
            glib::timeout_add_local_once(REVEAL_TIMEOUT, move || {
                if let Some(win) = weak.upgrade()
                    && win.reveal_pending.get()
                {
                    win.reveal_document();
                }
            });
            return;
        }
        self.reveal_document();
    }

    fn reveal_document(&self) {
        self.reveal_pending.set(false);
        self.stack.set_visible_child_name("document");
        self.set_document_actions_enabled(true);
        self.update_title();
    }

    /// Swaps "No Document Open" for a blank page while a file opens.
    fn begin_loading(&self) {
        if self.stack.visible_child_name().as_deref() == Some("empty") {
            self.stack.set_visible_child_name("loading");
        }
    }

    /// The file couldn't be opened: go back to "No Document Open" if nothing was showing.
    fn cancel_loading(&self) {
        if self.stack.visible_child_name().as_deref() == Some("loading") {
            self.stack.set_visible_child_name("empty");
        }
    }

    // =====================================================================
    // Formatted view
    // =====================================================================

    /// Loads the empty preview page. Its base address is the document's folder,
    /// so relative image paths like `images/photo.png` work.
    fn reload_preview(&self) {
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

    fn schedule_render(self: &Rc<Self>) {
        if self.render_scheduled.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(RENDER_DELAY, move || {
            if let Some(win) = weak.upgrade() {
                win.render_scheduled.set(false);
                win.render_now();
            }
        });
    }

    /// Converts the current text and swaps it into the preview page.
    /// Only the page's contents change, so the scroll position is kept.
    fn render_now(&self) {
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

    fn run_js(&self, script: &str) {
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
                self.find.show_count(
                    message["position"].as_i64().unwrap_or(0) as i32,
                    message["count"].as_i64().unwrap_or(0) as i32,
                );
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
            let context = WidgetExt::display(&self.window).app_launch_context();
            let weak = Rc::downgrade(self);
            glib::spawn_future_local(async move {
                if let Err(err) = gio::AppInfo::launch_default_for_uri_future(&uri, Some(&context)).await
                    && let Some(win) = weak.upgrade()
                {
                    eprintln!("LeafMD: could not open link: {err}");
                    win.toast(&format!("Couldn't open the link: {}", err.message()));
                }
            });
        } else if uri.starts_with("file://") {
            self.ask_open_file(gio::File::for_uri(&uri));
        }
        true
    }

    /// A clicked link to a file on this computer. Text files (markdown, LICENSE,
    /// .txt and so on) open in a new LeafMD window; anything else, like an image,
    /// PDF or folder, opens in its usual app.
    fn open_local_link(self: &Rc<Self>, uri: &str) {
        // A "#heading" part names a spot in the file, used after it opens
        let (file_uri, heading) = match uri.split_once('#') {
            Some((file_uri, heading)) => (file_uri, Some(heading.to_string())),
            None => (uri, None),
        };
        let file = gio::File::for_uri(file_uri);
        let win = self.clone();
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
                    win.toast(&format!("“{name}” doesn't exist"));
                    return;
                }
            };
            let is_text = info.file_type() == gio::FileType::Regular
                && info
                    .content_type()
                    .is_some_and(|kind| gio::content_type_is_a(&kind, "text/plain"));
            if is_text {
                // Opens in its own window, so this document stays open too
                let target = match heading {
                    Some(heading) => format!("{}#{heading}", file.uri()),
                    None => file.uri().to_string(),
                };
                if let Some(app) = win.window.application() {
                    app.activate_action("open-in-new-window", Some(&target.to_variant()));
                }
            } else {
                let context = WidgetExt::display(&win.window).app_launch_context();
                if let Err(err) =
                    gio::AppInfo::launch_default_for_uri_future(&file.uri(), Some(&context)).await
                {
                    win.toast(&format!("Couldn't open the link: {}", err.message()));
                }
            }
        });
    }

    // =====================================================================
    // Files
    // =====================================================================

    fn ask_new(self: &Rc<Self>) {
        self.confirm_discard(|win| {
            win.set_text("");
            win.set_file(None);
            win.show_document();
            // A new document needs somewhere to type, so show the raw text
            if win.view_mode.get() == ViewMode::Preview {
                win.set_view(ViewMode::Split);
            }
            win.source_view.grab_focus();
        });
    }

    /// Closes the document but keeps the window, going back to the
    /// "No Document Open" page. LeafMD starts on that page next time too.
    fn ask_close_document(self: &Rc<Self>) {
        self.confirm_discard(|win| {
            win.find.bar.set_search_mode(false);
            win.set_text("");
            win.reveal_pending.set(false);
            win.stack.set_visible_child_name("empty");
            // After switching page, so the title goes back to "LeafMD"
            win.set_file(None);
            win.set_document_actions_enabled(false);
            // The closed file was left out of the list while open; show it again
            win.update_recent_menu();
            state::forget_last_file();
        });
    }

    fn ask_open(self: &Rc<Self>) {
        self.confirm_discard(|win| {
            let win = win.clone();
            glib::spawn_future_local(async move {
                let dialog = gtk::FileDialog::builder()
                    .title("Open Markdown File")
                    .modal(true)
                    .filters(&file_filters())
                    .build();
                if let Ok(file) = dialog.open_future(Some(&win.window)).await {
                    win.load_file(file).await;
                }
            });
        });
    }

    pub fn ask_open_file(self: &Rc<Self>, file: gio::File) {
        self.ask_open_file_at(file, None);
    }

    /// Opens a file in this window, scrolled to `heading` if one is given.
    pub fn open_file_at(self: &Rc<Self>, file: gio::File, heading: Option<String>) {
        self.ask_open_file_at(file, heading);
    }

    /// The address of the file this window shows, if any.
    pub fn file_uri(&self) -> Option<String> {
        self.file.borrow().as_ref().map(|file| file.uri().to_string())
    }

    /// False while the window shows the "No Document Open" start page.
    pub fn has_document(&self) -> bool {
        self.stack.visible_child_name().as_deref() == Some("document")
    }

    /// Scrolls the formatted view to the heading with this id (like "part-two").
    pub fn scroll_to_heading(&self, heading: &str) {
        let id = serde_json::to_string(heading).unwrap_or_default();
        self.run_js(&format!("document.getElementById({id})?.scrollIntoView();"));
    }

    /// Opens a file (asking about unsaved changes first), then scrolls the
    /// formatted view to `heading` if one is given.
    fn ask_open_file_at(self: &Rc<Self>, file: gio::File, heading: Option<String>) {
        self.confirm_discard(move |win| {
            *win.pending_heading.borrow_mut() = heading;
            win.begin_loading();
            let win = win.clone();
            glib::spawn_future_local(async move {
                win.load_file(file).await;
            });
        });
    }

    async fn load_file(self: &Rc<Self>, file: gio::File) {
        let bytes = match file.load_contents_future().await {
            Ok((bytes, _)) => bytes,
            Err(err) => {
                self.cancel_loading();
                self.toast(&format!("Could not open the file: {}", err.message()));
                // A recent file that has been moved or deleted drops off the list
                self.forget_recent(&file.uri());
                return;
            }
        };
        let Ok(text) = String::from_utf8(bytes.to_vec()) else {
            self.cancel_loading();
            self.toast("This file isn't plain text, so LeafMD can't open it");
            return;
        };
        self.set_text(&text);
        self.set_file(Some(file.clone()));
        self.show_document();
        self.remember_recent(&file);
    }

    /// Opens the file LeafMD had open last time, if it's still there.
    /// If it has been moved or deleted, the normal start page shows instead.
    pub fn open_last_file(self: &Rc<Self>) {
        let last = state::load_last_file();
        let Some(uri) = state::file_to_reopen(last.as_deref(), &self.recent.borrow()) else {
            return;
        };
        let file = gio::File::for_uri(&uri);
        if !file.query_exists(gio::Cancellable::NONE) {
            return;
        }
        // Set before the window is first drawn, so it opens blank, not "No Document Open"
        self.begin_loading();
        let win = self.clone();
        glib::spawn_future_local(async move {
            win.load_file(file).await;
        });
    }

    // =====================================================================
    // Recent files
    // =====================================================================

    /// Moves a file to the top of the recent list.
    fn remember_recent(&self, file: &gio::File) {
        let uri = file.uri().to_string();
        // Start from the saved list, which other LeafMD windows may have added to
        let mut recent = state::load_recent();
        recent.retain(|u| *u != uri);
        recent.insert(0, uri);
        recent.truncate(state::RECENT_LIMIT);
        state::save_recent(&recent);
        *self.recent.borrow_mut() = recent;
        self.update_recent_menu();
    }

    fn forget_recent(&self, uri: &str) {
        let mut recent = state::load_recent();
        let before = recent.len();
        recent.retain(|u| u != uri);
        if recent.len() != before {
            state::save_recent(&recent);
        }
        *self.recent.borrow_mut() = recent;
        self.update_recent_menu();
    }

    /// Picks up changes other LeafMD windows made to the recent list.
    fn reload_recent(&self) {
        *self.recent.borrow_mut() = state::load_recent();
        self.update_recent_menu();
    }

    fn clear_recent(&self) {
        self.recent.borrow_mut().clear();
        state::save_recent(&[]);
        state::forget_last_file();
        self.update_recent_menu();
    }

    /// Rebuilds the list under the arrow next to Open: each file's name with
    /// its folder in brackets, then "Clear Recent Files". The open file is skipped.
    fn update_recent_menu(&self) {
        let current = self.file.borrow().as_ref().map(|f| f.uri().to_string());
        let files = gio::Menu::new();
        for uri in self.recent.borrow().iter() {
            if Some(uri) == current.as_ref() {
                continue;
            }
            let file = gio::File::for_uri(uri);
            // Hide files that aren't there right now, but keep them on the list:
            // they may be on a drive that's just unplugged
            if !file.query_exists(gio::Cancellable::NONE) {
                continue;
            }
            let name = file
                .basename()
                .map(|n| n.display().to_string())
                .unwrap_or_else(|| uri.clone());
            let folder = file
                .parent()
                .and_then(|dir| dir.basename())
                .map(|n| n.display().to_string());
            let label = match folder {
                Some(folder) => format!("{name} ({folder})"),
                None => name,
            };
            // Underscores in menu labels mean "keyboard shortcut letter", so double them
            let item = gio::MenuItem::new(Some(&label.replace('_', "__")), None);
            item.set_action_and_target_value(Some("win.open-recent"), Some(&uri.to_variant()));
            files.append_item(&item);
        }
        if files.n_items() == 0 {
            // No action, so it shows greyed out
            files.append(Some("No Recent Files"), None);
        }

        self.recent_menu.remove_all();
        self.recent_menu.append_section(Some("Recent Files"), &files);
        // Going by the saved list, not what's showing: the open file and missing
        // files aren't shown, but there's still something to clear
        if !self.recent.borrow().is_empty() {
            let clear = gio::Menu::new();
            clear.append(Some("Clear Recent Files"), Some("win.clear-recent"));
            self.recent_menu.append_section(None, &clear);
        }
    }

    /// Saves to the current file, or asks where to save a new document.
    /// Returns true if it was saved.
    async fn save(self: &Rc<Self>) -> bool {
        let file = self.file.borrow().clone();
        match file {
            Some(file) => {
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

    async fn save_as(self: &Rc<Self>) -> bool {
        let dialog = gtk::FileDialog::builder()
            .title("Save Markdown File")
            .modal(true)
            .initial_name(self.save_name())
            .filters(&file_filters())
            .build();
        let Ok(file) = dialog.save_future(Some(&self.window)).await else {
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
        match result {
            Ok(_) => {
                // Only mark as saved if nothing was typed while saving
                if self.text() == text {
                    self.buffer.set_modified(false);
                }
                self.toast("Saved");
                true
            }
            Err((_, err)) => {
                self.toast(&format!("Could not save: {}", err.message()));
                false
            }
        }
    }

    /// Runs `then` straight away if there are no unsaved changes. Otherwise asks
    /// the user to save, discard or cancel, and only runs `then` if they didn't cancel.
    fn confirm_discard(self: &Rc<Self>, then: impl FnOnce(&Rc<Self>) + 'static) {
        if !self.buffer.is_modified() {
            then(self);
            return;
        }
        let dialog = adw::AlertDialog::new(
            Some("Save Changes?"),
            Some(&format!(
                "“{}” has unsaved changes. If you don't save them, they will be lost.",
                self.display_name()
            )),
        );
        dialog.add_responses(&[
            ("cancel", "_Cancel"),
            ("discard", "_Discard"),
            ("save", "_Save"),
        ]);
        dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
        dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("save"));
        dialog.set_close_response("cancel");

        let win = self.clone();
        glib::spawn_future_local(async move {
            match dialog.choose_future(Some(&win.window)).await.as_str() {
                "discard" => then(&win),
                // Only carry on if the save actually worked
                "save" if win.save().await => then(&win),
                _ => {}
            }
        });
    }

    fn set_file(&self, file: Option<gio::File>) {
        *self.file.borrow_mut() = file;
        self.update_title();
        self.reload_preview();
    }

    /// Replaces the editor text without adding it to the undo history.
    fn set_text(&self, text: &str) {
        self.buffer.begin_irreversible_action();
        self.buffer.set_text(text);
        self.buffer.end_irreversible_action();
        self.buffer.set_modified(false);
        self.buffer.place_cursor(&self.buffer.start_iter());
        self.source_view
            .scroll_to_iter(&mut self.buffer.start_iter(), 0.0, false, 0.0, 0.0);
    }

    fn text(&self) -> String {
        let (start, end) = self.buffer.bounds();
        self.buffer.text(&start, &end, false).to_string()
    }

    // =====================================================================
    // Small helpers
    // =====================================================================

    fn action(&self, name: &str) -> gio::SimpleAction {
        self.window
            .lookup_action(name)
            .and_downcast::<gio::SimpleAction>()
            .expect("LeafMD action is registered")
    }

    fn display_name(&self) -> String {
        self.file
            .borrow()
            .as_ref()
            .and_then(|file| file.basename())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Untitled".into())
    }

    fn save_name(&self) -> String {
        match self.file.borrow().as_ref() {
            Some(_) => self.display_name(),
            None => "Untitled.md".into(),
        }
    }

    fn update_title(&self) {
        if self.stack.visible_child_name().as_deref() != Some("document") {
            self.title.set_title("LeafMD");
            self.title.set_subtitle("");
            self.window.set_title(Some("LeafMD"));
            return;
        }
        let dot = if self.buffer.is_modified() { "• " } else { "" };
        let name = format!("{dot}{}", self.display_name());
        let folder = self
            .file
            .borrow()
            .as_ref()
            .and_then(|file| file.parent())
            .and_then(|dir| dir.path())
            .map(|path| match path.strip_prefix(glib::home_dir()) {
                Ok(rest) if rest.as_os_str().is_empty() => "~".to_string(),
                Ok(rest) => format!("~/{}", rest.display()),
                Err(_) => path.display().to_string(),
            })
            .unwrap_or_default();
        self.title.set_title(&name);
        self.title.set_subtitle(&folder);
        self.window.set_title(Some(&format!("{name} - LeafMD")));
    }

    /// Matches the editor colours and the formatted view to the system's light or dark style.
    fn update_style(&self) {
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

    fn toast(&self, message: &str) {
        self.toasts.add_toast(
            adw::Toast::builder()
                .title(message)
                .use_markup(false)
                .build(),
        );
    }

    /// A short guide to how LeafMD works.
    fn show_help(&self) {
        let page = adw::PreferencesPage::new();
        let sections: [(&str, &[(&str, &str)]); 5] = [
            (
                "The Three Views",
                &[
                    ("Formatted", "The document as it looks when rendered. Just for reading, with nothing to edit by accident."),
                    ("Raw", "The markdown text itself. Type here to make changes."),
                    ("Split", "Raw text on the left, formatted on the right. The formatted side updates as you type, and both sides scroll together."),
                ],
            ),
            (
                "Editing",
                &[
                    ("Editing the raw text", "The raw text can always be edited in Raw or Split view. Changes are only written to the file when you save."),
                    ("Unsaved changes", "A dot (•) before the file name means there are unsaved changes. LeafMD asks before closing or opening another file."),
                    ("Jump to a word", "In Split view, double-click a word on the formatted side to select it in the raw text."),
                    ("Lists", "In a list, Enter starts the next item for you: another bullet, the next number, or an unticked checkbox. Press Enter on an empty item to end the list, or Shift+Enter for a new line without a marker."),
                    ("Pasting a link", "Select some text and paste a web address over it to turn the text into a link. Pasting anything else replaces the text as usual."),
                ],
            ),
            (
                "Formatting",
                &[
                    ("Formatting bar", "In Raw and Split view, a row of buttons above the text adds markdown for you. Select some text and press Bold, for example, to put ** around it."),
                    ("Nothing selected", "Bold, italic and the other text styles apply to the word the cursor is in. Lists, headings and quotes apply to the cursor's line."),
                    ("Undoing a format", "Press the same button again to take the formatting off, or press Ctrl+Z."),
                    ("Markdown cheatsheet", "Shows the most common markdown and what each one looks like. Open it from the main menu, the Cheatsheet button, or Shift+F1."),
                ],
            ),
            (
                "Find and Replace",
                &[
                    ("Find", "Press Ctrl+F, click the magnifying glass at the top of the window, or choose Find from the main menu, then type. Every match is highlighted, and Enter and Shift+Enter move between them. In Raw and Split view the raw text is searched; in Formatted view, the formatted text is."),
                    ("Replace", "Press Ctrl+H, or the replace button in the find bar, to replace one match at a time or all of them at once. One Ctrl+Z undoes Replace All. Replacing works in Raw and Split view."),
                    ("Options", "The menu at the end of the find bar has Match Case and Whole Words Only."),
                ],
            ),
            (
                "Files and Links",
                &[
                    ("Opening files", "Use Open, drop a file onto the window, or double-click a markdown file in your file manager."),
                    ("Recent files", "The arrow next to Open lists the files you opened recently. When you start LeafMD, it reopens the last file you had open. Clear Recent Files empties the list, and LeafMD then forgets the last file too."),
                    ("Closing a document", "Close Document in the main menu closes the file but keeps the window open, and LeafMD starts with no document next time. Closing the window instead (Ctrl+W) brings the file back next time."),
                    ("Links", "Web links open in your browser. Links to other markdown or text files open them in a new LeafMD window, and links to headings scroll to them."),
                    ("Images", "Images next to the file and images online are shown in the formatted view."),
                ],
            ),
        ];
        for (title, rows) in sections {
            let group = adw::PreferencesGroup::builder().title(title).build();
            for (row_title, row_text) in rows {
                group.add(
                    &adw::ActionRow::builder()
                        .title(*row_title)
                        .subtitle(*row_text)
                        .build(),
                );
            }
            page.add(&group);
        }

        let content = adw::ToolbarView::new();
        content.add_top_bar(&adw::HeaderBar::new());
        content.set_content(Some(&page));
        adw::Dialog::builder()
            .title("Help")
            .content_width(520)
            .content_height(640)
            .child(&content)
            .build()
            .present(Some(&self.window));
    }

    /// Lists every keyboard shortcut.
    fn show_shortcuts(&self) {
        // Built like Help rather than with adw::ShortcutsDialog, which always
        // has a search box. Keep the keys in step with set_shortcuts in main.rs.
        let page = adw::PreferencesPage::new();
        let sections: [(&str, &[(&str, &str)]); 5] = [
            (
                "Views",
                &[
                    ("Formatted view", "Ctrl+1"),
                    ("Raw text", "Ctrl+2"),
                    ("Split view", "Ctrl+3"),
                ],
            ),
            (
                "Files",
                &[
                    ("New document", "Ctrl+N"),
                    ("Open", "Ctrl+O"),
                    ("Save", "Ctrl+S"),
                    ("Save as", "Ctrl+Shift+S"),
                    ("Close window", "Ctrl+W or Ctrl+Q"),
                ],
            ),
            (
                "Formatting",
                &[
                    ("Bold", "Ctrl+B"),
                    ("Italic", "Ctrl+I"),
                    ("Strikethrough", "Ctrl+Shift+X"),
                    ("Inline code", "Ctrl+E"),
                    ("Link", "Ctrl+K"),
                    ("New line without a list marker", "Shift+Enter"),
                ],
            ),
            (
                "Find",
                &[
                    ("Find", "Ctrl+F"),
                    ("Find and replace", "Ctrl+H"),
                    ("Next match", "F3"),
                    ("Previous match", "Shift+F3"),
                    ("Close the find bar", "Esc"),
                ],
            ),
            (
                "Help",
                &[
                    ("Help", "F1"),
                    ("Markdown cheatsheet", "Shift+F1"),
                    ("Keyboard shortcuts", "Ctrl+?"),
                ],
            ),
        ];
        for (title, items) in sections {
            let group = adw::PreferencesGroup::builder().title(title).build();
            for (name, keys) in items {
                let row = adw::ActionRow::builder().title(*name).build();
                row.add_suffix(
                    &gtk::Label::builder()
                        .label(*keys)
                        .css_classes(["dim-label"])
                        .build(),
                );
                group.add(&row);
            }
            page.add(&group);
        }

        let content = adw::ToolbarView::new();
        content.add_top_bar(&adw::HeaderBar::new());
        content.set_content(Some(&page));
        adw::Dialog::builder()
            .title("Keyboard Shortcuts")
            .content_width(520)
            .content_height(640)
            .child(&content)
            .build()
            .present(Some(&self.window));
    }

    /// The most common markdown, with what each one looks like.
    fn show_cheatsheet(&self) {
        // Each row: the markdown to type, and a preview drawn with GTK's own
        // text styles. "accent" colours the preview like a link.
        type Row = (&'static str, &'static str, &'static str);
        let sections: [(&str, &str, &[Row]); 6] = [
            (
                "Text",
                "",
                &[
                    ("**Bold text**", "<b>Bold text</b>", ""),
                    ("*Italic text*", "<i>Italic text</i>", ""),
                    ("***Bold and italic***", "<b><i>Bold and italic</i></b>", ""),
                    ("~~Strikethrough~~", "<s>Strikethrough</s>", ""),
                    ("`Inline code`", "<tt>Inline code</tt>", ""),
                ],
            ),
            (
                "Headings",
                "Use one to six # signs, then a space. More # signs make a smaller heading.",
                &[
                    ("# Heading 1", "<span size='xx-large' weight='bold'>Heading 1</span>", ""),
                    ("## Heading 2", "<span size='x-large' weight='bold'>Heading 2</span>", ""),
                    ("### Heading 3", "<span size='large' weight='bold'>Heading 3</span>", ""),
                ],
            ),
            (
                "Lists",
                "Leave a blank line before a list. Indent with two spaces to put one list inside another.",
                &[
                    ("- Item", "•  Item", ""),
                    ("1. Item", "1.  Item", ""),
                    ("- [ ] To do", "☐  To do", ""),
                    ("- [x] Done", "☑  Done", ""),
                    ("  - Nested item", "      ◦  Nested item", ""),
                ],
            ),
            (
                "Links and Images",
                "",
                &[
                    ("[LeafMD](https://example.com)", "<u>LeafMD</u>", "accent"),
                    ("https://example.com", "<u>https://example.com</u>", "accent"),
                    ("[See below](#heading-1)", "<u>See below</u> (jumps to a heading)", "accent"),
                    ("![A cat](cat.png)", "Shows cat.png, described as \u{201c}A cat\u{201d}", ""),
                ],
            ),
            (
                "Blocks",
                "",
                &[
                    ("> Quoted text", "<span alpha='50%'>▎</span> <i>Quoted text</i>", ""),
                    ("```\nCode block\n```", "<tt>Code block</tt>", ""),
                    ("| A | B |\n| - | - |\n| 1 | 2 |", "A table with columns A and B", ""),
                    ("---", "A dividing line across the page", ""),
                ],
            ),
            (
                "Paragraphs and More",
                "",
                &[
                    ("First paragraph\n\nSecond paragraph", "A blank line starts a new paragraph", ""),
                    ("Line one\\\nLine two", "Line one\nLine two", ""),
                    ("Text[^1]\n\n[^1]: The note", "Text<sup>1</sup>, with the note at the bottom", ""),
                    ("\\*Not italic\\*", "*Not italic*", ""),
                ],
            ),
        ];

        let page = adw::PreferencesPage::new();
        for (title, description, rows) in sections {
            let group = adw::PreferencesGroup::builder()
                .title(title)
                .description(description)
                .build();
            for (markdown, preview, class) in rows {
                // The markdown side can be selected and copied
                let row = adw::ActionRow::builder()
                    .title(format!("<tt>{}</tt>", glib::markup_escape_text(markdown)))
                    .title_selectable(true)
                    .build();
                let label = gtk::Label::builder()
                    .label(*preview)
                    .use_markup(true)
                    .wrap(true)
                    .xalign(0.0)
                    .width_chars(24)
                    .max_width_chars(24)
                    .build();
                if !class.is_empty() {
                    label.add_css_class(class);
                }
                row.add_suffix(&label);
                group.add(&row);
            }
            page.add(&group);
        }

        let content = adw::ToolbarView::new();
        content.add_top_bar(&adw::HeaderBar::new());
        content.set_content(Some(&page));
        adw::Dialog::builder()
            .title("Markdown Cheatsheet")
            .content_width(600)
            .content_height(680)
            .child(&content)
            .build()
            .present(Some(&self.window));
    }

    fn show_about(&self) {
        let about = adw::AboutDialog::builder()
            .application_name("LeafMD")
            .application_icon(crate::APP_ID)
            .version(env!("CARGO_PKG_VERSION"))
            .comments(ABOUT_TEXT)
            .developer_name("Jereme Hancock")
            .copyright("© 2026 Jereme Hancock")
            .license_type(gtk::License::Gpl30)
            .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues"))
            // Wider than GNOME's default, so the Built With rows fit on one line
            .content_width(560)
            .build();
        // Custom links instead of libadwaita's built-in website row: that row
        // shows the address itself, sits apart from other links, and there can
        // only be one. These two read "Website" and "GitHub", side by side.
        about.add_link("Website", "https://leafmd.app/");
        about.add_link("GitHub", env!("CARGO_PKG_REPOSITORY"));
        // Credits page, in order. A custom section instead of .developers(),
        // because that one's heading always says "Code by". "Name URL" makes
        // the name a link.
        about.add_credit_section(
            Some("Developed by"),
            &["Jereme Hancock https://jeremehancock.com"],
        );
        // Right under the developer, so it's seen without scrolling
        about.add_credit_section(
            Some("AI Disclosure"),
            &["This project was created with the help of AI."],
        );
        // The language and libraries LeafMD is made with. Version numbers are
        // the ones LeafMD is running with right now.
        let built_with = [
            format!("Rust {}, the programming language https://www.rust-lang.org", env!("LEAFMD_RUST_VERSION")),
            format!(
                "GTK {}.{}.{}, the user interface toolkit https://www.gtk.org",
                gtk::major_version(),
                gtk::minor_version(),
                gtk::micro_version()
            ),
            format!(
                "libadwaita {}.{}.{}, GNOME's look and widgets https://gnome.pages.gitlab.gnome.org/libadwaita/",
                adw::major_version(),
                adw::minor_version(),
                adw::micro_version()
            ),
            format!(
                "GtkSourceView {}.{}.{}, the raw text editor https://gitlab.gnome.org/GNOME/gtksourceview",
                sourceview5::major_version(),
                sourceview5::minor_version(),
                sourceview5::micro_version()
            ),
            format!(
                "WebKitGTK {}.{}.{}, draws the formatted view https://webkitgtk.org",
                webkit6::functions::major_version(),
                webkit6::functions::minor_version(),
                webkit6::functions::micro_version()
            ),
            "comrak, turns markdown into HTML https://github.com/kivikakk/comrak".to_string(),
            "ammonia, removes anything unsafe from the HTML https://github.com/rust-ammonia/ammonia"
                .to_string(),
        ];
        let built_with: Vec<&str> = built_with.iter().map(String::as_str).collect();
        about.add_credit_section(Some("Built With"), &built_with);
        about.present(Some(&self.window));
    }

    fn save_state(&self) {
        let (width, height) = self.window.default_size();
        SavedState {
            view: self.view_mode.get(),
            width,
            height,
            maximized: self.window.is_maximized(),
        }
        .save();
    }
}

/// The row of formatting buttons. Each button runs "win.format" with the
/// format's name, so the buttons, the heading menu and the keyboard shortcuts
/// all share the same code.
fn build_format_bar() -> gtk::ScrolledWindow {
    let bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    // GNOME's toolbar style: flat buttons with even spacing
    bar.add_css_class("toolbar");

    let button = |name: &str, tooltip: &str| {
        gtk::Button::builder()
            .icon_name(format!("leafmd-format-{name}-symbolic"))
            .tooltip_text(tooltip)
            .action_name("win.format")
            .action_target(&name.to_variant())
            .build()
    };
    let separator = || {
        let line = gtk::Separator::new(gtk::Orientation::Vertical);
        line.set_margin_top(6);
        line.set_margin_bottom(6);
        line
    };

    for (name, tooltip) in [
        ("bold", "Bold (Ctrl+B)"),
        ("italic", "Italic (Ctrl+I)"),
        ("strikethrough", "Strikethrough (Ctrl+Shift+X)"),
        ("code", "Inline code (Ctrl+E)"),
        ("link", "Link (Ctrl+K)"),
    ] {
        bar.append(&button(name, tooltip));
    }
    bar.append(&separator());

    let headings = gio::Menu::new();
    for level in 1..=3 {
        headings.append(
            Some(&format!("Heading {level}")),
            Some(&format!("win.format::heading-{level}")),
        );
    }
    bar.append(
        &gtk::MenuButton::builder()
            .icon_name("leafmd-format-heading-symbolic")
            .always_show_arrow(true)
            .tooltip_text("Heading")
            .menu_model(&headings)
            .build(),
    );
    bar.append(&separator());

    for (name, tooltip) in [
        ("bullet-list", "Bulleted list"),
        ("numbered-list", "Numbered list"),
        ("task-list", "Checklist"),
        ("quote", "Quote"),
    ] {
        bar.append(&button(name, tooltip));
    }
    bar.append(&separator());
    bar.append(&button("code-block", "Code block"));
    bar.append(&button("table", "Table"));

    // The cheatsheet button sits at the far end
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    bar.append(&spacer);
    bar.append(
        &gtk::Button::builder()
            .label("Cheatsheet")
            .tooltip_text("Markdown cheatsheet (Shift+F1)")
            .action_name("win.cheatsheet")
            .css_classes(["flat"])
            .build(),
    );

    // In a very narrow window the buttons scroll sideways instead of
    // stopping the window from getting smaller
    gtk::ScrolledWindow::builder()
        .child(&bar)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .build()
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

fn file_filters() -> gio::ListStore {
    let markdown = gtk::FileFilter::new();
    markdown.set_name(Some("Markdown"));
    markdown.add_mime_type("text/markdown");
    for suffix in ["md", "markdown", "mdown", "mkd"] {
        markdown.add_suffix(suffix);
    }

    let text = gtk::FileFilter::new();
    text.set_name(Some("Plain text"));
    text.add_mime_type("text/plain");

    let all = gtk::FileFilter::new();
    all.set_name(Some("All files"));
    all.add_pattern("*");

    let filters = gio::ListStore::new::<gtk::FileFilter>();
    filters.append(&markdown);
    filters.append(&text);
    filters.append(&all);
    filters
}

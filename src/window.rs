//! The LeafMD window: the toolbar, the raw editor, the formatted view, and
//! opening and saving files.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;
use webkit6::prelude::*;

use crate::render;
use crate::state::{SavedState, ViewMode};

/// The empty page (styles only) that the formatted view is drawn into.
const PREVIEW_PAGE: &str = include_str!("preview.html");

/// How long to wait after the last keystroke before updating the formatted view.
const RENDER_DELAY: Duration = Duration::from_millis(150);

/// Actions that only make sense while a document is open.
const DOCUMENT_ACTIONS: [&str; 4] = ["save", "save-as", "edit", "view"];

pub struct Window {
    pub window: adw::ApplicationWindow,
    title: adw::WindowTitle,
    toasts: adw::ToastOverlay,
    stack: gtk::Stack,
    paned: gtk::Paned,
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

        header.pack_start(
            &gtk::Button::builder()
                .icon_name("document-open-symbolic")
                .tooltip_text("Open (Ctrl+O)")
                .action_name("win.open")
                .build(),
        );
        header.pack_start(
            &gtk::Button::builder()
                .icon_name("document-save-symbolic")
                .tooltip_text("Save (Ctrl+S)")
                .action_name("win.save")
                .build(),
        );

        // pack_end fills from the right edge inwards: menu, then Edit, then the view buttons
        let menu = gio::Menu::new();
        menu.append(Some("New"), Some("win.new"));
        menu.append(Some("Save As…"), Some("win.save-as"));
        menu.append(Some("About LeafMD"), Some("win.about"));
        header.pack_end(
            &gtk::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .menu_model(&menu)
                .primary(true)
                .tooltip_text("Main Menu")
                .build(),
        );
        header.pack_end(
            &gtk::ToggleButton::builder()
                .icon_name("document-edit-symbolic")
                .tooltip_text("Edit (Ctrl+E)")
                .action_name("win.edit")
                .build(),
        );

        // Three buttons tied to one "view" action, so exactly one is pressed at a time
        let view_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        view_buttons.add_css_class("linked");
        for (mode, label, tooltip) in [
            (ViewMode::Preview, "Formatted", "Formatted view (Ctrl+1)"),
            (ViewMode::Source, "Raw", "Raw text (Ctrl+2)"),
            (ViewMode::Split, "Split", "Split view (Ctrl+3)"),
        ] {
            view_buttons.append(
                &gtk::ToggleButton::builder()
                    .label(label)
                    .tooltip_text(tooltip)
                    .action_name("win.view")
                    .action_target(&mode.as_str().to_variant())
                    .build(),
            );
        }
        header.pack_end(&view_buttons);

        // ---- Raw editor ----
        let buffer = sourceview5::Buffer::new(None);
        if let Some(language) = sourceview5::LanguageManager::default().language("markdown") {
            buffer.set_language(Some(&language));
        }
        buffer.set_highlight_syntax(true);

        let source_view = sourceview5::View::with_buffer(&buffer);
        source_view.set_show_line_numbers(true);
        source_view.set_highlight_current_line(true);
        source_view.set_monospace(true);
        source_view.set_wrap_mode(gtk::WrapMode::WordChar);
        source_view.set_tab_width(4);
        source_view.set_auto_indent(true);
        source_view.set_top_margin(12);
        source_view.set_bottom_margin(12);
        source_view.set_left_margin(8);
        source_view.set_right_margin(12);
        // Read-only until Edit is turned on
        source_view.set_editable(false);
        source_view.set_cursor_visible(false);

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

        let stack = gtk::Stack::new();
        stack.add_named(&empty_page, Some("empty"));
        stack.add_named(&paned, Some("document"));

        let toasts = adw::ToastOverlay::new();
        toasts.set_child(Some(&stack));

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header);
        toolbar_view.set_content(Some(&toasts));
        window.set_content(Some(&toolbar_view));

        let win = Rc::new(Self {
            window,
            title,
            toasts,
            stack,
            paned,
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
        });

        win.setup_actions(saved.view);
        win.setup_signals();
        win.set_document_actions_enabled(false);
        win.apply_view(saved.view);
        win.update_style();
        win.update_title();
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
        add("about", |win| win.show_about());

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
        });
        self.window.add_action(&view);

        // "edit" is on or off
        let edit = gio::SimpleAction::new_stateful("edit", None, &false.to_variant());
        let weak = Rc::downgrade(self);
        edit.connect_activate(move |action, _| {
            let Some(win) = weak.upgrade() else { return };
            let on = action.state().and_then(|s| s.get::<bool>()).unwrap_or(false);
            win.set_editing(!on);
        });
        self.window.add_action(&edit);
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
                win.update_style();
                win.render_now();
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
            glib::Propagation::Proceed
        });
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
    }

    fn set_view(&self, mode: ViewMode) {
        WidgetExt::activate_action(&self.window, "win.view", Some(&mode.as_str().to_variant()))
            .ok();
    }

    fn set_editing(&self, on: bool) {
        self.action("edit").set_state(&on.to_variant());
        self.source_view.set_editable(on);
        self.source_view.set_cursor_visible(on);
        if on {
            // You can only type in the raw text, so switch to split if it is hidden
            if self.view_mode.get() == ViewMode::Preview {
                self.set_view(ViewMode::Split);
            }
            self.source_view.grab_focus();
        }
    }

    fn set_document_actions_enabled(&self, enabled: bool) {
        for name in DOCUMENT_ACTIONS {
            self.action(name).set_enabled(enabled);
        }
    }

    fn show_document(&self) {
        self.stack.set_visible_child_name("document");
        self.set_document_actions_enabled(true);
        self.update_title();
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
        self.run_js(&format!(
            "document.getElementById('content').innerHTML = {html_as_js_string};"
        ));
    }

    fn run_js(&self, script: &str) {
        self.webview.evaluate_javascript(
            script,
            None,
            None,
            gio::Cancellable::NONE,
            |result| {
                if let Err(err) = result {
                    eprintln!("LeafMD: could not update the formatted view: {err}");
                }
            },
        );
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
        if action.navigation_type() == webkit6::NavigationType::LinkClicked {
            gtk::UriLauncher::new(&uri).launch(
                Some(&self.window),
                gio::Cancellable::NONE,
                |_| {},
            );
        } else if uri.starts_with("file://") {
            self.ask_open_file(gio::File::for_uri(&uri));
        }
        true
    }

    // =====================================================================
    // Files
    // =====================================================================

    fn ask_new(self: &Rc<Self>) {
        self.confirm_discard(|win| {
            win.set_text("");
            win.set_file(None);
            win.show_document();
            win.set_editing(true);
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
        self.confirm_discard(move |win| {
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
                self.toast(&format!("Could not open the file: {}", err.message()));
                return;
            }
        };
        let Ok(text) = String::from_utf8(bytes.to_vec()) else {
            self.toast("This file isn't plain text, so LeafMD can't open it");
            return;
        };
        self.set_text(&text);
        self.set_file(Some(file));
        self.show_document();
        // Files always open read-only
        self.set_editing(false);
    }

    /// Saves to the current file, or asks where to save a new document.
    /// Returns true if it was saved.
    async fn save(self: &Rc<Self>) -> bool {
        let file = self.file.borrow().clone();
        match file {
            Some(file) => self.write_to(&file).await,
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
        self.set_file(Some(file));
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
                "save" => {
                    if win.save().await {
                        then(&win);
                    }
                }
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

    fn show_about(&self) {
        adw::AboutDialog::builder()
            .application_name("LeafMD")
            .version(env!("CARGO_PKG_VERSION"))
            .comments(env!("CARGO_PKG_DESCRIPTION"))
            .build()
            .present(Some(&self.window));
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

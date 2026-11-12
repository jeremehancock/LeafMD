//! The LeafMD window: the toolbar, the formatting bar, find and replace, Help,
//! the cheatsheet and About. The document it shows is in document.rs.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::time::Duration;

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;

use crate::document::{self, Document};
use crate::export;
use crate::file_change::FileBanner;
use crate::find::FindBar;
use crate::folder_tree::{self, FilesPage};
use crate::headings::HeadingsPage;
use crate::opening::{self, FolderFacts, Target, WindowFacts};
use crate::side_panel::{SidePanel, WidthWatcher};
use crate::state::{self, SavedState, ViewMode};
use crate::text_size;

/// When a file is opening, the longest LeafMD waits for the formatted view to
/// finish drawing before showing the document anyway.
const REVEAL_TIMEOUT: Duration = Duration::from_secs(1);

/// The description in the About window.
const ABOUT_TEXT: &str = "LeafMD is a markdown viewer and editor that keeps things simple.

Read a document cleanly formatted, work directly in the raw text, or see both side by side and watch your changes appear as you type.

Rather than trying to do everything, LeafMD focuses on doing the essentials well. It is ready to use the moment you open it, and the few options it has stay out of your way, so you can focus on reading and writing.

LeafMD was created with the help of AI.";

/// Actions that only make sense while a document is open.
const DOCUMENT_ACTIONS: [&str; 11] = [
    "close-document",
    "save",
    "save-as",
    "print",
    "export-pdf",
    "export-html",
    "view",
    "find",
    "find-replace",
    "find-next",
    "find-previous",
];

/// Every open LeafMD window, used longest ago first, so the last one is the
/// most recently used.
pub type Windows = Rc<RefCell<Vec<Rc<Window>>>>;

/// A file waiting to be read and shown in a tab, with a heading to scroll to.
struct WaitingFile {
    file: gio::File,
    heading: Option<String>,
}

/// Opens a file, wherever it should go (see opening.rs). `asked_from` is the
/// window that asked, or None for a file from outside LeafMD.
pub fn open_file(
    app: &adw::Application,
    windows: &Windows,
    file: gio::File,
    heading: Option<String>,
    asked_from: Option<&Rc<Window>>,
) {
    let uri = file.uri().to_string();
    // A copy, because presenting a window can change the order of the list
    let list = windows.borrow().clone();
    let facts: Vec<WindowFacts> = list
        .iter()
        .map(|win| WindowFacts { has_file: win.has_file(&uri), is_empty: win.is_empty() })
        .collect();
    let asked = asked_from.and_then(|asked| list.iter().position(|win| Rc::ptr_eq(win, asked)));
    match opening::choose_window(&facts, asked) {
        Target::AlreadyOpen(position) => {
            let win = &list[position];
            win.window.present();
            win.select_file(&uri, heading.as_deref());
        }
        Target::Use(position) => {
            let win = &list[position];
            win.window.present();
            win.queue_file(file, heading);
        }
        Target::NewWindow => {
            // Told about the file before it appears, so it starts blank. Once it's
            // on screen, leaving "No Document Open" would be a visible fade.
            let win = Window::open_new(app, windows);
            win.queue_file(file, heading);
            win.window.present();
        }
    }
}

/// Opens a folder from outside LeafMD (`leafmd .`) in the side panel, in the
/// window the rule in opening.rs picks, and returns that window.
pub fn open_folder(app: &adw::Application, windows: &Windows, folder: gio::File) -> Rc<Window> {
    let list = windows.borrow().clone();
    let facts: Vec<FolderFacts> = list
        .iter()
        .map(|win| FolderFacts {
            has_folder: win.has_folder(&folder),
            is_empty: win.is_empty() && win.folder.borrow().is_none(),
        })
        .collect();
    let win = match opening::choose_window_for_folder(&facts) {
        Target::AlreadyOpen(position) | Target::Use(position) => list[position].clone(),
        Target::NewWindow => Window::open_new(app, windows),
    };
    win.choose_folder(folder);
    win.window.present();
    win
}

pub struct Window {
    pub window: adw::ApplicationWindow,
    /// Every window, including this one, so this one can open files and find
    /// them in the others. Each window leaves the list when it closes.
    windows: Windows,
    /// Files waiting to be read, in order. They're read one at a time so their
    /// tabs land in that order, whichever takes longest to read. A file stays
    /// here while it's being read, so opening it again doesn't read it twice.
    waiting_files: RefCell<VecDeque<WaitingFile>>,
    /// True while the waiting files are being read
    reading_files: Cell<bool>,
    /// The order of the tabs being brought back at startup, as they were saved
    restored_order: RefCell<Vec<String>>,
    /// The tabs still to come back at startup, apart from the selected one, which
    /// comes back first. A file stays here while it's being read, so opening it
    /// again doesn't read it twice.
    restoring_files: RefCell<Vec<String>>,
    /// True until those files have started being read
    restoring_not_started: Cell<bool>,
    title: adw::WindowTitle,
    toasts: adw::ToastOverlay,
    stack: gtk::Stack,
    view_switcher: adw::ToggleGroup,
    /// The row of formatting buttons above the raw text (Raw and Split view only)
    format_bar: gtk::ScrolledWindow,
    /// The find and replace bar (Ctrl+F, Ctrl+H)
    pub find: FindBar,
    /// The magnifying glass in the header bar that opens the find bar
    find_button: gtk::ToggleButton,
    /// The list under the arrow next to Open
    recent_menu: gio::Menu,
    /// Recently opened files as file:// addresses, newest first
    recent: RefCell<Vec<String>>,
    /// The open documents, one page per tab
    tab_view: adw::TabView,
    /// The document whose state the find bar is showing, kept so that state
    /// can be saved when another tab is selected
    shown_document: RefCell<Weak<Document>>,
    /// True while a tab's saved state is being put into the find bar, so that
    /// doesn't count as the user closing the bar
    loading_find: Cell<bool>,
    /// True while a file that has been read is being put into a new tab, so
    /// the Files page waits for the file instead of following the empty tab
    adding_file_tab: Cell<bool>,
    /// True while LeafMD itself is closing a tab or the window, as opposed to
    /// a tab being dragged to another window
    closing_tab: Cell<bool>,
    /// The tab whose right-click menu was opened last, which is the one the
    /// menu's items act on. It may not be the selected tab.
    menu_page: glib::WeakRef<adw::TabPage>,
    /// The view last chosen in this window, which new tabs start in
    last_view: Cell<ViewMode>,
    /// Set after the user has answered "discard" or "save" when closing.
    close_confirmed: Cell<bool>,
    /// The banner that says another app changed the open file
    pub file_banner: FileBanner,
    /// Holds the side panel on the left and everything else on the right. In a
    /// narrow window it's "collapsed": the panel slides over the document instead.
    split: adw::OverlaySplitView,
    /// The header bar, tab bar and formatting bar (and the bottom bar in a
    /// narrow window), which Distraction Free hides
    controls: adw::ToolbarView,
    /// The main menu's button, so F10 can bring the hidden header bar back
    menu_button: gtk::MenuButton,
    /// Distraction Free: only the document shows. This window only, always
    /// off when a window opens.
    distraction_free: Cell<bool>,
    /// Whether line numbers show while in Distraction Free. They start hidden
    /// each time, and Line Numbers in the menu shows them.
    distraction_free_numbers: Cell<bool>,
    /// True while the main menu is open because F10 brought the hidden bars
    /// back for it, so they go again when it closes
    menu_from_key: Cell<bool>,
    side_panel: SidePanel,
    /// The button at the top of the side panel that shows the folder above the
    /// one the Files page shows. Only there while the Files page is.
    up_button: gtk::Button,
    /// How many folders up the user has gone with the up button, and for which
    /// file (its address, or None with no file). Tied to the file rather than
    /// reset on every update of the Files page, because that also happens for
    /// small things like the unsaved dot appearing. Switching tabs clears it,
    /// and saving somewhere new changes the address, so it stops applying.
    went_up: RefCell<Option<(Option<String>, u32)>>,
    /// The Files page of the side panel
    files: Rc<FilesPage>,
    /// The Headings page of the side panel
    headings: Rc<HeadingsPage>,
    /// The word count in the document's bottom-right corner
    word_label: gtk::Label,
    /// The folder the user chose for this window (with `leafmd .` or Open
    /// Folder…). Until there is one, the Files page shows the selected document's.
    folder: RefCell<Option<gio::File>>,
    /// Whether the side panel should be beside the document when the window is
    /// wide enough. In a narrow window it only slides out when asked.
    panel_wanted: Cell<bool>,
    /// The side panel's width as the user last dragged it in this window (or
    /// as it was remembered when the window opened). The panel shows narrower
    /// when the window hasn't room for it, without changing this.
    panel_width: Cell<i32>,
    /// Holds the split view and notices how wide it's made
    split_holder: WidthWatcher,
    /// The width the split view was last given, or 0 before the window is
    /// first laid out
    holder_width: Cell<i32>,
    /// The see-through strip on the document's left edge for dragging the
    /// panel wider or narrower. Only there while the panel is beside the document.
    resize_handle: gtk::Box,
    /// How big the text is in this window, as a percentage: 100 is the size
    /// set in Preferences. Every tab in the window shares it, and it's never
    /// saved, so each window starts at 100.
    zoom: Cell<u32>,
    /// The percentage in the zoom row at the top of the main menu
    zoom_label: gtk::Button,
}

impl Window {
    /// A new window, added to the list of windows.
    pub fn open_new(app: &adw::Application, windows: &Windows) -> Rc<Self> {
        let win = Self::new(app, windows.clone());
        let list = windows.clone();
        win.window.connect_destroy(move |closed| {
            let leaving = {
                let mut list = list.borrow_mut();
                list.iter()
                    .position(|win| win.window == *closed)
                    .map(|position| list.remove(position))
            };
            // Let go of it only now, once the list is free for anything that needs it
            drop(leaving);
        });
        windows.borrow_mut().push(win.clone());
        win
    }

    fn new(app: &adw::Application, windows: Windows) -> Rc<Self> {
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

        // Opens and closes the side panel. It stays pressed in while the panel
        // shows, because the "side-panel" action it runs is a yes or no setting.
        // It's at the far left, nearest the panel, and stays in the header bar
        // in narrow windows.
        header.pack_start(
            &gtk::ToggleButton::builder()
                .icon_name("leafmd-side-panel-symbolic")
                .tooltip_text("Side panel (F9)")
                .action_name("win.side-panel")
                .build(),
        );

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
                // Header bar buttons are flat by default. Raised gives Open and
                // its arrow one shared background, so they read as one button.
                .css_classes(["raised"])
                .build(),
        );
        // Starts a new document. Uses LeafMD's own icon, so it can't change with
        // the icon theme or a GNOME update.
        let new_button = gtk::Button::builder()
            .icon_name("leafmd-new-symbolic")
            .tooltip_text("New document (Ctrl+T)")
            .action_name("win.new")
            .build();
        header.pack_start(&new_button);

        // pack_end fills from the right edge inwards: the menu, then the view buttons.
        // The menu is grouped into sections (shown with dividers); shortcuts appear automatically.
        let file_section = gio::Menu::new();
        file_section.append(Some("New"), Some("win.new"));
        file_section.append(Some("New Window"), Some("win.new-window"));
        file_section.append(Some("Open…"), Some("win.open"));
        file_section.append(Some("Open Folder…"), Some("win.open-folder"));
        file_section.append(Some("Close Document"), Some("win.close-document"));
        let save_section = gio::Menu::new();
        save_section.append(Some("Save"), Some("win.save"));
        save_section.append(Some("Save As…"), Some("win.save-as"));
        let share_section = gio::Menu::new();
        share_section.append(Some("Print…"), Some("win.print"));
        share_section.append(Some("Export as PDF…"), Some("win.export-pdf"));
        share_section.append(Some("Export as HTML…"), Some("win.export-html"));
        let find_section = gio::Menu::new();
        find_section.append(Some("Find…"), Some("win.find"));
        find_section.append(Some("Find and Replace…"), Some("win.find-replace"));
        // Shows a tick while the panel is open, because the action is a yes or no
        let panel_section = gio::Menu::new();
        panel_section.append(Some("Side Panel"), Some("win.side-panel"));
        panel_section.append(Some("Line Numbers"), Some("win.line-numbers"));
        // Shared by every window, so it's the app's action rather than the window's
        panel_section.append(Some("Check Spelling"), Some("app.spell-check"));
        panel_section.append(Some("Distraction Free"), Some("win.distraction-free"));
        panel_section.append(Some("Full Screen"), Some("win.fullscreen"));
        let app_section = gio::Menu::new();
        app_section.append(Some("Preferences"), Some("win.preferences"));
        app_section.append(Some("Help"), Some("win.help"));
        app_section.append(Some("Markdown Cheatsheet"), Some("win.cheatsheet"));
        app_section.append(Some("Keyboard Shortcuts"), Some("win.shortcuts"));
        app_section.append(Some("About LeafMD"), Some("win.about"));
        // The zoom row at the top: a placeholder item that the row's widgets
        // fill in, once the menu button has made its popover (below)
        let zoom_section = gio::Menu::new();
        let zoom_item = gio::MenuItem::new(None, None);
        zoom_item.set_attribute_value("custom", Some(&"zoom".to_variant()));
        zoom_section.append_item(&zoom_item);
        let menu = gio::Menu::new();
        menu.append_section(None, &zoom_section);
        menu.append_section(None, &file_section);
        menu.append_section(None, &save_section);
        menu.append_section(None, &share_section);
        menu.append_section(None, &find_section);
        menu.append_section(None, &panel_section);
        menu.append_section(None, &app_section);
        let menu_button = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&menu)
            .primary(true)
            .tooltip_text("Main Menu")
            .build();
        header.pack_end(&menu_button);
        let zoom_label = gtk::Button::builder()
            .label("100%")
            .tooltip_text("Normal size (Ctrl+0)")
            .action_name("win.zoom")
            .action_target(&"reset".to_variant())
            .css_classes(["flat"])
            .hexpand(true)
            .build();
        if let Some(popover) = menu_button.popover().and_downcast::<gtk::PopoverMenu>() {
            popover.add_child(&zoom_row(&zoom_label), "zoom");
        }
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
        // Added after the view switcher, so it lands just to the switcher's left.
        // A word rather than an icon, like Open.
        let save_button = gtk::Button::builder()
            .label("Save")
            .tooltip_text("Save (Ctrl+S)")
            .action_name("win.save")
            .build();
        header.pack_end(&save_button);

        // ---- Formatting bar ----
        let format_bar = build_format_bar();

        // ---- Find and replace ----
        let find = FindBar::new();

        // ---- Layout ----
        // The tab bar shows itself only with two or more tabs, like gedit. It
        // handles tabs getting narrower and scrolling sideways by itself.
        let tab_view = adw::TabView::new();
        // Not all of the built-in shortcuts: Ctrl+Home and Ctrl+End belong to the
        // raw text. Ctrl+Page Up and Down are in setup_tab_signals, because these
        // built-in ones stop at the ends.
        tab_view.set_shortcuts(
            adw::TabViewShortcuts::CONTROL_TAB
                | adw::TabViewShortcuts::CONTROL_SHIFT_TAB
                | adw::TabViewShortcuts::ALT_DIGITS,
        );
        // What right-clicking a tab shows
        let moves = gio::Menu::new();
        moves.append(Some("Move Left"), Some("win.tab-move-left"));
        moves.append(Some("Move Right"), Some("win.tab-move-right"));
        moves.append(Some("Move to New Window"), Some("win.tab-move-to-new-window"));
        let closes = gio::Menu::new();
        closes.append(Some("Close"), Some("win.tab-close"));
        let tab_menu = gio::Menu::new();
        tab_menu.append_section(None, &moves);
        tab_menu.append_section(None, &closes);
        tab_view.set_menu_model(Some(&tab_menu));

        let tab_bar = adw::TabBar::builder()
            .view(&tab_view)
            .autohide(true)
            .expand_tabs(true)
            .build();
        keep_selected_tab_in_view(&tab_bar);

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
        // The word count floats in the document's bottom-right corner. It's
        // part of the document page, so it goes when no document is open.
        // Clicks go straight through it to the text underneath.
        let word_label = gtk::Label::builder()
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .can_target(false)
            .css_classes(["leafmd-word-count"])
            .build();
        let document_overlay = gtk::Overlay::new();
        document_overlay.set_child(Some(&tab_view));
        document_overlay.add_overlay(&word_label);
        stack.add_named(&document_overlay, Some("document"));

        let toasts = adw::ToastOverlay::new();
        toasts.set_child(Some(&stack));

        // Two layers of bars. The outer one holds the window's controls, which
        // Distraction Free hides. The inner one holds the bars that only appear
        // when they're needed (the file-changed banner and find), which stay.
        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header);
        toolbar_view.add_top_bar(&tab_bar);
        toolbar_view.add_top_bar(&format_bar);
        let file_banner = FileBanner::new();
        let inner_bars = adw::ToolbarView::new();
        inner_bars.add_css_class("leafmd-inner-bars");
        load_window_style();
        inner_bars.add_top_bar(&file_banner.revealer);
        inner_bars.add_top_bar(&find.bar);
        inner_bars.set_content(Some(&toasts));
        toolbar_view.set_content(Some(&inner_bars));

        // ---- Side panel ----
        let files = FilesPage::new();
        let headings = HeadingsPage::new();
        let side_panel = SidePanel::new();
        side_panel.add_page("files", "Files", "leafmd-folder-symbolic", &files.widget);
        side_panel.add_page("headings", "Headings", "leafmd-headings-symbolic", &headings.widget);
        side_panel.set_visible_page(state::load_panel_page());
        let up_button = gtk::Button::builder()
            .icon_name("go-up-symbolic")
            .tooltip_text("Show the folder above")
            .sensitive(false)
            .build();
        side_panel.pack_start(&up_button);
        let panel_wanted = state::load_panel_open();

        // The strip for dragging the panel wider or narrower: see
        // setup_panel_resizing. It lies over the document's left edge rather
        // than the panel's right edge, where the panel's lists have their
        // scrollbars. It's see-through, so the line between them looks the same.
        let resize_handle = gtk::Box::builder()
            .width_request(6)
            .halign(gtk::Align::Start)
            .cursor(&gdk::Cursor::from_name("col-resize", None).expect("GTK knows the resize pointer"))
            .visible(false)
            .build();
        let content = gtk::Overlay::new();
        content.set_child(Some(&toolbar_view));
        content.add_overlay(&resize_handle);

        let split = adw::OverlaySplitView::builder()
            .sidebar(&side_panel.widget)
            .content(&content)
            .min_sidebar_width(f64::from(state::PANEL_WIDTH))
            .max_sidebar_width(f64::from(state::PANEL_WIDTH))
            .show_sidebar(panel_wanted)
            // LeafMD decides when the panel shows as the window changes width
            // (see setup_panel_signals), rather than libadwaita reopening it
            // every time the window gets wide again
            .pin_sidebar(true)
            .build();
        // The split view sits in a holder whose only job is to notice the
        // width it's given, so the panel can give up room before the document
        // does (see setup_panel_resizing)
        let split_holder = WidthWatcher::new(&split);
        window.set_content(Some(&split_holder));

        // Below 920 pixels the panel would squeeze the document, so it slides
        // over it instead. This keeps the document at least 680 pixels wide
        // whenever the panel is beside it, which the header bar needs.
        let narrow = adw::Breakpoint::new(
            adw::BreakpointCondition::parse("max-width: 919sp")
                .expect("the width rule is written correctly"),
        );
        narrow.add_setter(&split, "collapsed", Some(&true.to_value()));
        window.add_breakpoint(narrow);

        // ---- Compact layout for narrow windows ----
        // Below 680 pixels the header bar can't hold everything, so these buttons
        // move to a bar along the bottom. "sp" grows with GNOME's Large Text
        // setting, because the wide layout's words get wider with it too.
        //
        // Having a breakpoint also means the window's smallest size comes from
        // width_request above, rather than from what's inside it. So the wide
        // layout has to fit in 680 pixels or its edges get cut off. It needs
        // about 650 on a desktop with minimize and maximize buttons. Anything
        // added to the header bar eats into that, and also needs moving here.
        //
        // Only one breakpoint applies at a time, the last one added that fits,
        // so this one also slides the side panel over the document, like the
        // one above.
        let bottom_bar = gtk::ActionBar::new();
        bottom_bar.set_visible(false);
        toolbar_view.add_bottom_bar(&bottom_bar);
        let compact = adw::Breakpoint::new(
            adw::BreakpointCondition::parse("max-width: 679sp")
                .expect("the width rule is written correctly"),
        );
        compact.add_setter(&bottom_bar, "visible", Some(&true.to_value()));
        compact.add_setter(&split, "collapsed", Some(&true.to_value()));
        find.add_compact_rules(&compact);
        {
            // The real buttons move, rather than there being a copy in each place,
            // so they keep their state: pressed in, greyed out, which view is active.
            let header = header.clone();
            let bottom_bar = bottom_bar.clone();
            let new_button = new_button.clone();
            let save_button = save_button.clone();
            let find_button = find_button.clone();
            let view_switcher = view_switcher.clone();
            let window = window.clone();
            compact.connect_apply(move |_| {
                // Moving buttons makes the window forget where typing was going,
                // which would stop someone typing mid-word as they cross the width
                let focus = gtk::prelude::GtkWindowExt::focus(&window);
                header.remove(&new_button);
                header.remove(&save_button);
                header.remove(&find_button);
                header.remove(&view_switcher);
                bottom_bar.pack_start(&new_button);
                bottom_bar.pack_start(&save_button);
                bottom_bar.pack_start(&find_button);
                bottom_bar.pack_end(&view_switcher);
                if let Some(focus) = focus {
                    focus.grab_focus();
                }
            });
        }
        {
            let header = header.clone();
            let bottom_bar = bottom_bar.clone();
            let new_button = new_button.clone();
            let save_button = save_button.clone();
            let find_button = find_button.clone();
            let view_switcher = view_switcher.clone();
            let window = window.clone();
            compact.connect_unapply(move |_| {
                let focus = gtk::prelude::GtkWindowExt::focus(&window);
                bottom_bar.remove(&new_button);
                bottom_bar.remove(&save_button);
                bottom_bar.remove(&find_button);
                bottom_bar.remove(&view_switcher);
                // New was the last on the left. The other three fill in from
                // the menu inwards, as they did when the header bar was built.
                header.pack_start(&new_button);
                header.pack_end(&find_button);
                header.pack_end(&view_switcher);
                header.pack_end(&save_button);
                if let Some(focus) = focus {
                    focus.grab_focus();
                }
            });
        }
        window.add_breakpoint(compact);

        let win = Rc::new(Self {
            window,
            windows,
            waiting_files: RefCell::new(VecDeque::new()),
            reading_files: Cell::new(false),
            restored_order: RefCell::new(Vec::new()),
            restoring_files: RefCell::new(Vec::new()),
            restoring_not_started: Cell::new(false),
            title,
            toasts,
            stack,
            view_switcher,
            format_bar,
            find,
            find_button,
            recent_menu,
            recent: RefCell::new(state::load_recent()),
            tab_view,
            shown_document: RefCell::new(Weak::new()),
            loading_find: Cell::new(false),
            adding_file_tab: Cell::new(false),
            closing_tab: Cell::new(false),
            menu_page: glib::WeakRef::new(),
            last_view: Cell::new(saved.view),
            close_confirmed: Cell::new(false),
            file_banner,
            split,
            side_panel,
            up_button,
            went_up: RefCell::new(None),
            files,
            headings,
            word_label,
            folder: RefCell::new(None),
            panel_wanted: Cell::new(panel_wanted),
            panel_width: Cell::new(state::load_panel_width()),
            split_holder,
            holder_width: Cell::new(0),
            resize_handle,
            controls: toolbar_view,
            menu_button,
            distraction_free: Cell::new(false),
            distraction_free_numbers: Cell::new(false),
            menu_from_key: Cell::new(false),
            zoom: Cell::new(text_size::ZOOM_NORMAL),
            zoom_label,
        });
        text_size::install();
        win.window.add_css_class(&text_size::zoom_class(text_size::ZOOM_NORMAL));

        win.setup_actions(saved.view);
        win.setup_signals();
        win.set_document_actions_enabled(false);
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
        add("new-window", |win| {
            win.new_sibling_window();
        });
        add("open", |win| win.ask_open());
        add("open-folder", |win| win.ask_open_folder());
        add("close-document", |win| win.ask_close_document());
        add("close-tab", |win| win.close_tab());
        add("clear-recent", |win| win.clear_recent());

        // The right-click menu on a tab
        add("tab-move-left", |win| {
            if let Some(page) = win.menu_page.upgrade() {
                win.tab_view.reorder_backward(&page);
            }
        });
        add("tab-move-right", |win| {
            if let Some(page) = win.menu_page.upgrade() {
                win.tab_view.reorder_forward(&page);
            }
        });
        add("tab-move-to-new-window", |win| win.move_menu_tab_to_new_window());
        add("tab-close", |win| {
            if let Some(page) = win.menu_page.upgrade() {
                win.tab_view.close_page(&page);
            }
        });

        // Opening a recent file: the menu item carries the file's address
        let open_recent = gio::SimpleAction::new("open-recent", Some(glib::VariantTy::STRING));
        let weak = Rc::downgrade(self);
        open_recent.connect_activate(move |_, param| {
            if let (Some(win), Some(uri)) = (weak.upgrade(), param.and_then(|p| p.str())) {
                win.open_file_here(gio::File::for_uri(uri), None);
            }
        });
        self.window.add_action(&open_recent);
        add("save", |win| {
            if let Some(doc) = win.current() {
                glib::spawn_future_local(async move {
                    doc.save().await;
                });
            }
        });
        add("save-as", |win| {
            if let Some(doc) = win.current() {
                glib::spawn_future_local(async move {
                    doc.save_as().await;
                });
            }
        });
        // Print and the two exports always use the formatted document (export.rs)
        add("print", |win| {
            if let Some(doc) = win.current() {
                export::print(&doc);
            }
        });
        add("export-pdf", |win| {
            if let Some(doc) = win.current() {
                glib::spawn_future_local(async move { export::export_pdf(&doc).await });
            }
        });
        add("export-html", |win| {
            if let Some(doc) = win.current() {
                glib::spawn_future_local(async move { export::export_html(&doc).await });
            }
        });
        add("help", |win| win.show_help());
        add("cheatsheet", |win| win.show_cheatsheet());

        // The formatting buttons and their shortcuts: the target names the format
        let format = gio::SimpleAction::new("format", Some(glib::VariantTy::STRING));
        let weak = Rc::downgrade(self);
        format.connect_activate(move |_, param| {
            if let (Some(doc), Some(name)) =
                (weak.upgrade().and_then(|win| win.current()), param.and_then(|p| p.str()))
            {
                doc.apply_format(name);
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

        // The side panel: on or off. The button and the menu item show its state.
        let panel = gio::SimpleAction::new_stateful(
            "side-panel",
            None,
            &self.split.shows_sidebar().to_variant(),
        );
        let weak = Rc::downgrade(self);
        panel.connect_activate(move |_, _| {
            if let Some(win) = weak.upgrade() {
                win.toggle_panel();
            }
        });
        self.window.add_action(&panel);

        // Which page the side panel shows: "files" or "headings". The
        // switcher at the bottom of the panel changes it; this action lets it
        // be read and set from outside too (the end-to-end tests do).
        let panel_page = gio::SimpleAction::new_stateful(
            "side-panel-page",
            Some(glib::VariantTy::STRING),
            &self.side_panel.visible_page().to_variant(),
        );
        let weak = Rc::downgrade(self);
        panel_page.connect_activate(move |_, param| {
            if let (Some(win), Some(page)) = (weak.upgrade(), param.and_then(|p| p.str())) {
                win.side_panel.set_visible_page(page);
            }
        });
        self.window.add_action(&panel_page);

        // Line Numbers in the menu, ticked while this window shows them
        let line_numbers = gio::SimpleAction::new_stateful(
            "line-numbers",
            None,
            &document::line_numbers_wanted().to_variant(),
        );
        let weak = Rc::downgrade(self);
        line_numbers.connect_activate(move |_, _| {
            if let Some(win) = weak.upgrade() {
                win.toggle_line_numbers();
            }
        });
        self.window.add_action(&line_numbers);

        // Distraction Free: on or off for this window. The menu shows a tick.
        let distraction_free = gio::SimpleAction::new_stateful("distraction-free", None, &false.to_variant());
        let weak = Rc::downgrade(self);
        distraction_free.connect_activate(move |action, _| {
            if let Some(win) = weak.upgrade() {
                let on = !win.distraction_free.get();
                action.set_state(&on.to_variant());
                win.set_distraction_free(on);
            }
        });
        self.window.add_action(&distraction_free);

        // Full screen (F11). The tick follows the window, which can also leave
        // full screen by other routes.
        let fullscreen = gio::SimpleAction::new_stateful("fullscreen", None, &false.to_variant());
        let weak = Rc::downgrade(self);
        fullscreen.connect_activate(move |_, _| {
            if let Some(win) = weak.upgrade() {
                win.window.set_fullscreened(!win.window.is_fullscreen());
            }
        });
        self.window.add_action(&fullscreen);
        let weak = Rc::downgrade(self);
        self.window.connect_fullscreened_notify(move |window| {
            if let Some(win) = weak.upgrade() {
                win.action("fullscreen").set_state(&window.is_fullscreen().to_variant());
            }
        });

        // Zoom: "in", "out" or "reset". Its state is the percentage as text
        // ("110"), which the zoom row in the menu shows.
        let zoom = gio::SimpleAction::new_stateful(
            "zoom",
            Some(glib::VariantTy::STRING),
            &text_size::ZOOM_NORMAL.to_string().to_variant(),
        );
        let weak = Rc::downgrade(self);
        zoom.connect_activate(move |_, param| {
            if let (Some(win), Some(how)) = (weak.upgrade(), param.and_then(|p| p.str())) {
                win.set_zoom(text_size::zoom_after(win.zoom.get(), how));
            }
        });
        self.window.add_action(&zoom);
        // Separate actions for the menu row's two buttons, so each can be greyed
        // out on its own at the smallest and largest sizes
        add("zoom-in", |win| win.set_zoom(text_size::zoom_after(win.zoom.get(), "in")));
        add("zoom-out", |win| win.set_zoom(text_size::zoom_after(win.zoom.get(), "out")));
        add("preferences", |win| crate::preferences::show(&win.window));

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
            if win.current().map(|doc| doc.view_mode.get()) != ViewMode::parse(&name) {
                win.action("view").activate(Some(&name.to_variant()));
            }
        });
    }

    fn setup_signals(self: &Rc<Self>) {
        self.setup_tab_signals();
        self.setup_panel_signals();
        self.setup_panel_resizing();
        self.setup_distraction_free();

        // The banner's buttons act on the document it's showing
        let weak = Rc::downgrade(self);
        self.file_banner.ignore.connect_clicked(move |_| {
            if let Some(doc) = weak.upgrade().and_then(|win| win.current()) {
                doc.ignore_file_change();
            }
        });
        let weak = Rc::downgrade(self);
        self.file_banner.action.connect_clicked(move |_| {
            if let Some(doc) = weak.upgrade().and_then(|win| win.current()) {
                glib::spawn_future_local(async move {
                    doc.banner_action().await;
                });
            }
        });

        // Follow the light or dark style (the system's, or the one chosen in Preferences)
        let weak = Rc::downgrade(self);
        adw::StyleManager::default().connect_dark_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_style();
            }
        });
        // ...and the accent colour chosen in GNOME Settings, which the
        // formatted view uses for links and ticked boxes
        let weak = Rc::downgrade(self);
        adw::StyleManager::default().connect_accent_color_notify(move |_| {
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
            win.open_file_here(file, None);
            true
        });
        self.window.add_controller(drop);

        // A tab dragged from another window can normally only be dropped on a tab
        // bar, and a window showing "No Document Open" has none (the bar hides
        // itself until there are two tabs). Without this, libadwaita treats the
        // drop as "away from any tab bar" and gives the tab a third window. So an
        // empty window takes a dragged tab dropped anywhere on it.
        //
        // While it's dragged, libadwaita takes the tab out of its window, and only
        // its own tab bars can put it into another (it keeps that step private).
        // So the drop is reported as not done, which makes libadwaita put the tab
        // back where it was, and then it's moved here the ordinary way, like Move
        // to New Window does.
        let tab_drop = gtk::DropTarget::new(adw::TabPage::static_type(), gdk::DragAction::MOVE);
        let weak = Rc::downgrade(self);
        tab_drop.connect_accept(move |_, _| {
            weak.upgrade().is_some_and(|win| win.shows_no_document())
        });
        let weak = Rc::downgrade(self);
        tab_drop.connect_drop(move |_, value, _, _| {
            if let (Some(win), Ok(page)) = (weak.upgrade(), value.get::<adw::TabPage>()) {
                win.take_tab_when_back(page);
            }
            false
        });
        self.window.add_controller(tab_drop);

        // Switching to this window: refresh the recent list, which another
        // LeafMD window may have changed, and check the open file, in case it
        // changed somewhere that doesn't send change notices (like a network drive)
        let weak = Rc::downgrade(self);
        self.window.connect_is_active_notify(move |window| {
            if window.is_active()
                && let Some(win) = weak.upgrade()
            {
                win.became_most_recent();
                win.reload_recent();
                if let Some(doc) = win.current() {
                    doc.schedule_file_check();
                }
                // A folder outside the home folder doesn't send change notices,
                // so the tree catches up with changes made in other apps now
                if win.split.shows_sidebar() {
                    win.files.catch_up();
                }
            }
        });

        self.setup_find_signals();

        // Ask before closing with unsaved changes
        let weak = Rc::downgrade(self);
        self.window.connect_close_request(move |_| {
            let Some(win) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            if !win.close_confirmed.get() {
                let unsaved: Vec<_> = win
                    .documents()
                    .into_iter()
                    .filter(|doc| doc.buffer.is_modified())
                    .collect();
                match unsaved.as_slice() {
                    [] => {}
                    [only] => {
                        win.select_document(only);
                        let closing = win.clone();
                        only.confirm_discard(move |_| {
                            closing.close_confirmed.set(true);
                            closing.window.close();
                        });
                        return glib::Propagation::Stop;
                    }
                    _ => {
                        win.ask_about_unsaved_documents(unsaved);
                        return glib::Propagation::Stop;
                    }
                }
            }
            win.save_state();
            // Each window records its tabs as it closes, so the window closed last
            // decides what opens next time. A window with no saved file (start
            // page or a new, never-saved document) leaves that unchanged. This
            // reads the saved recent list because another window may have cleared it.
            let tabs: Vec<_> = win.documents().iter().map(|doc| doc.file_uri()).collect();
            let selected = win
                .tab_view
                .selected_page()
                .map(|page| win.tab_view.page_position(&page) as usize);
            if let Some(tabs) = state::tabs_to_save(&tabs, selected, &state::load_recent()) {
                state::save_tabs(&tabs);
            }
            // The tabs go with the window, so this doesn't count as them being
            // dragged away
            win.closing_tab.set(true);
            for doc in win.documents() {
                doc.forget();
            }
            glib::Propagation::Proceed
        });
    }

    // =====================================================================
    // Side panel
    // =====================================================================

    /// Dragging the panel's edge, and keeping the document at least 680
    /// pixels wide while the panel is beside it.
    fn setup_panel_resizing(self: &Rc<Self>) {
        // Each time the window is laid out, the panel's width is worked out
        // for the new width before the panel and document are laid out, so
        // the two always add up. (Waiting until afterwards, as this first did,
        // never caught up while a window was being dragged narrower by hand,
        // because GTK is busy drawing the whole time.)
        let weak = Rc::downgrade(self);
        self.split_holder.connect_resized(move |width| {
            if let Some(win) = weak.upgrade()
                && win.holder_width.replace(width) != width
            {
                win.update_panel_width();
            }
        });

        // Dragging the strip on the document's left edge. The drag is watched
        // from the box holding everything, which stays put, rather than from
        // the strip, which moves with the panel's edge as it's dragged, so how
        // far the pointer has moved is measured from somewhere fixed.
        // It's checked before anything else sees the press, so a press on the
        // strip doesn't also place the cursor in the document.
        let drag = gtk::GestureDrag::new();
        drag.set_propagation_phase(gtk::PropagationPhase::Capture);
        // The panel's width when the drag started, or None when the press
        // wasn't on the strip, which leaves it to the document as usual
        let start_width: Rc<Cell<Option<i32>>> = Rc::default();
        let weak = Rc::downgrade(self);
        let start = start_width.clone();
        drag.connect_drag_begin(move |drag, x, y| {
            let Some(win) = weak.upgrade() else { return };
            let on_handle = win.resize_handle.is_visible()
                && win
                    .split_holder
                    .pick(x, y, gtk::PickFlags::DEFAULT)
                    .is_some_and(|widget| widget == win.resize_handle);
            if on_handle {
                start.set(Some(win.shown_panel_width()));
                drag.set_state(gtk::EventSequenceState::Claimed);
            } else {
                start.set(None);
                drag.set_state(gtk::EventSequenceState::Denied);
            }
        });
        let weak = Rc::downgrade(self);
        let start = start_width.clone();
        drag.connect_drag_update(move |_, offset_x, _| {
            let (Some(win), Some(start)) = (weak.upgrade(), start.get()) else { return };
            let wanted = start + offset_x.round() as i32;
            win.panel_width.set(wanted.clamp(state::PANEL_WIDTH_MIN, win.panel_width_limit()));
            win.update_panel_width();
        });
        let weak = Rc::downgrade(self);
        drag.connect_drag_end(move |_, _, _| {
            // Saved once the drag is over, and only if the width changed (a
            // click on the strip without moving changes nothing)
            if let (Some(win), Some(start)) = (weak.upgrade(), start_width.take())
                && win.panel_width.get() != start
            {
                state::save_panel_width(win.panel_width.get());
            }
        });
        self.split_holder.add_controller(drag);

        // The strip is only there while the panel sits beside the document
        let weak = Rc::downgrade(self);
        self.split.connect_show_sidebar_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_panel_width();
            }
        });
        let weak = Rc::downgrade(self);
        self.split.connect_collapsed_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.update_panel_width();
            }
        });
        self.update_panel_width();
    }

    /// The most the panel can be beside the document: 400 pixels, or less if
    /// the document would then be narrower than 680 (which the header bar
    /// needs), but never less than 240. "sp" grows with GNOME's Large Text
    /// setting, like the 920sp at which the panel starts sliding over the
    /// document, so the two agree.
    fn panel_width_limit(&self) -> i32 {
        // Before the window is first laid out it has no width yet. Assume
        // there's room, rather than starting at 240 and jumping a moment later.
        let width = self.holder_width.get();
        if width == 0 {
            return state::PANEL_WIDTH_MAX;
        }
        let document_min = adw::LengthUnit::Sp.to_px(680.0, Some(&self.split_holder.settings()));
        let room = width - document_min.round() as i32;
        room.clamp(state::PANEL_WIDTH, state::PANEL_WIDTH_MAX)
    }

    /// The width the panel shows at right now: the chosen width beside the
    /// document (or less if the window hasn't room), and 240 while it slides
    /// over the document in a narrow window.
    fn shown_panel_width(&self) -> i32 {
        if self.split.is_collapsed() {
            state::PANEL_WIDTH
        } else {
            self.panel_width.get().min(self.panel_width_limit())
        }
    }

    /// Sets the panel to the width it should show at, and shows the strip
    /// for dragging it only while the panel sits beside the document.
    fn update_panel_width(&self) {
        let width = f64::from(self.shown_panel_width());
        // The smallest width can't be more than the largest, so whichever
        // would cross the other goes second
        if width > self.split.max_sidebar_width() {
            self.split.set_max_sidebar_width(width);
            self.split.set_min_sidebar_width(width);
        } else {
            self.split.set_min_sidebar_width(width);
            self.split.set_max_sidebar_width(width);
        }
        self.resize_handle.set_visible(!self.split.is_collapsed() && self.split.shows_sidebar());
    }

    fn setup_panel_signals(self: &Rc<Self>) {
        // A window that opens on the Headings page says so from the start
        self.update_panel_title();

        // The panel showing or hiding, by any route: the button, the menu and
        // F9 follow it, and the Files page catches up if it was out of date
        let weak = Rc::downgrade(self);
        self.split.connect_show_sidebar_notify(move |split| {
            if let Some(win) = weak.upgrade() {
                win.action("side-panel").set_state(&split.shows_sidebar().to_variant());
                win.update_files_page();
            }
        });

        // The window getting narrow enough that the panel slides over the
        // document, or wide enough for it to sit beside it again. Narrow, the
        // panel tucks away, and only comes out when asked. Wide, it's back if
        // the user had it open.
        let weak = Rc::downgrade(self);
        self.split.connect_collapsed_notify(move |split| {
            if let Some(win) = weak.upgrade() {
                split.set_show_sidebar(
                    !split.is_collapsed() && win.panel_wanted.get() && !win.distraction_free.get(),
                );
            }
        });

        // Escape tucks the panel away while it's over the document. Checked
        // before anything else sees the key, so it doesn't also close the find bar.
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, _| {
            match weak.upgrade() {
                Some(win)
                    if key == gdk::Key::Escape
                        && win.split.is_collapsed()
                        && win.split.shows_sidebar() =>
                {
                    win.split.set_show_sidebar(false);
                    // Typing goes back to the document the panel was covering
                    if let Some(doc) = win.current() {
                        win.focus_document(&doc);
                    }
                    glib::Propagation::Stop
                }
                // F10 opens the main menu by pressing its button, which can't
                // happen while Distraction Free has the bars hidden. So bring
                // them in first, and open the menu once they've arrived.
                Some(win)
                    if key == gdk::Key::F10
                        && win.distraction_free.get()
                        && !win.controls.reveals_top_bars() =>
                {
                    win.reveal_controls(true);
                    win.menu_from_key.set(true);
                    let weak = Rc::downgrade(&win);
                    win.menu_button.add_tick_callback(move |button, _| {
                        let Some(win) = weak.upgrade() else {
                            return glib::ControlFlow::Break;
                        };
                        let arrived = button
                            .ancestor(gtk::Revealer::static_type())
                            .and_then(|r| r.downcast::<gtk::Revealer>().ok())
                            .is_none_or(|r| r.is_child_revealed());
                        if !win.controls.reveals_top_bars() {
                            win.menu_from_key.set(false);
                            return glib::ControlFlow::Break;
                        }
                        if arrived {
                            button.popup();
                            return glib::ControlFlow::Break;
                        }
                        glib::ControlFlow::Continue
                    });
                    glib::Propagation::Stop
                }
                // Then, once the find bar (which closes with Escape too) is out
                // of the way, it steps back one mode at a time: Full Screen
                // first, then Distraction Free with the next press
                Some(win)
                    if key == gdk::Key::Escape
                        && !win.find.bar.is_search_mode()
                        && (win.distraction_free.get() || win.window.is_fullscreen()) =>
                {
                    if win.window.is_fullscreen() {
                        win.window.set_fullscreened(false);
                    } else {
                        win.action("distraction-free").activate(None);
                    }
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        self.window.add_controller(keys);

        // The up button: one folder further up, for the file you're looking at
        let weak = Rc::downgrade(self);
        self.up_button.connect_clicked(move |_| {
            if let Some(win) = weak.upgrade() {
                win.go_up();
            }
        });

        // Another page chosen in the panel: remembered for new windows and
        // next time, and the title at the top follows it
        let weak = Rc::downgrade(self);
        self.side_panel.connect_page_changed(move |page| {
            let Some(win) = weak.upgrade() else { return };
            win.action("side-panel-page").set_state(&page.to_variant());
            state::save_panel_page(page);
            win.update_panel_title();
        });

        // Picking a heading jumps to it. Over the document, the panel then
        // tucks away, the same as after picking a file.
        let weak = Rc::downgrade(self);
        self.headings.connect_jump(move |line| {
            let Some(win) = weak.upgrade() else { return };
            if win.split.is_collapsed() {
                win.split.set_show_sidebar(false);
            }
            if let Some(doc) = win.current() {
                doc.jump_to_line(line);
                win.headings.set_top_line(line as f64);
            }
        });

        // Picking a file in the tree opens it like any file opened from this
        // window. Over the document, the panel then tucks away.
        let weak = Rc::downgrade(self);
        self.files.connect_open(move |file| {
            if let Some(win) = weak.upgrade() {
                win.open_file_here(file, None);
                if win.split.is_collapsed() {
                    win.split.set_show_sidebar(false);
                }
            }
        });
    }

    /// Turns Distraction Free on or off for this window. On, the header bar,
    /// tab bar, formatting bar and side panel are hidden, and the document
    /// fills the window. The bars slide back over the document while the
    /// pointer is at the top or bottom edge (see setup_distraction_free).
    /// Off, everything comes back as it was, including the side panel if it
    /// was open.
    fn set_distraction_free(&self, on: bool) {
        self.distraction_free.set(on);
        if on {
            self.controls.add_css_class("leafmd-distraction-free");
        } else {
            self.controls.remove_css_class("leafmd-distraction-free");
        }
        self.controls.set_extend_content_to_top_edge(on);
        self.controls.set_extend_content_to_bottom_edge(on);
        self.reveal_controls(!on);
        self.distraction_free_numbers.set(false);
        self.update_line_numbers();
        self.word_label.set_visible(!on);
        if on {
            self.split.set_show_sidebar(false);
        } else {
            self.split.set_show_sidebar(!self.split.is_collapsed() && self.panel_wanted.get());
        }
    }

    fn reveal_controls(&self, reveal: bool) {
        self.controls.set_reveal_top_bars(reveal);
        self.controls.set_reveal_bottom_bars(reveal);
    }

    /// In Distraction Free, brings the bars back while they're wanted: the
    /// pointer at the window's top or bottom edge shows them, and they go again
    /// once it moves back over the document. They stay while one of their menus
    /// is open, and F10 (the main menu's key) brings them back too.
    fn setup_distraction_free(self: &Rc<Self>) {
        // How close to the edge, in pixels, the pointer has to be
        const EDGE: f64 = 6.0;
        // How far past the bars it can wander before they hide again
        const SLACK: f64 = 24.0;
        let motion = gtk::EventControllerMotion::new();
        // Heard before the document gets the movement, as the formatted view
        // might otherwise keep it to itself
        motion.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        motion.connect_motion(move |_, _, y| {
            let Some(win) = weak.upgrade() else { return };
            if !win.distraction_free.get() {
                return;
            }
            let height = f64::from(win.controls.height());
            let shown = win.controls.reveals_top_bars();
            if !shown && (y <= EDGE || y >= height - EDGE) {
                win.reveal_controls(true);
            } else if shown
                && y > f64::from(win.controls.top_bar_height()) + SLACK
                && y < height - f64::from(win.controls.bottom_bar_height()) - SLACK
                && !win.menu_is_open()
                // The bars moving in for F10 count as the pointer moving
                && !win.menu_from_key.get()
            {
                win.reveal_controls(false);
            }
        });
        self.controls.add_controller(motion);

        let weak = Rc::downgrade(self);
        self.menu_button.connect_active_notify(move |button| {
            let Some(win) = weak.upgrade() else { return };
            if !win.distraction_free.get() {
                return;
            }
            if button.is_active() {
                win.reveal_controls(true);
            } else if win.menu_from_key.replace(false) {
                // Opened with F10, so the keyboard is in use: tuck the bars away
                // again rather than waiting for the pointer to move
                win.reveal_controls(false);
            }
        });
    }

    /// Whether a menu from the bars is open (typing goes to it while it is),
    /// so they don't hide from under it.
    fn menu_is_open(&self) -> bool {
        self.menu_button.is_active()
            || gtk::prelude::GtkWindowExt::focus(&self.window)
                .is_some_and(|widget| widget.ancestor(gtk::Popover::static_type()).is_some())
    }

    /// F9, the panel button or Side Panel in the menu. In a wide window this
    /// is remembered, for this window, new windows and next time. In a narrow
    /// one the panel is only sliding out for a moment, so nothing changes.
    fn toggle_panel(&self) {
        let show = !self.split.shows_sidebar();
        if !self.split.is_collapsed() {
            self.panel_wanted.set(show);
            state::save_panel_open(show);
        }
        self.split.set_show_sidebar(show);
    }

    /// Shows the side panel and remembers it as open, for Open Folder… and
    /// `leafmd .`, which ask to see a folder. That includes sliding it over
    /// the document in a narrow window.
    fn show_panel_for_folder(self: &Rc<Self>) {
        self.panel_wanted.set(true);
        state::save_panel_open(true);
        self.split.set_show_sidebar(true);
        // A window that isn't on screen yet doesn't know its width. If it turns
        // out narrow, the panel tucks away as it appears, so it's brought out
        // again once the window is showing.
        if !self.window.is_mapped() {
            let weak = Rc::downgrade(self);
            let handler: Rc<Cell<Option<glib::SignalHandlerId>>> = Rc::default();
            let id = {
                let handler = handler.clone();
                self.window.connect_map(move |window| {
                    if let Some(id) = handler.take() {
                        window.disconnect(id);
                    }
                    let weak = weak.clone();
                    glib::idle_add_local_once(move || {
                        if let Some(win) = weak.upgrade() {
                            win.split.set_show_sidebar(true);
                        }
                    });
                })
            };
            handler.set(Some(id));
        }
    }

    /// Makes `folder` this window's chosen folder and shows it in the panel.
    pub fn choose_folder(self: &Rc<Self>, folder: gio::File) {
        *self.folder.borrow_mut() = Some(folder);
        self.show_panel_for_folder();
        self.update_files_page();
    }

    /// Whether this window's chosen folder is this one.
    pub fn has_folder(&self, folder: &gio::File) -> bool {
        self.folder.borrow().as_ref().is_some_and(|chosen| chosen.equal(folder))
    }

    /// Open Folder…: lets the user pick a folder for the side panel. The
    /// picker shares it with LeafMD wherever it is, including outside home.
    fn ask_open_folder(self: &Rc<Self>) {
        // Start somewhere real: the tree's folder or the document's, else the
        // home folder. Only folders in the home folder are used, because the
        // dialog runs outside the sandbox and LeafMD's paths elsewhere (like a
        // folder shared through the portal) don't mean the same thing there.
        let home = gio::File::for_path(glib::home_dir());
        let near = self
            .files
            .root()
            .map(|(folder, _)| folder)
            .or_else(|| self.current().and_then(|doc| doc.file.borrow().as_ref()?.parent()));
        let start = near.filter(|folder| folder.has_prefix(&home) || folder.equal(&home)).unwrap_or(home);
        let win = self.clone();
        glib::spawn_future_local(async move {
            let dialog = gtk::FileDialog::builder()
                .title("Open Folder")
                .modal(true)
                .initial_folder(&start)
                .build();
            let Ok(folder) = dialog.select_folder_future(Some(&win.window)).await else { return };
            // An empty window takes the folder. One with a document open
            // doesn't, so its panel never shows a folder unrelated to the
            // document beside it: the folder goes where `leafmd .` would put
            // it (a window that already has it, an empty one, or a new one).
            if win.is_empty() {
                win.choose_folder(folder);
            } else if let Some(app) =
                win.window.application().and_then(|app| app.downcast::<adw::Application>().ok())
            {
                open_folder(&app, &win.windows, folder);
            }
        });
    }

    /// Points the Files page at the folder to show (see
    /// `folder_tree::folder_to_show` for the rules), and highlights the
    /// selected tab's file. Nothing is read while the panel is hidden; it
    /// catches up when shown.
    fn update_files_page(self: &Rc<Self>) {
        // A file going into a new tab: the tab is selected a moment before the
        // file is put in it. Without this the tree would empty itself for that
        // moment (an Untitled tab has no folder) and rebuild, closing the
        // folders that were open in it.
        if !self.split.shows_sidebar() || self.adding_file_tab.get() {
            return;
        }
        let current = self.current().filter(|_| {
            self.stack.visible_child_name().as_deref() == Some("document")
        });
        let file = current.and_then(|doc| doc.file.borrow().clone());
        let chosen = self.folder.borrow().clone();
        let (mut folder, mut shared_whole) = folder_tree::folder_to_show(chosen.as_ref(), file.as_ref());
        // As many folders up as the user went with the up button, for this file
        let key = file.as_ref().map(|file| file.uri().to_string());
        let levels = match &*self.went_up.borrow() {
            Some((went_for, levels)) if *went_for == key => *levels,
            _ => 0,
        };
        for _ in 0..levels {
            let Some(shown) = folder.as_ref() else { break };
            let Some((above, above_shared)) =
                folder_tree::folder_above(shown, chosen.as_ref(), folder_tree::can_show_unchosen)
            else {
                break;
            };
            (folder, shared_whole) = (Some(above), above_shared);
        }
        self.files.set_root(folder.as_ref(), shared_whole);
        self.update_panel_title();
        self.update_up_button();
        self.files.set_highlight(file);
    }

    /// The up button: shows one folder further up than the Files page does now.
    fn go_up(self: &Rc<Self>) {
        let file = self
            .current()
            .filter(|_| self.stack.visible_child_name().as_deref() == Some("document"))
            .and_then(|doc| doc.file.borrow().clone());
        let key = file.map(|file| file.uri().to_string());
        let levels = match &*self.went_up.borrow() {
            Some((went_for, levels)) if *went_for == key => *levels,
            _ => 0,
        };
        *self.went_up.borrow_mut() = Some((key, levels + 1));
        self.update_files_page();
    }

    /// Greys out the up button when there's nothing above the Files page's
    /// folder that it could list, and names the folder above in its tooltip.
    fn update_up_button(&self) {
        let chosen = self.folder.borrow().clone();
        let above = self.files.root().and_then(|(folder, _)| {
            folder_tree::folder_above(&folder, chosen.as_ref(), folder_tree::can_show_unchosen)
        });
        // The very top of the disk is called "/"
        let name = above.as_ref().and_then(|(above, _)| above.basename()).map(|name| name.display().to_string());
        self.up_button.set_sensitive(above.is_some());
        self.up_button.set_tooltip_text(Some(&match name {
            Some(name) => format!("Show the folder above ({name})"),
            None => "Show the folder above".to_string(),
        }));
    }

    /// The title at the top of the side panel: the Files page's folder name,
    /// or "Headings".
    fn update_panel_title(&self) {
        self.up_button.set_visible(self.side_panel.visible_page() != "headings");
        let title = if self.side_panel.visible_page() == "headings" {
            "Headings".to_string()
        } else {
            self.files
                .root()
                .and_then(|(folder, _)| folder.basename())
                .map(|name| name.display().to_string())
                .unwrap_or_else(|| "Files".to_string())
        };
        self.side_panel.set_title(&title);
    }

    // =====================================================================
    // Headings and word count
    // =====================================================================

    /// The selected document's headings or word count changed (after typing,
    /// or a file loading): the Headings page and the label catch up.
    pub fn document_facts_changed(&self, doc: &Document) {
        self.headings.show(Some(&doc.headings()));
        self.headings.set_top_line(doc.top_line());
        self.update_word_label();
    }

    /// Shows the selected document's word count, or its selection's.
    pub fn update_word_label(&self) {
        if let Some(doc) = self.current() {
            self.word_label.set_label(&doc.word_label());
        }
    }

    /// The selected document scrolled: the Headings page highlights the
    /// section now at the top.
    pub fn top_line_changed(&self, line: f64) {
        self.headings.set_top_line(line);
    }

    // =====================================================================
    // Tabs
    // =====================================================================

    fn setup_tab_signals(self: &Rc<Self>) {
        // Next and previous tab with Ctrl+Page Down and Up, and gedit's
        // Ctrl+Alt+Page Down and Up. The capture phase means the window sees these
        // first, because the raw text has its own use for Ctrl+Page Down: moving
        // the cursor sideways, which it would do instead of changing tabs.
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            let step = match key {
                gdk::Key::Page_Down | gdk::Key::KP_Page_Down => 1,
                gdk::Key::Page_Up | gdk::Key::KP_Page_Up => -1,
                _ => return glib::Propagation::Proceed,
            };
            if !modifiers.contains(gdk::ModifierType::CONTROL_MASK)
                || modifiers.contains(gdk::ModifierType::SHIFT_MASK)
            {
                return glib::Propagation::Proceed;
            }
            let Some(win) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            // Tabs don't change behind an open dialog
            if win.window.visible_dialog().is_some() {
                return glib::Propagation::Proceed;
            }
            win.select_neighbour_tab(step);
            glib::Propagation::Stop
        });
        self.window.add_controller(keys);

        // Selecting a tab: the bars around the document switch over to it
        let weak = Rc::downgrade(self);
        self.tab_view.connect_selected_page_notify(move |_| {
            if let Some(win) = weak.upgrade() {
                win.selected_tab_changed();
            }
        });

        // Closing a tab, by any route, comes through here. A tab with unsaved
        // changes only closes once the user has answered.
        let weak = Rc::downgrade(self);
        self.tab_view.connect_close_page(move |_, page| {
            if let Some(win) = weak.upgrade() {
                win.close_page(page.clone());
            }
            glib::Propagation::Stop
        });

        // A tab has been dragged out of the tab bar and dropped away from any
        // other window: this makes the window it lands in
        let weak = Rc::downgrade(self);
        self.tab_view.connect_create_window(move |_| {
            weak.upgrade()?.new_sibling_window().map(|win| win.tab_view.clone())
        });

        // Right-clicking a tab: note which one, and what can be done with it.
        // The menu closing also comes through here, with no tab. That's ignored,
        // because it can happen just before the chosen item runs.
        let weak = Rc::downgrade(self);
        self.tab_view.connect_setup_menu(move |view, page| {
            if let (Some(win), Some(page)) = (weak.upgrade(), page) {
                win.menu_page.set(Some(page));
                let position = view.page_position(page);
                win.action("tab-move-left").set_enabled(position > 0);
                win.action("tab-move-right").set_enabled(position < view.n_pages() - 1);
            }
        });

        // Adding a tab comes through here too, but only a tab arriving from
        // another window needs anything done: it now belongs to this window, and
        // the bars around it have to show what it remembers. The window it landed
        // in may also have been on the "No Document Open" page.
        let weak = Rc::downgrade(self);
        self.tab_view.connect_page_attached(move |_, page, _| {
            let (Some(win), Some(doc)) = (weak.upgrade(), Document::for_widget(&page.child()))
            else {
                return;
            };
            if doc.window().is_some_and(|window| Rc::ptr_eq(&window, &win)) {
                return;
            }
            doc.set_window(Rc::downgrade(&win));
            doc.show_line_numbers(win.wants_line_numbers());
            doc.set_zoom(win.zoom.get());
            win.update_tab(&doc);
            win.reveal_document();
            win.selected_tab_changed();
            win.update_recent_menu();
            // Typing and Ctrl+Z should reach the document that just moved. This
            // waits until the move has finished, because a window that has only
            // just opened picks its own focus first.
            let weak = Rc::downgrade(&win);
            glib::idle_add_local_once(move || {
                if let Some(win) = weak.upgrade()
                    && win.is_current(&doc)
                {
                    win.focus_document(&doc);
                }
            });
        });

        // A tab left for another window. If it was the only one, there's nothing
        // left in this window, so it closes. Otherwise that file can now be
        // offered in this window's recent files.
        let weak = Rc::downgrade(self);
        self.tab_view.connect_page_detached(move |view, _, _| {
            let Some(win) = weak.upgrade().filter(|win| !win.closing_tab.get()) else {
                return;
            };
            if view.n_pages() == 0 {
                win.window.close();
            } else {
                win.update_recent_menu();
            }
        });
    }

    /// Opens another window on the "No Document Open" page, starting in the view
    /// last chosen in this one.
    fn new_sibling_window(&self) -> Option<Rc<Window>> {
        let app = self.window.application().and_downcast::<adw::Application>()?;
        let win = Window::open_new(&app, &self.windows);
        win.start_in_view(self.last_view.get());
        win.window.present();
        Some(win)
    }

    /// Sets the view for a window that has no tabs yet.
    fn start_in_view(&self, view: ViewMode) {
        self.last_view.set(view);
        self.action("view").set_state(&view.as_str().to_variant());
        self.view_switcher.set_active_name(Some(view.as_str()));
    }

    /// Move to New Window in a tab's right-click menu.
    fn move_menu_tab_to_new_window(&self) {
        if let (Some(page), Some(win)) = (self.menu_page.upgrade(), self.new_sibling_window()) {
            self.tab_view.transfer_page(&page, &win.tab_view, 0);
        }
    }

    /// Makes the text in every tab of this window bigger or smaller: swaps the
    /// window's zoom style name (for the raw text) and tells each document (for
    /// the formatted view and the raw text's column).
    fn set_zoom(&self, zoom: u32) {
        let old = self.zoom.replace(zoom);
        self.window.remove_css_class(&text_size::zoom_class(old));
        self.window.add_css_class(&text_size::zoom_class(zoom));
        text_size::zoom_documents(&self.documents(), zoom);
        self.action("zoom").set_state(&zoom.to_string().to_variant());
        self.zoom_label.set_label(&format!("{zoom}%"));
        self.action("zoom-in").set_enabled(zoom < text_size::ZOOM_MAX);
        self.action("zoom-out").set_enabled(zoom > text_size::ZOOM_MIN);
    }

    /// Every document in this window, in tab order.
    fn documents(&self) -> Vec<Rc<Document>> {
        (0..self.tab_view.n_pages())
            .filter_map(|position| Document::for_widget(&self.tab_view.nth_page(position).child()))
            .collect()
    }

    /// Selects a document's tab.
    fn select_document(&self, doc: &Document) {
        self.tab_view.set_selected_page(&self.tab_view.page(&doc.paned));
    }

    /// A new, empty document that belongs to this window, not in a tab yet.
    fn new_document(self: &Rc<Self>, view: ViewMode) -> Rc<Document> {
        let doc = Document::new(view);
        doc.set_window(Rc::downgrade(self));
        doc.source_view.set_show_line_numbers(self.wants_line_numbers());
        doc.set_zoom(self.zoom.get());
        doc.apply_view(view);
        doc.update_style();
        doc
    }

    /// Adds a tab with a new, empty document and selects it.
    fn add_tab(self: &Rc<Self>, view: ViewMode) -> Rc<Document> {
        let doc = self.new_document(view);
        // Selected straight away, so the tab bar never shows it unselected. The
        // first tab in a window is selected by this too.
        let page = self.tab_view.append(&doc.paned);
        self.update_tab(&doc);
        self.tab_view.set_selected_page(&page);
        doc
    }

    /// Shows a document's name and location on its tab.
    pub fn update_tab(&self, doc: &Document) {
        let page = self.tab_view.page(&doc.paned);
        let dot = if doc.buffer.is_modified() { "• " } else { "" };
        page.set_title(&format!("{dot}{}", doc.display_name()));
        // A tab's tooltip is read as markup, so a name like "R&D.md" needs escaping
        let location = match doc.folder().as_str() {
            "" => doc.display_name(),
            folder => format!("{}/{}", folder.trim_end_matches('/'), doc.display_name()),
        };
        page.set_tooltip(&glib::markup_escape_text(&location));
    }

    /// Another tab was selected: everything around the document follows it.
    fn selected_tab_changed(&self) {
        // Going up with the up button lasts until another tab is selected
        *self.went_up.borrow_mut() = None;
        let doc = self.current();
        // Keep what the find bar was showing with the tab it belonged to
        if let Some(previous) = self.shown_document.borrow().upgrade() {
            let same = doc.as_ref().is_some_and(|doc| Rc::ptr_eq(doc, &previous));
            if !same {
                self.save_find_state(&previous);
            }
        }
        *self.shown_document.borrow_mut() = doc.as_ref().map_or_else(Weak::new, Rc::downgrade);

        let Some(doc) = doc else { return };
        doc.draw_preview_if_waiting();
        self.show_view_of(&doc);
        self.load_find_state(&doc);
        self.update_format_bar();
        self.update_title();
        doc.update_file_banner();
        self.document_facts_changed(&doc);
        // In case the file changed somewhere that doesn't send change notices
        doc.schedule_file_check();
    }

    /// Makes the view switcher show this document's view. Doesn't change the view
    /// again, because the document is already in it.
    fn show_view_of(&self, doc: &Document) {
        let view = doc.view_mode.get();
        self.action("view").set_state(&view.as_str().to_variant());
        self.view_switcher.set_active_name(Some(view.as_str()));
    }

    fn save_find_state(&self, doc: &Document) {
        let mut state = doc.find_state.borrow_mut();
        state.open = self.find.bar.is_search_mode();
        state.query = self.find.entry.text().to_string();
        state.replace_open = self.find.replace_toggle.is_active();
        state.replace_text = self.find.replace_entry.text().to_string();
        let focus = gtk::prelude::GtkWindowExt::focus(&self.window);
        state.focus_in_bar = focus.as_ref().is_some_and(|focus| focus.is_ancestor(&self.find.bar));
        state.focus_in_replace =
            focus.is_some_and(|focus| focus.is_ancestor(&self.find.replace_entry));
    }

    /// Puts a tab's find state into the find bar, without moving its cursor.
    fn load_find_state(&self, doc: &Document) {
        let (open, query, replace_open, replace_text) = {
            let state = doc.find_state.borrow();
            (state.open, state.query.clone(), state.replace_open, state.replace_text.clone())
        };
        // Normally the tab is searching for this already, and setting it again
        // does nothing. It differs if the tab was left within a moment of typing,
        // before the search had started.
        if open {
            doc.search_settings
                .set_search_text(Some(query.as_str()).filter(|query| !query.is_empty()));
        }
        self.loading_find.set(true);
        // Setting the text empties the box first, even to the same text
        if self.find.entry.text() != query {
            self.find.entry.set_text(&query);
        }
        self.find.replace_entry.set_text(&replace_text);
        self.find.replace_toggle.set_active(replace_open);
        self.find.bar.set_search_mode(open);
        self.loading_find.set(false);

        // Match Case and Whole Words Only apply to every tab in the window
        doc.search_settings
            .set_case_sensitive(self.find_option("find-match-case"));
        doc.search_settings
            .set_at_word_boundaries(self.find_option("find-whole-words"));
        if !open {
            self.find.show_count(0, 0);
        } else if doc.searching_preview() {
            self.preview_resume_find();
        } else {
            self.update_raw_find_count();
        }

        // GTK puts the focus in the find box whenever the bar opens. Someone who
        // was typing in the document should carry on there, or their next letters
        // would change the search. Not while a dialog is asking something, because
        // the focus belongs in that.
        if open && self.window.visible_dialog().is_none() {
            let (in_bar, in_replace) = {
                let state = doc.find_state.borrow();
                (state.focus_in_bar, state.focus_in_replace)
            };
            // The formatted view has no replace box
            if in_replace && !doc.searching_preview() {
                self.find.replace_entry.grab_focus();
            } else if in_bar {
                self.find.entry.grab_focus();
            } else {
                self.focus_document(doc);
            }
        }
    }

    /// Goes to the next (`step` 1) or previous (`step` -1) tab, wrapping round.
    fn select_neighbour_tab(&self, step: i32) {
        let count = self.tab_view.n_pages();
        let Some(selected) = self.tab_view.selected_page().filter(|_| count > 1) else {
            return;
        };
        let position = (self.tab_view.page_position(&selected) + step).rem_euclid(count);
        self.tab_view.set_selected_page(&self.tab_view.nth_page(position));
    }

    /// Ctrl+W: closes the selected tab, or the window if that's the last tab.
    fn close_tab(&self) {
        if self.tab_view.n_pages() <= 1 {
            self.window.close();
        } else if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.close_page(&page);
        }
    }

    /// Asks about unsaved changes, then closes the tab if the user agrees.
    fn close_page(self: &Rc<Self>, page: adw::TabPage) {
        let Some(doc) = Document::for_widget(&page.child()) else {
            self.finish_closing_page(&page, true);
            return;
        };
        if !doc.buffer.is_modified() {
            self.finish_closing_page(&page, true);
            doc.forget();
            return;
        }
        // So it's clear which document the question is about
        self.tab_view.set_selected_page(&page);
        let win = self.clone();
        glib::spawn_future_local(async move {
            let close = doc.ask_to_save().await;
            win.finish_closing_page(&page, close);
            if close {
                doc.forget();
            }
        });
    }

    fn finish_closing_page(&self, page: &adw::TabPage, close: bool) {
        self.closing_tab.set(true);
        self.tab_view.close_page_finish(page, close);
        self.closing_tab.set(false);
        if close && self.tab_view.n_pages() == 0 {
            self.show_start_page();
        } else if close {
            self.update_recent_menu();
        }
    }

    /// The last tab was closed but the window stays: back to "No Document Open".
    /// LeafMD starts on that page next time too.
    fn show_start_page(&self) {
        self.loading_find.set(true);
        self.find.bar.set_search_mode(false);
        self.loading_find.set(false);
        self.file_banner.hide();
        self.stack.set_visible_child_name("empty");
        self.headings.show(None);
        self.set_document_actions_enabled(false);
        // After switching page, so the title goes back to "LeafMD"
        self.update_title();
        // The closed file was left out of the list while open; show it again
        self.update_recent_menu();
        state::forget_last_file();
    }

    /// The question when closing a window with two or more unsaved tabs: one
    /// dialog listing them, with a tick box for each, like gedit's.
    fn ask_about_unsaved_documents(self: &Rc<Self>, unsaved: Vec<Rc<Document>>) {
        let question = document::save_changes_question(
            &format!(
                "There are {} documents with unsaved changes. Save changes before closing?",
                unsaved.len()
            ),
            "Select the documents you want to save:",
            "_Save",
        );

        // One line for each document: its tick box and name, with its folder to
        // tell apart two files with the same name. The whole line can be clicked,
        // because the name is inside the tick box. Scrolls if there are many.
        let list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .build();
        let mut ticks = Vec::new();
        for doc in &unsaved {
            let name = gtk::Label::builder()
                .label(doc.display_name())
                .ellipsize(gtk::pango::EllipsizeMode::Middle)
                .xalign(0.0)
                .build();
            let folder = gtk::Label::builder()
                .label(doc.folder())
                .ellipsize(gtk::pango::EllipsizeMode::Start)
                .xalign(1.0)
                .hexpand(true)
                .css_classes(["dim-label", "caption"])
                .build();
            let line = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            line.append(&name);
            line.append(&folder);
            let tick = gtk::CheckButton::builder()
                .active(true)
                .child(&line)
                .margin_top(3)
                .margin_bottom(3)
                .margin_start(6)
                .margin_end(9)
                .build();
            list.append(
                &gtk::ListBoxRow::builder()
                    .child(&tick)
                    .activatable(false)
                    .focusable(false)
                    .build(),
            );
            ticks.push(tick);
        }
        for tick in &ticks {
            // Discarding everything is always the clearly labelled button
            let (save, all) = (question.button("save"), ticks.clone());
            tick.connect_toggled(move |_| {
                if let Some(save) = &save {
                    save.set_sensitive(all.iter().any(|tick| tick.is_active()));
                }
            });
        }
        let warning = gtk::Label::builder()
            .label("If you don't save, all your changes will be permanently lost.")
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        question.add_extra(
            &gtk::ScrolledWindow::builder()
                .child(&list)
                .has_frame(true)
                .hscrollbar_policy(gtk::PolicyType::Never)
                .propagate_natural_height(true)
                .max_content_height(200)
                .build(),
        );
        question.add_extra(&warning);

        let win = self.clone();
        glib::spawn_future_local(async move {
            match question.ask(Some(&win.window)).await.as_str() {
                "discard" => {}
                "save" => {
                    let chosen = unsaved.iter().zip(&ticks).filter(|(_, tick)| tick.is_active());
                    for (doc, _) in chosen {
                        // So any further question is clearly about this document
                        win.select_document(doc);
                        // Stopping here keeps the window open, so nothing is
                        // lost, including the documents that weren't ticked
                        if !doc.save().await {
                            return;
                        }
                    }
                }
                _ => return,
            }
            win.close_confirmed.set(true);
            win.window.close();
        });
    }


    // =====================================================================
    // Views and editing
    // =====================================================================

    fn apply_view(&self, mode: ViewMode) {
        let Some(doc) = self.current() else { return };
        doc.apply_view(mode);
        self.last_view.set(mode);
        self.update_format_bar();
        // A selection only counts while the raw text shows, and the section
        // at the top now comes from whichever view is showing
        self.update_word_label();
        self.headings.set_top_line(doc.top_line());
        // An open find bar now searches whichever text is showing
        if self.find.bar.is_search_mode() {
            self.refresh_find();
        }
    }

    /// The formatting bar (and its shortcuts) only work where the raw text can be edited.
    fn update_format_bar(&self) {
        let editable = self.stack.visible_child_name().as_deref() == Some("document")
            && self.current().is_some_and(|doc| !doc.searching_preview());
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
        find.entry.connect_search_changed(move |entry| {
            let Some(win) = weak.upgrade() else { return };
            // The box also changes when another tab is selected. That tab has
            // already searched for this, and its cursor should stay where it is.
            // The box being emptied on the way arrives straight away, while the
            // tab's text is being put in. The text itself arrives a moment later,
            // when it matches what the tab remembers.
            if win.loading_find.get() {
                return;
            }
            let unchanged = win
                .current()
                .is_some_and(|doc| doc.find_state.borrow().query == entry.text());
            if !unchanged {
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
                && !win.loading_find.get()
            {
                win.close_find();
            }
        });

    }

    /// Ctrl+F (or Ctrl+H with `replace`): shows the find bar and puts the
    /// cursor in it, starting with the selected text if there is some.
    fn open_find(self: &Rc<Self>, replace: bool) {
        let Some(doc) = self.current() else { return };
        let find = &self.find;
        if !doc.searching_preview()
            && let Some((start, end)) = doc.buffer.selection_bounds()
        {
            let selected = doc.buffer.text(&start, &end, false);
            if !selected.contains('\n') && selected.chars().count() <= 200 {
                find.entry.set_text(&selected);
            }
        }
        if replace && !doc.searching_preview() {
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
        let Some(doc) = self.current() else { return };
        doc.search_settings.set_search_text(None);
        doc.run_js("window.leafmd && window.leafmd.clearFind();");
        self.focus_document(&doc);
    }

    /// Puts the keyboard's focus in whichever text this document is showing.
    fn focus_document(&self, doc: &Document) {
        if doc.searching_preview() {
            doc.webview.grab_focus();
        } else {
            doc.source_view.grab_focus();
        }
    }

    /// The search text, an option or the view changed: search again from the
    /// current spot, so typing more letters keeps the same match where it can.
    pub fn refresh_find(&self) {
        let Some(doc) = self.current() else { return };
        let find = &self.find;
        let query = find.query();
        doc.find_state.borrow_mut().query = query.clone().unwrap_or_default();
        doc.search_settings
            .set_case_sensitive(self.find_option("find-match-case"));
        doc.search_settings
            .set_at_word_boundaries(self.find_option("find-whole-words"));
        doc.search_settings.set_search_text(query.as_deref());

        if doc.searching_preview() {
            self.preview_find(0);
            return;
        }
        // Only one side shows highlights at a time
        doc.run_js("window.leafmd && window.leafmd.clearFind();");
        if query.is_some() {
            let from = match doc.buffer.selection_bounds() {
                Some((start, _)) => start,
                None => doc.buffer.iter_at_mark(&doc.buffer.get_insert()),
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
        let Some(doc) = self.current() else { return };
        if !self.find.bar.is_search_mode() || self.find.query().is_none() {
            self.open_find(false);
            return;
        }
        if doc.searching_preview() {
            self.preview_find(step);
            return;
        }
        let from = match doc.buffer.selection_bounds() {
            Some((start, end)) => {
                if step > 0 {
                    end
                } else {
                    start
                }
            }
            None => doc.buffer.iter_at_mark(&doc.buffer.get_insert()),
        };
        self.raw_find_from(&from, step < 0);
    }

    /// Selects the first match after (or before) `from` in the raw text and
    /// scrolls it into view. Wraps around at the end of the document.
    fn raw_find_from(&self, from: &gtk::TextIter, backward: bool) {
        let Some(doc) = self.current() else { return };
        let context = &doc.search_context;
        let found = if backward {
            context.backward(from)
        } else {
            context.forward(from)
        };
        if let Some((start, end, _wrapped)) = found {
            doc.buffer.select_range(&start, &end);
            // Leave some room above and below, rather than at the very edge
            doc.source_view
                .scroll_to_mark(&doc.buffer.get_insert(), 0.15, false, 0.0, 0.0);
        }
        self.update_raw_find_count();
    }

    /// Updates "3 of 12" for the raw text.
    pub fn update_raw_find_count(&self) {
        let Some(doc) = self.current() else { return };
        if !self.find.bar.is_search_mode() || doc.searching_preview() {
            return;
        }
        let context = &doc.search_context;
        let position = doc
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
        let Some(doc) = self.current() else { return };
        let Some(query) = self.find.query() else {
            doc.run_js("window.leafmd && window.leafmd.clearFind();");
            self.find.show_count(0, 0);
            return;
        };
        let query = serde_json::to_string(&query).unwrap_or_default();
        let match_case = self.find_option("find-match-case");
        let whole_words = self.find_option("find-whole-words");
        doc.run_js(&format!(
            "window.leafmd && window.leafmd.find({query}, {match_case}, {whole_words}, {step});"
        ));
    }

    /// A tab showing the formatted view was selected with its find bar open. Its
    /// page remembers the search, so this only gets the count back for the find
    /// bar, without moving to another match or scrolling. If the search options
    /// changed while the tab was away, it searches again.
    fn preview_resume_find(&self) {
        let Some(doc) = self.current() else { return };
        let Some(query) = self.find.query() else {
            self.preview_find(0);
            return;
        };
        let query = serde_json::to_string(&query).unwrap_or_default();
        let match_case = self.find_option("find-match-case");
        let whole_words = self.find_option("find-whole-words");
        doc.run_js(&format!(
            "window.leafmd && window.leafmd.resumeFind({query}, {match_case}, {whole_words});"
        ));
    }

    /// Replaces the selected match, then moves to the next one. If no match is
    /// selected yet, it just goes to the next one, so you see what will change.
    fn replace_one(self: &Rc<Self>) {
        let Some(doc) = self.current() else { return };
        if doc.searching_preview() || self.find.query().is_none() {
            return;
        }
        let replacement = self.find.replace_entry.text();
        if let Some((mut start, mut end)) = doc.buffer.selection_bounds()
            && doc.search_context.occurrence_position(&start, &end) > 0
        {
            doc.buffer.begin_user_action();
            let replaced = doc.search_context.replace(&mut start, &mut end, &replacement);
            doc.buffer.end_user_action();
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
        let Some(doc) = self.current() else { return };
        if doc.searching_preview() || self.find.query().is_none() {
            return;
        }
        let count = doc.search_context.occurrences_count();
        let replacement = self.find.replace_entry.text();
        doc.buffer.begin_user_action();
        let result = doc.search_context.replace_all(&replacement);
        doc.buffer.end_user_action();
        // Clicking the button left the focus on it, where Ctrl+Z does nothing.
        // Only the raw text can undo, so that's where the focus goes.
        if result.is_ok() && count > 0 {
            doc.source_view.grab_focus();
        }
        match result {
            Ok(()) if count == 1 => self.toast("Replaced 1 match"),
            Ok(()) if count > 1 => self.toast(&format!("Replaced {count} matches")),
            Ok(()) => {}
            Err(err) => self.toast(&format!("Couldn't replace: {}", err.message())),
        }
        self.update_raw_find_count();
    }


    /// Shows the open document. If it has just been loaded into an empty
    /// window and the formatted view is still drawing it, waits for that first
    /// (at most REVEAL_TIMEOUT), so the document appears all at once.
    fn show_document(self: &Rc<Self>) {
        let Some(doc) = self.current() else { return };
        let opening = self.stack.visible_child_name().as_deref() == Some("loading");
        if opening && doc.webview.is_visible() && !doc.preview_ready.get() {
            doc.reveal_pending.set(true);
            let weak = Rc::downgrade(self);
            glib::timeout_add_local_once(REVEAL_TIMEOUT, move || {
                if let Some(win) = weak.upgrade()
                    && win.current().is_some_and(|doc| doc.reveal_pending.get())
                {
                    win.reveal_document();
                }
            });
            return;
        }
        self.reveal_document();
    }

    pub fn reveal_document(&self) {
        for doc in self.documents() {
            doc.reveal_pending.set(false);
        }
        self.stack.set_visible_child_name("document");
        self.set_document_actions_enabled(true);
        self.update_title();
    }

    /// Moves a tab that was dropped on this empty window into it, once
    /// libadwaita has put it back in the window it was dragged from (see the tab
    /// drop target in setup_signals). Checks every frame-or-so for up to a
    /// second; if the tab never reappears, nothing happens.
    fn take_tab_when_back(self: &Rc<Self>, page: adw::TabPage) {
        let weak = Rc::downgrade(self);
        let mut tries = 0;
        glib::timeout_add_local(Duration::from_millis(20), move || {
            tries += 1;
            let Some(win) = weak.upgrade() else {
                return glib::ControlFlow::Break;
            };
            let source = win.windows.borrow().iter().find_map(|other| {
                let view = &other.tab_view;
                (0..view.n_pages())
                    .any(|position| view.nth_page(position) == page)
                    .then(|| view.clone())
            });
            match source {
                Some(view) if view == win.tab_view => glib::ControlFlow::Break,
                Some(view) => {
                    view.transfer_page(&page, &win.tab_view, 0);
                    glib::ControlFlow::Break
                }
                None if tries < 50 => glib::ControlFlow::Continue,
                None => glib::ControlFlow::Break,
            }
        });
    }

    /// Whether the window is showing the "No Document Open" page.
    fn shows_no_document(&self) -> bool {
        self.stack.visible_child_name().as_deref() == Some("empty")
    }

    /// Swaps "No Document Open" for a blank page while a file opens.
    fn begin_loading(&self) {
        if self.shows_no_document() {
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
    // Files
    // =====================================================================

    /// New: an empty document in a new tab. Nothing is asked, because no other
    /// document is replaced.
    fn ask_new(self: &Rc<Self>) {
        // A new document needs somewhere to type, so show the raw text
        let view = match self.last_view.get() {
            ViewMode::Preview => ViewMode::Split,
            view => view,
        };
        let doc = self.add_tab(view);
        doc.set_file(None);
        self.show_document();
        doc.source_view.grab_focus();
    }

    /// Close Document: closes the selected tab. If it's the last one, the
    /// window stays open on the "No Document Open" page.
    fn ask_close_document(self: &Rc<Self>) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.close_page(&page);
        }
    }

    fn ask_open(self: &Rc<Self>) {
        let win = self.clone();
        glib::spawn_future_local(async move {
            let dialog = gtk::FileDialog::builder()
                .title("Open Markdown File")
                .modal(true)
                .filters(&file_filters())
                .build();
            if let Ok(file) = dialog.open_future(Some(&win.window)).await {
                win.open_file_here(file, None);
            }
        });
    }

    /// Opens a file asked for from this window: Open, Recent files, a dropped
    /// file or a link. Scrolls to `heading` if one is given. Nothing is asked
    /// about unsaved changes, because no other document is replaced.
    pub fn open_file_here(self: &Rc<Self>, file: gio::File, heading: Option<String>) {
        if let Some(app) = self
            .window
            .application()
            .and_then(|app| app.downcast::<adw::Application>().ok())
        {
            open_file(&app, &self.windows, file, heading, Some(self));
        }
    }

    /// Moves this window to the end of the list of windows.
    fn became_most_recent(self: &Rc<Self>) {
        let mut list = self.windows.borrow_mut();
        if let Some(position) = list.iter().position(|win| Rc::ptr_eq(win, self)) {
            let win = list.remove(position);
            list.push(win);
        }
    }

    /// Whether one of this window's tabs shows this file.
    pub fn has_file(&self, uri: &str) -> bool {
        self.documents().iter().any(|doc| doc.file_uri().as_deref() == Some(uri))
            || self.waiting_files.borrow().iter().any(|waiting| waiting.file.uri() == uri)
            || self.restoring_files.borrow().iter().any(|restoring| restoring == uri)
    }

    /// True while the window shows the "No Document Open" page with nothing on the way.
    pub fn is_empty(&self) -> bool {
        self.tab_view.n_pages() == 0
            && self.waiting_files.borrow().is_empty()
            && self.restoring_files.borrow().is_empty()
    }

    /// Selects the tab showing this file, scrolled to `heading` if one is given.
    pub fn select_file(&self, uri: &str, heading: Option<&str>) {
        let documents = self.documents();
        let Some(doc) = documents.iter().find(|doc| doc.file_uri().as_deref() == Some(uri)) else {
            return;
        };
        self.select_document(doc);
        if let Some(heading) = heading {
            // A tab still drawing for the first time scrolls once it has finished
            if doc.preview_ready.get() {
                doc.scroll_to_heading(heading);
            } else {
                *doc.pending_heading.borrow_mut() = Some(heading.to_string());
            }
        }
    }

    /// Adds a file to the ones waiting to open in this window.
    pub fn queue_file(self: &Rc<Self>, file: gio::File, heading: Option<String>) {
        // Set before the window is first drawn, so it opens blank, not "No Document Open"
        self.begin_loading();
        self.waiting_files.borrow_mut().push_back(WaitingFile { file, heading });
        if !self.reading_files.replace(true) {
            let win = self.clone();
            glib::spawn_future_local(async move {
                win.read_waiting_files().await;
            });
        }
    }

    /// Reads the waiting files one at a time, and shows each in a tab. Because a
    /// tab only appears once its file has been read, a file that can't be opened
    /// leaves nothing behind but the message saying why.
    async fn read_waiting_files(self: &Rc<Self>) {
        loop {
            let next = self
                .waiting_files
                .borrow()
                .front()
                .map(|waiting| (waiting.file.clone(), waiting.heading.clone()));
            let Some((file, heading)) = next else { break };
            let text = self.read_file(&file).await;
            self.waiting_files.borrow_mut().pop_front();
            if let Some(text) = text {
                self.show_file(file, heading, text);
            } else if self.is_empty() {
                self.cancel_loading();
            }
        }
        self.reading_files.set(false);
        self.read_restoring_files();
    }

    /// The file's text, or None after saying why it couldn't be opened.
    async fn read_file(&self, file: &gio::File) -> Option<String> {
        let bytes = match file.load_contents_future().await {
            Ok((bytes, _)) => bytes,
            Err(err) => {
                self.toast(&format!("Could not open the file: {}", err.message()));
                // A recent file that has been moved or deleted drops off the list
                self.forget_recent(&file.uri());
                return None;
            }
        };
        match String::from_utf8(bytes.to_vec()) {
            Ok(text) => Some(text),
            Err(_) => {
                self.toast("This file isn't plain text, so LeafMD can't open it");
                None
            }
        }
    }

    /// Shows a file that has been read: in a new tab, or in place of an empty
    /// new document that nothing has been typed in, so that isn't left behind.
    fn show_file(self: &Rc<Self>, file: gio::File, heading: Option<String>, text: String) {
        let doc = match self.current().filter(|doc| doc.is_untouched()) {
            Some(doc) => {
                // It's a new document's view, which is Split if that was the way
                // to have somewhere to type. A file starts in the usual one.
                self.apply_view(self.last_view.get());
                self.show_view_of(&doc);
                doc
            }
            None => {
                self.adding_file_tab.set(true);
                let doc = self.add_tab(self.last_view.get());
                self.adding_file_tab.set(false);
                doc
            }
        };
        *doc.pending_heading.borrow_mut() = heading;
        // Gives the tab its file, which also brings the Files page up to date
        doc.show_file(file.clone(), text);
        self.show_document();
        self.remember_recent(&file);
    }

    /// Brings back the tabs from the window closed last, the ones still there.
    /// If none are, the normal start page shows instead. The selected file comes
    /// first, the normal way, which puts it on screen as soon as it's ready. The
    /// others follow once it's there, so none of them takes its place.
    pub fn open_last_file(self: &Rc<Self>) {
        let last = state::load_last_file();
        let tabs = state::load_saved_tabs();
        let reopen = state::tabs_to_reopen(
            last.as_deref(),
            tabs.as_deref(),
            &self.recent.borrow(),
            |uri| gio::File::for_uri(uri).query_exists(gio::Cancellable::NONE),
        );
        let Some(reopen) = reopen else { return };
        *self.restoring_files.borrow_mut() =
            reopen.files.iter().filter(|uri| **uri != reopen.selected).cloned().collect();
        self.restoring_not_started.set(true);
        self.queue_file(gio::File::for_uri(&reopen.selected), None);
        *self.restored_order.borrow_mut() = reopen.files;
    }

    /// Starts reading the other tabs to bring back. Each file is read by itself,
    /// so a slow or stuck one, say on a network drive, only holds up its own tab.
    fn read_restoring_files(self: &Rc<Self>) {
        if !self.restoring_not_started.replace(false) {
            return;
        }
        for uri in self.restoring_files.borrow().iter() {
            let (win, uri) = (self.clone(), uri.clone());
            glib::spawn_future_local(async move {
                let file = gio::File::for_uri(&uri);
                let text = win.read_file(&file).await;
                win.restoring_files.borrow_mut().retain(|restoring| *restoring != uri);
                // The window may have been closed while this was being read
                let open = win.windows.borrow().iter().any(|other| Rc::ptr_eq(other, &win));
                match text {
                    Some(text) if open => win.add_restored_tab(file, text),
                    None if win.is_empty() => win.cancel_loading(),
                    _ => {}
                }
            });
        }
    }

    /// Adds a tab for a file being brought back at startup, without selecting it or
    /// drawing its formatted view. It starts in the view that was saved.
    fn add_restored_tab(self: &Rc<Self>, file: gio::File, text: String) {
        let doc = self.new_document(self.last_view.get());
        doc.preview_waiting.set(true);
        let tabs: Vec<_> = self.documents().iter().map(|doc| doc.file_uri()).collect();
        let position =
            opening::restored_tab_position(&self.restored_order.borrow(), &tabs, &file.uri());
        self.tab_view.insert(&doc.paned, position as i32);
        doc.show_file(file, text);
        // The selected file couldn't be read, so this one, as the first tab to
        // arrive, was selected in its place and is what the window now shows
        if self.stack.visible_child_name().as_deref() == Some("loading") {
            self.show_document();
        }
        self.update_recent_menu();
    }

    // =====================================================================
    // Recent files
    // =====================================================================

    /// Moves a file to the top of the recent list.
    pub fn remember_recent(&self, file: &gio::File) {
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

    pub fn forget_recent(&self, uri: &str) {
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
    /// its folder in brackets, then "Clear Recent Files". Files open in this
    /// window are skipped, since they're already here.
    fn update_recent_menu(&self) {
        let files = gio::Menu::new();
        for uri in self.recent.borrow().iter() {
            if self.has_file(uri) {
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





    // =====================================================================
    // Small helpers
    // =====================================================================

    /// The document the window's buttons and shortcuts act on.
    pub fn current(&self) -> Option<Rc<Document>> {
        Document::for_widget(&self.tab_view.selected_page()?.child())
    }

    /// Whether this document is the one the window is showing.
    pub fn is_current(&self, doc: &Document) -> bool {
        self.current().is_some_and(|current| std::ptr::eq(&*current, doc))
    }

    /// This window as the shared handle the rest of LeafMD holds. None while
    /// it's being built or after it has closed.
    fn rc(&self) -> Option<Rc<Self>> {
        self.windows.borrow().iter().find(|win| std::ptr::eq(&***win, self)).cloned()
    }

    fn action(&self, name: &str) -> gio::SimpleAction {
        self.window
            .lookup_action(name)
            .and_downcast::<gio::SimpleAction>()
            .expect("LeafMD action is registered")
    }


    pub fn update_title(&self) {
        // The side panel follows the document whose name is showing
        if let Some(win) = self.rc() {
            win.update_files_page();
        }
        let Some(doc) = self.current().filter(|_| {
            self.stack.visible_child_name().as_deref() == Some("document")
        }) else {
            self.title.set_title("LeafMD");
            self.title.set_subtitle("");
            self.window.set_title(Some("LeafMD"));
            return;
        };
        let dot = if doc.buffer.is_modified() { "• " } else { "" };
        let name = format!("{dot}{}", doc.display_name());
        self.title.set_title(&name);
        self.title.set_subtitle(&doc.folder());
        self.window.set_title(Some(&format!("{name} - LeafMD")));
    }

    /// Shows or hides line numbers in every document, and ticks Line Numbers
    /// in the menu to match. Called when the setting or Distraction Free changes.
    /// Turns spell checking on or off in every document in this window.
    pub fn update_spell_check(&self, on: bool) {
        for doc in self.documents() {
            doc.check_spelling(on);
        }
    }

    pub fn update_line_numbers(&self) {
        let show = self.wants_line_numbers();
        for doc in self.documents() {
            doc.show_line_numbers(show);
        }
        self.action("line-numbers").set_state(&show.to_variant());
    }

    /// Whether this window's documents show line numbers: the Line Numbers
    /// setting, except in Distraction Free, where they're hidden unless asked
    /// for there.
    fn wants_line_numbers(&self) -> bool {
        if self.distraction_free.get() {
            self.distraction_free_numbers.get()
        } else {
            document::line_numbers_wanted()
        }
    }

    /// Line Numbers in the menu: flips what this window shows, and makes that
    /// the setting for every window. So in Distraction Free (where they're
    /// hidden) it always shows them, even if the setting was already on.
    fn toggle_line_numbers(&self) {
        let show = !self.wants_line_numbers();
        self.distraction_free_numbers.set(show);
        if let Some(app) = self.window.application() {
            // Calls update_line_numbers on every window, this one included
            app.change_action_state("line-numbers", &show.to_variant());
        }
        self.update_line_numbers();
    }

    /// Matches every document's colours to the light or dark style in use.
    fn update_style(&self) {
        for doc in self.documents() {
            doc.update_style();
        }
    }

    pub fn toast(&self, message: &str) {
        self.toasts.add_toast(
            adw::Toast::builder()
                .title(message)
                .use_markup(false)
                .build(),
        );
    }

    /// A short guide to how LeafMD works.
    fn show_help(&self) {
        const TERMINAL_ROW: &str = "From a terminal";
        const TERMINAL_SETUP_URL: &str = "https://leafmd.app/#terminal";
        let page = adw::PreferencesPage::new();
        let sections: [(&str, &[(&str, &str)]); 10] = [
            (
                "The Three Views",
                &[
                    ("Formatted", "The document as it looks when rendered. Just for reading, with nothing to edit by accident."),
                    ("Raw", "The markdown text itself. Type here to make changes. In a wide window it sits in a comfortable column down the middle."),
                    ("Split", "Raw text on the left, formatted on the right. The formatted side updates as you type, and both sides scroll together."),
                    ("Line numbers", "The raw text has no line numbers unless you turn them on with Line Numbers in the main menu. The choice applies to every window, and LeafMD remembers it."),
                    ("Light, dark and accent colours", "LeafMD follows your system's light or dark style. To keep it always light or always dark, choose Light or Dark next to Style in Preferences (main menu, or Ctrl+comma); Follow System goes back to matching the system. Links and ticked checkboxes in the formatted view use the accent colour you choose in GNOME Settings."),
                    ("Narrow windows", "When the window is narrow, the New, Save and Find buttons and the view buttons move to a bar along the bottom. They work the same way there, and move back when the window is wider."),
                ],
            ),
            (
                "Editing",
                &[
                    ("Editing the raw text", "The raw text can always be edited in Raw or Split view. Changes are only written to the file when you save."),
                    ("Unsaved changes", "A dot (•) before the file name means there are unsaved changes. LeafMD asks before closing them, and if a window with several unsaved tabs closes, you choose which ones to save."),
                    ("Jump to a word", "In Split view, double-click a word on the formatted side to select it in the raw text."),
                    ("Lists", "In a list, Enter starts the next item for you: another bullet, the next number, or an unticked checkbox. Press Enter on an empty item to end the list, or Shift+Enter for a new line without a marker."),
                    ("Pasting a link", "Select some text and paste a web address over it to turn the text into a link. Pasting anything else replaces the text as usual."),
                    ("Spelling", "Misspelt words in the raw text get a red wavy underline. Code and web addresses aren't checked. Right-click an underlined word for suggested spellings, or choose Add to Dictionary to stop it being underlined for good, or Ignore to stop it until LeafMD closes. Check Spelling in the main menu turns it off or on in every window. Languages in the right-click menu changes the language, from the ones your system has dictionaries for."),
                    ("Word count", "The bottom-right corner shows how many words the document has, and about how long it takes to read at 200 words a minute. It counts the words a reader sees: markdown marks and link addresses aren't counted, but code is. Select some text, in the raw text or the formatted view, to count just that, such as 35 of 1,240 words."),
                ],
            ),
            (
                "Tabs",
                &[
                    ("Opening a tab", "New opens an empty document in a new tab, and any file you open gets a tab of its own, including files from your file manager and links. The tab bar appears once a window has two or more tabs. A file that's already open isn't opened again: LeafMD shows its tab."),
                    ("Moving between tabs", "Click a tab, press Ctrl+Tab or Ctrl+Shift+Tab, or press Alt and a number to go to a tab by its position. Each tab keeps its own text, view, find bar and undo history."),
                    ("Closing a tab", "Use the tab's close button or Ctrl+W. Closing the last tab closes the window. Ctrl+Q closes the window with all its tabs."),
                    ("Rearranging tabs", "Drag a tab along the tab bar to change the order, onto another LeafMD window to move it there, or away from the tab bars to give it a window of its own. Right-clicking a tab offers the same moves. A moved tab keeps its unsaved changes and undo history."),
                    ("New windows", "Press Ctrl+N or choose New Window from the main menu for another window. Right-clicking LeafMD's icon offers New Window too."),
                ],
            ),
            (
                "Side Panel",
                &[
                    ("Opening the panel", "Press F9, click the button at the left end of the header bar, or choose Side Panel from the main menu. The panel stays open, in every window and next time you start LeafMD, until you close it the same way."),
                    ("Making it wider", "Drag the line between the panel and the document to make the panel wider or narrower, to see more of long names. LeafMD remembers the width for new windows and next time. If the window gets too narrow, the panel shrinks first, so the document keeps its room."),
                    ("Files and Headings", "The panel has two pages. Switch between them with the buttons along its bottom. The panel opens on the page you used last."),
                    ("Files", "The panel shows a folder as a tree. Click a folder to open or close it, and click a file to open it in a tab. The open document's file is highlighted. The arrow keys move around the tree, Right and Left open and close folders, and Enter opens a file."),
                    ("Which folder", "The panel always shows the folder of the document you're looking at, and moves when you switch tabs. The up arrow at the top of the panel shows the folder above, until you switch tabs or open a file. To browse a folder before opening anything in it, use Open Folder… in the main menu (Ctrl+Shift+O), or leafmd . in a terminal. If the window already has a document open, the folder opens in a new window."),
                    ("Greyed-out files", "Images, PDFs and other files that aren't text are greyed out, because LeafMD can't open them. Hidden files, like .git, aren't listed."),
                    ("Folders LeafMD can't see", "When you open a document from outside your home folder, LeafMD is given that one file, not the folder it's in. Choose the folder with Open Folder… to show its other files."),
                    ("Headings", "Lists the document's headings, indented to match their level. Click one, or press Enter on it, to jump there. In Raw and Split view the cursor goes to the start of the heading, ready for typing. As you scroll, the heading of the section you're reading is highlighted."),
                    ("Narrow windows", "When the window is narrow, the panel slides over the document instead of sitting beside it, and tucks away when you pick a file or a heading, or press Escape."),
                ],
            ),
            (
                "Distraction Free",
                &[
                    ("Just your document", "Press Ctrl+Shift+D, or choose Distraction Free from the main menu, to hide the header bar, tabs, formatting bar, side panel, word count and line numbers. Only that window changes, and every window starts with it off."),
                    ("Getting the bars back", "Move the pointer to the top or bottom edge of the window and the bars slide in over the document. They slide away when you move back to your text. F10 brings them in with the main menu open."),
                    ("Still there when needed", "Find (Ctrl+F) and the banner about a file changed by another app still appear. Choosing Line Numbers from the menu shows them here too."),
                    ("Leaving", "Press Escape or Ctrl+Shift+D, or choose Distraction Free again. Everything comes back as it was."),
                    ("Full screen", "Press F11, or choose Full Screen from the main menu, to fill the screen. It works with or without Distraction Free; use both for nothing but your text. Escape leaves full screen first, and a second Escape leaves Distraction Free."),
                ],
            ),
            (
                "Fonts and Zoom",
                &[
                    ("Fonts", "Choose Preferences from the main menu, or press Ctrl+comma, to pick the font and size for the raw text and for the formatted view. Changes show straight away in every window, and LeafMD remembers them."),
                    ("Starting fonts", "The raw text starts in the monospace font set in GNOME Settings, and keeps following it until you choose another. The formatted view starts in Adwaita Sans. The Reset button beside a font you've changed puts it back."),
                    ("Zoom", "Ctrl+plus makes the text bigger and Ctrl+minus smaller, in both views, and Ctrl+0 goes back to normal. Holding Ctrl and turning the mouse wheel does the same. The row at the top of the main menu shows how far you've zoomed, and clicking the percentage goes back to 100%."),
                    ("Which tabs", "Zoom covers every tab in that window, including tabs you open or move there later. Other windows keep their own zoom. Each time LeafMD starts, zoom is back to 100%; for a lasting change, use Preferences."),
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
                    ("Find", "Press Ctrl+F, click the magnifying glass, or choose Find from the main menu, then type. Every match is highlighted, and Enter and Shift+Enter move between them. In Raw and Split view the raw text is searched; in Formatted view, the formatted text is."),
                    ("Replace", "Press Ctrl+H, or the replace button in the find bar, to replace one match at a time or all of them at once. One Ctrl+Z undoes Replace All. Replacing works in Raw and Split view."),
                    ("Options", "The menu at the end of the find bar has Match Case and Whole Words Only."),
                ],
            ),
            (
                "Printing and Exporting",
                &[
                    ("Print", "Press Ctrl+P, or choose Print… from the main menu, to print the formatted document. It prints formatted whichever view is showing, even Raw."),
                    ("Export as PDF", "Choose Export as PDF… from the main menu to save the formatted document as a PDF, next to the file unless you choose another place. It looks the same as printing it would."),
                    ("Export as HTML", "Choose Export as HTML… to save the formatted document as one web page that opens in any browser. Pictures from your computer are packed inside it, so it still shows them after being emailed or moved, while online pictures stay links. The page is light or dark to match the reader's own setting."),
                    ("How it looks", "Printed pages and PDFs are always dark text on white, and use the font and size from Preferences, whatever the zoom. What's printed or exported is the text as it is now, including changes you haven't saved."),
                ],
            ),
            (
                "Files and Links",
                &[
                    ("Opening files", "Use Open, drop a file onto the window, or double-click a markdown file in your file manager. It opens in a new tab, in the LeafMD window you used last."),
                    (TERMINAL_ROW, "Type leafmd notes.md in a terminal to open a file, or leafmd . to open the folder you're in, in the side panel. It takes a one-time setup, and Setup shows you how."),
                    ("Recent files", "The arrow next to Open lists the files you opened recently. When you start LeafMD, it reopens the saved files from the window you closed last, in the same tabs. Clear Recent Files empties the list, and LeafMD then has nothing to reopen."),
                    ("Closing a document", "Close Document in the main menu closes the selected tab. If it's the last one, the window stays open and LeafMD starts with no document next time. Closing the window instead (Ctrl+W on the last tab, or Ctrl+Q) brings its files back next time."),
                    ("Changes from other apps", "If another app changes or deletes the open file, a banner appears across the top of the window, and nothing changes until you choose. Reload brings in the new version, and Ignore keeps what you have. If you save a file that changed without you noticing, LeafMD asks first."),
                    ("Links", "Web links open in your browser. Links to other markdown or text files open them in a new tab, and links to headings scroll to them."),
                    ("Images", "Images next to the file and images online are shown in the formatted view."),
                ],
            ),
        ];
        for (title, rows) in sections {
            let group = adw::PreferencesGroup::builder().title(title).build();
            for (row_title, row_text) in rows {
                let row = adw::ActionRow::builder()
                    .title(*row_title)
                    .subtitle(*row_text)
                    .build();
                // The steps live on the website, so that they only ever need
                // updating in one place, even for copies of LeafMD already installed.
                if *row_title == TERMINAL_ROW {
                    row.add_suffix(
                        &gtk::LinkButton::builder()
                            .uri(TERMINAL_SETUP_URL)
                            .label("Setup")
                            .valign(gtk::Align::Center)
                            .build(),
                    );
                }
                group.add(&row);
            }
            page.add(&group);
        }

        let content = adw::ToolbarView::new();
        content.add_top_bar(&adw::HeaderBar::new());
        content.set_content(Some(&page));
        let dialog = adw::Dialog::builder()
            .title("Help")
            .content_width(520)
            .content_height(640)
            .child(&content)
            .build();
        self.present_reading_dialog(&dialog);
    }

    /// Shows one of LeafMD's reading dialogs (Help, Keyboard Shortcuts and the
    /// cheatsheet) without a focus outline on its first row.
    ///
    /// When a dialog opens, GTK puts the keyboard's focus on the first thing in
    /// it that can take focus, which in these dialogs is the first row. If the
    /// dialog was opened from the keyboard (F1, say), GTK has already decided the
    /// keyboard is in use and draws its blue focus outline around that row,
    /// which looks like the row was picked. Hiding the outline doesn't move the
    /// focus: the arrow keys and Page Down still scroll straight away, and the
    /// outline comes back as soon as the keyboard moves between rows, the same
    /// as after opening the dialog with the mouse.
    fn present_reading_dialog(&self, dialog: &adw::Dialog) {
        // Letting go of a key after the focus has moved turns the outline back
        // on. That can happen after the dialog appears: a quick F1 is let go
        // first, but the Ctrl and Shift of Ctrl+? are often still held. So the
        // outline is hidden once the dialog is on screen, and again each time
        // a key is let go, until a key is pressed in the dialog (moving around
        // with Down or Tab, which should show it).
        let hide_outline = {
            let window = self.window.downgrade();
            move || {
                let window = window.clone();
                // After GTK's own handling of the key, which is what turns it on
                glib::idle_add_local_once(move || {
                    if let Some(window) = window.upgrade() {
                        window.set_focus_visible(false);
                    }
                });
            }
        };
        let keys_used = Rc::new(Cell::new(false));
        let keys = gtk::EventControllerKey::builder()
            .propagation_phase(gtk::PropagationPhase::Capture)
            .build();
        let used = keys_used.clone();
        keys.connect_key_pressed(move |_, _, _, _| {
            used.set(true);
            glib::Propagation::Proceed
        });
        let hide = hide_outline.clone();
        keys.connect_key_released(move |_, _, _, _| {
            if !keys_used.get() {
                hide();
            }
        });
        dialog.add_controller(keys);
        dialog.connect_map(move |_| hide_outline());
        dialog.present(Some(&self.window));
    }

    /// Lists every keyboard shortcut.
    fn show_shortcuts(&self) {
        // Built like Help rather than with adw::ShortcutsDialog, which always
        // has a search box. Keep the keys in step with set_shortcuts in main.rs.
        let page = adw::PreferencesPage::new();
        let sections: [(&str, &[(&str, &str)]); 6] = [
            (
                "Views",
                &[
                    ("Formatted view", "Ctrl+1"),
                    ("Raw text", "Ctrl+2"),
                    ("Split view", "Ctrl+3"),
                    ("Side panel", "F9"),
                    ("Distraction Free", "Ctrl+Shift+D"),
                    ("Full screen", "F11"),
                    ("Leave full screen, then Distraction Free", "Escape"),
                    ("Zoom in", "Ctrl+Plus or Ctrl+="),
                    ("Zoom out", "Ctrl+Minus"),
                    ("Normal size", "Ctrl+0"),
                    ("Zoom with the mouse", "Ctrl+wheel"),
                ],
            ),
            (
                "Files",
                &[
                    ("New document", "Ctrl+T"),
                    ("New window", "Ctrl+N"),
                    ("Open", "Ctrl+O"),
                    ("Open folder", "Ctrl+Shift+O"),
                    ("Save", "Ctrl+S"),
                    ("Save as", "Ctrl+Shift+S"),
                    ("Print", "Ctrl+P"),
                    ("Close window", "Ctrl+Q"),
                ],
            ),
            (
                "Tabs",
                &[
                    ("Close tab", "Ctrl+W"),
                    ("Next tab", "Ctrl+Tab, Ctrl+Page Down or Ctrl+Alt+Page Down"),
                    ("Previous tab", "Ctrl+Shift+Tab, Ctrl+Page Up or Ctrl+Alt+Page Up"),
                    ("Go to a tab by its position", "Alt+1 to Alt+9"),
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
                    ("Preferences", "Ctrl+,"),
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
                    // The longer lists of keys wrap, so they fit a narrow window
                    &gtk::Label::builder()
                        .label(*keys)
                        .css_classes(["dim-label"])
                        .wrap(true)
                        .max_width_chars(22)
                        .justify(gtk::Justification::Right)
                        .xalign(1.0)
                        .build(),
                );
                group.add(&row);
            }
            page.add(&group);
        }

        let content = adw::ToolbarView::new();
        content.add_top_bar(&adw::HeaderBar::new());
        content.set_content(Some(&page));
        let dialog = adw::Dialog::builder()
            .title("Keyboard Shortcuts")
            .content_width(520)
            .content_height(640)
            .child(&content)
            .build();
        self.present_reading_dialog(&dialog);
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
        let dialog = adw::Dialog::builder()
            .title("Markdown Cheatsheet")
            .content_width(600)
            .content_height(680)
            .child(&content)
            .build();
        self.present_reading_dialog(&dialog);
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
            match env!("LEAFMD_LIBSPELLING_VERSION") {
                "" => "libspelling, checks spelling https://gitlab.gnome.org/GNOME/libspelling".to_string(),
                version => format!(
                    "libspelling {version}, checks spelling https://gitlab.gnome.org/GNOME/libspelling"
                ),
            },
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
            view: self.current().map_or(self.last_view.get(), |doc| doc.view_mode.get()),
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
/// Styles for the window's two layers of bars (see the Layout part of
/// Window::new).
///
/// In Distraction Free the outer bars slide in over the document instead of
/// above it, so they need a background of their own (normally they're
/// see-through, over the window's). A soft shadow separates them from the text.
///
/// libadwaita leaves a 6 pixel gap under a top bar, which suits a header bar
/// but would sit between the document and the inner bars even while they're
/// hidden, so the inner bars don't have it.
/// The zoom row at the top of the main menu, like GNOME Text Editor's: Zoom
/// Out, the zoom as a percentage (clicking it goes back to 100%), and Zoom In.
/// Keeps the selected tab in view when the tab bar gets narrower.
///
/// The tab bar scrolls to a tab when it's selected, but not when the window
/// is made narrower, which can leave the selected tab scrolled out of sight
/// until another one is picked. libadwaita has no way to ask it to scroll to a
/// tab, so this looks inside the tab bar: its tabs sit in a scrolled area, and
/// when that area's visible width shrinks, it's scrolled just enough to show
/// the selected tab (the one in GTK's "selected" state). If a future
/// libadwaita builds its tab bar differently and nothing is found, this does
/// nothing and the tab bar behaves as it always did.
fn keep_selected_tab_in_view(tab_bar: &adw::TabBar) {
    // Two scrolled areas: one for pinned tabs (LeafMD has none) and one for
    // the rest, which holds the tabs
    for scroller in descendants(tab_bar.upcast_ref())
        .into_iter()
        .filter_map(|widget| widget.downcast::<gtk::ScrolledWindow>().ok())
    {
        let adjustment = scroller.hadjustment();
        let visible = Rc::new(Cell::new(adjustment.page_size()));
        adjustment.connect_page_size_notify(move |adjustment| {
            let narrower = adjustment.page_size() < visible.get();
            visible.set(adjustment.page_size());
            if !narrower {
                return;
            }
            // Once GTK has finished laying the tabs out at the new width
            let scroller = scroller.clone();
            glib::idle_add_local_once(move || scroll_to_selected_tab(&scroller));
        });
    }
}

fn scroll_to_selected_tab(scroller: &gtk::ScrolledWindow) {
    let Some(tabs) = scroller.child() else { return };
    let selected = descendants(&tabs).into_iter().find(|widget| {
        widget.type_().name() == "AdwTab" && widget.state_flags().contains(gtk::StateFlags::SELECTED)
    });
    let Some(bounds) = selected.and_then(|tab| tab.compute_bounds(scroller)) else {
        return;
    };
    // Where the tab is, measured along everything the bar can scroll through
    let adjustment = scroller.hadjustment();
    let start = adjustment.value() + f64::from(bounds.x());
    let end = start + f64::from(bounds.width());
    if start < adjustment.value() {
        adjustment.set_value(start);
    } else if end > adjustment.value() + adjustment.page_size() {
        adjustment.set_value(end - adjustment.page_size());
    }
}

/// Every widget inside this one, at any depth.
fn descendants(widget: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut found = Vec::new();
    let mut child = widget.first_child();
    while let Some(current) = child {
        found.push(current.clone());
        found.extend(descendants(&current));
        child = current.next_sibling();
    }
    found
}

fn zoom_row(percentage: &gtk::Button) -> gtk::Box {
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .margin_start(6)
        .margin_end(6)
        .build();
    let zoom_out = gtk::Button::builder()
        .icon_name("zoom-out-symbolic")
        .tooltip_text("Zoom out (Ctrl+Minus)")
        .action_name("win.zoom-out")
        .css_classes(["flat", "circular"])
        .build();
    let zoom_in = gtk::Button::builder()
        .icon_name("zoom-in-symbolic")
        .tooltip_text("Zoom in (Ctrl+Plus)")
        .action_name("win.zoom-in")
        .css_classes(["flat", "circular"])
        .build();
    row.append(&zoom_out);
    row.append(percentage);
    row.append(&zoom_in);
    row
}

fn load_window_style() {
    static LOADED: std::sync::Once = std::sync::Once::new();
    LOADED.call_once(|| {
        let Some(display) = gdk::Display::default() else {
            return;
        };
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "toolbarview.leafmd-distraction-free > .top-bar,
             toolbarview.leafmd-distraction-free > .bottom-bar {
                background-color: var(--headerbar-bg-color);
                box-shadow: 0 1px 3px rgb(0 0 6 / 12%);
            }
            /* The word count: small, faint text on a see-through patch of the
               document's own background, so it stays readable over text
               without drawing the eye. The right margin clears the scroll bar. */
            label.leafmd-word-count {
                margin: 0 18px 10px 0;
                padding: 2px 8px;
                border-radius: 6px;
                font-size: 0.8em;
                color: color-mix(in srgb, var(--view-fg-color) 55%, transparent);
                background-color: var(--view-bg-color);
            }
            toolbarview.leafmd-inner-bars > .top-bar,
            toolbarview.leafmd-inner-bars > .top-bar > windowhandle,
            toolbarview.leafmd-inner-bars > .top-bar > windowhandle > box {
                padding: 0;
                margin: 0;
                min-height: 0;
            }",
        );
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
}

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
pub fn file_filters() -> gio::ListStore {
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
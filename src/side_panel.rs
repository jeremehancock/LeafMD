//! The side panel on the left of each window. It holds pages (Files and
//! Headings), shows one at a time, and has its own small header bar with a
//! title and a switcher along its bottom for choosing the page.
//!
//! Showing, hiding and remembering the panel is the window's job (window.rs),
//! because it depends on the window's width. This file only builds what's
//! inside the panel, so a later feature can add a page with one call.

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use std::cell::RefCell;

pub struct SidePanel {
    /// The whole panel, which the window puts in its split view
    pub widget: adw::ToolbarView,
    /// The title at the top of the panel, set by the page that's showing
    title: adw::WindowTitle,
    /// The pages, one showing at a time
    stack: adw::ViewStack,
    /// The bar for switching pages. Hidden while there's only one page, since
    /// there's nothing to switch to.
    switcher_bar: adw::ViewSwitcherBar,
}

impl SidePanel {
    pub fn new() -> Self {
        let title = adw::WindowTitle::new("Files", "");
        let header = adw::HeaderBar::new();
        header.set_title_widget(Some(&title));

        let stack = adw::ViewStack::new();
        // Along the bottom of the panel, like GNOME apps do in narrow spaces
        let switcher_bar = adw::ViewSwitcherBar::builder().stack(&stack).build();

        let widget = adw::ToolbarView::new();
        widget.add_top_bar(&header);
        widget.set_content(Some(&stack));
        widget.add_bottom_bar(&switcher_bar);

        Self { widget, title, stack, switcher_bar }
    }

    /// Adds a page. `name` is how LeafMD refers to it, `title` and `icon` are
    /// what the page switcher shows once there are two or more pages.
    pub fn add_page(&self, name: &str, title: &str, icon: &str, child: &impl IsA<gtk::Widget>) {
        self.stack.add_titled_with_icon(child, Some(name), title, icon);
        self.switcher_bar.set_reveal(self.stack.pages().n_items() > 1);
    }

    /// Sets the title at the top of the panel, like the folder's name for Files.
    pub fn set_title(&self, title: &str) {
        self.title.set_title(title);
    }

    /// Shows the page called `name`, if there is one.
    pub fn set_visible_page(&self, name: &str) {
        if self.stack.child_by_name(name).is_some() {
            self.stack.set_visible_child_name(name);
        }
    }

    /// The name of the page showing.
    pub fn visible_page(&self) -> String {
        self.stack.visible_child_name().map(|name| name.to_string()).unwrap_or_default()
    }

    /// Runs `changed` with the page's name whenever another page is shown,
    /// whether by the switcher or by LeafMD.
    pub fn connect_page_changed(&self, changed: impl Fn(&str) + 'static) {
        self.stack.connect_visible_child_name_notify(move |stack| {
            if let Some(name) = stack.visible_child_name() {
                changed(&name);
            }
        });
    }
}

// ---------------------------------------------------------------------------
// A holder that notices its width
// ---------------------------------------------------------------------------

mod imp {
    use super::*;

    /// What to run with the width after each layout pass
    pub type OnResize = Box<dyn Fn(i32)>;

    #[derive(Default)]
    pub struct WidthWatcher {
        pub on_resize: RefCell<Option<OnResize>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WidthWatcher {
        const NAME: &'static str = "LeafMDWidthWatcher";
        type Type = super::WidthWatcher;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for WidthWatcher {
        fn constructed(&self) {
            self.parent_constructed();
            // adw::Bin normally hands its layout to a separate helper (a
            // "layout manager"), and while it has one GTK never calls
            // size_allocate below. Without it, this widget lays out its
            // child itself.
            self.obj().set_layout_manager(None::<gtk::LayoutManager>);
        }
    }

    impl WidgetImpl for WidthWatcher {
        /// As big as the child needs, the same as a plain box
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            self.obj()
                .child()
                .map(|child| child.measure(orientation, for_size))
                .unwrap_or((0, 0, -1, -1))
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            // Said before the child is laid out, so anything that depends on
            // the width (the side panel's) is set in time for this same layout
            if let Some(on_resize) = self.on_resize.borrow().as_ref() {
                on_resize(width);
            }
            if let Some(child) = self.obj().child() {
                child.allocate(width, height, baseline, None);
            }
        }
    }

    impl BinImpl for WidthWatcher {}
}

glib::wrapper! {
    /// Holds one widget and gives it all its room, like a plain box, but also
    /// says how wide it was made each time GTK lays the window out. The window
    /// uses it around the side panel and document, so the panel can give up
    /// room before the document does.
    pub struct WidthWatcher(ObjectSubclass<imp::WidthWatcher>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WidthWatcher {
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        let watcher: Self = glib::Object::new();
        watcher.set_child(Some(child));
        watcher
    }

    /// Runs `resized` with the width each time GTK lays the window out, just
    /// before the child is laid out at that width.
    pub fn connect_resized(&self, resized: impl Fn(i32) + 'static) {
        self.imp().on_resize.replace(Some(Box::new(resized)));
    }
}

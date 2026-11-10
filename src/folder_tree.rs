//! The Files page of the side panel: a tree of a folder's contents.
//!
//! GTK does the heavy lifting. A DirectoryList reads one folder in the
//! background and keeps itself current when files change. A TreeListModel
//! stacks those lists into a tree, and only reads a folder once someone expands
//! it, so a huge folder like `node_modules` costs nothing until it's opened.
//!
//! Which folder to show, and what happens when a file is picked, is decided
//! by the window (window.rs).

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

use adw::prelude::*;
use gtk::{gdk, gio, glib};

/// What LeafMD asks GIO about each file. "fast-content-type" is the file type
/// guessed from the name alone, so no file is ever read to sort the list.
const ATTRIBUTES: &str = "standard::name,standard::display-name,standard::type,\
standard::is-hidden,standard::is-backup,standard::fast-content-type";

/// The tooltip on files LeafMD can't open.
const CANT_OPEN: &str = "LeafMD can't open this type of file";

/// Whether LeafMD can open a file of this type, going by its name only.
///
/// Anything GIO counts as a kind of plain text can be opened: markdown, but
/// also JSON, shell scripts and source code. So can files whose type GIO can't
/// tell from the name, like `LICENSE`. If one of those turns out not to be text,
/// LeafMD's usual "This file isn't plain text" message says so when it's opened.
/// Everything else GIO recognises (images, PDFs, archives, videos) can't be.
pub fn can_open(content_type: Option<&str>) -> bool {
    match content_type {
        None | Some("") => true,
        Some(kind) => gio::content_type_is_unknown(kind) || gio::content_type_is_a(kind, "text/plain"),
    }
}

/// The folder the document portal shares files and folders through. A folder
/// LeafMD was given from outside the home folder appears under here.
pub fn portal_folder() -> PathBuf {
    glib::user_runtime_dir().join("doc")
}

/// Whether LeafMD can see a folder's real contents.
///
/// Outside Flatpak (`cargo run`), it always can. Inside, the home folder is
/// shared with LeafMD, so folders in it are real. Some folders outside it, like
/// `/tmp`, also exist inside the sandbox but are its own empty copies, so they'd
/// list the wrong files without any error.
///
/// Things that came through the portal live under `portal/<id>/`. A folder
/// shared whole (with `leafmd .` or Open Folder…) is `portal/<id>/notes`, and
/// it and its subfolders are real. A document shared by itself is
/// `portal/<id>/a.md`, so its folder is the bare `portal/<id>`, where only that
/// one file is visible.
pub fn can_see(folder: &Path, chosen: bool, in_flatpak: bool, home: &Path, portal: &Path) -> bool {
    if !in_flatpak {
        return true;
    }
    if let Ok(inside) = folder.strip_prefix(portal) {
        return chosen || inside.components().count() >= 2;
    }
    folder.starts_with(home)
}

/// Whether the tree could list a folder that the user didn't choose, in the
/// sandbox LeafMD is running in. False for the folder of a document shared
/// with LeafMD by itself, where only that one file is visible.
pub fn can_show_unchosen(folder: &gio::File) -> bool {
    folder.path().is_none_or(|path| {
        can_see(&path, false, Path::new("/.flatpak-info").exists(), &glib::home_dir(), &portal_folder())
    })
}

/// Which folder the tree shows, and whether it's the window's chosen folder.
///
/// - The chosen folder (from Open Folder… or `leafmd .`) while the selected
///   document is inside it, or there's no document with a file.
/// - Otherwise the tree follows the document, so it never shows a folder that
///   has nothing to do with the tab you're on. It stays on the folder it
///   `showing` (when that isn't the chosen one) while the document is
///   somewhere inside it, so opening a file in a subfolder from the tree
///   doesn't make the tree jump into the subfolder. Otherwise it moves to the
///   document's own folder.
/// - A document outside the chosen folder whose folder can't be listed (one
///   shared by itself from outside home) leaves the tree on the chosen folder,
///   rather than swapping it for a "can't see" message.
/// - With no chosen folder, a tab without a file yet (Untitled, or still
///   being read) keeps whatever the tree shows. With no tabs at all, nothing.
///
/// `can_show` says whether the tree could list a folder nobody chose.
pub fn folder_to_show(
    chosen: Option<&gio::File>,
    document: Option<&gio::File>,
    showing: Option<&gio::File>,
    has_document: bool,
    can_show: impl Fn(&gio::File) -> bool,
) -> (Option<gio::File>, bool) {
    // The folder that follows the document, if there's a document with a file
    let following = document.map(|document| match showing {
        Some(showing) if document.has_prefix(showing) => Some(showing.clone()),
        _ => document.parent(),
    });
    match (chosen, document) {
        (Some(chosen), Some(document)) if !document.has_prefix(chosen) => {
            match following.flatten().filter(|folder| can_show(folder)) {
                Some(folder) => (Some(folder), false),
                None => (Some(chosen.clone()), true),
            }
        }
        (Some(chosen), _) => (Some(chosen.clone()), true),
        (None, Some(_)) => (following.flatten(), false),
        (None, None) if has_document => (showing.cloned(), false),
        (None, None) => (None, false),
    }
}

/// The file a row in a DirectoryList stands for. GTK puts it on every row.
fn file_of(info: &gio::FileInfo) -> Option<gio::File> {
    info.attribute_object("standard::file").and_downcast::<gio::File>()
}

fn is_folder(info: &gio::FileInfo) -> bool {
    info.file_type() == gio::FileType::Directory
}

/// Hidden files and backups (like `notes.md~`) aren't listed.
fn is_listed(info: &gio::FileInfo) -> bool {
    !info.is_hidden() && !info.is_backup()
}

/// One folder's contents, as the tree lists them: hidden files left out,
/// folders first, then names alphabetically ignoring case. Also returns the
/// DirectoryList underneath, to watch for errors.
fn folder_model(folder: &gio::File) -> (gtk::SortListModel, gtk::DirectoryList) {
    let list = gtk::DirectoryList::new(Some(ATTRIBUTES), Some(folder));
    // Keeps the list current when files are added, removed or renamed
    list.set_monitored(true);
    let filter = gtk::CustomFilter::new(|item| {
        item.downcast_ref::<gio::FileInfo>().is_some_and(is_listed)
    });
    let filtered = gtk::FilterListModel::new(Some(list.clone()), Some(filter));
    let sorter = gtk::CustomSorter::new(|a, b| {
        let (Some(a), Some(b)) = (a.downcast_ref::<gio::FileInfo>(), b.downcast_ref::<gio::FileInfo>())
        else {
            return gtk::Ordering::Equal;
        };
        is_folder(b)
            .cmp(&is_folder(a))
            .then_with(|| a.display_name().to_lowercase().cmp(&b.display_name().to_lowercase()))
            .then_with(|| a.name().cmp(&b.name()))
            .into()
    });
    (gtk::SortListModel::new(Some(filtered), Some(sorter)), list)
}

/// What runs when the user picks a file in the tree.
type OpenFile = Box<dyn Fn(gio::File)>;

/// What the page shows instead of the tree.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Message {
    /// No folder chosen and no document with a folder
    NoFolder,
    /// The folder holds a document that was shared with LeafMD by itself
    /// (opened from outside the home folder), so only that one file is visible
    OnlyOneFile,
    /// LeafMD can't see the folder's real contents for another reason, like a
    /// folder outside the home folder given without the `leafmd` command
    CantSee,
}

/// Where the "Learn More" link under the can't-see messages goes: the README's
/// plain explanation of why Flatpak apps only see what they're given
const CANT_SEE_HELP_URL: &str = "https://github.com/jeremehancock/LeafMD#folders-outside-your-home-folder";

pub struct FilesPage {
    /// The whole page: the tree, or a message in its place
    pub widget: gtk::Stack,
    list_view: gtk::ListView,
    selection: gtk::SingleSelection,
    message: adw::StatusPage,
    /// "Learn More" under the message, shown only when LeafMD can't see a folder
    learn_more: gtk::LinkButton,
    /// The folder the tree shows, and whether the user chose it
    root: RefCell<Option<(gio::File, bool)>>,
    /// The file to highlight: the selected tab's
    highlight: RefCell<Option<gio::File>>,
    /// True until the highlighted file's row has been found and selected. The
    /// folders leading to it are read one at a time, so this can take a few goes.
    revealing: Cell<bool>,
    /// True while a go at that is waiting to run
    reveal_queued: Cell<bool>,
    /// Folders to expand again after the tree is rebuilt
    expand_again: RefCell<Vec<gio::File>>,
    /// The chosen folder and the folders that were expanded in it, kept while
    /// the tree shows another tab's folder, so going back opens them again
    chosen_expanded: RefCell<Option<(gio::File, Vec<gio::File>)>>,
    /// Runs when the user picks a file
    on_open: RefCell<Option<OpenFile>>,
}

impl FilesPage {
    pub fn new() -> Rc<Self> {
        let selection = gtk::SingleSelection::builder()
            .autoselect(false)
            .can_unselect(true)
            .build();
        let list_view = gtk::ListView::builder()
            .model(&selection)
            // GNOME's sidebar look: rounded rows with a coloured highlight
            .css_classes(["navigation-sidebar"])
            .build();
        let scrolled = gtk::ScrolledWindow::builder()
            .child(&list_view)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .build();

        let open_folder = gtk::Button::builder()
            .label("Open Folder…")
            .action_name("win.open-folder")
            .halign(gtk::Align::Center)
            .css_classes(["pill"])
            .build();
        let learn_more = gtk::LinkButton::builder()
            .uri(CANT_SEE_HELP_URL)
            .label("Learn More")
            .halign(gtk::Align::Center)
            .build();
        let buttons = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .build();
        buttons.append(&open_folder);
        buttons.append(&learn_more);
        let message = adw::StatusPage::builder()
            .icon_name("leafmd-folder-symbolic")
            .child(&buttons)
            // Smaller text and icon, made for narrow places like this panel
            .css_classes(["compact"])
            .build();

        let widget = gtk::Stack::new();
        widget.add_named(&scrolled, Some("tree"));
        widget.add_named(&message, Some("message"));

        let page = Rc::new(Self {
            widget,
            list_view,
            selection,
            message,
            learn_more,
            root: RefCell::new(None),
            highlight: RefCell::new(None),
            revealing: Cell::new(false),
            reveal_queued: Cell::new(false),
            expand_again: RefCell::new(Vec::new()),
            chosen_expanded: RefCell::new(None),
            on_open: RefCell::new(None),
        });
        page.setup_rows();
        page.setup_keys();
        page.show_message(Message::NoFolder);
        page
    }

    /// Sets what happens when the user picks a file.
    pub fn connect_open(&self, open: impl Fn(gio::File) + 'static) {
        *self.on_open.borrow_mut() = Some(Box::new(open));
    }

    /// How each row looks, and clicking on it.
    fn setup_rows(self: &Rc<Self>) {
        let factory = gtk::SignalListItemFactory::new();
        let weak = Rc::downgrade(self);
        factory.connect_setup(move |_, item| {
            let Some(item) = item.downcast_ref::<gtk::ListItem>() else { return };
            let icon = gtk::Image::new();
            let label = gtk::Label::builder()
                .xalign(0.0)
                .ellipsize(gtk::pango::EllipsizeMode::Middle)
                .build();
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            row.append(&icon);
            row.append(&label);
            // The indent for each level, and the arrow beside folders
            let expander = gtk::TreeExpander::builder().child(&row).indent_for_icon(true).build();
            item.set_child(Some(&expander));

            // One click opens a file or opens and closes a folder, anywhere on
            // the row. Claiming the click stops the list from also handling it
            // (a double-click would otherwise open a folder and close it again).
            let click = gtk::GestureClick::builder()
                .button(gdk::BUTTON_PRIMARY)
                .propagation_phase(gtk::PropagationPhase::Capture)
                .build();
            click.connect_pressed(|gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
            });
            let (weak, item) = (weak.clone(), item.downgrade());
            click.connect_released(move |gesture, presses, x, y| {
                let Some(widget) = gesture.widget().filter(|widget| widget.contains(x, y)) else {
                    return;
                };
                if presses != 1 {
                    return;
                }
                if let (Some(page), Some(row)) = (
                    weak.upgrade(),
                    item.upgrade().and_then(|item| item.item()).and_downcast::<gtk::TreeListRow>(),
                ) {
                    // Keyboard focus goes to that row, so the arrow keys carry on
                    // from there. Claiming the click stopped GTK doing this
                    // itself. The list also keeps its own note of which row has
                    // the keyboard, which only scroll_to updates.
                    page.list_view.scroll_to(row.position(), gtk::ListScrollFlags::FOCUS, None);
                    widget.grab_focus();
                    page.pick(&row);
                }
            });
            expander.add_controller(click);
        });
        factory.connect_bind(|_, item| {
            let Some(item) = item.downcast_ref::<gtk::ListItem>() else { return };
            let (Some(expander), Some(row)) = (
                item.child().and_downcast::<gtk::TreeExpander>(),
                item.item().and_downcast::<gtk::TreeListRow>(),
            ) else {
                return;
            };
            let Some(info) = row.item().and_downcast::<gio::FileInfo>() else { return };
            let Some(line) = expander.child() else { return };
            let icon = line.first_child().and_downcast::<gtk::Image>();
            let label = line.last_child().and_downcast::<gtk::Label>();
            let (Some(icon), Some(label)) = (icon, label) else { return };

            expander.set_list_row(Some(&row));
            label.set_label(&info.display_name());
            let folder = is_folder(&info);
            let openable = folder || can_open(info.attribute_string("standard::fast-content-type").as_deref());
            icon.set_icon_name(Some(if folder { "leafmd-folder-symbolic" } else { "leafmd-document-symbolic" }));
            // A file LeafMD can't open is greyed out, can't be highlighted or
            // opened, and says why when hovered
            if openable {
                line.remove_css_class("dim-label");
            } else {
                line.add_css_class("dim-label");
            }
            line.set_tooltip_text(if openable { None } else { Some(CANT_OPEN) });
            item.set_selectable(openable);
            item.set_activatable(openable);
        });
        factory.connect_unbind(|_, item| {
            if let Some(expander) = item
                .downcast_ref::<gtk::ListItem>()
                .and_then(|item| item.child())
                .and_downcast::<gtk::TreeExpander>()
            {
                expander.set_list_row(None);
            }
        });
        self.list_view.set_factory(Some(&factory));

        // Enter on a row. (Clicks are handled above, so only keys get here.)
        let weak = Rc::downgrade(self);
        self.list_view.connect_activate(move |_, position| {
            let Some(page) = weak.upgrade() else { return };
            if let Some(row) = page.selection.item(position).and_downcast::<gtk::TreeListRow>() {
                page.pick(&row);
            }
        });
    }

    /// The arrow keys, worked out from the row that has the keyboard. Up and
    /// Down move between rows. Right opens a folder and Left closes it, as in
    /// other trees. GTK's own Up and Down start from its own note of where
    /// the keyboard is, which goes out of date after a click, so the first
    /// press after one went nowhere. Moving the keyboard this way also leaves
    /// the highlight on the selected tab's file.
    fn setup_keys(self: &Rc<Self>) {
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            let (Some(page), true) = (weak.upgrade(), modifiers.is_empty()) else {
                return glib::Propagation::Proceed;
            };
            let Some(row) = page.focused_row() else {
                return glib::Propagation::Proceed;
            };
            let position = row.position();
            let rows = page.selection.n_items();
            match key {
                gdk::Key::Down | gdk::Key::KP_Down if position + 1 < rows => {
                    page.list_view.scroll_to(position + 1, gtk::ListScrollFlags::FOCUS, None);
                }
                gdk::Key::Up | gdk::Key::KP_Up if position > 0 => {
                    page.list_view.scroll_to(position - 1, gtk::ListScrollFlags::FOCUS, None);
                }
                gdk::Key::Right | gdk::Key::KP_Right if row.is_expandable() && !row.is_expanded() => {
                    page.set_expanded(&row, true);
                }
                gdk::Key::Left | gdk::Key::KP_Left if row.is_expanded() => {
                    page.set_expanded(&row, false);
                }
                gdk::Key::Up | gdk::Key::KP_Up | gdk::Key::Down | gdk::Key::KP_Down => {}
                _ => return glib::Propagation::Proceed,
            }
            glib::Propagation::Stop
        });
        self.list_view.add_controller(keys);
    }

    /// The row the keyboard is on. That's not always the highlighted one: the
    /// highlight is the selected tab's file, and arrow keys onto a folder or a
    /// greyed-out file move the keyboard without moving the highlight.
    fn focused_row(&self) -> Option<gtk::TreeListRow> {
        let focus = self.list_view.root()?.focus()?;
        // The focus is on GTK's own row, which holds the expander LeafMD made
        let expander = focus
            .downcast_ref::<gtk::TreeExpander>()
            .cloned()
            .or_else(|| focus.first_child().and_downcast::<gtk::TreeExpander>())?;
        expander.list_row()
    }

    /// Opens or closes a folder for the user. The list keeps its own note of
    /// which row has the keyboard, and loses it when rows appear or disappear
    /// underneath, so the next arrow key would start from the wrong row. So
    /// once GTK has finished adding or removing them, the note is set again.
    fn set_expanded(&self, row: &gtk::TreeListRow, expand: bool) {
        row.set_expanded(expand);
        let (list_view, row) = (self.list_view.downgrade(), row.downgrade());
        glib::idle_add_local_once(move || {
            if let (Some(list_view), Some(row)) = (list_view.upgrade(), row.upgrade()) {
                list_view.scroll_to(row.position(), gtk::ListScrollFlags::FOCUS, None);
            }
        });
    }

    /// The user clicked a row or pressed Enter on it.
    fn pick(&self, row: &gtk::TreeListRow) {
        let Some(info) = row.item().and_downcast::<gio::FileInfo>() else { return };
        if is_folder(&info) {
            self.set_expanded(row, !row.is_expanded());
            return;
        }
        if !can_open(info.attribute_string("standard::fast-content-type").as_deref()) {
            return;
        }
        if let (Some(file), Some(open)) = (file_of(&info), self.on_open.borrow().as_ref()) {
            open(file);
        }
    }

    /// The folder the tree shows, and whether the user chose it.
    pub fn root(&self) -> Option<(gio::File, bool)> {
        self.root.borrow().clone()
    }

    /// Shows a folder, or a message when there's none. `chosen` says the user
    /// chose it (as opposed to it being the selected document's folder).
    pub fn set_root(self: &Rc<Self>, folder: Option<&gio::File>, chosen: bool) {
        let same = match (&*self.root.borrow(), folder) {
            (Some((now, was_chosen)), Some(folder)) => now.equal(folder) && *was_chosen == chosen,
            (None, None) => true,
            _ => false,
        };
        if same {
            return;
        }
        // Leaving the chosen folder: note what was expanded in it
        if let Some((leaving, true)) = self.root.borrow().clone() {
            *self.chosen_expanded.borrow_mut() = Some((leaving, self.expanded_folders()));
        }
        *self.root.borrow_mut() = folder.map(|folder| (folder.clone(), chosen));
        // Coming back to it: expand those folders again
        let again = match (folder, &*self.chosen_expanded.borrow()) {
            (Some(folder), Some((saved, expanded))) if chosen && saved.equal(folder) => expanded.clone(),
            _ => Vec::new(),
        };
        *self.expand_again.borrow_mut() = again;
        self.build_tree();
    }

    /// The folders expanded in the tree now, outermost first.
    fn expanded_folders(&self) -> Vec<gio::File> {
        let Some(tree) = self.selection.model().and_downcast::<gtk::TreeListModel>() else {
            return Vec::new();
        };
        (0..tree.n_items())
            .filter_map(|position| tree.row(position).filter(|row| row.is_expanded()))
            .filter_map(|row| row.item().and_downcast::<gio::FileInfo>())
            .filter_map(|info| file_of(&info))
            .collect()
    }

    /// Lists the root folder from scratch, or shows why it can't.
    fn build_tree(self: &Rc<Self>) {
        let Some((folder, chosen)) = self.root.borrow().clone() else {
            self.selection.set_model(None::<&gio::ListModel>);
            self.show_message(Message::NoFolder);
            return;
        };
        let visible = folder.path().is_none_or(|path| {
            can_see(
                &path,
                chosen,
                Path::new("/.flatpak-info").exists(),
                &glib::home_dir(),
                &portal_folder(),
            )
        });
        if !visible {
            self.selection.set_model(None::<&gio::ListModel>);
            // Inside the portal's folder, the only things LeafMD can't see are
            // the neighbours of a document that was shared by itself
            let shared_alone = folder.path().is_some_and(|path| path.starts_with(portal_folder()));
            self.show_message(if shared_alone { Message::OnlyOneFile } else { Message::CantSee });
            return;
        }

        let (model, list) = folder_model(&folder);
        // A folder that can't be read at all, say one that doesn't exist inside
        // the sandbox, says so rather than looking empty
        let weak = Rc::downgrade(self);
        list.connect_error_notify(move |list| {
            if list.error().is_some()
                && let Some(page) = weak.upgrade()
            {
                page.show_message(Message::CantSee);
            }
        });
        let tree = gtk::TreeListModel::new(model, false, false, |item| {
            let info = item.downcast_ref::<gio::FileInfo>()?;
            if !is_folder(info) {
                return None;
            }
            Some(folder_model(&file_of(info)?).0.upcast())
        });
        // Rows arrive a folder at a time, which may be what a highlight or a
        // folder to expand again is waiting for
        let weak = Rc::downgrade(self);
        tree.connect_items_changed(move |_, _, _, _| {
            if let Some(page) = weak.upgrade() {
                page.queue_reveal();
            }
        });
        self.selection.set_model(Some(&tree));
        self.widget.set_visible_child_name("tree");
        self.revealing.set(self.highlight.borrow().is_some());
        self.queue_reveal();
    }

    fn show_message(&self, message: Message) {
        let (title, description) = match message {
            Message::NoFolder => (
                "No Folder Open",
                "Open a folder to see its files here",
            ),
            Message::OnlyOneFile => (
                "Can't See This Folder",
                "LeafMD was given this one document, not the folder it's in. Choose the folder with Open Folder… to show its files here.",
            ),
            Message::CantSee => (
                "Can't See This Folder",
                "LeafMD hasn't been given access to this folder. Choose it with Open Folder… to show its files here.",
            ),
        };
        self.learn_more.set_visible(message != Message::NoFolder);
        self.message.set_title(title);
        self.message.set_description(Some(description));
        self.widget.set_visible_child_name("message");
    }

    /// Highlights the selected tab's file, opening the folders leading to it.
    pub fn set_highlight(self: &Rc<Self>, file: Option<gio::File>) {
        let same = match (&*self.highlight.borrow(), &file) {
            (Some(now), Some(file)) => now.equal(file),
            (None, None) => true,
            _ => false,
        };
        if same {
            return;
        }
        *self.highlight.borrow_mut() = file;
        self.revealing.set(true);
        self.queue_reveal();
    }

    /// Runs `reveal` once things settle, rather than in the middle of the tree
    /// changing, which GTK doesn't allow.
    fn queue_reveal(self: &Rc<Self>) {
        if self.reveal_queued.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::idle_add_local_once(move || {
            if let Some(page) = weak.upgrade() {
                page.reveal_queued.set(false);
                page.reveal();
            }
        });
    }

    /// Takes the next steps towards showing the highlighted file and expanding
    /// the folders that were open before a rebuild. Each folder has to be read
    /// before the rows inside it exist, so this runs again as they arrive.
    fn reveal(&self) {
        let Some(tree) = self.selection.model().and_downcast::<gtk::TreeListModel>() else {
            return;
        };
        let Some((root, _)) = self.root.borrow().clone() else { return };

        // Folders that were open before the tree was rebuilt, outermost first
        let mut waiting = Vec::new();
        for folder in self.expand_again.take() {
            match find_row(&tree, &folder) {
                Some((_, row)) => row.set_expanded(true),
                // Not read yet, or gone. Kept for the next go if its parent is
                // open, since the rows may still be on their way.
                None => waiting.push(folder),
            }
        }
        waiting.retain(|folder| {
            folder.parent().is_some_and(|parent| {
                parent.equal(&root) || find_row(&tree, &parent).is_some_and(|(_, row)| row.is_expanded())
            })
        });
        *self.expand_again.borrow_mut() = waiting;

        if !self.revealing.get() {
            return;
        }
        let target = self.highlight.borrow().clone();
        let Some(relative) = target.as_ref().and_then(|target| root.relative_path(target)) else {
            // Not inside this folder (or nothing to highlight)
            self.selection.unselect_all();
            self.revealing.set(false);
            return;
        };
        let mut step = root;
        for part in relative.components() {
            step = step.child(part);
            let Some((position, row)) = find_row(&tree, &step) else {
                // The rows for this folder are still on their way
                return;
            };
            if row.is_expandable() {
                row.set_expanded(true);
            } else {
                self.selection.select_item(position, true);
                self.list_view.scroll_to(position, gtk::ListScrollFlags::NONE, None);
                self.revealing.set(false);
                return;
            }
        }
    }

    /// For a folder outside the home folder, where LeafMD isn't told about
    /// changes: checks each open folder against what's on disk, and if any
    /// differ, lists the tree again with the same folders open.
    pub fn catch_up(self: &Rc<Self>) {
        let Some((root, _)) = self.root.borrow().clone() else { return };
        if !root.path().is_some_and(|path| path.starts_with(portal_folder())) {
            return;
        }
        let Some(tree) = self.selection.model().and_downcast::<gtk::TreeListModel>() else {
            return;
        };
        // Each open folder with what the tree shows in it
        let mut shown = vec![(root.clone(), names_in(&tree.model()))];
        for position in 0..tree.n_items() {
            if let Some(row) = tree.row(position).filter(|row| row.is_expanded())
                && let (Some(info), Some(children)) =
                    (row.item().and_downcast::<gio::FileInfo>(), row.children())
                && let Some(folder) = file_of(&info)
            {
                shown.push((folder, names_in(&children)));
            }
        }
        let weak: Weak<Self> = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            for (folder, names) in &shown {
                if names_on_disk(folder).await.is_some_and(|on_disk| on_disk != *names) {
                    let Some(page) = weak.upgrade() else { return };
                    // Only if it's still showing the same folder
                    if page.root.borrow().as_ref().is_some_and(|(now, _)| now.equal(&root)) {
                        *page.expand_again.borrow_mut() =
                            shown.iter().skip(1).map(|(folder, _)| folder.clone()).collect();
                        page.revealing.set(page.highlight.borrow().is_some());
                        page.build_tree();
                    }
                    return;
                }
            }
        });
    }
}

/// The row for a file, and its position, among the rows the tree has now.
fn find_row(tree: &gtk::TreeListModel, file: &gio::File) -> Option<(u32, gtk::TreeListRow)> {
    (0..tree.n_items()).find_map(|position| {
        let row = tree.row(position)?;
        let info = row.item().and_downcast::<gio::FileInfo>()?;
        file_of(&info)?.equal(file).then_some((position, row))
    })
}

/// The names in one folder's list in the tree.
fn names_in(model: &impl IsA<gio::ListModel>) -> HashSet<String> {
    (0..model.n_items())
        .filter_map(|position| model.item(position).and_downcast::<gio::FileInfo>())
        .map(|info| info.name().to_string_lossy().into_owned())
        .collect()
}

/// The names the tree would list for a folder, read from disk now. None if it
/// can't be read.
async fn names_on_disk(folder: &gio::File) -> Option<HashSet<String>> {
    let entries = folder
        .enumerate_children_future(ATTRIBUTES, gio::FileQueryInfoFlags::NONE, glib::Priority::DEFAULT)
        .await
        .ok()?;
    let mut names = HashSet::new();
    loop {
        let batch = entries.next_files_future(100, glib::Priority::DEFAULT).await.ok()?;
        if batch.is_empty() {
            return Some(names);
        }
        names.extend(
            batch
                .iter()
                .filter(|info| is_listed(info))
                .map(|info| info.name().to_string_lossy().into_owned()),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use gtk::gio;
    use gtk::prelude::*;

    use super::{can_open, can_see, folder_to_show};

    fn at(path: &str) -> gio::File {
        gio::File::for_path(path)
    }

    /// The folder shown for a chosen folder, a document and what the tree shows
    /// now, where only folders under /home can be listed without being chosen
    fn shown(chosen: Option<&str>, document: Option<&str>, showing: Option<&str>) -> (Option<String>, bool) {
        let (folder, is_chosen) = folder_to_show(
            chosen.map(at).as_ref(),
            document.map(at).as_ref(),
            showing.map(at).as_ref(),
            true,
            |folder| folder.path().is_some_and(|path| path.starts_with("/home")),
        );
        (folder.and_then(|folder| folder.path()).map(|path| path.display().to_string()), is_chosen)
    }

    #[test]
    fn the_chosen_folder_shows_while_the_document_is_in_it() {
        let notes = Some("/home/me/notes");
        assert_eq!(shown(notes, Some("/home/me/notes/a.md"), None), (Some("/home/me/notes".into()), true));
        // Files in subfolders don't move it
        assert_eq!(shown(notes, Some("/home/me/notes/docs/setup.md"), None), (Some("/home/me/notes".into()), true));
        // Nor does an Untitled document
        assert_eq!(shown(notes, None, None), (Some("/home/me/notes".into()), true));
    }

    #[test]
    fn a_document_outside_the_chosen_folder_shows_its_own_folder() {
        let notes = Some("/home/me/notes");
        assert_eq!(shown(notes, Some("/home/me/work/plan.md"), None), (Some("/home/me/work".into()), false));
        // Opening a file in a subfolder of that folder from the tree keeps it
        assert_eq!(
            shown(notes, Some("/home/me/work/old/2024.md"), Some("/home/me/work")),
            (Some("/home/me/work".into()), false)
        );
        // Back to a document in the chosen folder
        assert_eq!(
            shown(notes, Some("/home/me/notes/a.md"), Some("/home/me/work")),
            (Some("/home/me/notes".into()), true)
        );
    }

    #[test]
    fn a_document_shared_by_itself_leaves_the_chosen_folder_showing() {
        assert_eq!(
            shown(Some("/home/me/notes"), Some("/run/user/1000/doc/abc/a.md"), None),
            (Some("/home/me/notes".into()), true)
        );
    }

    #[test]
    fn with_no_chosen_folder_the_tree_follows_the_document() {
        assert_eq!(shown(None, Some("/home/me/work/plan.md"), None), (Some("/home/me/work".into()), false));
        assert_eq!(
            shown(None, Some("/home/me/notes/docs/setup.md"), Some("/home/me/notes")),
            (Some("/home/me/notes".into()), false)
        );
        // A tab still being read keeps what's showing
        assert_eq!(shown(None, None, Some("/home/me/notes")), (Some("/home/me/notes".into()), false));
    }

    #[test]
    fn text_files_of_every_kind_can_be_opened() {
        for kind in [
            "text/markdown",
            "text/plain",
            "application/json",
            "application/x-shellscript",
            "text/x-rust",
        ] {
            assert!(can_open(Some(kind)), "{kind} should open");
        }
    }

    #[test]
    fn files_whose_type_is_unknown_can_be_opened() {
        // Like LICENSE or a program called `build`: nothing in the name says
        assert!(can_open(Some("application/octet-stream")));
        assert!(can_open(Some("")));
        assert!(can_open(None));
    }

    #[test]
    fn files_that_clearly_arent_text_are_greyed_out() {
        for kind in ["image/png", "application/pdf", "application/zip", "video/mp4", "font/ttf"] {
            assert!(!can_open(Some(kind)), "{kind} should be greyed out");
        }
    }

    const HOME: &str = "/home/someone";
    const PORTAL: &str = "/run/user/1000/doc";

    fn sees(folder: &str, chosen: bool, in_flatpak: bool) -> bool {
        can_see(Path::new(folder), chosen, in_flatpak, Path::new(HOME), Path::new(PORTAL))
    }

    #[test]
    fn outside_flatpak_every_folder_can_be_seen() {
        assert!(sees("/tmp/notes", false, false));
    }

    #[test]
    fn folders_in_the_home_folder_can_be_seen() {
        assert!(sees("/home/someone/notes", false, true));
        assert!(sees("/home/someone", true, true));
    }

    #[test]
    fn other_folders_may_be_the_sandboxs_own_copies() {
        assert!(!sees("/tmp/notes", true, true));
        assert!(!sees("/home/someoneelse", true, true));
    }

    #[test]
    fn a_folder_shared_whole_can_be_seen_but_not_a_lone_documents_folder() {
        // `leafmd .` outside home, or Open Folder…
        assert!(sees("/run/user/1000/doc/abc123/notes", true, true));
        // The same folder when a document in it is selected, and its subfolders
        assert!(sees("/run/user/1000/doc/abc123/notes", false, true));
        assert!(sees("/run/user/1000/doc/abc123/notes/docs", false, true));
        // The folder of a document shared by itself holds only that document
        assert!(!sees("/run/user/1000/doc/abc123", false, true));
        assert!(!sees("/run/user/1000/doc", false, true));
    }
}

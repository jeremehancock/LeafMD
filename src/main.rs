mod find;
mod format;
mod render;
mod state;
mod window;

use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use window::Window;

pub const APP_ID: &str = "io.github.jeremehancock.LeafMD";

/// Every open LeafMD window. Each document gets its own window.
type Windows = Rc<RefCell<Vec<Rc<Window>>>>;

fn main() -> glib::ExitCode {
    // The view icons built into the program (see build.rs). GTK finds icons
    // under the app's resource path automatically.
    gio::resources_register_include!("leafmd.gresource").expect("LeafMD's built-in icons load");

    let app = adw::Application::builder()
        .application_id(APP_ID)
        // Lets LeafMD be started with a file, e.g. `leafmd notes.md` or a double-click
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|app| {
        sourceview5::init();
        set_shortcuts(app);
    });

    let windows = Windows::default();

    let w = windows.clone();
    app.connect_activate(move |app| {
        // Started from the app grid with no file: pick up where you left off.
        // Clicking the app icon while LeafMD is already open just shows it.
        let existing = w.borrow().last().cloned();
        match existing {
            Some(win) => win.window.present(),
            None => {
                let win = new_window(app, &w);
                win.window.present();
                win.open_last_file();
            }
        }
    });

    // Files opened from the file manager or command line
    let w = windows.clone();
    app.connect_open(move |app, files, _hint| {
        for file in files {
            open_in_window(app, &w, file.clone(), None);
        }
    });

    // Links to other text files ask for this, with the file's address
    // (optionally ending in #heading)
    let open_link = gio::SimpleAction::new("open-in-new-window", Some(glib::VariantTy::STRING));
    let w = windows.clone();
    let app_ref = app.downgrade();
    open_link.connect_activate(move |_, param| {
        let (Some(app), Some(uri)) = (app_ref.upgrade(), param.and_then(|p| p.str())) else {
            return;
        };
        let (file_uri, heading) = match uri.split_once('#') {
            Some((file_uri, heading)) => (file_uri, Some(heading)),
            None => (uri, None),
        };
        // "#My%20Heading" in a link means the heading id "My Heading"
        let heading = heading.map(|h| {
            glib::Uri::unescape_string(h, None::<&str>)
                .map(|s| s.to_string())
                .unwrap_or_else(|| h.to_string())
        });
        open_in_window(&app, &w, gio::File::for_uri(file_uri), heading);
    });
    app.add_action(&open_link);

    app.run()
}

fn new_window(app: &adw::Application, windows: &Windows) -> Rc<Window> {
    let win = Window::new(app);
    let w = windows.clone();
    win.window.connect_destroy(move |closed| {
        w.borrow_mut().retain(|win| win.window != *closed);
    });
    windows.borrow_mut().push(win.clone());
    win
}

/// Shows a file: brings its window forward if it's already open, otherwise
/// uses an empty LeafMD window or opens a new one.
fn open_in_window(app: &adw::Application, windows: &Windows, file: gio::File, heading: Option<String>) {
    let uri = file.uri().to_string();
    let (already_open, empty) = {
        let list = windows.borrow();
        (
            list.iter().find(|win| win.file_uri().as_deref() == Some(uri.as_str())).cloned(),
            list.iter().find(|win| !win.has_document()).cloned(),
        )
    };
    if let Some(win) = already_open {
        win.window.present();
        if let Some(heading) = heading {
            win.scroll_to_heading(&heading);
        }
        return;
    }
    let win = empty.unwrap_or_else(|| new_window(app, windows));
    win.window.present();
    win.open_file_at(file, heading);
}

fn set_shortcuts(app: &adw::Application) {
    let shortcuts: &[(&str, &[&str])] = &[
        ("win.new", &["<Control>n"]),
        ("win.open", &["<Control>o"]),
        ("win.save", &["<Control>s"]),
        ("win.save-as", &["<Control><Shift>s"]),
        ("win.help", &["F1"]),
        ("win.cheatsheet", &["<Shift>F1"]),
        ("win.shortcuts", &["<Control>question"]),
        ("win.view::preview", &["<Control>1"]),
        ("win.view::source", &["<Control>2"]),
        ("win.view::split", &["<Control>3"]),
        // Formatting the raw text (only works in Raw and Split view)
        ("win.format::bold", &["<Control>b"]),
        ("win.format::italic", &["<Control>i"]),
        ("win.format::strikethrough", &["<Control><Shift>x"]),
        ("win.format::code", &["<Control>e"]),
        ("win.format::link", &["<Control>k"]),
        // Find and replace
        ("win.find", &["<Control>f"]),
        ("win.find-replace", &["<Control>h"]),
        ("win.find-next", &["F3"]),
        ("win.find-previous", &["<Shift>F3"]),
        // "window.close" is built into GTK and goes through the unsaved-changes check
        ("window.close", &["<Control>w", "<Control>q"]),
    ];
    for &(action, keys) in shortcuts {
        app.set_accels_for_action(action, keys);
    }
}

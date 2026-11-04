mod render;
mod state;
mod window;

use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use window::Window;

pub const APP_ID: &str = "io.github.jeremehancock.LeafMD";

/// LeafMD has a single window. This holds on to it while it is open.
type CurrentWindow = Rc<RefCell<Option<Rc<Window>>>>;

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

    let current = CurrentWindow::default();

    let c = current.clone();
    app.connect_activate(move |app| {
        get_window(app, &c).window.present();
    });

    let c = current.clone();
    app.connect_open(move |app, files, _hint| {
        let win = get_window(app, &c);
        win.window.present();
        if let Some(file) = files.first() {
            win.ask_open_file(file.clone());
        }
    });

    app.run()
}

fn get_window(app: &adw::Application, current: &CurrentWindow) -> Rc<Window> {
    if let Some(win) = current.borrow().as_ref() {
        return win.clone();
    }
    let win = Window::new(app);
    let c = current.clone();
    win.window.connect_destroy(move |_| {
        c.borrow_mut().take();
    });
    *current.borrow_mut() = Some(win.clone());
    win
}

fn set_shortcuts(app: &adw::Application) {
    let shortcuts: [(&str, &[&str]); 10] = [
        ("win.new", &["<Control>n"]),
        ("win.open", &["<Control>o"]),
        ("win.save", &["<Control>s"]),
        ("win.save-as", &["<Control><Shift>s"]),
        ("win.help", &["F1"]),
        ("win.shortcuts", &["<Control>question"]),
        ("win.view::preview", &["<Control>1"]),
        ("win.view::source", &["<Control>2"]),
        ("win.view::split", &["<Control>3"]),
        // "window.close" is built into GTK and goes through the unsaved-changes check
        ("window.close", &["<Control>w", "<Control>q"]),
    ];
    for (action, keys) in shortcuts {
        app.set_accels_for_action(action, keys);
    }
}

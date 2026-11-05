mod file_change;
mod find;
mod format;
mod render;
mod state;
mod typing;
mod window;

use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::io::IsTerminal;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use window::Window;

pub const APP_ID: &str = "io.github.jeremehancock.LeafMD";

/// Every open LeafMD window. Each document gets its own window.
type Windows = Rc<RefCell<Vec<Rc<Window>>>>;

/// Always draws icons from LeafMD's own small theme (data/icon-theme, installed
/// with the Flatpak) instead of the icon theme the user's system has chosen.
/// Anything that theme doesn't draw comes from Adwaita, which comes with the GNOME
/// runtime. That covers the icons GTK adds by itself too, like the window's close
/// button. Without this, the same icon looks different from one distro to the next.
/// For example, the main menu button is three lines on Ubuntu but three dots with Papirus.
///
/// A theme is needed for the icons GTK asks for by name, like maximize, because
/// icons bundled inside the program can't replace ones that Adwaita also has.
/// When the theme isn't installed (`cargo run`), GTK falls back to Adwaita.
fn use_same_icons_everywhere() {
    if let Some(settings) = gtk::Settings::default() {
        settings.set_gtk_icon_theme_name(Some("LeafMD"));
    }
}

/// Whether LeafMD should hand over to a copy of itself that isn't tied to the
/// terminal, so the prompt comes straight back.
///
/// Only when the program's input and both of its outputs are all a terminal.
/// If any of them is something else, LeafMD was started by the app grid or the
/// file manager, or someone sent its output to a file because they want to keep
/// it, so it stays as it is.
///
/// Also never when an option was given: `--help` has to print before LeafMD
/// exits, a mistyped option has to show its error, and `--foreground` is an
/// option that asks to stay, so it doesn't need a rule of its own.
fn should_detach<A: AsRef<OsStr>>(all_terminals: bool, args: &[A]) -> bool {
    all_terminals && !args.iter().any(|arg| arg.as_ref().as_bytes().starts_with(b"-"))
}

/// Starts a copy of LeafMD that carries on after this one exits, with the same
/// arguments and working folder, so `leafmd notes.md` still finds the file.
///
/// The copy's input and outputs go nowhere, so it doesn't see a terminal and
/// won't try to detach again. An error means no copy started.
fn start_detached_copy(args: &[OsString]) -> std::io::Result<()> {
    let mut copy = Command::new(std::env::current_exe()?);
    copy.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // setsid cuts the copy loose from the terminal, so closing the terminal
    // doesn't close LeafMD. Safe here because it only runs that one system
    // call, in the new process just before it starts.
    unsafe {
        copy.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    copy.spawn().map(|_| ())
}

fn main() -> glib::ExitCode {
    // Give the terminal back before anything else starts. If the copy can't be
    // started, carry on here instead, so LeafMD still opens.
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let all_terminals = std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal()
        && std::io::stderr().is_terminal();
    if should_detach(all_terminals, &args) && start_detached_copy(&args).is_ok() {
        return glib::ExitCode::SUCCESS;
    }

    // The view icons built into the program (see build.rs). GTK finds icons
    // under the app's resource path automatically.
    gio::resources_register_include!("leafmd.gresource").expect("LeafMD's built-in icons load");

    let app = adw::Application::builder()
        .application_id(APP_ID)
        // Lets LeafMD be started with a file, e.g. `leafmd notes.md` or a double-click
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // Nothing reads this option: passing it is enough to stop should_detach above.
    // Listing it here means GTK accepts it and shows it in `--help`.
    app.add_main_option(
        "foreground",
        glib::Char::from(0),
        glib::OptionFlags::NONE,
        glib::OptionArg::None,
        "Stay attached to the terminal, showing messages there, until LeafMD closes",
        None,
    );

    app.connect_startup(|app| {
        sourceview5::init();
        use_same_icons_everywhere();
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

#[cfg(test)]
mod tests {
    use super::should_detach;

    #[test]
    fn detaches_from_a_terminal_with_no_arguments_or_only_files() {
        assert!(should_detach::<&str>(true, &[]));
        assert!(should_detach(true, &["notes.md"]));
        assert!(should_detach(true, &["my notes.md", "/tmp/todo.md"]));
    }

    #[test]
    fn stays_when_not_started_from_a_terminal() {
        // The app grid, the file manager, or output sent to a file
        assert!(!should_detach::<&str>(false, &[]));
        assert!(!should_detach(false, &["notes.md"]));
    }

    #[test]
    fn stays_when_given_an_option() {
        assert!(!should_detach(true, &["--foreground"]));
        assert!(!should_detach(true, &["--foreground", "notes.md"]));
        assert!(!should_detach(true, &["notes.md", "--help"]));
        assert!(!should_detach(true, &["--not-a-real-option"]));
    }
}

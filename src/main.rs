mod dialog;
mod document;
mod file_change;
mod find;
mod format;
mod opening;
mod render;
mod state;
mod typing;
mod window;

use std::ffi::{OsStr, OsString};
use std::io::IsTerminal;
use std::ops::ControlFlow;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use adw::prelude::*;
use gtk::{gio, glib};

use window::{Window, Windows};

pub const APP_ID: &str = "io.github.jeremehancock.LeafMD";

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
/// option that asks to stay, so it doesn't need a rule of its own. The exception
/// is `--new-window`, which has nothing to print and gives the prompt back like
/// opening a file does.
fn should_detach<A: AsRef<OsStr>>(all_terminals: bool, args: &[A]) -> bool {
    all_terminals
        && !args.iter().any(|arg| {
            let arg = arg.as_ref().as_bytes();
            arg.starts_with(b"-") && arg != b"--new-window"
        })
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

    app.add_main_option(
        "new-window",
        glib::Char::from(0),
        glib::OptionFlags::NONE,
        glib::OptionArg::None,
        "Open a new window",
        None,
    );

    // This runs before the rest of startup, and works out whether another copy of
    // LeafMD is already running. Registering is what finds that out. Either way
    // the action below then runs in whichever copy is in charge, and startup
    // carries on as normal. So with no file, the usual "activate" step finds the
    // new window and just shows it, rather than reopening the last file.
    app.connect_handle_local_options(|app, options| {
        if options.contains("new-window") {
            if let Err(err) = app.register(gio::Cancellable::NONE) {
                eprintln!("LeafMD: could not start: {err}");
                return ControlFlow::Break(glib::ExitCode::FAILURE);
            }
            app.activate_action("new-window", None);
        }
        ControlFlow::Continue(())
    });

    app.connect_startup(|app| {
        sourceview5::init();
        use_same_icons_everywhere();
        set_shortcuts(app);
    });

    let windows = Windows::default();

    // New Window from the app launcher or `leafmd --new-window`. Inside LeafMD,
    // Ctrl+N uses the window's own action instead, to carry its view across.
    let new_window = gio::SimpleAction::new("new-window", None);
    let (w, weak_app) = (windows.clone(), app.downgrade());
    new_window.connect_activate(move |_, _| {
        if let Some(app) = weak_app.upgrade() {
            Window::open_new(&app, &w).window.present();
        }
    });
    app.add_action(&new_window);

    let w = windows.clone();
    app.connect_activate(move |app| {
        // Started from the app grid with no file: pick up where you left off.
        // Clicking the app icon while LeafMD is already open just shows it.
        let existing = w.borrow().last().cloned();
        match existing {
            Some(win) => win.window.present(),
            None => {
                // Told about its files before it appears, so it starts blank. Once
                // it's on screen, leaving "No Document Open" would be a visible fade.
                let win = Window::open_new(app, &w);
                win.open_last_file();
                win.window.present();
            }
        }
    });

    // Files opened from the file manager or command line
    app.connect_open(move |app, files, _hint| {
        for file in files {
            window::open_file(app, &windows, file.clone(), None, None);
        }
    });

    app.run()
}

fn set_shortcuts(app: &adw::Application) {
    let shortcuts: &[(&str, &[&str])] = &[
        ("win.new", &["<Control>t"]),
        ("win.new-window", &["<Control>n"]),
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
        ("win.close-tab", &["<Control>w"]),
        // "window.close" is built into GTK and goes through the unsaved-changes check
        ("window.close", &["<Control>q"]),
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
    fn detaches_for_a_new_window() {
        assert!(should_detach(true, &["--new-window"]));
        assert!(should_detach(true, &["--new-window", "notes.md"]));
        // Only the terminal decides that, and other options still stay
        assert!(!should_detach(false, &["--new-window"]));
        assert!(!should_detach(true, &["--new-window", "--foreground"]));
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

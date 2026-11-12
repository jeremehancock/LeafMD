mod dialog;
mod export;
mod document;
mod file_change;
mod find;
mod folder_tree;
mod format;
mod headings;
mod opening;
mod preferences;
mod raw_view;
mod render;
mod side_panel;
mod spelling;
mod state;
mod text_size;
mod typing;
mod window;
mod word_count;

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

/// Tells libadwaita which style to use. "system" is libadwaita's own starting
/// choice (follow the desktop, light if it has no preference), not its
/// "default", which would mean "whatever the app's settings say".
fn apply_style(style: &str) {
    let scheme = match style {
        "light" => adw::ColorScheme::ForceLight,
        "dark" => adw::ColorScheme::ForceDark,
        _ => adw::ColorScheme::PreferLight,
    };
    adw::StyleManager::default().set_color_scheme(scheme);
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
        // LeafMD's raw text colours (data/resources/styles), built into the program
        sourceview5::StyleSchemeManager::default()
            .append_search_path("resource:///io/github/jeremehancock/LeafMD/styles/");
        set_shortcuts(app);
        // Light, dark or the desktop's style, as chosen in Preferences. Set
        // before the first window is built, so a dark choice doesn't start light.
        apply_style(state::load_style());
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

    // The Line Numbers setting: one for every window, remembered between runs.
    // The menu item is each window's own "line-numbers" action, whose tick
    // shows what that window shows (Distraction Free hides them); it changes
    // this setting through change_action_state.
    let line_numbers =
        gio::SimpleAction::new_stateful("line-numbers", None, &state::load_line_numbers().to_variant());
    let w = windows.clone();
    line_numbers.connect_change_state(move |action, value| {
        let Some(show) = value.and_then(|v| v.get::<bool>()) else { return };
        action.set_state(&show.to_variant());
        state::save_line_numbers(show);
        for win in w.borrow().iter() {
            win.update_line_numbers();
        }
    });
    app.add_action(&line_numbers);

    // Check Spelling in the main menu and in the raw text's right-click menu:
    // one choice for every window and tab, remembered like Line Numbers.
    let spell_check =
        gio::SimpleAction::new_stateful("spell-check", None, &state::load_spell_check().to_variant());
    let w = windows.clone();
    spell_check.connect_change_state(move |action, value| {
        let Some(on) = value.and_then(|v| v.get::<bool>()) else { return };
        action.set_state(&on.to_variant());
        state::save_spell_check(on);
        for win in w.borrow().iter() {
            win.update_spell_check(on);
        }
    });
    app.add_action(&spell_check);

    // Style in Preferences: "system", "light" or "dark", for every window at
    // once and remembered between runs. Nothing else needs telling: the raw
    // text, the formatted view and the header bar all follow libadwaita's
    // light-or-dark answer, which apply_style changes.
    let style = gio::SimpleAction::new_stateful(
        "style",
        Some(glib::VariantTy::STRING),
        &state::load_style().to_variant(),
    );
    style.connect_change_state(|action, value| {
        let Some(chosen) = value.and_then(|v| v.get::<String>()) else { return };
        let Some(chosen) = state::STYLES.into_iter().find(|style| *style == chosen) else {
            return;
        };
        action.set_state(&chosen.to_variant());
        state::save_style(chosen);
        apply_style(chosen);
    });
    app.add_action(&style);

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

    // Files and folders opened from the file manager or command line. Each
    // folder opens in the side panel (`leafmd .`), and files given alongside
    // one open as tabs in that folder's window.
    app.connect_open(move |app, given, _hint| {
        let is_folder = |file: &gio::File| {
            file.query_file_type(gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE)
                == gio::FileType::Directory
        };
        let (folders, files): (Vec<_>, Vec<_>) = given.iter().cloned().partition(is_folder);
        let folder_window = folders
            .into_iter()
            .map(|folder| window::open_folder(app, &windows, folder))
            .last();
        for file in files {
            match &folder_window {
                Some(win) => win.open_file_here(file, None),
                None => window::open_file(app, &windows, file, None, None),
            }
        }
    });

    app.run()
}

fn set_shortcuts(app: &adw::Application) {
    let shortcuts: &[(&str, &[&str])] = &[
        ("win.new", &["<Control>t"]),
        ("win.new-window", &["<Control>n"]),
        ("win.open", &["<Control>o"]),
        ("win.open-folder", &["<Control><Shift>o"]),
        ("win.side-panel", &["F9"]),
        ("win.distraction-free", &["<Control><Shift>d"]),
        ("win.fullscreen", &["F11"]),
        ("win.save", &["<Control>s"]),
        ("win.save-as", &["<Control><Shift>s"]),
        ("win.print", &["<Control>p"]),
        ("win.help", &["F1"]),
        ("win.cheatsheet", &["<Shift>F1"]),
        ("win.shortcuts", &["<Control>question"]),
        ("win.view::preview", &["<Control>1"]),
        ("win.view::source", &["<Control>2"]),
        ("win.view::split", &["<Control>3"]),
        // Zoom, in both views. Ctrl+= is Ctrl+plus without Shift on most keyboards.
        ("win.zoom::in", &["<Control>plus", "<Control>equal", "<Control>KP_Add"]),
        ("win.zoom::out", &["<Control>minus", "<Control>KP_Subtract"]),
        ("win.zoom::reset", &["<Control>0", "<Control>KP_0"]),
        ("win.preferences", &["<Control>comma"]),
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

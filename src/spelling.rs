//! Spell checking in the raw text, using libspelling (the same library GNOME
//! Text Editor uses). libspelling underlines misspelt words as the text
//! changes, skips anything GtkSourceView's markdown colouring marks as code or
//! an address, and supplies the right-click menu: suggestions, Add to
//! Dictionary, Ignore, Check Spelling and Languages.
//!
//! LeafMD adds three things around it:
//!   - one checker shared by every document, so Ignore and the language apply
//!     everywhere at once (both belong to the checker)
//!   - one on/off choice for every window and tab, the "spell-check" action
//!     set up in main.rs, which the right-click Check Spelling item also
//!     changes, and which is remembered like Line Numbers
//!   - remembering the language the user picks from Languages

use std::cell::OnceCell;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

use crate::raw_view::RawView;
use crate::state;

/// The language used when the system's own has no dictionary
const FALLBACK_LANGUAGE: &str = "en_US";

thread_local! {
    static CHECKER: OnceCell<libspelling::Checker> = const { OnceCell::new() };
}

/// The checker every document shares, made the first time it's needed.
fn checker() -> libspelling::Checker {
    CHECKER.with(|cell| cell.get_or_init(make_checker).clone())
}

/// Picks the language: the one the user chose last time, if it still has a
/// dictionary; else the system's; else English (US). Once picked, any change
/// made from the Languages menu is remembered.
fn make_checker() -> libspelling::Checker {
    // libspelling's one-time setup, needed before anything else of its is used
    libspelling::init();
    let provider = libspelling::Provider::default();
    let language = state::load_spell_check_language()
        .filter(|chosen| provider.supports_language(chosen))
        .or_else(|| system_language(&provider))
        .unwrap_or_else(|| FALLBACK_LANGUAGE.to_string());
    let checker = libspelling::Checker::new(Some(&provider), Some(&language));
    checker.connect_language_notify(|checker| {
        if let Some(language) = checker.language() {
            state::save_spell_check_language(&language);
        }
    });
    checker
}

/// The first of the system's languages that has a dictionary, such as "en_GB"
/// for someone whose system is in British English.
///
/// GLib lists each language from most to least specific: "en_GB.UTF-8",
/// "en_GB", "en", and finally "C" (no language at all). The ".UTF-8" forms
/// and "C" are skipped. libspelling's own choice isn't used, because with no
/// matching dictionary it falls back to the first one it has (Australian
/// English, alphabetically), and LeafMD wants US English then.
fn system_language(provider: &libspelling::Provider) -> Option<String> {
    glib::language_names()
        .iter()
        .map(|name| name.as_str())
        .filter(|name| !name.contains('.') && *name != "C" && *name != "POSIX")
        .find(|name| provider.supports_language(name))
        .map(str::to_string)
}

/// Whether spell checking is on: the "spell-check" action's state. On when
/// there's no app yet, as on the first run.
pub fn wanted() -> bool {
    gio::Application::default()
        .and_then(|app| app.lookup_action("spell-check"))
        .and_then(|action| action.state())
        .and_then(|state| state.get::<bool>())
        .unwrap_or(true)
}

/// Starts spell checking one document's raw text and gives it the spelling
/// items in its right-click menu. The returned adapter is what joins the
/// checker to the text; keep it as long as the document.
pub fn attach(view: &RawView, buffer: &sourceview5::Buffer) -> libspelling::TextBufferAdapter {
    let adapter = libspelling::TextBufferAdapter::new(buffer, &checker());
    // The view draws its own wave under words with the checker's tag (see raw_view.rs)
    view.underline_tag(adapter.tag());
    view.set_extra_menu(Some(&adapter.menu_model()));
    // The menu's items are actions named "spelling.add", "spelling.ignore"
    // and so on, which the adapter provides
    view.insert_action_group("spelling", Some(&adapter));
    adapter.set_enabled(wanted());

    // The right-click menu's own Check Spelling only changes this document.
    // Pass the change on to the shared setting, which then changes every
    // other document and the main menu's tick to match.
    adapter.connect_enabled_notify(|adapter| {
        let on = adapter.is_enabled();
        if on != wanted()
            && let Some(app) = gio::Application::default()
        {
            app.change_action_state("spell-check", &on.to_variant());
        }
    });

    suggest_for_clicked_word(view.upcast_ref(), &adapter);
    adapter
}

/// libspelling suggests spellings for the word at the cursor, a moment after
/// the cursor stops moving. A right-click opens the menu straight away, so
/// the cursor is moved to the clicked word first (unless the click is inside
/// a selection, which GTK keeps) and the suggestions are updated at once. The
/// keys that open the menu (Menu, Shift+F10) update them too.
fn suggest_for_clicked_word(view: &sourceview5::View, adapter: &libspelling::TextBufferAdapter) {
    let click = gtk::GestureClick::builder()
        .button(gdk::BUTTON_SECONDARY)
        // Before the text view's own handling, which opens the menu
        .propagation_phase(gtk::PropagationPhase::Capture)
        .build();
    let weak = adapter.downgrade();
    click.connect_pressed(move |click, _, x, y| {
        let Some(adapter) = weak.upgrade() else { return };
        let Some(view) = click.widget().and_downcast::<gtk::TextView>() else {
            return;
        };
        let (bx, by) = view.window_to_buffer_coords(gtk::TextWindowType::Widget, x as i32, y as i32);
        if let Some(iter) = view.iter_at_location(bx, by) {
            let buffer = view.buffer();
            let in_selection = buffer
                .selection_bounds()
                .is_some_and(|(start, end)| iter.in_range(&start, &end));
            if !in_selection {
                buffer.place_cursor(&iter);
            }
        }
        adapter.update_corrections();
    });
    view.add_controller(click);

    let keys = gtk::EventControllerKey::builder()
        .propagation_phase(gtk::PropagationPhase::Capture)
        .build();
    let weak = adapter.downgrade();
    keys.connect_key_pressed(move |_, key, _, modifiers| {
        let opens_menu = key == gdk::Key::Menu
            || (key == gdk::Key::F10 && modifiers.contains(gdk::ModifierType::SHIFT_MASK));
        if opens_menu && let Some(adapter) = weak.upgrade() {
            adapter.update_corrections();
        }
        glib::Propagation::Proceed
    });
    view.add_controller(keys);
}

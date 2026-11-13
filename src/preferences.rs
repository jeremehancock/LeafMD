//! The Preferences window (Ctrl+,): the style (follow the desktop, light or
//! dark), the margins (none, narrow, normal or wide), the raw text's font and the
//! formatted view's font, and nothing else. Each choice shows everywhere at
//! once and is remembered. The style and margins are the app's "style" and
//! "margins" actions (set up in main.rs); the fonts' work is in text_size.rs,
//! and Reset puts a font back to the starting one.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

use crate::state::{self, FontFor, Margins};
use crate::text_size;

thread_local! {
    /// The Preferences window, while it's open, so a second Ctrl+, doesn't
    /// open another.
    static OPEN: RefCell<glib::WeakRef<adw::PreferencesDialog>> = RefCell::new(glib::WeakRef::new());
}

/// Opens Preferences over a window, or brings it there if it's already open.
pub fn show(parent: &impl IsA<gtk::Widget>) {
    if let Some(open) = OPEN.with(|open| open.borrow().upgrade()) {
        // A dialog belongs to one window, so it closes there and opens again here
        open.close();
    }
    let dialog = adw::PreferencesDialog::builder()
        .title("Preferences")
        .search_enabled(false)
        // Two groups of two short rows: no need for the full height on a big screen
        .content_height(420)
        .build();
    let page = adw::PreferencesPage::new();
    page.add(&appearance_group());
    page.add(&fonts_group());
    dialog.add(&page);
    OPEN.with(|open| open.borrow().set(Some(&dialog)));
    dialog.present(Some(parent));
}

/// The Appearance group: a Style row with Follow System, Light and Dark side
/// by side, and a Margins row with None, Narrow, Normal and Wide.
fn appearance_group() -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::builder().title("Appearance").build();
    // The names match the action's values in state::STYLES
    group.add(&choice_row(
        "Style",
        "style",
        &[("system", "Follow System"), ("light", "Light"), ("dark", "Dark")],
        state::load_style(),
    ));
    let margins: Vec<_> = Margins::ALL.iter().map(|m| (m.name(), m.label())).collect();
    group.add(&choice_row("Margins", "margins", &margins, state::current_margins().name()));
    group
}

/// A row of choices side by side, tied to one of the app's actions with a
/// word as its state (such as "style"). Clicking a choice changes the action,
/// which does the rest; `saved` is shown if the action can't be asked.
fn choice_row(title: &str, action: &'static str, choices: &[(&str, &str)], saved: &str) -> adw::ActionRow {
    let toggles = adw::ToggleGroup::builder().valign(gtk::Align::Center).build();
    for (name, label) in choices {
        toggles.add(adw::Toggle::builder().name(*name).label(*label).build());
    }
    let app = gtk::gio::Application::default();
    let current = app
        .as_ref()
        .and_then(|app| app.action_state(action))
        .and_then(|state| state.get::<String>())
        .unwrap_or_else(|| saved.to_string());
    toggles.set_active_name(Some(&current));
    toggles.connect_active_name_notify(move |toggles| {
        if let (Some(name), Some(app)) = (toggles.active_name(), gtk::gio::Application::default())
            && app.action_state(action).and_then(|s| s.get::<String>()).as_deref() != Some(name.as_str())
        {
            app.change_action_state(action, &name.to_variant());
        }
    });
    // And the other way: if the choice changes some other way while
    // Preferences is open, the buttons follow. The link is undone when the
    // row goes away.
    if let Some(app) = app {
        let weak = toggles.downgrade();
        let handler = app.connect_action_state_changed(Some(action), move |_, _, state| {
            if let (Some(toggles), Some(name)) = (weak.upgrade(), state.get::<String>()) {
                toggles.set_active_name(Some(&name));
            }
        });
        let handler = std::cell::RefCell::new(Some(handler));
        toggles.connect_destroy(move |_| {
            if let Some(handler) = handler.take() {
                app.disconnect(handler);
            }
        });
    }

    let row = adw::ActionRow::builder().title(title).build();
    row.add_suffix(&toggles);
    row
}

/// The Fonts group: a Raw Text row and a Formatted View row.
fn fonts_group() -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::builder().title("Fonts").build();
    group.add(&font_row("Raw Text", FontFor::Raw));
    group.add(&font_row("Formatted View", FontFor::Formatted));
    group
}

/// One font's row: the font button, and Reset beside it while the user's own
/// font is in use.
fn font_row(title: &str, which: FontFor) -> adw::ActionRow {

    let button = gtk::FontDialogButton::builder()
        .dialog(&gtk::FontDialog::builder().title("Choose a Font").build())
        // The picker shows family, style and size. Rare details like a
        // condensed width are dropped (see text_size::kept_parts).
        .level(gtk::FontLevel::Font)
        .use_font(false)
        .use_size(false)
        .valign(gtk::Align::Center)
        .build();
    button.set_font_desc(&text_size::current_font(which));

    let reset = gtk::Button::builder()
        .icon_name("edit-undo-symbolic")
        .tooltip_text("Reset to the starting font")
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .visible(state::load_font(which).is_some())
        .build();

    let row = adw::ActionRow::builder().title(title).build();
    row.add_suffix(&reset);
    row.add_suffix(&button);
    row.set_activatable_widget(Some(&button));

    // True while LeafMD itself puts a font into the button (after Reset, or to
    // drop a style the user picked), so that doesn't count as a choice
    let quiet = Rc::new(Cell::new(false));

    let reset_button = reset.clone();
    let quiet_now = quiet.clone();
    button.connect_font_desc_notify(move |button| {
        if quiet_now.get() {
            return;
        }
        let Some(font) = button.font_desc() else { return };
        let kept = text_size::kept_parts(&font);
        text_size::choose_font(which, Some(&kept));
        // Shows the font as saved
        quiet_now.set(true);
        button.set_font_desc(&kept);
        quiet_now.set(false);
        reset_button.set_visible(true);
    });

    let font_button = button.clone();
    reset.connect_clicked(move |reset| {
        text_size::choose_font(which, None);
        quiet.set(true);
        font_button.set_font_desc(&text_size::current_font(which));
        quiet.set(false);
        reset.set_visible(false);
        // The button that was clicked has gone, so the keyboard moves to the font
        font_button.grab_focus();
    });

    row
}

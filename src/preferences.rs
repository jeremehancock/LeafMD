//! The Preferences window (Ctrl+,): the raw text's font and the formatted
//! view's font, and nothing else. Choosing a font shows it everywhere at once
//! and remembers it; Reset goes back to the starting font. The work itself is
//! in text_size.rs.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

use crate::state::{self, FontFor};
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
        // Two short rows: no need for the full height on a big screen
        .content_height(320)
        .build();
    let page = adw::PreferencesPage::new();
    page.add(&font_group("Raw Text", FontFor::Raw));
    page.add(&font_group("Formatted View", FontFor::Formatted));
    dialog.add(&page);
    OPEN.with(|open| open.borrow().set(Some(&dialog)));
    dialog.present(Some(parent));
}

/// One group with a "Font" row: the font button, and Reset beside it while
/// the user's own font is in use.
fn font_group(title: &str, which: FontFor) -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::builder().title(title).build();

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

    let row = adw::ActionRow::builder().title("Font").build();
    row.add_suffix(&reset);
    row.add_suffix(&button);
    row.set_activatable_widget(Some(&button));
    group.add(&row);

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

    group
}

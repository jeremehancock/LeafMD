//! The find and replace bar that slides down under the toolbar (Ctrl+F, Ctrl+H).
//!
//! This file only builds the bar's widgets. What happens when you type or press
//! a button is in window.rs, because it needs the editor and the formatted view.

use gtk::{gio, prelude::*};
use sourceview5::prelude::*;

pub struct FindBar {
    /// GTK's sliding search bar. It has the close button and handles Escape.
    pub bar: gtk::SearchBar,
    pub entry: gtk::SearchEntry,
    /// "3 of 12", "No matches" and so on
    pub count: gtk::Label,
    /// Shows or hides the second row with the replace box
    pub replace_toggle: gtk::ToggleButton,
    pub replace_row: gtk::Box,
    pub replace_entry: gtk::Entry,
    /// What to look for in the raw text, and how (match case, whole words)
    pub settings: sourceview5::SearchSettings,
    /// Searches the raw text: highlights every match and counts them
    pub context: sourceview5::SearchContext,
}

impl FindBar {
    pub fn new(buffer: &sourceview5::Buffer) -> Self {
        let settings = sourceview5::SearchSettings::new();
        // After the last match, Enter goes back to the first
        settings.set_wrap_around(true);
        let context = sourceview5::SearchContext::new(buffer, Some(&settings));
        context.set_highlight(true);

        // ---- First row: what to find ----
        let entry = gtk::SearchEntry::builder()
            .placeholder_text("Find")
            .width_chars(26)
            // Start searching soon after typing stops
            .search_delay(100)
            .build();

        let count = gtk::Label::builder()
            .width_chars(11)
            .xalign(0.0)
            .css_classes(["dim-label", "numeric"])
            .build();

        // Up and down arrows joined into one pill
        let nav = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        nav.add_css_class("linked");
        nav.append(
            &gtk::Button::builder()
                .icon_name("go-up-symbolic")
                .tooltip_text("Previous match (Shift+Enter)")
                .action_name("win.find-previous")
                .build(),
        );
        nav.append(
            &gtk::Button::builder()
                .icon_name("go-down-symbolic")
                .tooltip_text("Next match (Enter)")
                .action_name("win.find-next")
                .build(),
        );

        let replace_toggle = gtk::ToggleButton::builder()
            .icon_name("leafmd-replace-symbolic")
            .tooltip_text("Replace (Ctrl+H)")
            .build();

        // The two options, shown as tick boxes in a small menu
        let options = gio::Menu::new();
        options.append(Some("Match Case"), Some("win.find-match-case"));
        options.append(Some("Whole Words Only"), Some("win.find-whole-words"));
        let options_button = gtk::MenuButton::builder()
            .icon_name("view-more-symbolic")
            .tooltip_text("Search options")
            .menu_model(&options)
            .build();

        let find_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        find_row.append(&entry);
        find_row.append(&count);
        find_row.append(&nav);
        find_row.append(&replace_toggle);
        find_row.append(&options_button);

        // ---- Second row: what to replace it with ----
        let replace_entry = gtk::Entry::builder()
            .placeholder_text("Replace with")
            .build();
        let replace_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        replace_row.append(&replace_entry);
        replace_row.append(
            &gtk::Button::builder()
                .label("Replace")
                .tooltip_text("Replace this match and go to the next (Enter)")
                .action_name("win.replace")
                .build(),
        );
        replace_row.append(
            &gtk::Button::builder()
                .label("Replace All")
                .action_name("win.replace-all")
                .build(),
        );
        replace_row.set_visible(false);

        // Both text boxes the same width, so they line up
        let sizes = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
        sizes.add_widget(&entry);
        sizes.add_widget(&replace_entry);

        let rows = gtk::Box::new(gtk::Orientation::Vertical, 6);
        rows.append(&find_row);
        rows.append(&replace_row);

        let bar = gtk::SearchBar::builder()
            .child(&rows)
            .show_close_button(true)
            .build();
        // Lets the bar close itself when Escape is pressed in the find box
        bar.connect_entry(&entry);

        let row = replace_row.clone();
        replace_toggle.connect_toggled(move |toggle| row.set_visible(toggle.is_active()));

        Self {
            bar,
            entry,
            count,
            replace_toggle,
            replace_row,
            replace_entry,
            settings,
            context,
        }
    }

    /// The text being looked for, or None if the find box is empty.
    pub fn query(&self) -> Option<String> {
        let text = self.entry.text();
        (!text.is_empty()).then(|| text.to_string())
    }

    /// Shows "3 of 12", "12 matches" or "No matches" beside the find box.
    /// `position` is which match is selected (0 if none is), and `count` is
    /// how many there are, or -1 while they're still being counted.
    pub fn show_count(&self, position: i32, count: i32) {
        let text = if self.query().is_none() || count < 0 {
            String::new()
        } else if count == 0 {
            "No matches".to_string()
        } else if position > 0 {
            format!("{position} of {count}")
        } else if count == 1 {
            "1 match".to_string()
        } else {
            format!("{count} matches")
        };
        self.count.set_label(&text);
        // The find box turns red when nothing matches
        if count == 0 && self.query().is_some() {
            self.entry.add_css_class("error");
        } else {
            self.entry.remove_css_class("error");
        }
    }
}

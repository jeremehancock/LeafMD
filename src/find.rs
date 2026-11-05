//! The find and replace bar that slides down under the toolbar (Ctrl+F, Ctrl+H).
//!
//! This file only builds the bar's widgets. What happens when you type or press
//! a button is in window.rs, because it needs the editor and the formatted view.

use gtk::{gio, prelude::*};
use sourceview5::prelude::*;

pub struct FindBar {
    /// GTK's sliding search bar. It has the close button and handles Escape.
    pub bar: gtk::SearchBar,
    /// The bar's contents: the find row, then the replace row
    rows: gtk::Box,
    /// The find box, then the count and buttons
    find_row: gtk::Box,
    /// Takes over from the bar's own close button in a narrow window
    compact_close: gtk::Button,
    /// Keeps the find and replace boxes the same width
    sizes: gtk::SizeGroup,
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

        // Everything after the find box, kept together so that it can wrap onto
        // a row of its own in a narrow window
        let tools = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        tools.append(&count);
        tools.append(&nav);
        tools.append(&replace_toggle);
        tools.append(&options_button);

        // Only shown in a narrow window. The bar's own close button keeps a whole
        // column to itself, which would leave the replace box too little room, so
        // there it's swapped for this one, sharing the find box's row.
        let compact_close = gtk::Button::builder()
            .icon_name("window-close-symbolic")
            .tooltip_text("Close (Esc)")
            .css_classes(["flat"])
            .visible(false)
            .build();
        let entry_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        entry_row.append(&entry);
        entry_row.append(&compact_close);

        let find_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        find_row.append(&entry_row);
        find_row.append(&tools);

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

        let closing = bar.clone();
        compact_close.connect_clicked(move |_| closing.set_search_mode(false));

        Self {
            bar,
            rows,
            find_row,
            compact_close,
            sizes,
            entry,
            count,
            replace_toggle,
            replace_row,
            replace_entry,
            settings,
            context,
        }
    }

    /// How the bar changes in a narrow window. It wraps rather than squeezing:
    /// the count and buttons go on their own row, which leaves the find box a
    /// whole row, and the replace box gets smaller beside its two buttons.
    pub fn add_compact_rules(&self, compact: &adw::Breakpoint) {
        let narrow_box = 8.to_value();
        compact.add_setter(
            &self.find_row,
            "orientation",
            Some(&gtk::Orientation::Vertical.to_value()),
        );
        compact.add_setter(&self.rows, "hexpand", Some(&true.to_value()));
        compact.add_setter(&self.bar, "show-close-button", Some(&false.to_value()));
        compact.add_setter(&self.compact_close, "visible", Some(&true.to_value()));
        compact.add_setter(&self.entry, "hexpand", Some(&true.to_value()));
        compact.add_setter(&self.entry, "width-chars", Some(&narrow_box));
        compact.add_setter(&self.replace_entry, "width-chars", Some(&narrow_box));
        compact.add_setter(&self.replace_entry, "hexpand", Some(&true.to_value()));
        // Pushes the buttons to the right-hand end of their row
        compact.add_setter(&self.count, "hexpand", Some(&true.to_value()));
        // The two boxes don't line up here, so they don't need to match
        compact.add_setter(
            &self.sizes,
            "mode",
            Some(&gtk::SizeGroupMode::None.to_value()),
        );
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

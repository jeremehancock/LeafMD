//! The list of language names that pops up after typing ``` and the first
//! letter of a language at the start of a line in the raw text, like ```py,
//! so code blocks are easy to label for colouring (see highlight.rs).
//!
//! It uses GtkSourceView's own suggestion pop-up (the kind code editors use
//! for completing words). GtkSourceView asks a "provider" what to offer
//! while the user types; this file is that provider, and offers nothing
//! anywhere except on a line opening a code block. Which lines count, and the
//! names, are in typing.rs.

use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;
use gtk::glib::subclass::prelude::*;
use sourceview5::subclass::prelude::*;

use crate::typing;

mod imp {
    use std::cell::RefCell;

    use super::*;

    /// One row of the list: a language name.
    #[derive(Default)]
    pub struct LanguageName {
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LanguageName {
        const NAME: &'static str = "LeafMDLanguageName";
        type Type = super::LanguageName;
        type Interfaces = (sourceview5::CompletionProposal,);
    }

    impl ObjectImpl for LanguageName {}
    impl CompletionProposalImpl for LanguageName {}

    /// What GtkSourceView asks for suggestions.
    #[derive(Default)]
    pub struct LanguageSuggestions {}

    #[glib::object_subclass]
    impl ObjectSubclass for LanguageSuggestions {
        const NAME: &'static str = "LeafMDLanguageSuggestions";
        type Type = super::LanguageSuggestions;
        type Interfaces = (sourceview5::CompletionProvider,);
    }

    impl ObjectImpl for LanguageSuggestions {}

    impl CompletionProviderImpl for LanguageSuggestions {
        // Called as the user types a word. Everywhere but a line opening a
        // code block, it offers nothing, so no list appears.
        fn populate(
            &self,
            context: &sourceview5::CompletionContext,
        ) -> Result<gio::ListModel, glib::Error> {
            let list = gio::ListStore::new::<super::LanguageName>();
            fill(&list, context);
            Ok(list.upcast())
        }

        // Called as the user keeps typing with the list showing: narrows it.
        // An empty list closes the pop-up.
        fn refilter(&self, context: &sourceview5::CompletionContext, model: &gio::ListModel) {
            if let Some(list) = model.downcast_ref::<gio::ListStore>() {
                fill(list, context);
            }
        }

        fn display(
            &self,
            _context: &sourceview5::CompletionContext,
            proposal: &sourceview5::CompletionProposal,
            cell: &sourceview5::CompletionCell,
        ) {
            // A row has several columns (an icon, text before, the text, more
            // details); only the name is shown
            let text = (cell.column() == sourceview5::CompletionColumn::TypedText)
                .then(|| proposal.downcast_ref::<super::LanguageName>())
                .flatten()
                .map(|row| row.imp().name.borrow().clone());
            cell.set_text(text.as_deref());
        }

        // Enter or a click: what's typed after ``` becomes the chosen name,
        // as one undo step
        fn activate(
            &self,
            context: &sourceview5::CompletionContext,
            proposal: &sourceview5::CompletionProposal,
        ) {
            let (Some(row), Some(buffer)) =
                (proposal.downcast_ref::<super::LanguageName>(), context.buffer())
            else {
                return;
            };
            let Some(typed) = typed_here(&buffer) else {
                return;
            };
            let mut end = buffer.iter_at_mark(&buffer.get_insert());
            let mut start = end;
            start.backward_chars(typed.chars().count() as i32);
            buffer.begin_user_action();
            buffer.delete(&mut start, &mut end);
            buffer.insert(&mut start, &row.imp().name.borrow());
            buffer.end_user_action();
        }

        fn key_activates(
            &self,
            _context: &sourceview5::CompletionContext,
            _proposal: &sourceview5::CompletionProposal,
            key: gdk::Key,
            _state: gdk::ModifierType,
        ) -> bool {
            matches!(key, gdk::Key::Return | gdk::Key::KP_Enter | gdk::Key::Tab)
        }
    }

    /// Fills `list` with the names matching what's typed after ``` on the
    /// cursor's line, or empties it when this isn't such a line.
    fn fill(list: &gio::ListStore, context: &sourceview5::CompletionContext) {
        let names = context
            .buffer()
            .and_then(|buffer| typed_here(&buffer))
            .map(|typed| typing::code_languages_starting(&typed))
            .unwrap_or_default();
        let rows: Vec<super::LanguageName> = names.into_iter().map(super::LanguageName::new).collect();
        list.splice(0, list.n_items(), &rows);
    }

    /// What's typed of a language name before the cursor, if the cursor's
    /// line opens a code block (see typing::code_language_typed).
    fn typed_here(buffer: &impl IsA<gtk::TextBuffer>) -> Option<String> {
        let buffer = buffer.as_ref();
        let cursor = buffer.iter_at_mark(&buffer.get_insert());
        let mut line_start = cursor;
        line_start.set_line_offset(0);
        let before_cursor = buffer.text(&line_start, &cursor, false);
        // Cheap test first: this runs as every word is typed, anywhere
        if !before_cursor.trim_start_matches(' ').starts_with("```") {
            return None;
        }
        let before_line = buffer.text(&buffer.start_iter(), &line_start, false);
        typing::code_language_typed(&before_cursor, &before_line).map(str::to_string)
    }
}

glib::wrapper! {
    pub struct LanguageName(ObjectSubclass<imp::LanguageName>)
        @implements sourceview5::CompletionProposal;
}

impl LanguageName {
    fn new(name: &str) -> Self {
        let row: Self = glib::Object::new();
        row.imp().name.replace(name.to_string());
        row
    }
}

glib::wrapper! {
    pub struct LanguageSuggestions(ObjectSubclass<imp::LanguageSuggestions>)
        @implements sourceview5::CompletionProvider;
}

/// Adds the language suggestions to a raw text view.
pub fn add_to(view: &sourceview5::View) {
    let completion = view.completion();
    // The first row is picked as soon as the list shows, so Enter takes it
    completion.set_select_on_show(true);
    completion.set_show_icons(false);
    completion.add_provider(&glib::Object::new::<LanguageSuggestions>());
}

//! Noticing when another app changes the open file: the rule for what counts as
//! a change, and the banner that appears across the top of the window.
//!
//! This file only decides what a difference means and builds the banner's widgets.
//! Watching the file and what the buttons do is in document.rs, because it needs
//! the editor.

use std::sync::Once;

use gtk::{gdk, prelude::*};

/// A version of the file on disk.
#[derive(Clone, Debug, PartialEq)]
pub enum Version {
    /// The file's exact bytes. These are kept rather than the editor's text,
    /// because the editor can differ from the file while there are unsaved changes.
    Contents(Vec<u8>),
    /// There's no file at that place.
    Missing,
}

/// What comparing the file on disk with the version LeafMD knows about found.
#[derive(Debug, PartialEq)]
pub enum Comparison {
    /// Nothing that matters changed, even if the file was rewritten or touched.
    Same,
    /// The file has different contents, or it came back after being deleted.
    Changed,
    /// The file is gone. It may have been deleted, or moved or renamed.
    Deleted,
    /// The file couldn't be read, for example because of a brief permissions
    /// problem. That doesn't mean it changed, so nothing should happen.
    Unknown,
}

/// Compares the version LeafMD knows about with what's on disk now. `on_disk` is
/// None when the file couldn't be read.
pub fn compare(known: &Version, on_disk: Option<&Version>) -> Comparison {
    match on_disk {
        None => Comparison::Unknown,
        Some(on_disk) if on_disk == known => Comparison::Same,
        Some(Version::Missing) => Comparison::Deleted,
        Some(Version::Contents(_)) => Comparison::Changed,
    }
}

/// The banner's words and button for a file that changed or disappeared:
/// the message, the action button's label, and whether that button loses work.
pub fn banner_text(
    file_name: &str,
    on_disk: &Version,
    unsaved_changes: bool,
) -> (String, &'static str, bool) {
    match (on_disk, unsaved_changes) {
        (Version::Missing, _) => (
            format!("“{file_name}” was deleted or moved by another app"),
            "Save Again",
            false,
        ),
        (Version::Contents(_), false) => (
            format!("“{file_name}” was changed by another app"),
            "Reload",
            false,
        ),
        (Version::Contents(_), true) => (
            format!("“{file_name}” was changed by another app, and you have unsaved changes"),
            "Discard and Reload",
            true,
        ),
    }
}

/// The banner: a message and two buttons that slide down under the header bar.
/// GNOME's ready-made banner only allows one button, and this needs two.
pub struct FileBanner {
    pub revealer: gtk::Revealer,
    message: gtk::Label,
    pub ignore: gtk::Button,
    /// Reload, Discard and Reload, or Save Again, depending on what happened
    pub action: gtk::Button,
}

impl FileBanner {
    pub fn new() -> Self {
        load_style();

        // Wraps, so a long file name still fits in a narrow window
        let message = gtk::Label::builder()
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .xalign(0.0)
            .hexpand(true)
            .build();
        // Clicking a button leaves the keyboard where it was, so the person can
        // carry on typing in the document straight afterwards
        let ignore = gtk::Button::builder()
            .label("Ignore")
            .focus_on_click(false)
            .valign(gtk::Align::Center)
            .build();
        let action = gtk::Button::builder()
            .focus_on_click(false)
            .valign(gtk::Align::Center)
            .build();

        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        row.add_css_class("leafmd-file-banner");
        row.append(&message);
        // Ignore goes on the far right so it never moves. The other button gets
        // wider when it becomes "Discard and Reload", and if it were on the right
        // it would push Ignore along, and a click meant for Ignore could land on it.
        row.append(&action);
        row.append(&ignore);

        let revealer = gtk::Revealer::builder()
            .transition_type(gtk::RevealerTransitionType::SlideDown)
            .child(&row)
            .build();

        Self { revealer, message, ignore, action }
    }

    pub fn show(&self, message: &str, action: &str, loses_work: bool) {
        self.message.set_text(message);
        self.action.set_label(action);
        if loses_work {
            self.action.add_css_class("destructive-action");
        } else {
            self.action.remove_css_class("destructive-action");
        }
        self.revealer.set_reveal_child(true);
    }

    pub fn hide(&self) {
        self.revealer.set_reveal_child(false);
    }
}

/// Gives the banner the same tint as GNOME's own banners.
fn load_style() {
    static LOADED: Once = Once::new();
    LOADED.call_once(|| {
        let Some(display) = gdk::Display::default() else {
            return;
        };
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            ".leafmd-file-banner {
                background-color: color-mix(in srgb, var(--accent-bg-color) 30%, transparent);
                padding: 6px 12px;
            }",
        );
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contents(text: &str) -> Version {
        Version::Contents(text.as_bytes().to_vec())
    }

    #[test]
    fn identical_contents_are_the_same() {
        assert_eq!(compare(&contents("hi"), Some(&contents("hi"))), Comparison::Same);
    }

    #[test]
    fn different_contents_are_a_change() {
        assert_eq!(compare(&contents("hi"), Some(&contents("hello"))), Comparison::Changed);
    }

    #[test]
    fn a_missing_file_is_deleted() {
        assert_eq!(compare(&contents("hi"), Some(&Version::Missing)), Comparison::Deleted);
    }

    #[test]
    fn a_file_coming_back_is_a_change() {
        assert_eq!(compare(&Version::Missing, Some(&contents("hi"))), Comparison::Changed);
    }

    #[test]
    fn a_file_still_missing_is_the_same() {
        assert_eq!(compare(&Version::Missing, Some(&Version::Missing)), Comparison::Same);
    }

    #[test]
    fn a_file_that_cant_be_read_doesnt_count_as_a_change() {
        assert_eq!(compare(&contents("hi"), None), Comparison::Unknown);
        assert_eq!(compare(&Version::Missing, None), Comparison::Unknown);
    }

    #[test]
    fn a_changed_file_offers_reload() {
        let (message, action, loses_work) = banner_text("notes.md", &contents("hi"), false);
        assert_eq!(message, "“notes.md” was changed by another app");
        assert_eq!(action, "Reload");
        assert!(!loses_work);
    }

    #[test]
    fn unsaved_changes_make_reloading_a_visible_loss() {
        let (message, action, loses_work) = banner_text("notes.md", &contents("hi"), true);
        assert_eq!(
            message,
            "“notes.md” was changed by another app, and you have unsaved changes"
        );
        assert_eq!(action, "Discard and Reload");
        assert!(loses_work);
    }

    #[test]
    fn a_deleted_file_offers_save_again_with_or_without_unsaved_changes() {
        for unsaved_changes in [false, true] {
            let (message, action, loses_work) =
                banner_text("notes.md", &Version::Missing, unsaved_changes);
            assert_eq!(message, "“notes.md” was deleted or moved by another app");
            assert_eq!(action, "Save Again");
            assert!(!loses_work);
        }
    }
}

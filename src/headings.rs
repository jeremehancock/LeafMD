//! Finds a document's headings, for the Headings page of the side panel.
//!
//! The markdown is read with the same rules as the formatted view, so only
//! real markdown headings count: `#` lines and text underlined with `===` or
//! `---`. A `#` line inside a code block is code, and `<h2>` written as HTML
//! isn't a markdown heading, so neither is listed.
//!
//! The Headings page shows them as a list, indented by level. Clicking one
//! asks the window to jump there, and as the document scrolls, the heading
//! of the section at the top is highlighted.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use comrak::nodes::NodeValue;
use comrak::{Arena, parse_document};
use gtk::glib;

use crate::{render, word_count};

/// What the page runs when a heading is picked, given the line it starts on
type JumpHandler = Box<dyn Fn(usize)>;

/// How far in each level below the document's top one is indented, in pixels
const INDENT: i32 = 12;

#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    /// 1 for `#`, 2 for `##`, and so on
    pub level: u8,
    /// The heading as it reads, without markdown marks
    pub text: String,
    /// The line of the raw text it starts on, counting from 1
    pub line: usize,
}

/// Every heading in the markdown, in order.
pub fn find(markdown: &str) -> Vec<Heading> {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &render::markdown_options());
    root.descendants()
        .filter_map(|node| {
            let data = node.data();
            let NodeValue::Heading(heading) = &data.value else {
                return None;
            };
            let level = heading.level;
            let line = data.sourcepos.start.line;
            // Squeeze the spaces the text picked up between its parts
            let text = word_count::reader_text(node)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            (!text.is_empty()).then_some(Heading { level, text, line })
        })
        .collect()
}

/// Which heading's section a line is in: the last heading starting at or
/// above it. `line` can be fractional (12.5 is halfway through line 12), and
/// a heading counts once its line is mostly at the top, so a heading jumped to
/// is highlighted even if the view stops a fraction above it. None above the
/// first heading.
pub fn section_at(headings: &[Heading], line: f64) -> Option<usize> {
    headings.iter().rposition(|heading| heading.line as f64 <= line + 0.5)
}

/// The Headings page of the side panel.
pub struct HeadingsPage {
    /// The whole page: the list, or a message in its place
    pub widget: gtk::Stack,
    list: gtk::ListBox,
    scrolled: gtk::ScrolledWindow,
    message: adw::StatusPage,
    /// The headings the list shows, so an unchanged list isn't rebuilt
    shown: RefCell<Vec<Heading>>,
    /// Which row is highlighted, if any
    highlighted: Cell<Option<usize>>,
    /// What to do when a heading is picked: given its line
    on_jump: RefCell<Option<JumpHandler>>,
}

impl HeadingsPage {
    pub fn new() -> Rc<Self> {
        let list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            // GNOME's sidebar look, the same as the Files page: rounded rows
            // with a coloured highlight
            .css_classes(["navigation-sidebar"])
            .build();
        let scrolled = gtk::ScrolledWindow::builder()
            .child(&list)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .build();
        let message = adw::StatusPage::builder()
            .icon_name("leafmd-headings-symbolic")
            .css_classes(["compact"])
            .build();
        let widget = gtk::Stack::new();
        widget.add_named(&scrolled, Some("list"));
        widget.add_named(&message, Some("message"));

        let page = Rc::new(Self {
            widget,
            list,
            scrolled,
            message,
            shown: RefCell::new(Vec::new()),
            highlighted: Cell::new(None),
            on_jump: RefCell::new(None),
        });
        page.show(None);

        // A click, or Enter on the row the keyboard is on
        let weak = Rc::downgrade(&page);
        page.list.connect_row_activated(move |_, row| {
            let Some(page) = weak.upgrade() else { return };
            let index = row.index();
            let line = usize::try_from(index)
                .ok()
                .and_then(|index| page.shown.borrow().get(index).map(|heading| heading.line));
            if let (Some(line), Some(jump)) = (line, page.on_jump.borrow().as_ref()) {
                jump(line);
            }
        });
        page
    }

    /// What to do when a heading is picked: given the line it starts on.
    pub fn connect_jump(&self, jump: impl Fn(usize) + 'static) {
        *self.on_jump.borrow_mut() = Some(Box::new(jump));
    }

    /// Shows a document's headings, or with None, that no document is open.
    pub fn show(&self, headings: Option<&[Heading]>) {
        let Some(headings) = headings else {
            self.show_message("No Document Open", "");
            self.shown.borrow_mut().clear();
            return;
        };
        if headings.is_empty() {
            self.show_message("No Headings", "Lines starting with # appear here");
            self.shown.borrow_mut().clear();
            return;
        }
        self.widget.set_visible_child_name("list");
        if *self.shown.borrow() == headings {
            return;
        }
        // Rebuilding the rows can move the list, so keep where it was scrolled
        let adjustment = self.scrolled.vadjustment();
        let scrolled_to = adjustment.value();
        self.list.remove_all();
        self.highlighted.set(None);
        let top_level = headings.iter().map(|heading| heading.level).min().unwrap_or(1);
        for heading in headings {
            let label = gtk::Label::builder()
                .label(&heading.text)
                .xalign(0.0)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .margin_start(INDENT * i32::from(heading.level - top_level))
                .build();
            let row = gtk::ListBoxRow::builder().child(&label).build();
            // Long headings are cut off with "…", so hovering shows them whole
            row.set_tooltip_text(Some(&heading.text));
            self.list.append(&row);
        }
        *self.shown.borrow_mut() = headings.to_vec();
        let adjustment_for_later = adjustment.clone();
        glib::idle_add_local_once(move || adjustment_for_later.set_value(scrolled_to));
        adjustment.set_value(scrolled_to);
    }

    fn show_message(&self, title: &str, description: &str) {
        self.message.set_title(title);
        self.message.set_description(Some(description).filter(|text| !text.is_empty()));
        self.widget.set_visible_child_name("message");
        self.list.remove_all();
        self.highlighted.set(None);
    }

    /// Highlights the heading of the section that `line` (the line at the
    /// top of the document) is in, and scrolls the list to it if it's out of
    /// sight. Nothing is highlighted above the first heading.
    pub fn set_top_line(&self, line: f64) {
        let index = section_at(&self.shown.borrow(), line);
        if index == self.highlighted.get() {
            return;
        }
        self.highlighted.set(index);
        let row = index.and_then(|index| self.list.row_at_index(index as i32));
        self.list.select_row(row.as_ref());
        if let Some(row) = row {
            self.keep_in_sight(&row);
        }
    }

    /// Scrolls the list just enough to show `row`, if it isn't showing already.
    fn keep_in_sight(&self, row: &gtk::ListBoxRow) {
        let Some(bounds) = row.compute_bounds(&self.list) else {
            return;
        };
        let adjustment = self.scrolled.vadjustment();
        let top = f64::from(bounds.y());
        let bottom = top + f64::from(bounds.height());
        if top < adjustment.value() {
            adjustment.set_value(top);
        } else if bottom > adjustment.value() + adjustment.page_size() {
            adjustment.set_value(bottom - adjustment.page_size());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary(markdown: &str) -> Vec<(u8, String, usize)> {
        find(markdown).into_iter().map(|h| (h.level, h.text, h.line)).collect()
    }

    #[test]
    fn a_short_outline() {
        let text = "# Garden Plan\n\nIntro.\n\n## Seeds\n\n### Tomatoes\n\nText\n\n## Beds\n";
        assert_eq!(
            summary(text),
            vec![
                (1, "Garden Plan".into(), 1),
                (2, "Seeds".into(), 5),
                (3, "Tomatoes".into(), 7),
                (2, "Beds".into(), 11),
            ]
        );
    }

    #[test]
    fn formatting_is_left_out() {
        assert_eq!(
            summary("## **Install** [LeafMD](https://leafmd.app)"),
            vec![(2, "Install LeafMD".into(), 1)]
        );
        assert_eq!(summary("# Use `leafmd .` here"), vec![(1, "Use leafmd . here".into(), 1)]);
    }

    #[test]
    fn code_blocks_and_html_have_no_headings() {
        assert!(summary("```\n# install the tools\n```").is_empty());
        assert!(summary("<h2>Install</h2>").is_empty());
    }

    #[test]
    fn underlined_headings() {
        assert_eq!(
            summary("Garden Plan\n===\n\nSeeds\n---\n"),
            vec![(1, "Garden Plan".into(), 1), (2, "Seeds".into(), 4)]
        );
    }

    #[test]
    fn which_section_a_line_is_in() {
        let headings = find("Intro\n\n# Garden Plan\n\n## Seeds\n\nText\n\n## Beds\n");
        // Lines 3, 5 and 9
        assert_eq!(section_at(&headings, 1.0), None);
        assert_eq!(section_at(&headings, 3.0), Some(0));
        assert_eq!(section_at(&headings, 7.2), Some(1));
        // A heading a fraction below the top already counts
        assert_eq!(section_at(&headings, 8.6), Some(2));
        assert_eq!(section_at(&headings, 40.0), Some(2));
    }

    #[test]
    fn empty_headings_are_skipped() {
        assert_eq!(summary("#\n\n## Seeds"), vec![(2, "Seeds".into(), 3)]);
    }
}

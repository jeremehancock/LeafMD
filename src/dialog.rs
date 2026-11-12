//! The questions LeafMD asks about saving, laid out like gedit's: a heading, some
//! text, and a row of equal buttons, which stack only when the window is too narrow.
//! (The window's width when the question opens decides that.)
//!
//! GNOME's ready-made question dialog stacks three buttons this long whatever
//! it's told, and colours a recommended button with the desktop's accent colour,
//! which can clash. Here only the button that loses work has a colour.

use std::cell::RefCell;
use std::future::poll_fn;
use std::rc::Rc;
use std::sync::Once;
use std::task::{Poll, Waker};

use adw::prelude::*;
use gtk::{gdk, glib};

/// How wide the question is when there's room. The window's own width limits it.
const WIDTH: i32 = 560;

/// In a window narrower than this, the buttons stack instead of sharing a row.
const STACK_BELOW: i32 = 480;

/// The gap kept between a floating question and each side of the window.
const SIDE_GAP: i32 = 24;

/// The space around the question's contents.
const MARGIN: i32 = 24;

/// The height of one stacked button and the gap after it, roughly.
const BUTTON_SPACE: i32 = 56;

/// One button: what it answers, its label, and whether it loses work (shown in red).
pub struct Answer {
    pub id: &'static str,
    pub label: &'static str,
    pub loses_work: bool,
}

/// The answer given so far, and who to wake up when it arrives.
#[derive(Default)]
struct Waiting {
    answer: Option<String>,
    waker: Option<Waker>,
}

pub struct Question {
    dialog: adw::Dialog,
    /// Where extra_child puts its widget, between the text and the buttons
    extra: gtk::Box,
    /// The heading, text and anything extra, and what scrolls them
    text: gtk::Box,
    scroller: gtk::ScrolledWindow,
    /// The buttons, side by side or stacked
    row: gtk::Box,
    buttons: Vec<(&'static str, gtk::Button)>,
    waiting: Rc<RefCell<Waiting>>,
}

impl Question {
    /// `default` is the answer Enter gives, and `cancel` the one Escape gives.
    pub fn new(
        heading: &str,
        body: &str,
        answers: &[Answer],
        default: &'static str,
        cancel: &'static str,
    ) -> Self {
        load_style();
        let waiting: Rc<RefCell<Waiting>> = Rc::default();
        let dialog = adw::Dialog::builder()
            .content_width(WIDTH)
            .follows_content_size(false)
            .build();

        let heading = gtk::Label::builder()
            .label(keep_last_words_together(heading))
            .wrap(true)
            .justify(gtk::Justification::Center)
            .css_classes(["title-3"])
            .build();
        let body = gtk::Label::builder()
            .label(keep_last_words_together(body))
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        let extra = gtk::Box::new(gtk::Orientation::Vertical, 12);
        extra.set_visible(false);
        let text = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(MARGIN)
            .margin_bottom(18)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .build();
        text.append(&heading);
        text.append(&body);
        text.append(&extra);

        // Homogeneous is what makes the buttons all the same width
        let row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .spacing(12)
            .margin_bottom(MARGIN)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .build();
        let mut buttons = Vec::new();
        for answer in answers {
            let button = gtk::Button::builder()
                .label(answer.label)
                .use_underline(true)
                .css_classes(["leafmd-question-button"])
                .build();
            if answer.loses_work {
                button.add_css_class("leafmd-loses-work");
            }
            row.append(&button);
            let (id, waiting, closing) = (answer.id, waiting.clone(), dialog.clone());
            button.connect_clicked(move |_| {
                give_answer(&waiting, id);
                closing.close();
            });
            buttons.push((answer.id, button));
        }
        if let Some((_, button)) = buttons.iter().find(|(id, _)| *id == default) {
            dialog.set_default_widget(Some(button));
            dialog.set_focus(Some(button));
        }
        // Escape, or anything else that closes it without a button
        let waiting_to_close = waiting.clone();
        dialog.connect_closed(move |_| give_answer(&waiting_to_close, cancel));

        // The text scrolls if a very short window can't fit it, so that the
        // buttons are always there
        let scroller = gtk::ScrolledWindow::builder()
            .child(&text)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_height(true)
            .vexpand(true)
            .build();
        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&scroller);
        content.append(&row);
        dialog.set_child(Some(&content));

        Self { dialog, extra, text, scroller, row, buttons, waiting }
    }

    /// Adds something between the text and the buttons.
    pub fn add_extra(&self, widget: &impl IsA<gtk::Widget>) {
        self.extra.append(widget);
        self.extra.set_visible(true);
    }

    /// One of the buttons, for greying it out.
    pub fn button(&self, id: &str) -> Option<gtk::Button> {
        self.buttons
            .iter()
            .find(|(button_id, _)| *button_id == id)
            .map(|(_, button)| button.clone())
    }

    /// Shows the question over `parent` and waits for the answer.
    pub async fn ask(&self, parent: Option<&impl IsA<gtk::Widget>>) -> String {
        // Laid out here, before the dialog is measured, because its height is
        // worked out from its width and from how the buttons are laid out.
        // Then again whenever the window changes size while it's open, or a
        // narrower window would squeeze the text in behind the buttons.
        let mut watching = Vec::new();
        if let Some(parent) = parent {
            let parent = parent.as_ref().clone();
            let layout = self.layout();
            layout.fit(parent.width(), parent.height());
            if let Some(window) = parent.root().and_downcast::<gtk::Window>() {
                for property in ["default-width", "default-height", "maximized", "fullscreened"] {
                    let (layout, parent) = (layout.clone(), parent.clone());
                    let id = window.connect_notify_local(Some(property), move |_, _| {
                        // After GTK has given the window its new size
                        let (layout, parent) = (layout.clone(), parent.clone());
                        glib::idle_add_local_once(move || layout.fit(parent.width(), parent.height()));
                    });
                    watching.push((window.clone(), id));
                }
            }
        }
        self.dialog.present(parent);
        let waiting = self.waiting.clone();
        let answer = poll_fn(move |context| {
            let mut waiting = waiting.borrow_mut();
            match waiting.answer.take() {
                Some(answer) => Poll::Ready(answer),
                None => {
                    waiting.waker = Some(context.waker().clone());
                    Poll::Pending
                }
            }
        })
        .await;
        for (window, id) in watching {
            window.disconnect(id);
        }
        answer
    }

    fn layout(&self) -> Layout {
        Layout {
            dialog: self.dialog.clone(),
            text: self.text.clone(),
            scroller: self.scroller.clone(),
            row: self.row.clone(),
            buttons: self.buttons.iter().map(|(_, button)| button.clone()).collect(),
        }
    }
}

/// The parts of a question that change with the window's size.
#[derive(Clone)]
struct Layout {
    dialog: adw::Dialog,
    text: gtk::Box,
    scroller: gtk::ScrolledWindow,
    row: gtk::Box,
    /// In the order they sit in a row
    buttons: Vec<gtk::Button>,
}

impl Layout {
    /// Sizes the question for a window `width` by `height`, with the buttons
    /// in a row if it fits, otherwise stacked.
    fn fit(&self, width: i32, height: i32) {
        // A floating question needs a gap on each side, or it spills past
        // the window's edges and its heading is cut off. A narrow window
        // shows it as a panel along the bottom, which may fill the width.
        let room = if width < STACK_BELOW { width } else { width - 2 * SIDE_GAP };
        // The buttons share a row only if it fits. They're all as wide as
        // the widest ("Close without Saving"), so the row can need a little
        // more than WIDTH, and more than a window that isn't narrow has.
        let row_needs = self.row_width();
        if width < STACK_BELOW || row_needs > room {
            let content_width = room.min(WIDTH);
            self.dialog.set_content_width(content_width);
            self.stack_buttons(content_width, height);
        } else {
            self.dialog.set_content_width(room.min(WIDTH.max(row_needs)));
            self.unstack_buttons();
        }
        // The dialog doesn't shrink back by itself once it has been taller
        // (stacked buttons, more wrapped text), which left a gap above the
        // buttons after widening the window again. So its height is set from
        // what the contents need at the new width, but no taller than the window.
        if let Some(contents) = self.dialog.child() {
            let (_, natural, _, _) =
                contents.measure(gtk::Orientation::Vertical, self.dialog.content_width());
            self.dialog.set_content_height(natural.min(height - 2 * SIDE_GAP).max(1));
        }
    }

    /// How wide the buttons are side by side: each as wide as the widest,
    /// with the gaps between them and the margins either side. Worked out
    /// from the buttons, since the row may be stacked at the moment.
    fn row_width(&self) -> i32 {
        let widest = self
            .buttons
            .iter()
            .map(|button| button.measure(gtk::Orientation::Horizontal, -1).0)
            .max()
            .unwrap_or(0);
        let count = self.buttons.len() as i32;
        widest * count + self.row.spacing() * (count - 1).max(0) + MARGIN * 2
    }

    /// Too narrow for a row: one above the other, with the usual answer on top.
    /// `width` is the question's and `height` the window's. The text gets the
    /// room it needs, or what's left after the buttons in a short window, with
    /// the rest scrolling.
    fn stack_buttons(&self, width: i32, height: i32) {
        self.row.set_orientation(gtk::Orientation::Vertical);
        let mut previous: Option<&gtk::Button> = None;
        for button in self.buttons.iter().rev() {
            self.row.reorder_child_after(button, previous);
            previous = Some(button);
        }
        let room = height - self.buttons.len() as i32 * BUTTON_SPACE - MARGIN * 2;
        let (_, wanted, _, _) = self
            .text
            .measure(gtk::Orientation::Vertical, width - MARGIN * 2);
        self.scroller.set_min_content_height(wanted.min(room).max(0));
    }

    /// Back to a row, in the usual order, with the text at its natural height.
    fn unstack_buttons(&self) {
        self.row.set_orientation(gtk::Orientation::Horizontal);
        let mut previous: Option<&gtk::Button> = None;
        for button in &self.buttons {
            self.row.reorder_child_after(button, previous);
            previous = Some(button);
        }
        self.scroller.set_min_content_height(-1);
    }
}

/// Joins the last two words of each line with a space that can't break, so
/// wrapping never leaves one word alone on the last line, like "closing?"
/// under "Save changes to document “notes.md” before".
pub fn keep_last_words_together(text: &str) -> String {
    text.split('\n')
        .map(|line| match line.trim_end().rfind(' ') {
            Some(space) => format!("{}\u{a0}{}", &line[..space], &line[space + 1..]),
            None => line.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Records the first answer given. Closing the dialog after a button was
/// clicked also lands here, and mustn't replace what the button said.
fn give_answer(waiting: &RefCell<Waiting>, answer: &str) {
    let mut waiting = waiting.borrow_mut();
    if waiting.answer.is_none() {
        waiting.answer = Some(answer.to_string());
    }
    if let Some(waker) = waiting.waker.take() {
        waker.wake();
    }
}

/// A taller button with slightly lighter text, and a soft red for the one that
/// loses work, matching gedit's.
fn load_style() {
    static LOADED: Once = Once::new();
    LOADED.call_once(|| {
        let Some(display) = gdk::Display::default() else {
            return;
        };
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            ".leafmd-question-button { min-height: 32px; padding: 6px 12px; font-weight: 600; }
             .leafmd-loses-work {
                 font-weight: 500;
                 color: color-mix(in srgb, var(--destructive-color) 85%, transparent);
                 background-color: color-mix(in srgb, var(--destructive-bg-color) 15%, transparent);
             }
             .leafmd-loses-work:hover {
                 background-color: color-mix(in srgb, var(--destructive-bg-color) 25%, transparent);
             }
             .leafmd-loses-work:active {
                 background-color: color-mix(in srgb, var(--destructive-bg-color) 35%, transparent);
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

    #[test]
    fn last_two_words_stay_together() {
        assert_eq!(
            keep_last_words_together("Save changes before closing?"),
            "Save changes before\u{a0}closing?"
        );
        assert_eq!(
            keep_last_words_together("Two lines.\nSave changes?"),
            "Two\u{a0}lines.\nSave\u{a0}changes?"
        );
        assert_eq!(keep_last_words_together("Saved"), "Saved");
    }
}

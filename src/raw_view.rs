//! The raw text editor: GtkSourceView's editor, plus LeafMD's own wavy
//! underline under misspelt words.
//!
//! GTK's built-in "error" underline is a tiny wave pressed right against the
//! letters, which is hard to see. So the spell checker's own underline is
//! switched off in LeafMD's colour schemes (data/resources/styles), and this
//! view draws a thicker wave a little below each word instead. Which words
//! are misspelt is still decided by the spell checker (see spelling.rs): it
//! marks them with a text tag, and this view draws under whatever has it.

use std::cell::RefCell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, graphene};
use sourceview5::subclass::prelude::*;

/// How far the wave sits below the bottom of the letters, in pixels
const GAP: f64 = 1.5;
/// How high each bump of the wave is
const AMPLITUDE: f64 = 1.5;
/// How wide one full wave (up and back down) is
const WAVELENGTH: f64 = 5.0;
/// How thick the line is
const THICKNESS: f64 = 1.3;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct RawView {
        /// The spell checker's tag on misspelt words, once it's attached
        pub misspelt: RefCell<Option<gtk::TextTag>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RawView {
        const NAME: &'static str = "LeafMDRawView";
        type Type = super::RawView;
        type ParentType = sourceview5::View;
    }

    impl ObjectImpl for RawView {}
    impl WidgetImpl for RawView {}
    impl ViewImpl for RawView {}

    impl TextViewImpl for RawView {
        fn snapshot_layer(&self, layer: gtk::TextViewLayer, snapshot: gtk::Snapshot) {
            self.parent_snapshot_layer(layer, snapshot.clone());
            if layer == gtk::TextViewLayer::AboveText {
                self.draw_waves(&snapshot);
            }
        }
    }

    impl RawView {
        /// Draws a wave under every misspelt word that's on screen.
        fn draw_waves(&self, snapshot: &gtk::Snapshot) {
            let Some(tag) = self.misspelt.borrow().clone() else { return };
            let view = self.obj();
            let view = view.upcast_ref::<gtk::TextView>();

            // Only the part of the text that's showing, so long documents stay quick
            let visible = view.visible_rect();
            // A point past the end of the text has no letter there, so the end
            // of the text stands in for it
            let buffer = view.buffer();
            let mut iter = view
                .iter_at_location(visible.x(), visible.y())
                .unwrap_or_else(|| buffer.start_iter());
            let last = view
                .iter_at_location(visible.x() + visible.width(), visible.y() + visible.height())
                .unwrap_or_else(|| buffer.end_iter());
            iter.set_line_offset(0);

            let mut segments = Vec::new();
            loop {
                // Move to where the next misspelt word starts
                if !iter.starts_tag(Some(&tag)) && !iter.has_tag(&tag) && !iter.forward_to_tag_toggle(Some(&tag)) {
                    break;
                }
                if iter > last {
                    break;
                }
                let mut end = iter;
                end.forward_to_tag_toggle(Some(&tag));
                // Each letter's box, grouped by row, since a word can wrap
                // onto the next row and then gets a wave on each
                let mut letter = iter;
                let mut row: Option<(f64, f64, f64, i32)> = None; // left, right, bottom, top
                while letter < end {
                    let place = view.iter_location(&letter);
                    let (left, right) = (place.x() as f64, (place.x() + place.width()) as f64);
                    let bottom = (place.y() + place.height()) as f64;
                    row = match row {
                        Some((l, r, b, top)) if top == place.y() => Some((l.min(left), r.max(right), b.max(bottom), top)),
                        Some((l, r, b, _)) => {
                            segments.push((l, r, b));
                            Some((left, right, bottom, place.y()))
                        }
                        None => Some((left, right, bottom, place.y())),
                    };
                    if !letter.forward_char() {
                        break;
                    }
                }
                if let Some((l, r, b, _)) = row
                    && r > l
                {
                    segments.push((l, r, b));
                }
                iter = end;
                if iter.is_end() {
                    break;
                }
            }
            if segments.is_empty() {
                return;
            }

            let colour = wave_colour();
            let bounds = graphene::Rect::new(
                visible.x() as f32,
                visible.y() as f32,
                visible.width() as f32,
                visible.height() as f32 + 8.0,
            );
            let cairo = snapshot.append_cairo(&bounds);
            cairo.set_source_rgba(
                colour.red() as f64,
                colour.green() as f64,
                colour.blue() as f64,
                colour.alpha() as f64,
            );
            cairo.set_line_width(THICKNESS);
            cairo.set_line_cap(gtk::cairo::LineCap::Round);
            cairo.set_line_join(gtk::cairo::LineJoin::Round);
            for (x1, x2, bottom) in segments {
                // A zigzag of half-waves, starting on the upswing
                let middle = bottom + GAP + AMPLITUDE;
                let mut x = x1;
                let mut up = true;
                cairo.move_to(x, middle);
                while x < x2 {
                    let next = (x + WAVELENGTH / 2.0).min(x2);
                    let peak = if up { middle - AMPLITUDE } else { middle + AMPLITUDE };
                    cairo.curve_to(x + WAVELENGTH / 4.0, peak, next - WAVELENGTH / 4.0, peak, next, middle);
                    x = next;
                    up = !up;
                }
            }
            let _ = cairo.stroke();
        }
    }
}

glib::wrapper! {
    pub struct RawView(ObjectSubclass<imp::RawView>)
        @extends sourceview5::View, gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl RawView {
    pub fn with_buffer(buffer: &sourceview5::Buffer) -> Self {
        glib::Object::builder().property("buffer", buffer).build()
    }

    /// Draws waves under text with this tag: the spell checker's mark for
    /// misspelt words.
    pub fn underline_tag(&self, tag: Option<gtk::TextTag>) {
        // The tag has no look of its own (LeafMD's colour schemes turn its
        // underline off), so GTK doesn't redraw when words gain or lose it.
        // The view asks for that itself, so the waves keep up with typing.
        if let Some(tag) = &tag {
            let buffer = self.buffer();
            for removing in [false, true] {
                let (view, watched) = (self.downgrade(), tag.clone());
                let redraw = move |_: &gtk::TextBuffer, changed: &gtk::TextTag, _: &gtk::TextIter, _: &gtk::TextIter| {
                    if *changed == watched
                        && let Some(view) = view.upgrade()
                    {
                        view.queue_draw();
                    }
                };
                if removing {
                    buffer.connect_remove_tag(redraw);
                } else {
                    buffer.connect_apply_tag(redraw);
                }
            }
        }
        self.imp().misspelt.replace(tag);
        self.queue_draw();
    }
}

/// A darker red on light backgrounds and a brighter one on dark, so the wave
/// stands out in both: GNOME's red 4 (#c01c28) and red 1 (#f66151).
fn wave_colour() -> gtk::gdk::RGBA {
    let dark = adw::StyleManager::default().is_dark();
    gtk::gdk::RGBA::parse(if dark { "#f66151" } else { "#c01c28" })
        .unwrap_or(gtk::gdk::RGBA::RED)
}

//! How big the text is, and in which font: the two fonts chosen in
//! Preferences, and each window's zoom on top of them.
//!
//! There is one rule for text size: the Preferences font times the window's
//! zoom. Zooming to 120% with a 12 point font simply uses 14.4 points, in both
//! views.
//!
//! - The raw text gets its font from one stylesheet shared by the whole app.
//!   It has a rule for every zoom step, such as
//!   `.leafmd-zoom-120 .leafmd-raw { font-size: 14.4pt; ... }`. Each window
//!   carries the `leafmd-zoom-<percent>` style name for its zoom, and each raw
//!   editor `leafmd-raw`, so zooming a window only swaps that one name.
//! - The formatted view gets its font through WebKit's font settings, which
//!   each document sets (see `Document::apply_fonts`).

use std::cell::OnceCell;
use std::rc::Rc;

use gtk::{gdk, pango};

use crate::document::Document;
use crate::state::{self, FontFor};

/// The smallest and largest zoom, and how far each Ctrl+plus or Ctrl+minus goes.
pub const ZOOM_MIN: u32 = 50;
pub const ZOOM_MAX: u32 = 300;
pub const ZOOM_STEP: u32 = 10;
pub const ZOOM_NORMAL: u32 = 100;

/// The formatted view's starting font, the same as before Preferences existed
/// (12 points is the 16 pixels the page used to set itself).
pub const FORMATTED_DEFAULT: &str = "Adwaita Sans 12";

/// Used for the raw text if GNOME's monospace font can't be read.
const RAW_FALLBACK: &str = "Monospace 11";

/// The size used for a font picked without one.
const FALLBACK_POINTS: f64 = 11.0;

/// What the zoom becomes after Zoom In ("in"), Zoom Out ("out") or Normal
/// Size ("reset"). Zoom stays between 50% and 300%, and anything that isn't a
/// step of 10 lands back on one.
pub fn zoom_after(current: u32, how: &str) -> u32 {
    let current = current.clamp(ZOOM_MIN, ZOOM_MAX) / ZOOM_STEP * ZOOM_STEP;
    match how {
        "in" => (current + ZOOM_STEP).min(ZOOM_MAX),
        "out" => current.saturating_sub(ZOOM_STEP).max(ZOOM_MIN),
        _ => ZOOM_NORMAL,
    }
}

/// The style name a window carries for its zoom, like "leafmd-zoom-120".
pub fn zoom_class(zoom: u32) -> String {
    format!("leafmd-zoom-{zoom}")
}

/// Every zoom step, from 50 to 300.
fn zoom_steps() -> impl Iterator<Item = u32> {
    (ZOOM_MIN..=ZOOM_MAX).step_by(ZOOM_STEP as usize)
}

/// Keeps the parts of a font LeafMD uses: its family, how heavy it is (such
/// as Bold), whether it slants (Italic) and its size. Rarer details, like a
/// condensed width, are left out, so what's saved and shown matches what
/// both views can show.
pub fn kept_parts(font: &pango::FontDescription) -> pango::FontDescription {
    let mut kept = pango::FontDescription::new();
    if let Some(family) = font.family() {
        kept.set_family(&family);
    }
    if font.set_fields().contains(pango::FontMask::WEIGHT) {
        kept.set_weight(font.weight());
    }
    if font.set_fields().contains(pango::FontMask::STYLE) {
        kept.set_style(font.style());
    }
    if font.size() > 0 {
        if font.is_size_absolute() {
            kept.set_absolute_size(f64::from(font.size()));
        } else {
            kept.set_size(font.size());
        }
    }
    kept
}

/// The starting font for one of the two: GNOME's monospace font for the raw
/// text (the setting in GNOME Settings), and Adwaita Sans 12 for the formatted view.
pub fn starting_font(which: FontFor) -> pango::FontDescription {
    let name = match which {
        FontFor::Raw => {
            let gnome = adw::StyleManager::default().monospace_font_name();
            if gnome.trim().is_empty() { RAW_FALLBACK.to_string() } else { gnome.to_string() }
        }
        FontFor::Formatted => FORMATTED_DEFAULT.to_string(),
    };
    kept_parts(&pango::FontDescription::from_string(&name))
}

/// The font in use for one of the two: the one chosen in Preferences, or the
/// starting font if none was.
pub fn current_font(which: FontFor) -> pango::FontDescription {
    state::load_font(which)
        .map(|saved| pango::FontDescription::from_string(&saved))
        .filter(|font| font.family().is_some_and(|family| !family.is_empty()))
        .map(|font| kept_parts(&font))
        .unwrap_or_else(|| starting_font(which))
}

/// A font's size in points (absolute sizes are in pixels, at 96 to the inch).
fn points(font: &pango::FontDescription) -> f64 {
    let size = f64::from(font.size()) / f64::from(pango::SCALE);
    if size <= 0.0 {
        FALLBACK_POINTS
    } else if font.is_size_absolute() {
        size * 72.0 / 96.0
    } else {
        size
    }
}

/// The formatted view's text size in pixels, at a zoom. 12 points at 100% is
/// 16 pixels.
pub fn pixels_at(font: &pango::FontDescription, zoom: u32) -> u32 {
    let pixels = points(font) * 96.0 / 72.0 * f64::from(zoom) / 100.0;
    pixels.round().max(1.0) as u32
}

/// A font at a zoom, for measuring how wide text will be.
pub fn font_at(font: &pango::FontDescription, zoom: u32) -> pango::FontDescription {
    let mut scaled = font.clone();
    let size = points(font) * f64::from(zoom) / 100.0;
    scaled.set_size((size * f64::from(pango::SCALE)).round() as i32);
    scaled
}

/// How heavy a font is, as CSS writes it: 400 is regular and 700 is bold.
pub fn css_weight(font: &pango::FontDescription) -> i32 {
    match font.weight() {
        pango::Weight::Thin => 100,
        pango::Weight::Ultralight => 200,
        pango::Weight::Light | pango::Weight::Semilight => 300,
        pango::Weight::Book | pango::Weight::Normal => 400,
        pango::Weight::Medium => 500,
        pango::Weight::Semibold => 600,
        pango::Weight::Bold => 700,
        pango::Weight::Ultrabold => 800,
        pango::Weight::Heavy | pango::Weight::Ultraheavy => 900,
        _ => 400,
    }
}

/// Whether a font slants, as CSS writes it.
pub fn css_style(font: &pango::FontDescription) -> &'static str {
    match font.style() {
        pango::Style::Italic => "italic",
        pango::Style::Oblique => "oblique",
        _ => "normal",
    }
}

/// A font family written safely inside CSS quotes.
pub fn css_family(family: &str) -> String {
    let escaped: String = family
        .chars()
        .filter(|c| !c.is_control())
        .flat_map(|c| match c {
            '"' | '\\' => vec!['\\', c],
            _ => vec![c],
        })
        .collect();
    format!("\"{escaped}\"")
}

/// The raw text stylesheet: one rule per zoom step.
pub fn raw_css(font: &pango::FontDescription) -> String {
    let family = font.family().map(|f| f.to_string()).unwrap_or_else(|| "Monospace".into());
    let family = css_family(&family);
    let base = points(font);
    let weight = css_weight(font);
    let style = css_style(font);
    zoom_steps()
        .map(|zoom| {
            let size = base * f64::from(zoom) / 100.0;
            format!(
                ".{} .leafmd-raw {{ font-family: {family}, monospace; font-size: {size:.2}pt; font-weight: {weight}; font-style: {style}; }}\n",
                zoom_class(zoom)
            )
        })
        .collect()
}

thread_local! {
    /// The raw text stylesheet, added to the display once and refilled whenever
    /// the font changes. (GTK only ever runs on one thread.)
    static RAW_STYLES: OnceCell<gtk::CssProvider> = const { OnceCell::new() };
}

/// Adds the raw text stylesheet and starts following GNOME's monospace font.
/// Safe to call for every window: only the first call does anything.
pub fn install() {
    let first = RAW_STYLES.with(|cell| {
        if cell.get().is_some() {
            return false;
        }
        let Some(display) = gdk::Display::default() else {
            return false;
        };
        let provider = gtk::CssProvider::new();
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let _ = cell.set(provider);
        true
    });
    if !first {
        return;
    }
    reload_raw_styles();
    // Following GNOME's monospace font, while the user hasn't chosen one
    adw::StyleManager::default().connect_monospace_font_name_notify(|_| {
        if state::load_font(FontFor::Raw).is_none() {
            fonts_changed();
        }
    });
}

fn reload_raw_styles() {
    let css = raw_css(&current_font(FontFor::Raw));
    RAW_STYLES.with(|cell| {
        if let Some(provider) = cell.get() {
            provider.load_from_string(&css);
        }
    });
}

/// A font changed (in Preferences, or GNOME's monospace font): shows it in
/// every open document in every window.
pub fn fonts_changed() {
    reload_raw_styles();
    for doc in Document::all() {
        doc.apply_fonts();
    }
}

/// Saves a font chosen in Preferences (None for Reset) and shows it everywhere.
pub fn choose_font(which: FontFor, font: Option<&pango::FontDescription>) {
    let text = font.map(|font| kept_parts(font).to_string());
    state::save_font(which, text.as_deref());
    fonts_changed();
}

/// Applies a zoom to a set of documents (a window's tabs).
pub fn zoom_documents(documents: &[Rc<Document>], zoom: u32) {
    for doc in documents {
        doc.set_zoom(zoom);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_goes_in_steps_of_ten_between_fifty_and_three_hundred() {
        assert_eq!(zoom_after(100, "in"), 110);
        assert_eq!(zoom_after(110, "in"), 120);
        assert_eq!(zoom_after(120, "out"), 110);
        assert_eq!(zoom_after(300, "in"), 300);
        assert_eq!(zoom_after(50, "out"), 50);
        assert_eq!(zoom_after(170, "reset"), 100);
        // Something off the steps lands back on one
        assert_eq!(zoom_after(1000, "in"), 300);
        assert_eq!(zoom_after(10, "out"), 50);
    }

    #[test]
    fn twelve_points_is_sixteen_pixels() {
        let font = pango::FontDescription::from_string("Adwaita Sans 12");
        assert_eq!(pixels_at(&font, 100), 16);
        assert_eq!(pixels_at(&font, 150), 24);
        assert_eq!(pixels_at(&font, 50), 8);
    }

    #[test]
    fn family_weight_slant_and_size_are_kept() {
        let font = pango::FontDescription::from_string("Hack Bold Italic Condensed 13");
        let kept = kept_parts(&font);
        assert_eq!(kept.to_string(), "Hack Bold Italic 13");
        assert_eq!(css_weight(&kept), 700);
        assert_eq!(css_style(&kept), "italic");
        let plain = kept_parts(&pango::FontDescription::from_string("Hack 11"));
        assert_eq!(css_weight(&plain), 400);
        assert_eq!(css_style(&plain), "normal");
    }

    #[test]
    fn the_raw_stylesheet_has_a_rule_per_zoom_step() {
        let css = raw_css(&pango::FontDescription::from_string("Fira Code 10"));
        assert_eq!(css.lines().count(), 26);
        assert!(css.contains(".leafmd-zoom-100 .leafmd-raw { font-family: \"Fira Code\", monospace; font-size: 10.00pt; font-weight: 400; font-style: normal; }"));
        assert!(css.contains(".leafmd-zoom-120 .leafmd-raw { font-family: \"Fira Code\", monospace; font-size: 12.00pt; font-weight: 400; font-style: normal; }"));
    }

    #[test]
    fn font_names_cant_break_out_of_the_stylesheet() {
        assert_eq!(css_family("Odd \"Font\" \\ }"), "\"Odd \\\"Font\\\" \\\\ }\"");
    }
}

//! Printing, Export as PDF and Export as HTML.
//!
//! All three start from the same thing: the document as a standalone web page
//! with the formatted view's styles (preview.css) built in. Printing and the
//! PDF load that page into a hidden WebView of their own and print it, so the
//! window's own formatted view (which may be dark, zoomed in, or showing find
//! highlights) is never touched. The HTML export writes the page to a file,
//! with pictures from the computer packed inside it.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::LazyLock;

use adw::prelude::*;
use gtk::{gio, glib};
use webkit6::prelude::*;

use crate::document::{Document, portal_answers};
use crate::headings;
use crate::render;
use crate::state::FontFor;
use crate::text_size;

/// The formatted view's styles, shared by the formatted view and exports.
const STYLES: &str = include_str!("preview.css");

/// The formatted view's empty page, with a placeholder where the styles go.
const PREVIEW_HTML: &str = include_str!("preview.html");
const STYLES_PLACEHOLDER: &str = "/* LEAFMD-STYLES: preview.css goes here (see document.rs) */";

/// The first line of the dark colours in preview.css. Exported web pages
/// reuse that block for readers whose desktop is dark.
const DARK_BLOCK_START: &str = ":root[data-theme=\"dark\"] {";

/// The empty page the formatted view is drawn into, with its styles in place.
pub fn preview_page() -> &'static str {
    static PAGE: LazyLock<String> =
        LazyLock::new(|| PREVIEW_HTML.replacen(STYLES_PLACEHOLDER, STYLES, 1));
    &PAGE
}

/// Which colours an exported page uses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colours {
    /// Always light: paper and PDFs
    Light,
    /// Light, or dark when the reader's desktop or browser is dark: web pages
    FollowReader,
}

/// How the text looks: what WebKit's settings and LeafMD's scripts set on the
/// formatted view, written out as CSS for a page that stands on its own.
#[derive(Clone, Debug)]
pub struct Look {
    /// The font from Preferences, if one is named
    pub family: Option<String>,
    /// Its size in pixels at 100% zoom (the window's zoom never applies)
    pub pixels: u32,
    /// CSS weight (400 regular, 700 bold) and slant (normal, italic)
    pub weight: i32,
    pub style: &'static str,
    /// The accent colour, for ticked checkboxes and the bar beside quotes
    pub accent: String,
    /// Link colours: the accent made readable on light and on dark
    pub link_light: String,
    pub link_dark: String,
}

impl Look {
    /// The formatted view's font from Preferences, and the desktop's accent colour.
    pub fn now() -> Self {
        let font = text_size::current_font(FontFor::Formatted);
        let accent = adw::StyleManager::default().accent_color();
        Self {
            family: font.family().map(|family| family.to_string()),
            pixels: text_size::pixels_at(&font, 100),
            weight: text_size::css_weight(&font),
            style: text_size::css_style(&font),
            accent: accent.to_rgba().to_string(),
            link_light: accent.to_standalone_rgba(false).to_string(),
            link_dark: accent.to_standalone_rgba(true).to_string(),
        }
    }

    /// The CSS that sets the font and colours. Fonts the reader's computer
    /// may not have are followed by common ones, ending at the browser's own.
    fn css(&self, colours: Colours) -> String {
        let family = self
            .family
            .as_deref()
            .map(|family| format!("{}, ", text_size::css_family(family)))
            .unwrap_or_default();
        let mut css = format!(
            ":root {{ --accent: {}; --link: {}; }}\n\
             body {{ font-family: {family}\"Adwaita Sans\", Cantarell, system-ui, sans-serif; \
             font-size: {}px; font-weight: {}; font-style: {}; }}\n",
            self.accent, self.link_light, self.pixels, self.weight, self.style
        );
        if colours == Colours::FollowReader {
            // Tells the browser the page has a dark style, so it draws its own
            // parts (such as scroll bars on wide code blocks) dark to match
            css.push_str(":root { color-scheme: light dark; }\n");
            css.push_str(&format!(
                "@media (prefers-color-scheme: dark) {{\n:root {{{}\n  --link: {};\n}}\n}}\n",
                dark_colours(),
                self.link_dark
            ));
        }
        css
    }
}

/// The dark colours from preview.css (what's inside its dark block).
fn dark_colours() -> &'static str {
    let start = STYLES.find(DARK_BLOCK_START).map(|at| at + DARK_BLOCK_START.len());
    let Some(start) = start else { return "" };
    let end = STYLES[start..].find('}').map_or(STYLES.len(), |at| start + at);
    STYLES[start..end].trim_end()
}

/// The exported page's title: the document's first top-level heading, or the
/// file's name without its ending ("todo" for todo.md) when it has none.
pub fn title(markdown: &str, file_name: &str) -> String {
    headings::find(markdown)
        .into_iter()
        .find(|heading| heading.level == 1 && !heading.text.trim().is_empty())
        .map(|heading| heading.text.trim().to_string())
        .unwrap_or_else(|| name_without_ending(file_name))
}

/// "garden.md" to "garden". A name with no ending, or only a leading dot
/// (".notes"), stays as it is.
fn name_without_ending(file_name: &str) -> String {
    match file_name.rfind('.') {
        Some(at) if at > 0 => file_name[..at].to_string(),
        _ => file_name.to_string(),
    }
}

/// The name an export suggests: "garden.md" to "garden.pdf", "README" to
/// "README.pdf".
pub fn suggested_name(file_name: &str, ending: &str) -> String {
    format!("{}.{ending}", name_without_ending(file_name))
}

/// The document as a complete web page with no scripts. With `pictures_from`
/// set, pictures stored on the computer (relative to that folder, or given as
/// file:// addresses) are packed into the page.
pub fn page(
    markdown: &str,
    title: &str,
    look: &Look,
    colours: Colours,
    pictures_from: Option<PathBuf>,
) -> String {
    let body = match pictures_from {
        Some(folder) => {
            render::markdown_to_export_html(markdown, move |src| packed_picture(&folder, src))
        }
        None => render::markdown_to_export_html(markdown, |_| None),
    };
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <meta name=\"generator\" content=\"LeafMD\">\n\
         <title>{}</title>\n<style>\n{STYLES}\n{}</style>\n</head>\n<body>\n\
         <article id=\"content\">\n{body}</article>\n</body>\n</html>\n",
        escape(title),
        look.css(colours)
    )
}

/// Text made safe to place inside HTML.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// A picture on this computer, as a `data:` address holding the picture
/// itself. None (keep the address as written) for online pictures, and for
/// ones that can't be read, so the browser shows their description.
fn packed_picture(folder: &std::path::Path, src: &str) -> Option<String> {
    let folder_uri = glib::filename_to_uri(folder, None).ok()?;
    let base = format!("{}/", folder_uri.trim_end_matches('/'));
    let uri = glib::Uri::resolve_relative(Some(&base), src, glib::UriFlags::NONE).ok()?;
    if !uri.starts_with("file:") {
        return None;
    }
    let path = gio::File::for_uri(&uri).path()?;
    let bytes = std::fs::read(&path).ok()?;
    let (guess, _) = gio::content_type_guess(Some(&path), &bytes[..]);
    let mime = gio::content_type_get_mime_type(&guess)?;
    if !mime.starts_with("image/") {
        return None;
    }
    Some(format!("data:{mime};base64,{}", glib::base64_encode(&bytes)))
}

// =========================================================================
// Print, Export as PDF and Export as HTML
// =========================================================================

/// The selected document, ready to print or export: its text as it is now
/// (unsaved changes included), its name and its folder.
struct Source {
    markdown: String,
    /// The file's name, or "Untitled"
    name: String,
    /// The folder the file is in, if it has been saved
    folder: Option<gio::File>,
    doc: Rc<Document>,
}

impl Source {
    fn of(doc: &Rc<Document>) -> Self {
        Self {
            markdown: doc.text(),
            name: doc.display_name(),
            folder: doc.file.borrow().as_ref().and_then(|file| file.parent()),
            doc: doc.clone(),
        }
    }

    fn title(&self) -> String {
        title(&self.markdown, &self.name)
    }

    /// The folder's address ending in "/", which relative picture paths like
    /// `img/beds.png` are found from.
    fn base_uri(&self) -> Option<String> {
        self.folder.as_ref().map(|folder| {
            let mut uri = folder.uri().to_string();
            if !uri.ends_with('/') {
                uri.push('/');
            }
            uri
        })
    }

    fn parent_window(&self) -> Option<gtk::Window> {
        self.doc.window().map(|win| win.window.clone().upcast())
    }
}

/// Print: the system's Print dialog for the selected document's formatted
/// text, light, at the Preferences size.
pub fn print(doc: &Rc<Document>) {
    let source = Source::of(doc);
    glib::spawn_future_local(async move {
        // Inside Flatpak the Print dialog comes from the desktop's print
        // portal (LeafMD has no printer access of its own). Asked first, so
        // a desktop without one gets a message rather than nothing at all.
        let in_flatpak = std::path::Path::new("/.flatpak-info").exists();
        if in_flatpak && !portal_answers("org.freedesktop.portal.Print").await {
            eprintln!("LeafMD: could not print: the desktop's print portal didn't answer");
            source.doc.toast("Couldn't print: printing isn't available on this desktop");
            return;
        }
        load_hidden_page(source, |source, view| {
            let operation = webkit6::PrintOperation::new(&view);
            let doc = source.doc.clone();
            operation.connect_failed(move |_, err| {
                eprintln!("LeafMD: could not print: {err}");
                doc.toast(&format!("Couldn't print: {}", err.message()));
            });
            keep(operation.upcast_ref());
            let view_too = view.clone();
            operation.connect_finished(move |operation| {
                let_go(operation.upcast_ref());
                let_go(view_too.upcast_ref());
            });
            let answer = operation.run_dialog(source.parent_window().as_ref());
            if answer == webkit6::PrintOperationResponse::Cancel {
                let_go(operation.upcast_ref());
                let_go(view.upcast_ref());
            }
        });
    });
}

/// Export as PDF: asks where to save, then saves the formatted document as a
/// PDF that looks the way printing it would.
pub async fn export_pdf(doc: &Rc<Document>) {
    let source = Source::of(doc);
    let Some(target) = choose_target(&source, "pdf", "Export as PDF", pdf_filter()).await else {
        return;
    };
    // WebKit prints to a file in LeafMD's own cache folder first, which the
    // file is then copied from. A file chosen through the desktop's file
    // portal can have an address the printing system can't write to.
    let temporary = glib::user_cache_dir().join(format!("leafmd-export-{}.pdf", std::process::id()));
    if let Some(parent) = temporary.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::remove_file(&temporary);
    let Ok(temporary_uri) = glib::filename_to_uri(&temporary, None) else {
        source.doc.toast("Couldn't export: LeafMD's cache folder can't be used");
        return;
    };
    load_hidden_page(source, move |source, view| {
        let settings = gtk::PrintSettings::new();
        settings.set_printer("Print to File");
        settings.set(gtk::PRINT_SETTINGS_OUTPUT_FILE_FORMAT, Some("pdf"));
        settings.set(gtk::PRINT_SETTINGS_OUTPUT_URI, Some(&temporary_uri));
        let page_setup = gtk::PageSetup::new();
        // The usual paper for the computer's language and region (A4 or Letter)
        page_setup.set_paper_size(&gtk::PaperSize::new(None));
        for set in [
            gtk::PageSetup::set_top_margin,
            gtk::PageSetup::set_bottom_margin,
            gtk::PageSetup::set_left_margin,
            gtk::PageSetup::set_right_margin,
        ] {
            set(&page_setup, 20.0, gtk::Unit::Mm);
        }
        let operation = webkit6::PrintOperation::builder()
            .web_view(&view)
            .print_settings(&settings)
            .page_setup(&page_setup)
            .build();
        let failed = Rc::new(Cell::new(false));
        let doc = source.doc.clone();
        let failed_too = failed.clone();
        operation.connect_failed(move |_, err| {
            failed_too.set(true);
            eprintln!("LeafMD: could not export a PDF: {err}");
            doc.toast(&format!("Couldn't export: {}", err.message()));
        });
        let doc = source.doc.clone();
        keep(operation.upcast_ref());
        operation.connect_finished(move |operation| {
            let_go(operation.upcast_ref());
            let_go(view.upcast_ref());
            if failed.get() {
                let _ = std::fs::remove_file(&temporary);
                return;
            }
            let doc = doc.clone();
            let target = target.clone();
            let temporary = temporary.clone();
            glib::spawn_future_local(async move {
                let made = gio::File::for_path(&temporary);
                let (copying, _) = made.copy_future(
                    &target,
                    gio::FileCopyFlags::OVERWRITE,
                    glib::Priority::DEFAULT,
                );
                let copied = copying.await;
                let _ = std::fs::remove_file(&temporary);
                report(&doc, &target, copied.map(|_| ()));
            });
        });
        operation.print();
    });
}

/// Export as HTML: asks where to save, then saves a single web page with the
/// styles and the computer's pictures packed inside it.
pub async fn export_html(doc: &Rc<Document>) {
    let source = Source::of(doc);
    let Some(target) = choose_target(&source, "html", "Export as HTML", html_filter()).await else {
        return;
    };
    let pictures_from = source.folder.as_ref().and_then(|folder| folder.path());
    let html = page(
        &source.markdown,
        &source.title(),
        &Look::now(),
        Colours::FollowReader,
        pictures_from,
    );
    let written = target
        .replace_contents_future(html.into_bytes(), None, false, gio::FileCreateFlags::NONE)
        .await
        .map(|_| ())
        .map_err(|(_, err)| err);
    report(&source.doc, &target, written);
}

/// Shows "Saved garden.pdf", or why it couldn't be saved.
fn report(doc: &Document, target: &gio::File, result: Result<(), glib::Error>) {
    match result {
        Ok(()) => {
            let name = target
                .basename()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            doc.toast(&format!("Saved {name}"));
        }
        Err(err) => {
            eprintln!("LeafMD: could not export: {err}");
            doc.toast(&format!("Couldn't export: {}", err.message()));
        }
    }
}

/// Asks where to save an export, suggesting the document's name with the
/// new ending, in the document's folder. None if the user cancels.
async fn choose_target(
    source: &Source,
    ending: &str,
    title: &str,
    filter: gtk::FileFilter,
) -> Option<gio::File> {
    let name = suggested_name(&source.name, ending);
    let filters = gio::ListStore::new::<gtk::FileFilter>();
    filters.append(&filter);
    let dialog = gtk::FileDialog::builder()
        .title(title)
        .modal(true)
        .initial_name(name)
        .filters(&filters)
        .default_filter(&filter)
        .build();
    if let Some(folder) = &source.folder {
        dialog.set_initial_folder(Some(folder));
    }
    dialog.save_future(source.parent_window().as_ref()).await.ok()
}

fn pdf_filter() -> gtk::FileFilter {
    let filter = gtk::FileFilter::new();
    filter.set_name(Some("PDF documents"));
    filter.add_mime_type("application/pdf");
    filter.add_suffix("pdf");
    filter
}

fn html_filter() -> gtk::FileFilter {
    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Web pages"));
    filter.add_mime_type("text/html");
    filter.add_suffix("html");
    filter.add_suffix("htm");
    filter
}

/// Loads the document, light and at the Preferences size, into a WebView of
/// its own that's never shown, then hands it over once it has finished
/// loading, pictures included. Scripts are off, as in the formatted view, and
/// nothing can make it go to another page.
fn load_hidden_page(source: Source, then: impl FnOnce(&Source, webkit6::WebView) + 'static) {
    let view = webkit6::WebView::new();
    if let Some(settings) = WebViewExt::settings(&view) {
        settings.set_enable_javascript_markup(false);
        settings.set_enable_developer_extras(false);
    }
    let html = page(&source.markdown, &source.title(), &Look::now(), Colours::Light, None);
    // Only LeafMD's own loading of the page is let through
    let loaded = Rc::new(Cell::new(false));
    let loaded_too = loaded.clone();
    view.connect_decide_policy(move |_, decision, kind| {
        if kind == webkit6::PolicyDecisionType::NavigationAction && loaded_too.get() {
            decision.ignore();
            return true;
        }
        false
    });
    let base = source.base_uri();
    let waiting = RefCell::new(Some((source, then)));
    view.connect_load_changed(move |view, event| {
        if event != webkit6::LoadEvent::Finished {
            return;
        }
        loaded.set(true);
        if let Some((source, then)) = waiting.borrow_mut().take() {
            then(&source, view.clone());
        }
    });
    keep(view.upcast_ref());
    view.load_html(&html, base.as_deref());
}

thread_local! {
    /// Hidden pages and print jobs still at work. Nothing else holds on to
    /// them, so without this they would be thrown away before they finish.
    static BUSY: RefCell<Vec<glib::Object>> = const { RefCell::new(Vec::new()) };
}

fn keep(object: &glib::Object) {
    BUSY.with(|busy| busy.borrow_mut().push(object.clone()));
}

fn let_go(object: &glib::Object) {
    BUSY.with(|busy| busy.borrow_mut().retain(|kept| kept != object));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look() -> Look {
        Look {
            family: Some("Adwaita Sans".into()),
            pixels: 16,
            weight: 400,
            style: "normal",
            accent: "rgb(53,132,228)".into(),
            link_light: "rgb(4,97,190)".into(),
            link_dark: "rgb(129,208,255)".into(),
        }
    }

    /// The smallest possible PNG picture (1 by 1 pixel).
    const PNG: &[u8] = &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
        0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];

    #[test]
    fn title_is_the_first_top_level_heading() {
        let markdown = "Intro\n\n## Budget\n\n# Community Garden Plan\n\n# Later";
        assert_eq!(title(markdown, "garden.md"), "Community Garden Plan");
    }

    #[test]
    fn title_falls_back_to_the_file_name() {
        assert_eq!(title("## Only a smaller heading", "todo.md"), "todo");
        assert_eq!(title("Text", "README"), "README");
        assert_eq!(title("Text", ".notes"), ".notes");
    }

    #[test]
    fn suggested_names() {
        assert_eq!(suggested_name("garden.md", "pdf"), "garden.pdf");
        assert_eq!(suggested_name("README", "html"), "README.html");
        assert_eq!(suggested_name("Untitled", "pdf"), "Untitled.pdf");
        assert_eq!(suggested_name("v1.2.notes.md", "html"), "v1.2.notes.html");
    }

    #[test]
    fn the_title_is_escaped() {
        let html = page("# Tom & <Jerry>", "Tom & <Jerry>", &look(), Colours::Light, None);
        assert!(html.contains("<title>Tom &amp; &lt;Jerry&gt;</title>"));
    }

    #[test]
    fn no_scripts_either_way() {
        let markdown = "# Hi\n\n<script>alert(1)</script>\n\n<img src=\"a.png\" onerror=\"alert(2)\">";
        for colours in [Colours::Light, Colours::FollowReader] {
            let html = page(markdown, "Hi", &look(), colours, None);
            assert!(!html.contains("<script"));
            assert!(!html.contains("onerror"));
            assert!(!html.contains("data-sourcepos"));
        }
    }

    #[test]
    fn light_only_or_following_the_reader() {
        let paper = page("Hi", "Hi", &look(), Colours::Light, None);
        assert!(!paper.contains("prefers-color-scheme"));
        let web = page("Hi", "Hi", &look(), Colours::FollowReader, None);
        assert!(web.contains("@media (prefers-color-scheme: dark)"));
        assert!(web.contains("color-scheme: light dark"));
        // The dark colours from preview.css, and the dark link colour
        assert!(web.contains("--bg: #1d1d20;"));
        assert!(web.contains("--link: rgb(129,208,255);"));
    }

    #[test]
    fn the_preferences_font_comes_first() {
        let html = page("Hi", "Hi", &look(), Colours::Light, None);
        assert!(html.contains(
            "font-family: \"Adwaita Sans\", \"Adwaita Sans\", Cantarell, system-ui, sans-serif"
        ));
        assert!(html.contains("font-size: 16px"));
    }

    #[test]
    fn code_colours_in_exports() {
        let markdown = "```python\nprint(\"hi\")  # say hi\n```\n";
        // A web page: coloured, with the dark colours for dark readers, no scripts
        let web = page(markdown, "x", &look(), Colours::FollowReader, None);
        assert!(web.contains("<span class=\"hl-comment\">"), "{web}");
        assert!(web.contains("--code-comment: #9a9996"), "dark colours missing");
        assert!(web.contains("--code-comment: #6a696e"), "light colours missing");
        assert!(!web.contains("<script"));
        // Paper and PDFs: the same coloured code, with only the light colours
        let paper = page(markdown, "x", &look(), Colours::Light, None);
        assert!(paper.contains("<span class=\"hl-comment\">"));
        assert!(paper.contains("--code-comment: #6a696e"));
        assert!(!paper.contains("prefers-color-scheme: dark"));
    }

    #[test]
    fn pictures_are_packed_in() {
        let folder = std::env::temp_dir().join(format!("leafmd-export-test-{}", std::process::id()));
        std::fs::create_dir_all(folder.join("img")).unwrap();
        std::fs::write(folder.join("img/beds.png"), PNG).unwrap();
        std::fs::write(folder.join("my photo.png"), PNG).unwrap();
        std::fs::write(folder.join("#1 why?.png"), PNG).unwrap();
        let markdown = "![](img/beds.png)\n\n![](my%20photo.png)\n\n![Plan](img/missing.png)\n\n\
                        ![](https://example.org/logo.png)\n\n![](%231%20why%3F.png)";
        let html = page(markdown, "x", &look(), Colours::FollowReader, Some(folder.clone()));
        std::fs::remove_dir_all(&folder).unwrap();
        assert_eq!(html.matches("src=\"data:image/png;base64,").count(), 3);
        assert!(html.contains("src=\"img/missing.png\""));
        assert!(html.contains("alt=\"Plan\""));
        assert!(html.contains("src=\"https://example.org/logo.png\""));
    }

    #[test]
    fn the_preview_page_has_its_styles() {
        assert!(preview_page().contains("--bg: #ffffff;"));
        assert!(!preview_page().contains(STYLES_PLACEHOLDER));
    }
}

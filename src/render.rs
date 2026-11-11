//! Turns markdown text into HTML that is safe to show in the formatted view.

use comrak::{Options, markdown_to_html};

/// The markdown features LeafMD understands (GitHub-style extras on top of
/// plain markdown). The formatted view, the Headings page and the word count
/// all read markdown with these, so they agree on what's a table, a heading
/// or a checklist.
pub fn markdown_options() -> Options<'static> {
    let mut options = Options::default();
    // GitHub-style extras
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options
}

/// Converts markdown to HTML, then strips anything unsafe from it.
///
/// Raw HTML inside the markdown is allowed through the converter on purpose
/// (lots of READMEs use it for things like centred images), and the cleaning
/// step afterwards removes scripts, event handlers, iframes and so on.
pub fn markdown_to_safe_html(markdown: &str) -> String {
    clean(&markdown_to_html(markdown, &view_options(true)))
}

/// Converts markdown to safe HTML for a printed page, a PDF or an exported
/// web page. The same as the formatted view's, but without the source line
/// tags (they mean nothing outside LeafMD), and every picture's address
/// (`src` on `img`) is handed to `picture`, which can give a new one, such as
/// the picture itself packed into a `data:` address. Returning None keeps the
/// address as written.
pub fn markdown_to_export_html<F>(markdown: &str, picture: F) -> String
where
    F: Fn(&str) -> Option<String> + Send + Sync + 'static,
{
    let html = markdown_to_html(markdown, &view_options(false));
    let mut cleaner = cleaner();
    cleaner.rm_generic_attributes(["data-sourcepos"]);
    cleaner.attribute_filter(move |element, attribute, value| {
        if element == "img" && attribute == "src" {
            return Some(picture(value).map(Into::into).unwrap_or(value.into()));
        }
        Some(value.into())
    });
    cleaner.clean(&html).to_string()
}

/// How markdown is turned into the formatted view's HTML. `source_lines`
/// tags each block with the lines it came from.
fn view_options(source_lines: bool) -> Options<'static> {
    let mut options = markdown_options();
    // Marks checklist items with class="task-list-item", so the formatted
    // view can show just the checkbox on them instead of a bullet as well
    options.render.tasklist_classes = true;
    // Gives each heading an id so links like [see below](#install) work
    options.extension.header_id_prefix = Some(String::new());
    options.render.r#unsafe = true;
    // Tags each block with the lines it came from (data-sourcepos="3:1-5:20"),
    // which click-to-jump and synced scrolling use to match the two views up
    options.render.sourcepos = source_lines;
    options
}

fn clean(html: &str) -> String {
    cleaner().clean(html).to_string()
}

/// What may stay in the HTML: ammonia's safe defaults (no scripts, event
/// handlers, iframes and so on), plus what LeafMD's pages need.
fn cleaner() -> ammonia::Builder<'static> {
    let mut cleaner = ammonia::Builder::default();
    cleaner
        // Task list checkboxes: - [x] done
        .add_tags(["input"])
        .add_tag_attributes("input", ["type", "checked", "disabled"])
        // ids for heading and footnote links, classes for styling, align for table
        // columns, and the source line tags
        .add_generic_attributes(["id", "class", "align", "data-sourcepos"]);
    cleaner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        let html = markdown_to_safe_html("# Title\n\nSome **bold** text.");
        assert!(html.contains("<h1"));
        assert!(html.contains("<strong") && html.contains(">bold</strong>"));
    }

    #[test]
    fn removes_scripts_and_event_handlers() {
        let html = markdown_to_safe_html(
            "<script>alert(1)</script>\n\n<img src=\"a.png\" onerror=\"alert(2)\">",
        );
        assert!(!html.contains("<script"));
        assert!(!html.contains("onerror"));
        assert!(html.contains("a.png"));
    }

    #[test]
    fn tags_blocks_with_source_lines() {
        let html = markdown_to_safe_html("# Title\n\nFirst paragraph.\n\nSecond paragraph.");
        assert!(html.contains("data-sourcepos=\"1:1-1:7\""));
        assert!(html.contains("data-sourcepos=\"5:1-5:17\""));
    }

    #[test]
    fn keeps_task_lists_and_tables() {
        let html = markdown_to_safe_html("- [x] done\n- [ ] todo\n\n| a | b |\n|---|---|\n| 1 | 2 |");
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("<table"));
    }

    #[test]
    fn export_has_no_source_lines_and_stays_safe() {
        let html = markdown_to_export_html(
            "# Title\n\n<script>alert(1)</script>\n\n<img src=\"a.png\" onerror=\"alert(2)\">",
            |_| None,
        );
        assert!(html.contains("<h1"));
        assert!(!html.contains("data-sourcepos"));
        assert!(!html.contains("<script"));
        assert!(!html.contains("onerror"));
        assert!(html.contains("src=\"a.png\""));
    }

    #[test]
    fn export_hands_every_picture_over() {
        let html = markdown_to_export_html(
            "![](img/a.png)\n\n<img src=\"b.png\">\n\n[a link](c.png)",
            |src| Some(format!("data:image/png;base64,{}", src.replace(['/', '.'], ""))),
        );
        assert!(html.contains("src=\"data:image/png;base64,imgapng\""));
        assert!(html.contains("src=\"data:image/png;base64,bpng\""));
        // Only pictures: a link to a picture is left alone
        assert!(html.contains("href=\"c.png\""));
    }

    #[test]
    fn export_keeps_a_document_s_own_data_pictures_out() {
        // A data: address written in the document isn't allowed, as on screen
        let html = markdown_to_export_html("<img src=\"data:image/png;base64,AAAA\">", |_| None);
        assert!(!html.contains("data:"));
    }

    #[test]
    fn marks_checklist_items_but_not_plain_ones() {
        // The class survives cleaning, and only the checklist line gets it
        let html = markdown_to_safe_html("- [ ] todo\n- plain");
        assert!(html.contains("<li class=\"task-list-item\""));
        assert_eq!(html.matches("task-list-item\"").count(), 1);
    }
}

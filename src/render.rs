//! Turns markdown text into HTML that is safe to show in the formatted view.

use comrak::{Options, markdown_to_html};

/// Converts markdown to HTML, then strips anything unsafe from it.
///
/// Raw HTML inside the markdown is allowed through the converter on purpose
/// (lots of READMEs use it for things like centred images), and the cleaning
/// step afterwards removes scripts, event handlers, iframes and so on.
pub fn markdown_to_safe_html(markdown: &str) -> String {
    let mut options = Options::default();
    // GitHub-style extras
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    // Gives each heading an id so links like [see below](#install) work
    options.extension.header_id_prefix = Some(String::new());
    options.render.r#unsafe = true;
    // Tags each block with the lines it came from (data-sourcepos="3:1-5:20"),
    // which click-to-jump and synced scrolling use to match the two views up
    options.render.sourcepos = true;

    let html = markdown_to_html(markdown, &options);
    clean(&html)
}

fn clean(html: &str) -> String {
    let mut cleaner = ammonia::Builder::default();
    cleaner
        // Task list checkboxes: - [x] done
        .add_tags(["input"])
        .add_tag_attributes("input", ["type", "checked", "disabled"])
        // ids for heading and footnote links, classes for styling, align for table
        // columns, and the source line tags
        .add_generic_attributes(["id", "class", "align", "data-sourcepos"]);
    cleaner.clean(html).to_string()
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
}

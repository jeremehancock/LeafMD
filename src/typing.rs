//! Help while typing in the raw text: what Enter does in a list, whether the
//! cursor is inside a code block, and whether pasted text is a web address.
//!
//! Like `format.rs`, everything here works on plain text so it can be tested
//! without a window. The window (`window.rs`) asks these functions what to do
//! and then makes the edit. Positions are counted in characters, not bytes,
//! the same way GTK counts them.

use crate::format::{self, LineStyle};

/// What pressing Enter should do on the current line.
#[derive(Debug, PartialEq, Eq)]
pub enum Enter {
    /// Nothing special: let the editor insert an ordinary new line.
    Normal,
    /// Insert this text at the cursor: a new line, then the next item's
    /// indent and marker (like "\n  - " or "\n3. "). Text after the cursor
    /// ends up after the marker, which splits the item in two.
    Continue(String),
    /// The item has nothing after its marker: clear the whole line, which
    /// ends the list.
    EndList,
}

/// Works out what Enter does on `line` (the whole line, without its "\n")
/// with the cursor `cursor` characters into it.
pub fn on_enter(line: &str, cursor: usize) -> Enter {
    let parsed = format::parse_line(line);
    let Some(style) = parsed.style else {
        return Enter::Normal;
    };
    if matches!(style, LineStyle::Heading(_)) {
        return Enter::Normal;
    }

    // Where the marker ends, in characters. With the cursor before that
    // (at the start of the line, or in the middle of "- "), Enter just
    // pushes the line down like it would anywhere else.
    let marker_end = line.chars().count() - parsed.rest.chars().count();
    if cursor < marker_end {
        return Enter::Normal;
    }
    if parsed.rest.trim().is_empty() {
        return Enter::EndList;
    }

    // The marker itself, as written after the indent: "- ", "* [x] ", "12) "
    let after_indent = &line[parsed.indent.len()..];
    let bullet = after_indent.chars().next().unwrap_or('-');
    let marker = match style {
        LineStyle::Bullet => format!("{bullet} "),
        // A new checklist item always starts unticked
        LineStyle::Task => format!("{bullet} [ ] "),
        LineStyle::Numbered => {
            let digits: String = after_indent.chars().take_while(|c| c.is_ascii_digit()).collect();
            // "." or ")", whichever this item used
            let delimiter = after_indent[digits.len()..].chars().next().unwrap_or('.');
            let next = digits.parse::<u64>().unwrap_or(0) + 1;
            format!("{next}{delimiter} ")
        }
        LineStyle::Heading(_) => return Enter::Normal,
    };
    Enter::Continue(format!("\n{}{marker}", parsed.indent))
}

/// Whether a line coming after `text_before` is inside a fenced code block,
/// that is, after an opening ``` or ~~~ line that hasn't been closed yet.
/// `text_before` is everything from the start of the document up to the
/// start of the line in question.
pub fn in_code_block(text_before: &str) -> bool {
    // The open fence's character and length. Markdown closes a fence only
    // with the same character, at least as many times, and nothing after it.
    let mut open: Option<(char, usize)> = None;
    for line in text_before.lines() {
        // A fence may be indented by up to three spaces; four or more makes
        // it an indented code line instead
        let spaces = line.chars().take_while(|&c| c == ' ').count();
        if spaces > 3 {
            continue;
        }
        let text = &line[spaces..];
        let Some(fence) = text.chars().next().filter(|c| matches!(c, '`' | '~')) else {
            continue;
        };
        let run = text.chars().take_while(|&c| c == fence).count();
        if run < 3 {
            continue;
        }
        match open {
            None => open = Some((fence, run)),
            Some((open_fence, open_run))
                if fence == open_fence && run >= open_run && text[run..].trim().is_empty() =>
            {
                open = None;
            }
            Some(_) => {}
        }
    }
    open.is_some()
}

/// Whether pasted text is a single web address, like `https://example.com/docs`.
/// Spaces and new lines around it are allowed (copying from a browser
/// sometimes adds them); spaces inside it aren't.
pub fn is_web_address(text: &str) -> bool {
    let text = text.trim();
    let rest = text
        .strip_prefix("https://")
        .or_else(|| text.strip_prefix("http://"));
    rest.is_some_and(|rest| !rest.is_empty() && !rest.contains(char::is_whitespace))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// on_enter with the cursor at the end of the line, the usual case
    fn at_end(line: &str) -> Enter {
        on_enter(line, line.chars().count())
    }

    fn cont(text: &str) -> Enter {
        Enter::Continue(text.to_string())
    }

    #[test]
    fn bullets_continue_with_the_same_character() {
        assert_eq!(at_end("- Milk"), cont("\n- "));
        assert_eq!(at_end("* Milk"), cont("\n* "));
        assert_eq!(at_end("+ Milk"), cont("\n+ "));
    }

    #[test]
    fn indent_is_kept() {
        assert_eq!(at_end("    - Eggs"), cont("\n    - "));
        assert_eq!(at_end("\t- Eggs"), cont("\n\t- "));
        assert_eq!(at_end("   3. Bake"), cont("\n   4. "));
    }

    #[test]
    fn splitting_an_item() {
        // Cursor just before "and": the window inserts "\n- " there, giving
        // "- Milk " and "- and eggs"
        let line = "- Milk and eggs";
        assert_eq!(on_enter(line, 7), cont("\n- "));
    }

    #[test]
    fn numbers_count_up_with_the_same_punctuation() {
        assert_eq!(at_end("2. Preheat the oven"), cont("\n3. "));
        assert_eq!(at_end("9) Serve"), cont("\n10) "));
        assert_eq!(at_end("1. Ünïcode"), cont("\n2. "));
    }

    #[test]
    fn checklist_items_continue_unticked() {
        assert_eq!(at_end("- [x] Book flights"), cont("\n- [ ] "));
        assert_eq!(at_end("- [ ] Pack"), cont("\n- [ ] "));
        assert_eq!(at_end("* [X] Done"), cont("\n* [ ] "));
    }

    #[test]
    fn empty_items_end_the_list() {
        assert_eq!(at_end("- "), Enter::EndList);
        assert_eq!(at_end("3. "), Enter::EndList);
        assert_eq!(at_end("- [ ] "), Enter::EndList);
        assert_eq!(at_end("- [ ]"), Enter::EndList);
        assert_eq!(at_end("  - "), Enter::EndList);
        // Only spaces after the marker still counts as empty
        assert_eq!(at_end("-    "), Enter::EndList);
    }

    #[test]
    fn plain_lines_and_headings_are_normal() {
        assert_eq!(at_end("Shopping list:"), Enter::Normal);
        assert_eq!(at_end("## Install"), Enter::Normal);
        assert_eq!(at_end(""), Enter::Normal);
        // No space after the marker, so not a list item
        assert_eq!(at_end("-Milk"), Enter::Normal);
        assert_eq!(at_end("2.Bake"), Enter::Normal);
    }

    #[test]
    fn cursor_before_or_inside_the_marker_is_normal() {
        assert_eq!(on_enter("- Milk", 0), Enter::Normal);
        assert_eq!(on_enter("- Milk", 1), Enter::Normal);
        assert_eq!(on_enter("  - Milk", 2), Enter::Normal);
        assert_eq!(on_enter("10. Ten", 2), Enter::Normal);
        assert_eq!(on_enter("- [ ] Task", 4), Enter::Normal);
        // Right after the marker counts as inside the item
        assert_eq!(on_enter("- Milk", 2), cont("\n- "));
    }

    #[test]
    fn code_blocks() {
        assert!(!in_code_block(""));
        assert!(!in_code_block("Some text\n"));
        assert!(in_code_block("```\n"));
        assert!(in_code_block("Intro\n```bash\nls\n"));
        assert!(!in_code_block("```bash\nls\n```\n"));
        assert!(in_code_block("~~~\n"));
        assert!(!in_code_block("~~~\ncode\n~~~\n"));
        // Up to three spaces of indent is still a fence; four is not
        assert!(in_code_block("   ```\n"));
        assert!(!in_code_block("    ```\n"));
        // A different fence character, a shorter fence, or text after it
        // doesn't close the block
        assert!(in_code_block("```\n~~~\n"));
        assert!(in_code_block("````\n```\n"));
        assert!(in_code_block("```\n``` not a close\n"));
        // An unclosed fence leaves everything after it in the block
        assert!(in_code_block("```\na\nb\nc\n"));
        // Two backticks is just inline code
        assert!(!in_code_block("``x``\n"));
    }

    #[test]
    fn web_addresses() {
        assert!(is_web_address("https://example.com/docs"));
        assert!(is_web_address("http://x.org"));
        assert!(is_web_address("  https://example.com  "));
        assert!(is_web_address("https://example.com\n"));
        assert!(!is_web_address("the manual"));
        assert!(!is_web_address("https://"));
        assert!(!is_web_address("https://example.com/my docs"));
        assert!(!is_web_address("www.example.com"));
        assert!(!is_web_address("see https://example.com"));
        assert!(!is_web_address(""));
    }
}

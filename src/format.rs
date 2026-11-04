//! The formatting bar: turning selected raw text into bold, a list, a heading
//! and so on. Everything here works on plain text, so it can be tested
//! without a window.
//!
//! Each function is given a "block": the whole lines the selection touches,
//! plus where the selection starts and ends inside it. Positions are counted
//! in characters (not bytes), the same way GTK counts them. Each function
//! returns the new text for those lines and where the selection should be
//! afterwards.
//!
//! Pressing a button a second time undoes it: bold text becomes plain again,
//! a bullet list goes back to plain lines, and so on.

/// The result of a formatting button: the new text for the block, and the
/// selection inside it (start and end in characters).
#[derive(Debug, PartialEq, Eq)]
pub struct Edit {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// Styles that wrap text in markers, like `**bold**`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inline {
    Bold,
    Italic,
    Strikethrough,
    Code,
}

impl Inline {
    fn marker(self) -> &'static str {
        match self {
            Inline::Bold => "**",
            Inline::Italic => "*",
            Inline::Strikethrough => "~~",
            Inline::Code => "`",
        }
    }

    /// Bold and italic both use stars, so "is this italic?" depends on how
    /// many stars are in a row: `*x*` and `***x***` are italic, `**x**` isn't.
    fn star_run_ok(self, run: usize) -> bool {
        match self {
            Inline::Italic => run % 2 == 1,
            Inline::Bold => run >= 2,
            _ => true,
        }
    }
}

/// Styles that apply to whole lines, set by a marker at the start of each line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineStyle {
    /// `# `, `## ` or `### `
    Heading(usize),
    /// `- `
    Bullet,
    /// `1. `, `2. `, ...
    Numbered,
    /// `- [ ] `
    Task,
}

/// The starter table the table button inserts.
const TABLE: &str = "| Column 1 | Column 2 |\n| -------- | -------- |\n| Text     | Text     |";

// =========================================================================
// Inline styles: bold, italic, strikethrough, inline code
// =========================================================================

/// Wraps the selection in the style's markers, or removes them if it already
/// has them. With nothing selected, the word under the cursor is used; if the
/// cursor isn't in a word, an empty pair of markers is added with the cursor
/// between them, ready to type.
///
/// A selection over several lines is styled line by line, because markdown
/// styles can't cross a blank line.
pub fn toggle_inline(block: &str, start: usize, end: usize, style: Inline) -> Edit {
    let m = style.marker();
    let (mut s, mut e) = (byte_at(block, start), byte_at(block, end));

    if s == e {
        let (ws, we) = word_around(block, s);
        if ws == we {
            // Not in a word. If the cursor sits in an empty pair (like **|**), remove it.
            if wrapped(block, s, s, style) == Some(Wrap::Outside) {
                let text = format!("{}{}", &block[..s - m.len()], &block[s + m.len()..]);
                let cursor = chars(&block[..s - m.len()]);
                return Edit { text, start: cursor, end: cursor };
            }
            let text = format!("{}{m}{m}{}", &block[..s], &block[s..]);
            let cursor = start + m.len();
            return Edit { text, start: cursor, end: cursor };
        }
        (s, e) = (ws, we);
    }

    // One piece per line, with spaces at either end left outside the markers
    // ("**word **" wouldn't be bold in markdown).
    let mut pieces = Vec::new();
    let mut line_start = 0;
    for line in block.split('\n') {
        let line_end = line_start + line.len();
        let (a, b) = (s.max(line_start), e.min(line_end));
        if a < b {
            let piece = &block[a..b];
            let a = a + (piece.len() - piece.trim_start().len());
            let b = b - (piece.len() - piece.trim_end().len());
            if a < b {
                pieces.push((a, b));
            }
        }
        line_start = line_end + 1;
    }
    if pieces.is_empty() {
        return Edit { text: block.to_string(), start, end };
    }

    // Only remove the style when every piece already has it. Otherwise add it
    // to the pieces that don't, so pressing the button gives a consistent result.
    let remove = pieces.iter().all(|&(a, b)| wrapped(block, a, b, style).is_some());
    let mut out = String::with_capacity(block.len() + pieces.len() * 2 * m.len());
    let mut copied = 0;
    let (mut sel_start, mut sel_end) = (None, 0);
    for &(a, b) in &pieces {
        let found = wrapped(block, a, b, style);
        let (inner_start, inner_end) = match (remove, found) {
            (true, Some(Wrap::Outside)) => {
                out.push_str(&block[copied..a - m.len()]);
                copied = b + m.len();
                (a, b)
            }
            (true, Some(Wrap::Inside)) => {
                out.push_str(&block[copied..a]);
                copied = b;
                (a + m.len(), b - m.len())
            }
            (false, None) => {
                out.push_str(&block[copied..a]);
                out.push_str(m);
                copied = b;
                (a, b)
            }
            // Adding, and this piece already has the style: leave it alone
            _ => {
                out.push_str(&block[copied..a]);
                copied = b;
                (a, b)
            }
        };
        sel_start.get_or_insert(out.len());
        out.push_str(&block[inner_start..inner_end]);
        sel_end = out.len();
        if !remove && found.is_none() {
            out.push_str(m);
        }
    }
    out.push_str(&block[copied..]);

    let sel_start = sel_start.unwrap_or(0);
    Edit {
        start: chars(&out[..sel_start]),
        end: chars(&out[..sel_end]),
        text: out,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wrap {
    /// The markers are just outside the text: **[word]**
    Outside,
    /// The markers are part of the text: [**word**]
    Inside,
}

/// Whether the text from byte `a` to `b` already has the style.
fn wrapped(block: &str, a: usize, b: usize, style: Inline) -> Option<Wrap> {
    let m = style.marker();
    let (before, inner, after) = (&block[..a], &block[a..b], &block[b..]);
    if before.ends_with(m)
        && after.starts_with(m)
        && style.star_run_ok(trailing_stars(before))
        && style.star_run_ok(leading_stars(after))
    {
        return Some(Wrap::Outside);
    }
    if inner.len() >= 2 * m.len()
        && inner.starts_with(m)
        && inner.ends_with(m)
        && style.star_run_ok(leading_stars(inner))
        && style.star_run_ok(trailing_stars(inner))
    {
        return Some(Wrap::Inside);
    }
    None
}

fn leading_stars(text: &str) -> usize {
    text.chars().take_while(|&c| c == '*').count()
}

fn trailing_stars(text: &str) -> usize {
    text.chars().rev().take_while(|&c| c == '*').count()
}

// =========================================================================
// Links
// =========================================================================

/// Turns the selection (or the word under the cursor) into a link.
///
/// - Plain text becomes `[text](url)` with "url" selected, ready to paste over.
/// - A web address becomes `[link text](address)` with "link text" selected.
/// - Nothing selected gives `[link text](url)` with "link text" selected.
pub fn link(block: &str, start: usize, end: usize) -> Edit {
    let (mut s, mut e) = (byte_at(block, start), byte_at(block, end));
    if s == e {
        (s, e) = word_around(block, s);
    }
    let selected = &block[s..e];
    let trimmed_start = s + (selected.len() - selected.trim_start().len());
    let selected = selected.trim();
    let (s, e) = (trimmed_start, trimmed_start + selected.len());

    let is_address = ["http://", "https://", "www.", "mailto:"]
        .iter()
        .any(|prefix| selected.starts_with(prefix))
        && !selected.contains(char::is_whitespace);

    let (inserted, sel_from, sel_len) = if selected.is_empty() {
        ("[link text](url)".to_string(), 1, "link text".len())
    } else if is_address {
        (format!("[link text]({selected})"), 1, "link text".len())
    } else {
        let text = format!("[{selected}](url)");
        let url_at = chars(selected) + 3;
        (text, url_at, 3)
    };
    let before = chars(&block[..s]);
    Edit {
        text: format!("{}{inserted}{}", &block[..s], &block[e..]),
        start: before + sel_from,
        end: before + sel_from + sel_len,
    }
}

// =========================================================================
// Line styles: headings and lists
// =========================================================================

/// One line split into its indent, its marker (like `- ` or `## `) and the rest.
/// Also used by `typing.rs`, so Enter and the formatting bar agree on what
/// counts as a list item.
pub(crate) struct Parsed<'a> {
    pub(crate) indent: &'a str,
    pub(crate) style: Option<LineStyle>,
    pub(crate) rest: &'a str,
}

pub(crate) fn parse_line(line: &str) -> Parsed<'_> {
    let text = line.trim_start_matches([' ', '\t']);
    let indent = &line[..line.len() - text.len()];
    let found = |style, marker_len: usize| Parsed {
        indent,
        style: Some(style),
        rest: &text[marker_len..],
    };

    // "# Heading": one to six #s, then a space (or nothing else on the line)
    let hashes = text.chars().take_while(|&c| c == '#').count();
    if (1..=6).contains(&hashes) {
        match text[hashes..].chars().next() {
            Some(' ') => return found(LineStyle::Heading(hashes), hashes + 1),
            None => return found(LineStyle::Heading(hashes), hashes),
            _ => {}
        }
    }

    let bytes = text.as_bytes();
    if bytes.len() >= 2 && matches!(bytes[0], b'-' | b'*' | b'+') && bytes[1] == b' ' {
        // "- [ ] task" or "- [x] done"
        if bytes.len() >= 5
            && bytes[2] == b'['
            && matches!(bytes[3], b' ' | b'x' | b'X')
            && bytes[4] == b']'
            && (bytes.len() == 5 || bytes[5] == b' ')
        {
            return found(LineStyle::Task, bytes.len().min(6));
        }
        return found(LineStyle::Bullet, 2);
    }

    // "1. item" or "1) item"
    let digits = text.chars().take_while(|c| c.is_ascii_digit()).count();
    if (1..=9).contains(&digits)
        && bytes.len() > digits + 1
        && matches!(bytes[digits], b'.' | b')')
        && bytes[digits + 1] == b' '
    {
        return found(LineStyle::Numbered, digits + 2);
    }

    Parsed { indent, style: None, rest: text }
}

/// One rewritten line: everything before `rest` is the prefix, which may
/// change. `rest` itself never changes, which lets the selection be moved
/// along with the text.
struct NewLine<'a> {
    old_prefix_len: usize,
    new_prefix: String,
    rest: &'a str,
}

/// Makes every line in the block a heading or list item. If they all already
/// are that kind, they go back to plain lines instead. Blank lines are skipped,
/// unless the cursor is on a blank line by itself (then it starts a new list).
pub fn toggle_line_style(block: &str, start: usize, end: usize, style: LineStyle) -> Edit {
    let lines: Vec<&str> = block.split('\n').collect();
    let parsed: Vec<Parsed> = lines.iter().map(|line| parse_line(line)).collect();
    let all_blank = lines.iter().all(|line| line.trim().is_empty());
    let is_target = |i: usize| all_blank || !lines[i].trim().is_empty();
    let remove = !all_blank
        && (0..lines.len())
            .filter(|&i| is_target(i))
            .all(|i| parsed[i].style == Some(style));

    let mut number = 0;
    let new_lines = lines
        .iter()
        .zip(&parsed)
        .enumerate()
        .map(|(i, (line, p))| {
            let old_prefix_len = line.len() - p.rest.len();
            let unchanged = NewLine {
                old_prefix_len,
                new_prefix: line[..old_prefix_len].to_string(),
                rest: p.rest,
            };
            if !is_target(i) {
                return unchanged;
            }
            // Headings can't be indented (4 spaces would make them code)
            let indent = match style {
                LineStyle::Heading(_) => "",
                _ => p.indent,
            };
            if remove {
                return NewLine { old_prefix_len, new_prefix: indent.to_string(), rest: p.rest };
            }
            if style == LineStyle::Numbered {
                number += 1;
                return NewLine {
                    old_prefix_len,
                    new_prefix: format!("{indent}{number}. "),
                    rest: p.rest,
                };
            }
            // Already this style: keep it (a ticked task stays ticked)
            if p.style == Some(style) {
                return unchanged;
            }
            let marker = match style {
                LineStyle::Heading(level) => format!("{} ", "#".repeat(level)),
                LineStyle::Bullet => "- ".to_string(),
                LineStyle::Task => "- [ ] ".to_string(),
                LineStyle::Numbered => unreachable!(),
            };
            NewLine { old_prefix_len, new_prefix: format!("{indent}{marker}"), rest: p.rest }
        })
        .collect::<Vec<_>>();
    rebuild(&lines, &new_lines, start, end)
}

/// Turns the lines into a quote (`> `), or back to plain lines if they already are one.
pub fn toggle_quote(block: &str, start: usize, end: usize) -> Edit {
    let lines: Vec<&str> = block.split('\n').collect();
    let quoted = |line: &str| line.trim_start().starts_with('>');
    let has_text: Vec<&&str> = lines.iter().filter(|line| !line.trim().is_empty()).collect();
    let remove = !has_text.is_empty() && has_text.iter().all(|line| quoted(line));

    let new_lines = lines
        .iter()
        .map(|line| {
            if remove {
                if !quoted(line) {
                    return NewLine { old_prefix_len: 0, new_prefix: String::new(), rest: line };
                }
                let after = &line.trim_start()[1..];
                let after = after.strip_prefix(' ').unwrap_or(after);
                return NewLine {
                    old_prefix_len: line.len() - after.len(),
                    new_prefix: String::new(),
                    rest: after,
                };
            }
            let prefix = if quoted(line) {
                ""
            } else if line.trim().is_empty() && lines.len() > 1 {
                // A blank line inside a quote keeps the quote going
                ">"
            } else {
                "> "
            };
            NewLine { old_prefix_len: 0, new_prefix: prefix.to_string(), rest: line }
        })
        .collect::<Vec<_>>();
    rebuild(&lines, &new_lines, start, end)
}

/// Joins rewritten lines back together and moves the selection with the text.
/// A position inside a line's old prefix ends up just after its new prefix.
fn rebuild(lines: &[&str], new_lines: &[NewLine], start: usize, end: usize) -> Edit {
    let mut text = String::new();
    // For each line: (char position in the old block, in the new block, old prefix, new prefix)
    let mut maps = Vec::with_capacity(lines.len());
    let (mut old_pos, mut new_pos) = (0, 0);
    for (i, (line, new)) in lines.iter().zip(new_lines).enumerate() {
        if i > 0 {
            text.push('\n');
        }
        let old_prefix = chars(&line[..new.old_prefix_len]);
        let new_prefix = chars(&new.new_prefix);
        maps.push((old_pos, new_pos, old_prefix, new_prefix));
        text.push_str(&new.new_prefix);
        text.push_str(new.rest);
        old_pos += chars(line) + 1;
        new_pos += new_prefix + chars(new.rest) + 1;
    }
    let map = |pos: usize| {
        let &(old_line, new_line, old_prefix, new_prefix) = maps
            .iter()
            .rev()
            .find(|(old_line, ..)| *old_line <= pos)
            .unwrap_or(&maps[0]);
        let column = pos - old_line;
        new_line + if column <= old_prefix { new_prefix } else { column - old_prefix + new_prefix }
    };
    Edit { start: map(start), end: map(end), text }
}

// =========================================================================
// Code blocks and tables
// =========================================================================

/// Puts the lines inside a ``` code block, or takes them out of one if the
/// block starts and ends with ``` lines.
pub fn toggle_code_block(block: &str) -> Edit {
    let lines: Vec<&str> = block.split('\n').collect();
    if lines.len() >= 2
        && lines[0].trim_start().starts_with("```")
        && lines[lines.len() - 1].trim() == "```"
    {
        let inner = lines[1..lines.len() - 1].join("\n");
        let end = chars(&inner);
        return Edit { text: inner, start: 0, end };
    }
    if block.trim().is_empty() {
        return Edit { text: "```\n\n```".to_string(), start: 4, end: 4 };
    }
    Edit {
        text: format!("```\n{block}\n```"),
        start: 4,
        end: 4 + chars(block),
    }
}

/// Adds a small table. On a blank line it replaces that line; otherwise it
/// goes below the block. Blank lines are added around it where needed, since a
/// table touching a paragraph would swallow it. "Column 1" ends up selected.
pub fn table(block: &str, line_before_is_blank: bool, line_after_is_blank: bool) -> Edit {
    let (text_before, gap_before) = if block.trim().is_empty() {
        (String::new(), if line_before_is_blank { "" } else { "\n" })
    } else {
        (block.to_string(), "\n\n")
    };
    let gap_after = if line_after_is_blank { "" } else { "\n" };
    let column_1 = chars(&text_before) + chars(gap_before) + 2;
    Edit {
        text: format!("{text_before}{gap_before}{TABLE}{gap_after}"),
        start: column_1,
        end: column_1 + "Column 1".len(),
    }
}

// =========================================================================
// Small helpers
// =========================================================================

fn chars(text: &str) -> usize {
    text.chars().count()
}

/// The byte position of a character position.
fn byte_at(text: &str, char_pos: usize) -> usize {
    text.char_indices().nth(char_pos).map_or(text.len(), |(byte, _)| byte)
}

/// The start and end (in bytes) of the word touching byte position `pos`, or
/// (pos, pos) if there isn't one.
fn word_around(text: &str, pos: usize) -> (usize, usize) {
    let is_word = |c: char| c.is_alphanumeric() || c == '_' || c == '\'';
    let start = text[..pos]
        .char_indices()
        .rev()
        .take_while(|&(_, c)| is_word(c))
        .last()
        .map_or(pos, |(i, _)| i);
    let end = text[pos..]
        .char_indices()
        .find(|&(_, c)| !is_word(c))
        .map_or(text.len(), |(i, _)| pos + i);
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit(text: &str, start: usize, end: usize) -> Edit {
        Edit { text: text.to_string(), start, end }
    }

    #[test]
    fn bold_wraps_and_unwraps_selection() {
        // "Some text": select "text"
        let bolded = toggle_inline("Some text", 5, 9, Inline::Bold);
        assert_eq!(bolded, edit("Some **text**", 7, 11));
        // Pressing again with the same selection removes it
        assert_eq!(toggle_inline(&bolded.text, 7, 11, Inline::Bold), edit("Some text", 5, 9));
        // Also works when the markers themselves are selected
        assert_eq!(toggle_inline("Some **text**", 5, 13, Inline::Bold), edit("Some text", 5, 9));
    }

    #[test]
    fn bold_and_italic_tell_their_stars_apart() {
        // Italic on bold text adds italic instead of removing anything
        assert_eq!(toggle_inline("**x**", 2, 3, Inline::Italic), edit("***x***", 3, 4));
        // Bold on italic text adds bold
        assert_eq!(toggle_inline("*x*", 1, 2, Inline::Bold), edit("***x***", 3, 4));
        // And each can be removed from bold-italic on its own
        assert_eq!(toggle_inline("***x***", 3, 4, Inline::Italic), edit("**x**", 2, 3));
        assert_eq!(toggle_inline("***x***", 3, 4, Inline::Bold), edit("*x*", 1, 2));
    }

    #[test]
    fn no_selection_uses_the_word_or_adds_a_pair() {
        // Cursor in the middle of "text"
        assert_eq!(toggle_inline("Some text", 7, 7, Inline::Italic), edit("Some *text*", 6, 10));
        // Cursor after a space: an empty pair, cursor inside
        assert_eq!(toggle_inline("Some ", 5, 5, Inline::Bold), edit("Some ****", 7, 7));
        // Pressing again right away removes the empty pair
        assert_eq!(toggle_inline("Some ****", 7, 7, Inline::Bold), edit("Some ", 5, 5));
    }

    #[test]
    fn spaces_stay_outside_and_lines_are_styled_separately() {
        assert_eq!(toggle_inline("a word here", 1, 7, Inline::Bold), edit("a **word** here", 4, 8));
        let two_lines = toggle_inline("one\n\ntwo", 0, 8, Inline::Strikethrough);
        assert_eq!(two_lines, edit("~~one~~\n\n~~two~~", 2, 14));
    }

    #[test]
    fn inline_code_and_non_ascii_text() {
        assert_eq!(toggle_inline("run ls -la", 4, 10, Inline::Code), edit("run `ls -la`", 5, 11));
        // é is one character but two bytes
        assert_eq!(toggle_inline("café ok", 0, 4, Inline::Bold), edit("**café** ok", 2, 6));
    }

    #[test]
    fn links() {
        assert_eq!(link("see docs", 4, 8), edit("see [docs](url)", 11, 14));
        assert_eq!(
            link("https://example.com", 0, 19),
            edit("[link text](https://example.com)", 1, 10)
        );
        assert_eq!(link("", 0, 0), edit("[link text](url)", 1, 10));
    }

    #[test]
    fn bullet_list_on_and_off() {
        let list = toggle_line_style("one\n\ntwo", 0, 8, LineStyle::Bullet);
        assert_eq!(list.text, "- one\n\n- two");
        let back = toggle_line_style(&list.text, 0, list.end, LineStyle::Bullet);
        assert_eq!(back.text, "one\n\ntwo");
    }

    #[test]
    fn switching_list_kinds_and_numbering() {
        let numbered = toggle_line_style("- a\n- b\n- c", 0, 11, LineStyle::Numbered);
        assert_eq!(numbered.text, "1. a\n2. b\n3. c");
        let tasks = toggle_line_style("1. a\n- [x] b", 0, 12, LineStyle::Task);
        // The ticked task stays ticked
        assert_eq!(tasks.text, "- [ ] a\n- [x] b");
        // Nested items keep their indent
        assert_eq!(toggle_line_style("  item", 0, 0, LineStyle::Bullet).text, "  - item");
    }

    #[test]
    fn cursor_moves_with_the_text() {
        // Cursor after "wor" in "word"
        assert_eq!(toggle_line_style("word", 3, 3, LineStyle::Bullet), edit("- word", 5, 5));
        // A blank line by itself starts a list item, cursor after the marker
        assert_eq!(toggle_line_style("", 0, 0, LineStyle::Task), edit("- [ ] ", 6, 6));
    }

    #[test]
    fn headings_replace_each_other() {
        assert_eq!(toggle_line_style("Title", 0, 0, LineStyle::Heading(2)).text, "## Title");
        assert_eq!(toggle_line_style("## Title", 4, 4, LineStyle::Heading(1)), edit("# Title", 3, 3));
        // The same level again makes it plain text
        assert_eq!(toggle_line_style("# Title", 4, 4, LineStyle::Heading(1)), edit("Title", 2, 2));
        // A bullet becomes a heading without its dash
        assert_eq!(toggle_line_style("- Title", 2, 2, LineStyle::Heading(3)).text, "### Title");
    }

    #[test]
    fn quotes() {
        let quoted = toggle_quote("one\n\ntwo", 0, 8);
        assert_eq!(quoted.text, "> one\n>\n> two");
        assert_eq!(toggle_quote(&quoted.text, 0, quoted.end).text, "one\n\ntwo");
    }

    #[test]
    fn code_blocks() {
        assert_eq!(toggle_code_block("let x = 1;"), edit("```\nlet x = 1;\n```", 4, 14));
        assert_eq!(toggle_code_block("```rust\nlet x = 1;\n```"), edit("let x = 1;", 0, 10));
        assert_eq!(toggle_code_block(""), edit("```\n\n```", 4, 4));
    }

    #[test]
    fn tables() {
        let on_blank_line = table("", true, true);
        assert_eq!(on_blank_line.text, TABLE);
        assert_eq!(&on_blank_line.text[on_blank_line.start..on_blank_line.end], "Column 1");
        // After a paragraph line, with text following: blank lines on both sides
        let after_text = table("Intro", false, false);
        assert_eq!(after_text.text, format!("Intro\n\n{TABLE}\n"));
        assert_eq!(&after_text.text[after_text.start..after_text.end], "Column 1");
    }
}

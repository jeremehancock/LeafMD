//! Counts the words in a document the way a reader would see them, and
//! writes the words for the label in the document's bottom-right corner
//! ("1,240 words · 7 min read").
//!
//! The markdown is read with the same rules as the formatted view, so marks
//! like `**` and `#`, link addresses, images and raw HTML aren't counted,
//! while the text of links and code is.

use comrak::arena_tree::NodeEdge;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, parse_document};

use crate::render;

/// How fast people are assumed to read, for the reading time
const WORDS_PER_MINUTE: usize = 200;

/// The number of words a reader would see in this markdown.
pub fn count_words(markdown: &str) -> usize {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &render::markdown_options());
    count_in_text(&reader_text(root))
}

/// The text a reader sees in a part of the parsed document (the whole
/// document, or one heading), with blocks separated by spaces so the last
/// word of one paragraph never runs into the first word of the next.
///
/// Images (their address and description) and raw HTML are left out. Links
/// keep their text and lose their address, because the address is never a
/// child of the link, only stored on it.
pub fn reader_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    // While inside an image or HTML, this is that node, and everything in it
    // is skipped until its end comes round
    let mut skipping: Option<&'a AstNode<'a>> = None;
    for edge in node.traverse() {
        match edge {
            NodeEdge::Start(child) => {
                if skipping.is_some() {
                    continue;
                }
                let data = child.data();
                match &data.value {
                    NodeValue::Image(_)
                    | NodeValue::HtmlInline(_)
                    | NodeValue::HtmlBlock(_)
                    | NodeValue::FrontMatter(_) => skipping = Some(child),
                    NodeValue::Text(words) => text.push_str(words),
                    NodeValue::Code(code) => text.push_str(&code.literal),
                    NodeValue::CodeBlock(block) => {
                        text.push(' ');
                        text.push_str(&block.literal);
                    }
                    NodeValue::SoftBreak | NodeValue::LineBreak => text.push(' '),
                    value if value.block() => text.push(' '),
                    _ => {}
                }
            }
            NodeEdge::End(child) => {
                if let Some(skipped) = skipping {
                    if std::ptr::eq(skipped, child) {
                        skipping = None;
                    }
                    continue;
                }
                if child.data().value.block() {
                    text.push(' ');
                }
            }
        }
    }
    text
}

/// The number of words in text that has no markdown in it, such as a
/// selection in the formatted view. Read as markdown, a line like
/// "1986. was a year" would lose its number to a numbered list.
pub fn count_plain_words(text: &str) -> usize {
    count_in_text(text)
}

/// Counts words in plain text: runs of letters or digits. An apostrophe or
/// hyphen with a letter or digit on both sides stays inside the word, so
/// "don't" and "well-known" are one word each, and a lone "-" is none.
fn count_in_text(text: &str) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut count = 0;
    let mut in_word = false;
    for (i, &c) in chars.iter().enumerate() {
        if c.is_alphanumeric() {
            if !in_word {
                count += 1;
                in_word = true;
            }
        } else {
            let joins = matches!(c, '\'' | '\u{2019}' | '-' | '\u{2010}' | '\u{2011}')
                && chars.get(i + 1).is_some_and(|next| next.is_alphanumeric());
            if !(in_word && joins) {
                in_word = false;
            }
        }
    }
    count
}

/// The words for the label: "0 words", "1 word · 1 min read",
/// "1,240 words · 7 min read", or with a selection, "35 of 1,240 words".
pub fn label_text(words: usize, selected: Option<usize>) -> String {
    let noun = if words == 1 { "word" } else { "words" };
    match selected {
        Some(selected) => format!("{} of {} {noun}", grouped(selected), grouped(words)),
        None if words == 0 => "0 words".to_string(),
        None => {
            let minutes = words.div_ceil(WORDS_PER_MINUTE);
            format!("{} {noun} · {minutes} min read", grouped(words))
        }
    }
}

/// Writes a number with commas between each group of three digits: 1,240.
/// (LeafMD isn't translated yet; this becomes the language's own way of
/// grouping digits when it is.)
fn grouped(number: usize) -> String {
    let digits = number.to_string();
    let mut out = String::new();
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_marks_and_link_addresses_dont_count() {
        assert_eq!(count_words("## Read the **whole** [guide](https://leafmd.app/guide)"), 4);
    }

    #[test]
    fn apostrophes_hyphens_and_numbers() {
        // "-" by itself isn't a word
        assert_eq!(count_words("Don't skip the well-known 3 steps - really."), 7);
        // A curly apostrophe works the same as a straight one
        assert_eq!(count_words("Don\u{2019}t stop"), 2);
    }

    #[test]
    fn list_markers_and_checkboxes_dont_count() {
        assert_eq!(count_words("- [ ] buy seeds\n1. water daily"), 4);
    }

    #[test]
    fn paragraphs_dont_run_together() {
        assert_eq!(count_words("one two\n\nthree"), 3);
        assert_eq!(count_words("# Title\nText"), 2);
        assert_eq!(count_words("| a | b |\n|---|---|\n| c | d |"), 4);
    }

    #[test]
    fn images_and_html_dont_count_but_code_does() {
        assert_eq!(count_words("![a garden photo](garden.png) Seeds"), 1);
        assert_eq!(count_words("<span class=\"note\">Hi</span> there"), 2);
        assert_eq!(count_words("<div class=\"box\">\n\n</div>\n"), 0);
        assert_eq!(count_words("Run `cargo build` now"), 4);
        assert_eq!(count_words("```\nlet total = 2;\n```"), 3);
    }

    #[test]
    fn plain_text_keeps_what_looks_like_markdown() {
        assert_eq!(count_plain_words("1986. was a year\n# of beds"), 6);
    }

    #[test]
    fn empty_text() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("---\n\n***"), 0);
    }

    #[test]
    fn label_wording() {
        assert_eq!(label_text(0, None), "0 words");
        assert_eq!(label_text(1, None), "1 word · 1 min read");
        assert_eq!(label_text(85, None), "85 words · 1 min read");
        assert_eq!(label_text(1240, None), "1,240 words · 7 min read");
        assert_eq!(label_text(1240, Some(35)), "35 of 1,240 words");
        assert_eq!(label_text(1_000_000, None), "1,000,000 words · 5000 min read");
    }

    #[test]
    fn reading_time_rounds_up() {
        assert_eq!(label_text(200, None), "200 words · 1 min read");
        assert_eq!(label_text(201, None), "201 words · 2 min read");
    }
}

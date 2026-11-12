//! Colours the code in fenced code blocks that name their language, like
//! ```rust, for the formatted view, printing and exporting.
//!
//! comrak (which turns markdown into HTML) hands each code block to this
//! file. Blocks whose language syntect knows come back as the same code with
//! its pieces labelled by colour group, like `<span class="hl-keyword">fn</span>`.
//! The labels only say what each piece is; the colours come from preview.css,
//! so they follow the light or dark style like the rest of the page.
//! Anything else (no language, a language syntect doesn't know) is written
//! as plain text, exactly as it was before colouring existed.
//!
//! syntect's own HTML writer labels nearly every character with several
//! nested names (2,000 lines of Rust became 5 MB of HTML, which took a second
//! to make and clean), so this writes one label per run of same-coloured text,
//! in the six groups the stylesheet colours. Coloured blocks are also kept
//! for next time, so typing only recolours the block that changed.
//!
//! The coloured code is also added after the page has been cleaned of
//! anything unsafe (render.rs), not before: cleaning all those labels took
//! most of the time. That's safe because this file writes them itself, from
//! code it has escaped. Until then, each block is a placeholder mark.
//!
//! comrak has a syntect helper of its own, but it throws away the block's
//! `data-sourcepos` tag (which double-click-to-jump and synced scrolling
//! need) and guesses a language for blocks that don't name one.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::rc::Rc;
use std::sync::OnceLock;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::html::{escape, write_opening_tag};
use syntect::easy::ScopeRangeIterator;
use syntect::parsing::{ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

/// The colour groups, as the class names preview.css colours. Each starts
/// with "hl-" so it can't clash with LeafMD's own classes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Group {
    Keyword,
    String,
    Comment,
    Number,
    Name,
    Type,
}

impl Group {
    fn class(self) -> &'static str {
        match self {
            Group::Keyword => "hl-keyword",
            Group::String => "hl-string",
            Group::Comment => "hl-comment",
            Group::Number => "hl-number",
            Group::Name => "hl-name",
            Group::Type => "hl-type",
        }
    }

    /// The group one of syntect's names belongs to, if any. Names are
    /// dotted, from general to specific, like "keyword.control.rust", and
    /// follow the naming most code editors share. Anything not listed
    /// (punctuation, operators, plain variables) keeps the text colour.
    fn of(scope: &str) -> Option<Group> {
        let starts = |prefixes: &[&str]| prefixes.iter().any(|p| scope.starts_with(p));
        if starts(&["comment"]) {
            Some(Group::Comment)
        } else if starts(&["string"]) {
            Some(Group::String)
        } else if starts(&["constant"]) {
            Some(Group::Number)
        } else if starts(&["keyword.operator"]) {
            // = + && and so on: colouring every one is noise
            None
        } else if starts(&["keyword", "storage"]) {
            Some(Group::Keyword)
        } else if starts(&[
            "entity.name.type",
            "entity.name.class",
            "entity.name.struct",
            "entity.name.enum",
            "entity.name.trait",
            "entity.other.inherited-class",
            "support.type",
            "support.class",
        ]) {
            Some(Group::Type)
        } else if starts(&[
            "entity.name",
            "entity.other.attribute-name",
            "support.function",
            "support.macro",
            "variable.function",
        ]) {
            Some(Group::Name)
        } else {
            None
        }
    }
}

/// The group a piece of code is in: decided by the innermost of its names
/// that belongs to one, so the quote marks around a string (named
/// "punctuation", inside "string") are coloured with the string.
fn group(stack: &ScopeStack, known: &mut HashMap<Scope, Option<Group>>) -> Option<Group> {
    stack
        .as_slice()
        .iter()
        .rev()
        .find_map(|scope| *known.entry(*scope).or_insert_with(|| Group::of(&scope.build_string())))
}

/// The coloured HTML for `code` in `syntax`, or None if syntect's patterns
/// fail on it (then it's shown plain rather than lost).
fn colour(code: &str, syntax: &SyntaxReference) -> Option<String> {
    let mut html = String::with_capacity(code.len() + code.len() / 2);
    let mut state = ParseState::new(syntax);
    let mut stack = ScopeStack::new();
    let mut known = HashMap::new();
    let mut open: Option<Group> = None;
    for line in LinesWithEndings::from(code) {
        let ops = state.parse_line(line, syntaxes()).ok()?;
        for (range, op) in ScopeRangeIterator::new(&ops, line) {
            stack.apply(op).ok()?;
            if range.is_empty() {
                continue;
            }
            let text = &line[range];
            // Spaces and new lines don't need a colour, so they don't end one
            let now = if text.trim().is_empty() { open } else { group(&stack, &mut known) };
            if now != open {
                if open.is_some() {
                    html.push_str("</span>");
                }
                if let Some(group) = now {
                    html.push_str("<span class=\"");
                    html.push_str(group.class());
                    html.push_str("\">");
                }
                open = now;
            }
            escape(&mut html, text).ok()?;
        }
    }
    if open.is_some() {
        html.push_str("</span>");
    }
    Some(html)
}

/// The character placeholders are made of. U+FDD0 is set aside by Unicode
/// for programs' own use and never means anything in text, and any in a
/// document are swapped out before rendering (see `without_marks`), so a
/// document can't fake a placeholder.
const MARK: char = '\u{FDD0}';

thread_local! {
    /// The coloured blocks of the page being rendered, in order; placeholder
    /// N in the page stands for block N here.
    static PENDING: RefCell<Vec<Rc<str>>> = RefCell::default();
}

/// Call before rendering a page: forgets the last page's blocks, and returns
/// the markdown with any placeholder characters in it replaced, so only the
/// placeholders this file writes can be found afterwards.
pub fn begin(markdown: &str) -> Cow<'_, str> {
    PENDING.with_borrow_mut(Vec::clear);
    if markdown.contains(MARK) {
        Cow::Owned(markdown.replace(MARK, "\u{FFFD}"))
    } else {
        Cow::Borrowed(markdown)
    }
}

/// Call after cleaning the page: puts each coloured block in place of its
/// placeholder.
pub fn finish(page: &str) -> String {
    PENDING.with_borrow_mut(|blocks| {
        let mut out = String::with_capacity(page.len() + blocks.iter().map(|b| b.len()).sum::<usize>());
        let mut rest = page;
        while let Some(start) = rest.find(MARK) {
            out.push_str(&rest[..start]);
            let after = &rest[start + MARK.len_utf8()..];
            let block = after.find(MARK).and_then(|end| {
                let number: usize = after[..end].parse().ok()?;
                Some((blocks.get(number)?.clone(), end))
            });
            match block {
                Some((html, end)) => {
                    out.push_str(&html);
                    rest = &after[end + MARK.len_utf8()..];
                }
                // Can't happen, since documents can't contain the mark; if
                // it somehow did, drop the stray mark and carry on
                None => rest = after,
            }
        }
        out.push_str(rest);
        blocks.clear();
        out
    })
}

/// Coloured blocks from earlier renders, by language and code, so typing in
/// one block doesn't recolour all the others. Emptied when it gets big, which
/// only means colouring again.
const KEEP_AT_MOST: usize = 4_000_000;

/// Coloured blocks by (language, code), and roughly how many bytes they hold.
type Kept = (HashMap<(String, String), Rc<str>>, usize);

thread_local! {
    static COLOURED: RefCell<Kept> = RefCell::default();
}

fn coloured(code: &str, syntax: &SyntaxReference) -> Option<Rc<str>> {
    let key = (syntax.name.clone(), code.to_string());
    if let Some(html) = COLOURED.with_borrow(|(kept, _)| kept.get(&key).cloned()) {
        return Some(html);
    }
    let html: Rc<str> = colour(code, syntax)?.into();
    COLOURED.with_borrow_mut(|(kept, size)| {
        if *size > KEEP_AT_MOST {
            kept.clear();
            *size = 0;
        }
        *size += key.1.len() + html.len();
        kept.insert(key, html.clone());
    });
    Some(html)
}

/// syntect's languages, loaded the first time a named code block is coloured
/// (it takes a moment) and kept for the rest of the run.
fn syntaxes() -> &'static SyntaxSet {
    static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines)
}

/// The language a code block's info string names, if syntect knows it.
/// `rust`, `Rust` and `rs` all find Rust; `sh` and `bash` find the shell.
/// Anything after the first word or comma (```rust,ignore) is left out.
pub fn language(info: &str) -> Option<&'static SyntaxReference> {
    let name = info.split([',', ' ']).next().unwrap_or("").trim();
    if name.is_empty() {
        return None;
    }
    // Names READMEs often use that syntect doesn't know by that name
    let name = match name.to_lowercase().as_str() {
        "shell" | "console" | "shellscript" | "shell-session" => "bash",
        "csharp" | "c#" => "cs",
        _ => name,
    };
    let syntaxes = syntaxes();
    syntaxes
        .find_syntax_by_token(name)
        // find_syntax_by_token only knows lowercase file extensions, and
        // matches names exactly, so ```Python and ```JSON need this
        .or_else(|| syntaxes.find_syntax_by_token(&name.to_lowercase()))
        // "Plain Text" is what syntect calls no colouring at all
        .filter(|syntax| syntax.name != "Plain Text")
}

/// What comrak calls to write a code block.
pub struct Highlighter;

impl SyntaxHighlighterAdapter for Highlighter {
    fn write_highlighted(&self, output: &mut dyn Write, lang: Option<&str>, code: &str) -> fmt::Result {
        let Some(syntax) = lang.and_then(language) else {
            return escape(output, code);
        };
        match coloured(code, syntax) {
            // A placeholder now; the coloured block goes in after cleaning
            Some(html) => {
                let number = PENDING.with_borrow_mut(|blocks| {
                    blocks.push(html);
                    blocks.len() - 1
                });
                write!(output, "{MARK}{number}{MARK}")
            }
            None => escape(output, code),
        }
    }

    // The <pre> and <code> tags keep everything comrak gives them, such as
    // data-sourcepos and class="language-rust"
    fn write_pre_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        write_opening_tag(output, "pre", attributes)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        write_opening_tag(output, "code", attributes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comrak::{Options, markdown_to_html_with_plugins, options::Plugins};

    fn html(markdown: &str) -> String {
        let mut options = Options::default();
        options.render.sourcepos = true;
        let mut plugins = Plugins::default();
        plugins.render.codefence_syntax_highlighter = Some(&Highlighter);
        let markdown = begin(markdown);
        finish(&markdown_to_html_with_plugins(&markdown, &options, &plugins))
    }

    #[test]
    fn a_rust_block_is_coloured_and_keeps_its_tags() {
        let out = html("```rust\nfn main() { println!(\"hi\"); } // greet\n```\n");
        assert!(out.contains("data-sourcepos=\"1:1-3:3\""), "{out}");
        assert!(out.contains("class=\"language-rust\""), "{out}");
        assert!(out.contains("<span class=\"hl-keyword\">fn </span>"), "{out}");
        assert!(out.contains("<span class=\"hl-string\">&quot;hi&quot;</span>"), "{out}");
        assert!(out.contains("<span class=\"hl-comment\">// greet"), "{out}");
        // The code itself is unchanged once the labels are taken out
        let text: String = out.split('<').map(|part| part.split_once('>').map_or(part, |p| p.1)).collect();
        assert!(text.contains("fn main() { println!(&quot;hi&quot;); } // greet"), "{text}");
    }

    #[test]
    fn every_suggested_name_is_coloured() {
        for name in crate::typing::CODE_LANGUAGES {
            assert!(language(name).is_some(), "{name} is suggested but not coloured");
        }
    }

    #[test]
    fn short_and_capitalised_names() {
        for name in ["sh", "bash", "shell", "console", "py", "Python", "js", "rs", "JSON", "yml", "yaml", "csharp"] {
            assert!(language(name).is_some(), "{name} should be known");
        }
        assert!(language("rust,ignore").is_some());
    }

    #[test]
    fn blocks_that_stay_one_colour() {
        for markdown in [
            "```\nfn main() {}\n```\n",
            "```typescript\nlet x: number = 1;\n```\n",
            "    fn main() {}\n",
        ] {
            let out = html(markdown);
            assert!(!out.contains("hl-"), "{markdown:?} gave {out}");
            assert!(out.contains("main()") || out.contains("number"), "{out}");
        }
    }

    #[test]
    fn code_is_escaped() {
        for markdown in ["```\n<b>&\n```\n", "```rust\nlet a = \"<b>&\";\n```\n"] {
            let out = html(markdown);
            assert!(!out.contains("<b>"), "{out}");
            assert!(out.contains("&lt;b&gt;"), "{out}");
            assert!(out.contains("&amp;"), "{out}");
        }
    }
}


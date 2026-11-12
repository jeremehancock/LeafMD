# Design

## Context

`src/render.rs` turns markdown into HTML with comrak, then cleans it with ammonia (which removes anything unsafe). The same function feeds the formatted view and, with small differences, printing and exporting. Every block carries a `data-sourcepos="12:1-15:3"` tag (the lines it came from), which double-click-to-jump and synced scrolling rely on, so a code block's `<pre>` must keep it.

comrak can hand each fenced code block to a "syntax highlighter adapter" (a small object it calls with the block's language and text, and which writes the HTML). It ships one built on syntect, but in its CSS-class mode that adapter writes `<pre class="syntax-highlighting">` and throws away the block's other attributes, including `data-sourcepos`. It also guesses a language from the first line when none is named, which the spec rules out.

The page's colours live in `src/preview.css` as variables on `:root`, with a `:root[data-theme="dark"]` block of dark versions. Printing always uses the light block. Exported web pages copy the dark block into a "reader is dark" rule (see `export.rs`), so anything defined as a variable in both blocks follows the reader automatically.

Find in the formatted view (`preview.js`) already joins the text of every piece inside one block before matching, so splitting code into coloured pieces doesn't break matches that cross colours.

## Goals / Non-Goals

**Goals:**
- Colours set by LeafMD's stylesheet, not baked into the HTML, so light, dark, print and export all work from one set of variables.
- The highlighter is built once per run, not per keystroke.

**Non-Goals:**
- A theme picker or a setting.
- Extra language definitions beyond syntect's built-in set (see Decisions 4).

## Decisions

**1. LeafMD's own small adapter instead of comrak's.**
A new `src/highlight.rs` implements comrak's adapter trait in about 60 lines on top of syntect directly:
- `write_pre_tag` and `write_code_tag` write the attributes comrak passes in unchanged, so `data-sourcepos` and `class="language-rust"` stay.
- `write_highlighted` looks the language up by name or short name (syntect's `find_syntax_by_token`, which knows `rs`, `py`, `sh` and so on). Not found or not named: write the code escaped, with no spans, exactly as today. Found: syntect's classed HTML generator writes `<span class="hl-keyword hl-control">fn</span>` and so on, already escaped.
- Fixing comrak's own adapter upstream was considered, but LeafMD would have to wait for a release, and the language guessing would still need turning off.

**2. Six colour groups, one label per run, added after cleaning.**
The first version used syntect's own HTML writer, which labels nearly every character with several nested names (`hl-meta hl-function hl-rust`...). Measured on 2,000 lines of Rust in 20 blocks, that made 5 MB of HTML and took about a second per render: 675 ms colouring and 186 ms cleaning. Since LeafMD re-renders after each pause in typing, that would have made typing stutter. So:
- `highlight.rs` walks syntect's names itself and sorts each piece of code into one of six groups (`hl-keyword`, `hl-string`, `hl-comment`, `hl-number`, `hl-name`, `hl-type`), by the innermost name that belongs to one, writing one `<span>` per run of the same group. Operators, punctuation and plain variables keep the text colour.
- The coloured blocks are added after ammonia has cleaned the page, as placeholders until then (a number between two U+FDD0 characters, a code point Unicode reserves for programs' own use). Any U+FDD0 in a document is replaced before rendering, so a document can't fake a placeholder; a test checks this. The coloured HTML is written by LeafMD from code it escaped itself, so it doesn't need cleaning. The document's own HTML is still cleaned exactly as before.
- Each coloured block is kept, by language and code, and reused while its text is unchanged, so typing only recolours the block being edited. The store is emptied when it passes about 4 MB.
The `hl-` prefix keeps the names from colliding with LeafMD's own classes such as `task-list-item`.

**3. Colours: a handful of variables, in both colour blocks.**
`preview.css` gets about six colour variables (`--code-keyword`, `--code-string`, `--code-comment`, `--code-number`, `--code-name`, `--code-type`) in `:root` and `:root[data-theme="dark"]`, picked from GNOME's palette (the same blues, greens, oranges, purples and greys GNOME Text Editor's Adwaita scheme uses), and a few rules mapping syntect's classes onto them, for example `.hl-comment { color: var(--code-comment); font-style: italic; }`. Each light colour must reach a contrast ratio of at least 4.5 to 1 on the light code card background, and each dark one on the dark card, checked with a short script during the work. Everything without a rule keeps the normal text colour. Print and export need no changes beyond what the variables give them.

**4. syntect's built-in languages, with its pure-Rust pattern matcher.**
syntect is added with `default-syntaxes`, `html` and `regex-onig`, and without `default-themes` (themes aren't used, since colours come from the stylesheet). The plan was `regex-fancy` (pure Rust), but measured on the same 2,000 lines, colouring a page for the first time took 750 ms with it and 200 ms with `regex-onig`, which compiles the Oniguruma C library with the C compiler the GNOME SDK already has.
Considered: the `two-face` crate, which adds bat's larger language set (TypeScript, TOML, Dockerfile and more). Rejected for now because it adds several more megabytes; it can be swapped in later without changing the specs' list of common languages.

**5. Built once, reused.**
syntect's language set takes a moment to load, so it's loaded the first time a named code block is rendered and kept for the rest of the run (`std::sync::OnceLock`). Documents with no named code blocks never load it.

**6. Credits.**
About's Built With gets "syntect, colours code blocks https://github.com/trishume/syntect" after ammonia. The README gets a new short "Built With" section listing the same libraries as About, with links, since it had none.

**7. Language suggestions use GtkSourceView's completion pop-up.**
GtkSourceView (the raw text editor) has a built-in suggestion pop-up that code editors use for word completion. LeafMD adds one "provider" (the object that tells the pop-up what to offer) by subclassing `sourceview5::CompletionProvider`:
- It only offers anything when the text before the cursor on that line is up to three spaces, ` ``` `, then one or more letters, digits, `+`, `#` or `-`, and the text above isn't inside an open fence (the existing `typing::in_code_block` answers that), so a closing fence never shows the list.
- The names come from a fixed list of about 30 lowercase names, the ones people actually write: `bash`, `c`, `cpp`, `csharp`, `css`, `diff`, `go`, `html`, `java`, `javascript`, `json`, `lua`, `makefile`, `markdown`, `perl`, `php`, `python`, `r`, `ruby`, `rust`, `scala`, `sh`, `shell`, `sql`, `xml`, `yaml` and similar. A unit test checks that every name on the list is one syntect colours, so the list can't offer a name that then stays one colour.
- It's interactive (pops up while typing), with no other providers, so the editor's usual word suggestions don't appear anywhere else.
- Requiring one letter before showing anything keeps ` ``` ` + Enter exactly as it is today. The pop-up selects its first row, so showing it straight after ` ``` ` would turn a plain Enter into a language.
- Alternative considered: a language menu on the Code block button. Rejected by the owner in favour of suggestions while typing, which also help people who type fences by hand.

## Risks / Trade-offs

- [Slower typing in documents with big code blocks, since the formatted view re-renders as you type] → Measured in release builds (2026-11-12), against today's uncoloured rendering. A README with 120 lines of code in 8 blocks: first render 13 ms (0.5 ms uncoloured), each render while typing in one block 2 ms. A document with 2,000 lines of code in 20 blocks: first render 200 ms (6 ms uncoloured), typing in one block 11 ms, rendering the same text again 1 ms. Loading syntect's languages costs about 5 ms, once per run. The owner said not to add the feature if it causes performance problems, so these numbers went to the owner before going on.
- [App size grows] → Measure the installed size before and after and say so in the release notes. Expected one to two megabytes.
- [A colour that reads well in light style is too dim in dark style, or the other way round] → The contrast check in Decision 3, plus pictures of both styles on the hidden desktop for the owner.
- [syntect's class names change between versions] → The CSS only relies on the top-level names, which follow the long-standing TextMate naming that syntect's syntaxes use.

- [The pop-up catching Enter in a way that fights the list-continuation code] → Enter only reaches the pop-up while it's showing, which is only on a fence line, where list continuation does nothing anyway. The end-to-end tests for lists and Enter run as before.

## Open Questions

None.

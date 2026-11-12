# Proposal

## Why

Code blocks in the formatted view are all one colour, so a long block of Rust or shell is harder to read than it is on GitHub or in a code editor. Colouring keywords, strings, comments and numbers makes code blocks easy to scan, and READMEs, LeafMD's most common documents, are full of them.

## What Changes

- Fenced code blocks that name their language (` ```rust `, ` ```bash `, ` ```python `, ` ```json ` and so on) are shown with coloured keywords, strings, comments, numbers and names in the formatted view.
- The colours come from GNOME's palette and have a light and a dark version, so they follow the Light, Dark or Follow System style like the rest of the page.
- Printed pages, PDFs and exported web pages get the same colours (light ones on paper and in PDFs; exported web pages follow the reader's light or dark style, as they already do).
- Typing ` ``` ` and the first letter of a language at the start of a line in the raw text pops up a short list of language names that narrows as you type (`p` offers `python`, `perl`, `php`). Enter or a click picks one; carrying on typing, or Escape, ignores it. Typing just ` ``` ` and Enter still starts a plain block, with no list in the way.
- Help explains naming a language after the opening ` ``` ` and lists the common short names, and the Cheatsheet's code block example names a language (` ```python `).
- Blocks with no language, a language LeafMD doesn't know, indented code and inline `code` stay as they are now: one colour.
- About's Built With list adds syntect, the library that does the colouring. The README gains a Built With section listing the same libraries as About.
- Documents still can't run scripts. The colouring is done by LeafMD before the page is shown, and only adds plain coloured text.

No new keyboard shortcut, menu item or setting (the suggestion list is GtkSourceView's own pop-up, the same kind code editors use). There's no switch to turn the colours off and no choice of colour themes.

**Out of scope**

- Colouring code inside the raw text. GtkSourceView already colours the markdown there, and colouring the code inside fences would be a separate change.
- Languages outside syntect's built-in set of about 50 (it covers the common ones: Bash and shell, C, C++, C#, CSS, Go, HTML, Java, JavaScript, JSON, Python, Ruby, Rust, SQL, XML and YAML among them; notably TypeScript, TOML, Kotlin and Swift aren't). Those blocks stay one colour.
- Guessing the language of a block that doesn't name one.
- Line numbers or a copy button on code blocks.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `formatted-view`: adds coloured code blocks, which languages get them, and that colours follow the light or dark style.
- `print-and-export`: printed pages, PDFs and exported web pages keep the code colours.
- `smart-typing`: adds language suggestions after ` ``` ` in the raw text.
- `help-and-about`: About's Built With section adds syntect; Help explains naming a code block's language; the Cheatsheet's code block names one.

## Impact

- New library: syntect (with its built-in language definitions and its pure-Rust pattern matcher, so nothing extra is needed from the GNOME runtime). The app grows by roughly a megabyte or two; the exact size is measured during the work.
- Code: `src/render.rs` hands named code blocks to a small highlighter that writes coloured spans while keeping each block's source line tags (used for click-to-jump and synced scrolling). `src/preview.css` gains the colours for light and dark. The raw text gets a small suggestion provider for GtkSourceView's completion pop-up (`src/document.rs` or a new file beside `typing.rs`). The cleaning step already allows the class names this uses; a test proves it.
- Docs: About and README credits, Help, README features, the app store description, the storefront at leafmd.app, docs/DEVELOPMENT.md, and the screenshots if a screenshot shows a code block.

# Tasks

## 1. The highlighter

- [x] 1.1 Add syntect to `Cargo.toml` (no default features; `default-syntaxes`, `html`, `regex-onig`, see design.md Decision 4) with a plain-English comment like the others, and note the installed app's size before the change (`flatpak info --show-size io.github.jeremehancock.LeafMD` or the size of `/app` in the build). Verify `cargo build` passes.
- [x] 1.2 Add `src/highlight.rs`: LeafMD's comrak adapter (keeps the `pre` and `code` attributes, colours only blocks whose named language syntect knows, `hl-` class prefix, language set loaded once). Unit tests: a ` ```rust ` block gets `hl-` spans and keeps `data-sourcepos`; ` ```sh `, ` ```py ` and ` ```js ` are found by their short names; no language, ` ```typescript ` and an indented block have no spans; `<b>` and `&` inside code come out escaped. Verify with `cargo test`.
- [x] 1.3 Use the adapter in `src/render.rs` for the formatted view and for print and export. Tests: the spans survive ammonia's cleaning, a ` ```html ` block containing `<script>alert(1)</script>` shows as text and adds no script element, and export HTML still has no `data-sourcepos`. Verify with `cargo test`.
- [x] 1.4 Time rendering a document with 2,000 lines of Rust in fenced blocks (a test or a small timing run in release mode). Verify it stays under about 50 ms; if not, follow the fallback in design.md's Risks before going on.
- [x] 1.5 Add `src/highlight.rs` to the file table in docs/DEVELOPMENT.md, and mention it in `src/render.rs`'s row. Verify by reading it.

## 2. The colours

- [x] 2.1 In `src/preview.css`, add the code colour variables to `:root` and `:root[data-theme="dark"]`, and the rules mapping `hl-` classes onto them. Check every colour reaches 4.5 to 1 contrast on its code card background with a short script (kept out of the repo). Verify with the script's output.
- [x] 2.2 On the hidden desktop, open a document with Rust, Bash, Python, JSON and YAML blocks in Split view, take pictures in light and dark, and switch the style in Preferences while it's open. Verify the colours look right in both and change straight away.
- [x] 2.3 Export the same document as PDF (with LeafMD in dark style) and as HTML. Verify the PDF uses the light colours and the HTML's dark rule contains the dark code colours (`grep` the exported file), with no `<script>`.

## 3. Language suggestions

- [x] 3.1 Add the list of language names and the "does this line want suggestions, and for what text" check as plain functions (beside `typing.rs`'s helpers), with unit tests: ` ```py ` wants `py`, `   ```py ` (three spaces) does, four spaces doesn't, ` ``` ` alone doesn't, ` ```py ` on a closing fence doesn't, `~~~` doesn't; and every listed name is one the highlighter colours. Verify with `cargo test`.
- [x] 3.2 Add the completion provider to the raw text view (subclassing `sourceview5::CompletionProvider`), interactive, filtering by what's typed, replacing it on activation. Verify on the hidden desktop: typing ` ```py ` shows the list, Enter gives ` ```python `, Escape then Enter leaves ` ```py `, and ` ``` ` + Enter shows nothing. Take a picture of the list in light and dark.
- [x] 3.3 Add `build-aux/end-to-end/code-language.sh`, following `paste-link.sh`: type ` ```py `, wait for the list, press Enter, type `print(1)`, save, and check the file holds ` ```python ` then `print(1)`; and a second check that ` ``` ` then Enter adds a plain new line. Verify it passes in `./build-aux/end-to-end-tests.sh`.

## 4. Credits and where users read about it

- [x] 4.1 About's Built With list (`src/window.rs`): add "syntect, colours code blocks" with its link after ammonia. Verify by opening About on the hidden desktop.
- [x] 4.2 README.md: add a "Built With" section listing Rust, GTK, libadwaita, GtkSourceView, WebKitGTK, libspelling, comrak, ammonia and syntect with links and what each does, matching About; and mention coloured code blocks and the language suggestions in the features list. Verify by reading it.
- [x] 4.3 Help (`show_help` in `src/window.rs`): explain naming a language after the opening ` ``` ` to colour the code, the common short names (`sh`, `py`, `js`, `rs`), the suggestions while typing, and that unknown languages stay one colour. Verify by reading the Help text in the running app.
- [x] 4.4 The app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and the storefront in `site/index.html`: mention coloured code blocks. Verify `./build-aux/flathub-lint.sh` passes.
- [x] 4.5 Markdown Cheatsheet (`src/window.rs`): change the Blocks example to ` ```python ` with `print("Hello")`, its formatted side showing the code. Verify by opening the cheatsheet on the hidden desktop.
- [x] 4.6 Screenshots: the sample document's only code block is a watering timetable with no language, so the screenshots don't change. Confirm by checking `data/screenshots/sample.md` again after the work; no new pictures needed.

## 5. Checks and installing

- [x] 5.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh` (which also vendors the new library the way the Flatpak build does), all passing with no warnings.
- [x] 5.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, note the installed size against 1.1, then run `./build-aux/end-to-end-tests.sh`, all scenarios passing (including the new code-language one), so the owner can try it. Colours themselves can't be checked by the end-to-end tests, which only check text, files and windows.

## Workflow follow-up

- The owner tries it in the installed Flatpak, then `/ship` commits and archives the change (copying the new rules into `openspec/specs/formatted-view`, `openspec/specs/print-and-export`, `openspec/specs/smart-typing` and `openspec/specs/help-and-about`).
- No ROADMAP item is added or ticked here: the owner plans to clear the roadmap down to Version 1.0 and Translations once this and the image drop change are done.

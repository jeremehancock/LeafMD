# Tasks

## 1. Finding out how printing works in the Flatpak

- [x] 1.1 Try both ways from design decision 5 in a quick throwaway build (not committed): a hidden WebView with a short document and a picture, printed (a) to a PDF file with no dialog and (b) through `PrintOperation::run_dialog`. Install it with `./build-aux/flatpak-install.sh` and check, on the owner's desktop with their permission or on the test desktop, whether (a) writes a PDF with the picture in it, and whether (b)'s dialog lists real printers or only "Print to File". Write down which way Print will use (A or B) in design.md's Open Questions, and whether the hidden WebView needed a hidden window (design Risks). Verify by looking at the PDF made by (a) and a picture of (b)'s dialog

## 2. One stylesheet, with rules for paper

- [x] 2.1 Move the `<style>` block from `src/preview.html` into a new `src/preview.css`, placed back into the page when it's loaded (design decision 1), with the dark colours marked so `export.rs` can reuse them. Verify with `cargo build`, and on the test desktop with pictures of the formatted view in light and dark style that look the same as before
- [x] 2.2 Add the `@media print` rules (design decision 1): code wraps, tables fit the page, no breaks inside code blocks, table rows and pictures, none just after headings, no find highlights. Verify later through task 3.3's PDF

## 3. The standalone page, PDF and HTML

- [x] 3.1 In `src/render.rs`, add the export cleaner (design decision 2): the same as today's, but without `data-sourcepos`, and with an attribute filter that hands each `img` `src` to a function given by the caller. Add unit tests: no `data-sourcepos`, still no `<script` or `onerror`, and the filter is called for `![](img/a.png)` and for raw `<img src="b.png">`. Verify with `cargo test`
- [x] 3.2 Create `src/export.rs` with the page builder (design decision 2): stylesheet, font block from Preferences at 100% zoom, accent colour, light only or light plus `prefers-color-scheme: dark`, the title from the first level-one heading (via `headings::find`) or the file name, no scripts, and picture packing for the HTML flavour. Unit tests: the title both ways, a picture next to a temporary markdown file becomes a `data:image/png;base64,` address, a missing picture is left as written, an `https://` picture is left alone, the light-only page has no `prefers-color-scheme`, and no `<script` in either. Verify with `cargo test`
- [x] 3.3 Export as PDF (design decision 4): the `win.export-pdf` action, the save dialog suggesting `<name>.pdf` in the document's folder with a PDF filter, the hidden WebView, printing to a temporary file in the cache folder, copying it into place, "Saved garden.pdf", and "Couldn't export:" with the reason on failure. Verify on the test desktop by calling the export code with a fixed file name from a temporary debug hook (removed afterwards, since the test desktop never opens file dialogs), then checking the PDF: light in dark style, Preferences size at 150% zoom, a long code line wrapped, the picture present, unsaved changes included and the tab still showing its dot
- [x] 3.4 Export as HTML (design decision 6): the `win.export-html` action, the save dialog suggesting `<name>.html`, writing the page, "Saved garden.html", and "Couldn't export:" on failure. Verify the same way as 3.3, then open the HTML file in a browser in light and dark style, and move it alone to another folder to check the picture still shows
- [x] 3.5 Print (design decision 5, using the way chosen in 1.1): the `win.print` action and `<Control>p` in `src/main.rs`, and "Couldn't print:" with the reason when the portal or dialog isn't there. Verify on the test desktop (which has no print portal) that Ctrl+P shows "Couldn't print:" and LeafMD keeps running; printing itself is in the owner's hand test (task 7.3)

## 4. The menu

- [x] 4.1 Add a menu section with Print…, Export as PDF… and Export as HTML… between the Save and Find sections, and add the three actions to `DOCUMENT_ACTIONS` (design decision 7). Verify on the test desktop with pictures of the menu with a document open (Print… showing Ctrl+P) and greyed out on "No Document Open", and that Ctrl+P there does nothing
- [x] 4.2 Add `build-aux/end-to-end/print-needs-document.sh` only if the tests can read whether `win.print` is enabled (print-and-export spec "Only with a document open": with no document, `win.print`, `win.export-pdf` and `win.export-html` are disabled; after opening notes.md, they're enabled). If it's added, also add an `end-to-end-tests` delta to this change raising the total in "Everything passes" from 46 to 47. Otherwise skip it and say so here. Verify with `./build-aux/end-to-end-tests.sh print-no-document` after installing

## 5. Keeping the selected tab in view

- [x] 5.1 In `src/window.rs`, find the tab bar's scrolled area and scroll the selected tab into view when its visible width shrinks (design decision 8), doing nothing if the scrolled area isn't found. Verify on the test desktop: 8 tabs, the eighth selected, the window narrowed from 1000 to 360 in small steps like a hand, with pictures showing the eighth tab fully visible at the end and the same tab still selected (`expect_title`). Also check that Alt+1 then narrowing keeps the first tab in view, and that dragging a tab along the bar still works

## 6. Everywhere people read about it

- [x] 6.1 Help (`show_help` in `src/window.rs`): add printing and exporting to the "Files and Links" section, or a short section of its own if it reads better (help-and-about spec "Reading about printing and exporting"). Keyboard Shortcuts (`show_shortcuts`): "Print" with "Ctrl+P" under Files. Verify by opening both in the installed build
- [x] 6.2 README.md: Print and the two exports in the features list, and Ctrl+P in the shortcuts table. docs/DEVELOPMENT.md: `src/export.rs` and `src/preview.css` in the file table. Verify by reading them in LeafMD
- [x] 6.3 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: mention printing and exporting to PDF and HTML in the description. (Release notes wait for `/ship`.) Verify with `./build-aux/sdk-check.sh`, which validates the file
- [x] 6.4 `site/index.html`: mention Print and Export where it fits without unbalancing the feature boxes. Verify by building the site with `build-aux/build-site.py` and looking at it
- [x] 6.5 ROADMAP.md: tick Print, Export to PDF and Export to HTML under "Sharing". Verify by reading it

## 7. Checks and trying it

- [x] 7.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate --strict print-and-export`, with no failures or warnings
- [x] 7.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh` and check every scenario passes (46, or 47 if task 4.2 added one)
- [ ] 7.3 Ask the owner to close every LeafMD window, reopen it and try what the tests can't: Ctrl+P printing to a real printer (or Print to File) from the Raw view in dark style, Export as PDF and Export as HTML through the real save dialogs, opening the HTML in a browser in light and dark style and after moving it elsewhere, and narrowing a window with many tabs

## Workflow follow-up

- `/ship` asks about the version, writes the release notes, commits, archives the change (which copies the spec changes into `openspec/specs/`) and opens the pull request.

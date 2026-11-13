# Tasks

## 1. Remembering the choice

- [x] 1.1 In `src/state.rs`, add a `Margins` enum (Narrow, Normal, Wide) with `load_margins` and `save_margins` for `margins=narrow|normal|wide` in the preferences group (anything missing or unreadable is Normal), and methods giving its raw characters (120/100/80), formatted column in em (62/52/44) and paper margin in mm (10/15/20). Add unit tests beside the style ones for a missing, valid and nonsense value and for Wide giving today's numbers (80, 44, 20); verify with `cargo test`.
- [x] 1.2 In `src/main.rs`, add the app-wide stateful `margins` action (string state `narrow`, `normal` or `wide`) beside `style`. On change, it saves the choice and asks every window to re-apply margins to each of its documents. Verify with `cargo build`.

## 2. The raw text column

- [x] 2.1 In `src/document.rs`, make `column_width` use the chosen number of characters instead of 80, and use the clamp in Split view as well as Raw view (in `apply_view`, `apply_fonts` and `show_line_numbers`). Add `Document::apply_margins` that re-measures and sets the clamp, called by the window when the setting changes. Update the comments that say Split fills its half. Verify on the hidden desktop: a 1400px window in Raw view shows about 100 characters per line at Normal and about 80 at Wide, and in Split view with the divider dragged far right the raw half keeps its column.

## 3. The formatted view column

- [x] 3.1 In `src/preview.css`, change `#content`'s `max-width` to `var(--leafmd-column, 52em)`, leaving the `@media print` block's `max-width: none` alone. In `src/document.rs`, set `--leafmd-column` with `run_js` once the page has loaded (beside `--leafmd-weight`) and in `apply_margins`. Verify on the hidden desktop: switching between Narrow, Normal and Wide in Formatted view changes the column at once without the scroll position jumping, and Split view's scrolling together still lines up afterwards.

## 4. The Margins row in Preferences

- [x] 4.1 In `src/preferences.rs`, add a Margins row under Style in the Appearance group, with a toggle group of Narrow, Normal and Wide showing the `margins` action's state, changing it when clicked, and following it if it changes while Preferences is open (as the Style row does). Make the dialog tall enough, and update the file's top comment. Verify by opening Preferences on the hidden desktop and taking a picture in light and in dark.

## 5. Paper and the exported web page

- [x] 5.1 In `src/export.rs`, add one helper that builds the `gtk::PageSetup` (the locale's usual paper, the chosen margin on all four sides) and use it for Export as PDF. Verify by exporting a PDF at each setting and measuring the margins (for example with `pdftotext -bbox-layout` and checking where the text starts). (Print was dropped from this change after the owner's try; see section 9 and design.md, decision 8.)
- [x] 5.2 In `src/export.rs`, add the column width to `Look` and have `Look::css` set `#content`'s `max-width` from it. Add a unit test that a page exported at Wide contains `max-width: 44em` and one at Narrow contains `62em`; verify with `cargo test`, and by opening an exported page in a browser at full width and at phone width (no sideways scrolling).

## 6. End-to-end test

- [x] 6.1 Add `build-aux/end-to-end/margins-remembered.sh` (after `style-remembered.sh`): the `margins` action starts as `normal`; setting it to `wide` saves `margins=wide` in the preferences group; after Ctrl+Q and opening notes.md again the action is still `wide`. Verify it passes in `./build-aux/end-to-end-tests.sh`.

## 7. Where users read about it

- [x] 7.1 Help (`show_help` in `src/window.rs`): mention the Margins choice where it describes Preferences, and that it also sets the page margins when printing; verify by reading the Help text in the running app. Keyboard Shortcuts don't change (no new shortcut).
- [x] 7.2 README.md: add Margins to the "Style, fonts and zoom" feature line, and the page margins to the "Print and export" line; verify by reading it.
- [x] 7.3 The app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and the storefront in `site/index.html`: mention choosing narrow, normal or wide margins; verify `./build-aux/flathub-lint.sh` passes.
- [x] 7.4 Retake the screenshots with `build-aux/take-screenshots.sh`, since the default line width changes, and check them as `data/screenshots/README.md` describes.
- [x] 7.5 ROADMAP.md: add "Margins: Narrow, Normal or Wide in Preferences" under Next (ticked when the change is archived by `/ship`); verify by reading it.

## 8. Checks and installing

- [x] 8.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, all passing with no warnings.
- [x] 8.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh`, all scenarios passing, so the owner can try it.

## 9. After the owner's first try

- [x] 9.1 Print: remove the `set_page_setup` call in `export::print`, so Print uses the Print dialog's margins as before; keep the helper for PDF. Update the comments. Verify with `cargo build` and by reading the code: Print's operation no longer gets a page setup.
- [x] 9.2 Add None to `state::Margins` (variant `FullWidth`, saved and shown as `none` and `None`, first in `ALL`), with no raw or formatted limit and a 5mm PDF margin, and move Narrow to 140 characters and 72em. Raw text gives the clamp its "unlimited" width for None; the formatted view and exported page get `max-width: none`. Update the unit tests (None round-trips, Narrow exports `72em`, None exports `max-width: none`); verify with `cargo test`, and on the hidden desktop that None fills a 1400px window in Raw and Formatted views and Narrow holds about 140 characters.
- [x] 9.3 In `src/preferences.rs`, put the two font rows into one Fonts group, titled "Raw Text" and "Formatted View", each keeping its font button and Reset; the Margins row offers None, Narrow, Normal and Wide. Verify with a picture of Preferences on the hidden desktop.
- [x] 9.4 Update Help, README, the app store description, leafmd.app and the ROADMAP item for None and for Margins covering PDFs but not printing (Help and README say to export a PDF for a printout with these margins); verify by reading them and with `./build-aux/flathub-lint.sh`.
- [x] 9.5 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, then rebuild and install with `./build-aux/flatpak-install.sh` and run `./build-aux/end-to-end-tests.sh`, all passing, so the owner can try it again.

## Workflow follow-up

- The owner tries it in the installed Flatpak, then `/ship` commits, archives the change (copying the new rules into `openspec/specs/views`, `formatted-view`, `text-size` and `print-and-export`), ticks the ROADMAP item and opens the pull request.

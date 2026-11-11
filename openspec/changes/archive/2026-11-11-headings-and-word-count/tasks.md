# Tasks

## 1. Finding headings and counting words

- [x] 1.1 Add `src/headings.rs` (design decision 1): a function that parses markdown with comrak, using the same extensions as `src/render.rs`, and returns each heading's level, plain text and starting line. It skips empty headings, and code blocks and HTML never produce headings. Add unit tests for the headings spec's examples: the Garden Plan outline (levels and lines), `## **Install** [LeafMD](https://leafmd.app)` giving "Install LeafMD", `# install the tools` inside a code block not listed, an underlined (`===` and `---`) heading listed, and `<h2>Install</h2>` not listed. Verify with `cargo test headings`
- [x] 1.2 Add `src/word_count.rs` (design decisions 2 and 3): `count_words(markdown)` following the word rules, and `label_text(words, selected)` giving "0 words", "1 word · 1 min read", "1,240 words · 7 min read" and "35 of 1,240 words". Add unit tests for every example in the word-count spec ("4 words", "7 words" with the lone hyphen, "4 words" for the list and checkbox), plus: curly apostrophes, a paragraph ending and the next starting not merging, image descriptions and raw HTML not counted, code counted, 200 words giving 1 minute and 201 giving 2. Verify with `cargo test word_count`
- [x] 1.3 Add both files to the file table in `docs/DEVELOPMENT.md`. Verify by reading the table

## 2. Documents keep their headings and count

- [x] 2.1 In `src/document.rs`, after the 150 millisecond pause, always work out the headings and word count (even when the formatted view is hidden), store them on the document, and tell the window if it's the selected tab (design decision 1). Work them out straight away when a document is created or a file is loaded, so a new tab shows "0 words" at once. Verify on the test desktop that typing in Raw view updates both, using a temporary log line that is removed afterwards
- [x] 2.2 Count the selection when the cursor or selection end moves, at most once per frame, and tell the window (design decision 2). Verify on the test desktop by selecting a paragraph and reading the label in a window screenshot
- [x] 2.3 Add `jump_to_line(line)` (design decision 5): cursor to the line start, raw text scrolled and focused in Raw and Split, `scrollToLine` in the formatted view in Formatted and Split, with the "ignore the next scroll" timers set on both sides in Split view. Verify with the jump-to-heading scenario (task 4.5)
- [x] 2.4 Report the line at the top of the document to the window (design decision 6): from the raw text's scrolling in Raw and Split (sharing the calculation with `sync_preview_to_editor`), and from the formatted view's `scroll` message in Formatted view, without changing when Split view syncs. Verify that synced scrolling in Split view still works on the test desktop, and that `cargo test` still passes

## 3. The word count label

- [x] 3.1 Add the label in an overlay around the window's tab view (design decision 8): bottom-right, small, dimmed, on a rounded see-through background, ignoring the pointer, and refreshed from the selected document on typing, selection and tab changes. Hidden with no document open. Verify with window screenshots on the test desktop in light and dark style, Formatted, Raw and Split view, and a narrow window with the bottom bar
- [x] 3.2 Raise the raw text's bottom margin to about 48 pixels and add matching bottom padding to the formatted page in `src/preview.html`, so the last line scrolls clear of the label. Verify with a screenshot scrolled to the end of a long document in each view
- [x] 3.3 Hide the label in Distraction Free and bring it back when leaving it, alongside the bars. Verify on the test desktop with Ctrl+Shift+D twice
- [x] 3.4 Help (`show_help` in `src/window.rs`): describe the word count, what counts as a word, the 200 words a minute reading time, and counting a selection in the raw text. Add the word count to Distraction Free's list of what's hidden. Verify by opening Help in the installed build

## 4. The Headings page

- [x] 4.1 Draw `leafmd-headings-symbolic` in `data/resources/icons` with filled shapes only, and list it in `data/resources/resources.gresource.xml`. Verify it shows in the page switcher in a test desktop screenshot, in light and dark style
- [x] 4.2 Build the Headings page (design decision 4): a list with the `navigation-sidebar` style, rows indented 12 pixels per level below the document's top level, long headings cut off with a tooltip, and "No Headings" ("Lines starting with # appear here") and "No Document Open" pages. Add it to the side panel after Files, so the switcher appears. The panel's title says "Headings" on this page and the folder's name on Files. Verify with screenshots on the test desktop of the Garden Plan outline, a document with no headings, and no document
- [x] 4.3 Activating a row (click, or Enter) calls `jump_to_line`, and in a narrow window tucks the panel away like picking a file. Rebuilding after typing or switching tabs keeps the list's scroll position. Verify on the test desktop in a 1200 and a 700 pixel window
- [x] 4.4 Highlight the heading of the section at the top of the document, at most once per frame, scrolling the list only when that row is out of sight, with nothing highlighted above the first heading (design decision 6). Verify with screenshots after scrolling in Formatted and Raw view, and back at the top
- [x] 4.5 Add the `win.side-panel-page` action and remember the page in `state.ini` as `[side-panel] page=` (design decision 7), with `SidePanel` gaining `set_visible_page`, `visible_page` and a change notice. Add a unit test in `src/state.rs` that a missing or unknown page reads as Files. Verify with `cargo test`
- [x] 4.6 Add `build-aux/end-to-end/headings-page-remembered.sh` (side-panel spec, "The side panel remembers its page": F9, `activate_action side-panel-page headings`, Ctrl+Q, start again, `expect_action_state side-panel-page headings`) and `build-aux/end-to-end/jump-to-heading.sh` (headings spec, "Jumping to a heading": Raw view, Headings page, click "Beds", type "Raised ", Ctrl+S, `expect_file_contains` "Raised ## Beds"). Verify with `./build-aux/end-to-end-tests.sh headings-page-remembered jump-to-heading` after installing
- [x] 4.7 Help: describe the Headings page, switching pages at the bottom of the panel, jumping, the highlight, and that the panel remembers its page. Verify by opening Help in the installed build

## 5. Everywhere else people read about it

- [x] 5.1 README.md: add the Headings page and the word count to the features list. The shortcuts table doesn't change. Verify by reading it in LeafMD
- [x] 5.2 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: add a line about the Headings page and the word count. (Release notes wait for `/ship`.) Verify with `./build-aux/sdk-check.sh`, which validates the file
- [x] 5.3 `site/index.html`: mention the Headings page and word count with the other features, if it fits the page. Verify by building the site with `build-aux/build-site.py` and reading it
- [x] 5.4 Retake the screenshots with `./build-aux/take-screenshots.sh`, since every window now shows the word count, and check them against `data/screenshots/README.md`. Verify by looking at each new picture
- [x] 5.5 Tick "Headings in the side panel" and "Word count" in ROADMAP.md. Verify by reading it

## 6. Formatted selections and Escape order

- [x] 6.1 Formatted view selections (design decision 2b): `src/preview.js` posts a `selection` message with the selected text at most once per frame; the document counts it, remembers which side was selected last, and `word_label` shows the most recent selection whose side is showing. Verify on the test desktop with screenshots: a selection in Formatted view, a raw then formatted selection in Split view, and a raw selection followed by Ctrl+1
- [x] 6.2 Escape leaves Full Screen before Distraction Free (design decision 9). Verify with the escape-full-screen-first scenario (task 6.4), and that `distraction-free-escape` and `full-screen` still pass
- [x] 6.3 Help: the Word count row says selecting text in either view counts it; the Full screen row says Escape leaves full screen first, then Distraction Free. Keyboard Shortcuts: the Escape row reads "Leave full screen, then Distraction Free". README: the same wording in the Distraction Free feature and the shortcuts table, and the Word count feature says selecting text counts it. Verify by opening Help and Keyboard Shortcuts in the installed build
- [x] 6.4 Add `build-aux/end-to-end/escape-full-screen-first.sh` (distraction-free spec, "Leaving with Escape", "Both on": Ctrl+Shift+D, F11, Escape, `expect_action_state fullscreen false` and `distraction-free true`, Escape, `distraction-free false`). Verify with `./build-aux/end-to-end-tests.sh escape-full-screen-first` after installing

## 7. Checks and trying it

- [x] 7.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate --strict headings-and-word-count`, with no failures or warnings
- [x] 7.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh` and check it ends with "44 of 44 passed"
- [x] 7.3 Ask the owner to close every LeafMD window, reopen it and try what the tests can't: how the label looks in light and dark style, selecting text in the raw text and the formatted view (including both sides of Split view), the Headings page's look and indents, clicking headings in Formatted and Split view, the highlight while scrolling, and a narrow window

## Workflow follow-up

- `/ship` writes the release notes, commits, archives the change (which copies the spec changes into `openspec/specs/`) and opens the pull request.

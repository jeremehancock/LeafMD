# Tasks

## 1. Comparing versions

- [x] 1.1 Add `src/file_change.rs` with the function that compares the known version with what's on disk (same, changed, deleted or unknown), and add it to the file table in `docs/DEVELOPMENT.md`. Verify with unit tests covering: identical contents, different contents, missing file, known-missing with the file back, and a read error counting as "unknown", which changes nothing
- [x] 1.2 Store the known version in the window, updated by `load_file` and `write_to` (the bytes actually written). Verify with `cargo build` and `cargo test`

## 2. The banner

- [x] 2.1 Build the banner widgets in `src/file_change.rs` (sliding container, wrapping label, two buttons, tint) and add it as a top bar directly under the header. Verify it builds and stays hidden when opening a file
- [x] 2.2 Show the right wording and buttons for each situation in the spec (changed, changed with unsaved changes, deleted), refreshing when the unsaved state changes. Verify by typing while the banner shows: the button changes to Discard and Reload

## 3. Noticing changes

- [x] 3.1 Start and stop watching in `set_file`, with the 500 millisecond wait and the counter that throws away results from a previous file. Verify by running `echo hi >> notes.md` in a terminal and seeing one banner, and that running it after opening a different file shows nothing
- [x] 3.2 Skip checks during a save and run one afterwards. Verify that Ctrl+S never shows the banner, including when typing during the save
- [x] 3.3 Add a check to the "window became active" handler. Verify by changing a file while another window is active, and seeing the banner on returning
- [x] 3.4 Try apps that save in different ways: `sed -i` and `mv` over the file (swapping in a new file), `touch` (no banner), rewriting identical text (no banner), and `rm` followed quickly by recreating the same text (no banner). Fix any false alarms found

## 4. The choices

- [x] 4.1 Ignore: close the banner and record the version it was showing. Verify that another change brings the banner back, and that saving after Ignore doesn't ask
- [x] 4.2 Reload and Discard and Reload: replace the text, keeping the cursor and scroll position, with undo history cleared. Verify every "Reloading" scenario in the spec, including the shorter file, Ctrl+Z, a file that isn't plain text, and that the formatted view keeps its scroll position
- [x] 4.3 Save Again: write the file back through the normal save. Verify that the file exists again and the banner closes. Also verify the deleted-file scenarios in the spec, including a rename in the file manager
- [x] 4.4 Close the banner when the document closes, or on New, opening another file and Save As. Verify the "belongs to the open file" scenarios

## 5. Saving safely

- [x] 5.1 Add the check before saving and the "Replace the Other Version?" dialog, with Cancel as the default. Verify that Cancel leaves the file untouched and Save Anyway replaces it
- [x] 5.2 Verify saving while closing: at "Save Changes?" choose Save, then Cancel at the new dialog, and the window stays open with changes intact

## 6. Words for users

- [x] 6.1 Add this to Help (`show_help` in `src/window.rs`), and verify by reading it with F1. Keyboard Shortcuts doesn't change, as there are no new shortcuts
- [x] 6.2 Update the README features, the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml`, and `site/index.html`. Verify `build-aux/flathub-lint.sh` passes

## 7. Trying it for real

- [x] 7.1 In the installed Flatpak, open a file outside the home folder, change it from a terminal, and verify the banner appears at the latest when returning to the window, and that Save asks first. Check the banner fits at the smallest window size, in light and dark styles
- [x] 7.2 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh` and `openspec validate --specs --strict`, and verify all pass with no warnings
- [x] 7.3 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, verify it finishes, and ask the owner to close every LeafMD window, reopen it and try the change

## Workflow follow-up

- `/ship` ticks "Reload when the file changes" in `ROADMAP.md` when it archives this change, once the owner has tried it.

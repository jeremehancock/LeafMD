# Tasks

## 1. Which folder the panel shows

- [x] 1.1 Change `folder_to_show` in `src/folder_tree.rs` as design.md says: drop `showing` and `can_show`, return the file's own folder when there is a file (with "shared whole" true when it's the chosen folder or inside it), and the chosen folder or nothing when there isn't. Update its doc comment and `update_files_page` in `src/window.rs`. Verify: `cargo build`.
- [x] 1.2 Rewrite the `folder_to_show` unit tests: a file in a subfolder shows the subfolder, going back to ~/Desktop after the home folder was shown shows ~/Desktop, a file inside the chosen folder shows its own folder marked as shared whole, a document shared by itself shows its own folder (not the chosen one), and no file shows the chosen folder or nothing. Verify: `cargo test`.

## 2. Open Folder… and the up button

- [x] 2.1 In `ask_open_folder` (`src/window.rs`), after a folder is picked: use this window if it's empty, otherwise call `open_folder` like `leafmd .` does. Verify on the hidden desktop: with plan.md open, picking ~/notes opens a second window with the panel on ~/notes and plan.md's window unchanged; from an empty window, no new window opens; picking a folder another window already has brings that window forward.
- [x] 2.2 Add the up button as design.md says: `set_start_button` on `SidePanel` (`src/side_panel.rs`), shown only on the Files page; the `went_up` window field applied in `update_files_page` and cleared in `selected_tab_changed`; greyed out and its tooltip updated with the folder showing. Add unit tests for the "can the folder above be listed" check (in the Flatpak: stops above home and above a shared folder; outside it: stops at `/`). Verify: `cargo test`, and pictures on the hidden desktop of the spec's "Going up", "Going up twice", "Back to the file's folder" and "Headings" scenarios.

## 3. Checking it in the app

- [x] 3.1 On the hidden desktop, check the `folder-tree` spec's scenarios with a picture of each step: "Opening a file in a subfolder", "Going back to a file in a narrower folder", "Saving somewhere new", "Starting from a chosen folder", "A document outside the chosen folder", "A document shared by itself", "A new document with a chosen folder", "Moving a tab to a new window" and "A subfolder of a chosen folder outside the home folder".

## 4. End-to-end scenarios

- [x] 4.1 Add `build-aux/end-to-end/open-folder-new-window.sh`: with a file open, Open Folder… on a folder opens a second window (`expect_windows 2`). The folder picker is a file dialog, which the tests can't drive; if there's no way around it (for example running `leafmd <folder>` to check the shared window choice instead), say so in the scenario's comment and rely on the hidden-desktop check in 2.1. Verify: the scenario passes, or the reason it was left out is noted.
- [x] 4.2 If the test helpers can read the side panel's title, add `build-aux/end-to-end/files-panel-file-folder.sh` following `open-from-tree.sh`: open a file from a subfolder in the tree and check the panel moved into it, then switch back. If they can't, note that and rely on the unit tests and pictures. Verify: the scenario passes, or the reason is noted.
- [x] 4.3 Run the existing `open-folder` and `open-from-tree` scenarios. Verify: both pass unchanged.

## 5. Telling people about it

- [x] 5.1 Rewrite Help's "Which folder" row (`show_help` in `src/window.rs`): the panel always shows the folder of the document you're looking at and moves when you switch tabs; the up button at the top of the panel shows the folder above; Open Folder… (Ctrl+Shift+O) or `leafmd .` is for browsing a folder before opening anything in it, in a new window if this one already has a document. Keyboard Shortcuts keeps "Open folder" (no new shortcut). Verify: picture of Help.
- [x] 5.2 Update the side panel paragraph in README.md, `site/index.html` and the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` with the same plain wording (including the up button), where they describe which folder shows. Verify: `./build-aux/sdk-check.sh` passes (it validates the metainfo), and `build-aux/build-site.py` builds the site.
- [x] 5.3 In ROADMAP.md, replace the "Whether the Files panel should go back to a narrower folder" idea in "Ideas, not decided" with an unticked item in "Small fixes" saying the panel now always shows the selected file's folder and Open Folder… uses a new window when one is in use, with an up button, so `/ship` can tick it when archived. Verify: the roadmap reads correctly.

## 6. Ready for the owner

- [x] 6.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh` and `openspec validate --specs --strict`. Verify: all pass with no warnings.
- [x] 6.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh`. Verify: every scenario passes.

## Workflow follow-up

- The owner tries it in the installed Flatpak (close every LeafMD window and reopen): the ~/notes and ~/notes/docs example, the Desktop and home folder example, Open Folder… with a document open and the up button.
- `/ship` commits, picks the version with the owner, archives the change (copying the specs into `openspec/specs/`) and ticks the roadmap item.

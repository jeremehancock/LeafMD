# Tasks

## 1. Check the risky part first

- [x] 1.1 Install the current build and, in a folder under `/tmp` with two markdown files and a subfolder, check from a scratch build that LeafMD receives `leafmd .` as a portal folder, can list it with `gtk::DirectoryList`, can open a file inside it, and whether adding a file shows up without reopening. Verify by writing down the results (paths seen, what worked) and telling the owner before going further; if opening files inside the shared folder fails, stop and report. Remove the test portal entries afterwards with `flatpak document-unexport --doc-id ID`

## 2. The panel container

- [x] 2.1 Add `src/side_panel.rs` with `SidePanel`: its own `ToolbarView` and header bar, an `adw::ViewStack` of pages, a page switcher hidden while there's one page, `add_page` and a way for a page to set the title. Verify with `cargo build` and a temporary test page during development
- [x] 2.2 Wrap the window's existing `ToolbarView` in an `adw::OverlaySplitView` with the panel on the left at 240 pixels, add a breakpoint that collapses it below 920sp, and move the compact breakpoint from 639sp to 679sp. Measure the wide header bar with minimize and maximize buttons on and adjust both numbers if needed, updating the specs to match. Verify every scenario in "Compact layout in narrow windows" and "Tabs in a narrow window", including 360 pixels with 8 tabs
- [x] 2.3 Draw `leafmd-side-panel-symbolic` with filled shapes, list it in `data/resources/resources.gresource.xml`, and add the toggle button at the left end of the header bar (it stays there in the compact layout). Verify it looks right in light and dark style
- [x] 2.4 Add the `win.side-panel` toggle action with F9, the Side Panel checkbox item in its own main menu section, and the button bound to it. Verify the "Opening and closing the side panel" scenarios and the main menu scenarios
- [x] 2.5 Save and load `[side-panel] open` in `src/state.rs`, change it only on a deliberate toggle in a wide window, Open Folder… or a folder from the command line, and handle collapsing and expanding as `design.md` decision 3 says. Add unit tests for loading a missing or bad value. Verify with `cargo test` and every scenario in "The side panel stays open until closed" and "The side panel in narrow windows"

## 3. The folder tree

- [x] 3.1 Add `can_open(content_type)` in `src/folder_tree.rs` with unit tests for `text/markdown`, `text/plain`, `application/json`, `application/x-shellscript`, `text/x-rust`, `application/octet-stream`, an empty type (openable) and `image/png`, `application/pdf`, `application/zip`, `video/mp4`, `font/ttf` (greyed). Verify with `cargo test`
- [x] 3.2 Build the Files page: `DirectoryList` with `monitored` on, `TreeListModel` creating child lists only on expand, filtering out hidden entries, sorting folders first then names ignoring case, rows with `TreeExpander`, an icon and the name, greyed rows that can't be activated and show "LeafMD can't open this type of file" on hover. Draw `leafmd-folder-symbolic` and `leafmd-document-symbolic` and register them. For folders under the portal's mount, re-list the expanded folders when the window becomes active again. Verify the "What the tree lists", "Files LeafMD can't open are greyed out" and "The tree stays current" scenarios (including "A new file outside the home folder") on a real project folder, and that expanding a folder with thousands of files doesn't freeze the window
- [x] 3.3 Add the window's chosen folder and the rule for which folder to show (chosen, else the selected document's folder, else "No Folder Open" with an Open Folder… button), refreshing on tab changes. Show the folder's display name as the panel title. Verify the "Which folder the tree shows" scenarios and the "Pages in the side panel" scenario
- [x] 3.4 Open files from the tree through the existing `open_file` path with the window as `asked_from`, single click or Enter, tucking the panel away when it's sliding over. Verify every "Opening files from the tree" scenario and "Picking a file"
- [x] 3.5 Highlight the selected tab's file and expand the folders leading to it, on startup and on every tab change. Verify every "The tree follows the selected tab" scenario, including Move to New Window
- [x] 3.6 Add Open Folder… (Ctrl+Shift+O) using `gtk::FileDialog::select_folder`, in the main menu after Open…, starting in the tree's or document's folder, opening the panel and remembering it as open. (It was removed for a while during testing, then brought back at the owner's request.) Verify "Open Folder opens the panel" and choosing a folder outside the home folder
- [x] 3.7 Add the "can't see this folder" message as `design.md` decision 9 describes, with an Open Folder… button and a Learn More link to the README's "Folders outside your home folder" section. Verify "A document from outside the home folder", "Choosing the folder" and "Without the short command"
- [x] 3.8 Keep the chosen folder only while its window is open: don't save it with the tabs or restore it at startup (it was saved at first, then dropped at the owner's request during testing). Verify the "A chosen folder lasts as long as its window" scenario

## 4. Folders on the command line

- [x] 4.1 Add `choose_window_for_folder` to `src/opening.rs` with unit tests: a window that already has the folder, an empty window, a window with documents only, no windows. Verify with `cargo test`
- [x] 4.2 In `main.rs`'s open handler, split folders from files, open each folder through `choose_window_for_folder` with the panel shown and remembered as open, send files on the same command line to the last folder's window, and skip reopening earlier tabs. Verify every scenario in "Folders on the command line" with the real `leafmd` command, inside and outside the home folder

## 5. Everything users read

- [x] 5.1 Add a side panel section to `show_help`, and mention `leafmd .` in the "From a terminal" row. Verify against the "Help window" and "Terminal command in Help" scenarios
- [x] 5.2 Add F9 (under Views) and Ctrl+Shift+O (under Files) to `show_shortcuts`. Verify both rows show with their keys
- [x] 5.3 Update `README.md`: the side panel and folder tree in the features, F9 and Ctrl+Shift+O in the shortcuts table, `leafmd .` in the terminal section. Verify it matches the specs
- [x] 5.4 Add `src/side_panel.rs` and `src/folder_tree.rs` to the file table in `docs/DEVELOPMENT.md`. Verify both rows are there
- [x] 5.5 Add the side panel to the description in `data/io.github.jeremehancock.LeafMD.metainfo.xml`. Verify with `./build-aux/flathub-lint.sh`
- [x] 5.6 Add the side panel to the features and `leafmd .` to the terminal section in `site/index.html`. Verify the page reads correctly in a browser at desktop and phone widths
- [x] 5.7 Move the folder tree idea in `ROADMAP.md` from "Ideas, not decided" to an unticked item under "Reading and writing comfort", for `/ship` to tick. Verify the item is present and the old idea line is gone
- [x] 5.8 Retake the screenshots with `build-aux/take-screenshots.sh`, since the header bar gains a button, and check them against `data/screenshots/README.md`. Verify the panel button shows and nothing else moved

## 6. Checking the whole thing

- [x] 6.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate folder-side-panel --strict`. Verify all pass with no warnings
- [x] 6.2 Re-run the main scenarios of `tabs`, `windows` and `recent-files` with the panel open and closed (opening files, dragging tabs between windows, reopening tabs at startup, the compact layout). Verify nothing in those specs broke
- [x] 6.3 Rebuild and install with `./build-aux/flatpak-install.sh`, then ask the owner to close every LeafMD window, reopen it and try the change, including `leafmd .` inside and outside the home folder. Verify with the owner, and don't close their running LeafMD yourself

## Workflow follow-up

- `/ship` handles commits, the version and release notes, pushing, the pull request and archiving, once the owner has tried it.
- Release notes should mention that the compact layout now starts below 680 pixels.
- At release time, ask the owner whether the store listing should gain a screenshot showing the side panel. That would change the "App store listing" requirement, so it's its own decision.

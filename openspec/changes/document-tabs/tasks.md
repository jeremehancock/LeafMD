# Tasks

Commit nothing until the owner asks; `/ship` handles commits. "Standard checks" below means `cargo build`, `cargo test` and `cargo clippy --all-targets -- -D warnings` all pass with no warnings.

## 1. Split the document out of the window, with no visible change

- [x] 1.1 Create `src/document.rs` with a `Document` struct holding the per-document state from `design.md` (buffer, raw text view, web view, file, watcher, versions, render and scroll state, search context), each window still having exactly one; verify the standard checks pass
- [x] 1.2 Move rendering, preview messages, jumping, scroll sync, navigation decisions, formatting and typing signals into `Document`, with a replaceable weak link to its window for toasts, dialogs and the title; verify the standard checks pass and the moved code still has its plain English comments
- [x] 1.3 Move opening, saving, reloading, the file watcher and banner logic into `Document`; verify the standard checks and existing unit tests pass
- [x] 1.4 Change `FindBar::new` to stop taking a buffer, with each `Document` owning its search context (saving and loading each tab's find bar state comes in 4.2, when there's more than one tab to load it for); verify the standard checks pass
- [x] 1.5 Route the window's dark style listener to its document, so `Document` doesn't connect to the global style manager itself; verify the standard checks pass
- [x] 1.6 Add `src/document.rs` to the file table in `docs/DEVELOPMENT.md` and update the `src/window.rs` row; verify both rows describe the split
- [x] 1.7 Run `./build-aux/sdk-check.sh` and `./build-aux/flatpak-install.sh`, then ask the owner to try the views, find and replace, saving, the reload banner, links and the narrow layout; verify the owner finds no change in behaviour before continuing

## 2. Tabs in a window

- [x] 2.1 Add `adw::TabView` and `adw::TabBar` (autohide on) to the layout in `design.md`, so a window holds a list of documents and actions run on the selected one; verify with the standard checks and that one document looks the same as before
- [x] 2.2 Update the title, the save and view controls, the find bar, the banner and document action greying when the selected tab changes, and run the file check on selection; verify by hand with two tabs that each of the title, view switcher, find bar and banner follows the selected tab
- [x] 2.3 Make the existing New action open a tab, move its shortcut to Ctrl+T and update the button's tooltip, leaving the button's icon and the menu item's name as they are; verify Ctrl+T, the menu item and New in both the header and bottom bar open a selected Untitled tab in Split view without asking anything, and with nothing open give one document and no tab bar
- [x] 2.4 Give each tab its name, unsaved dot and full location tooltip, using the same display rules as the title; verify a tab shows "• notes.md" while unsaved and the tooltip gives the full path
- [x] 2.5 Hook the unsaved-changes check into `close-page`, add `win.close-tab` (Ctrl+W), keep Ctrl+Q on the window, and update `close-document` for last-tab behaviour; verify every scenario in "Closing a tab" and "Closing a document" by hand
- [x] 2.6 Add the "Closing a window with several unsaved tabs" dialog with tick boxes, folders and Save greyed out when nothing is ticked, stopping at the first failure or cancel with nothing discarded; verify each scenario under that requirement by hand
- [x] 2.7 Turn on the chosen tab shortcuts and add Ctrl+Alt+Page Up and Down, leaving Ctrl+Home and Ctrl+End to the raw text; verify next, previous, wrapping, Alt+number and Ctrl+Home by hand, with the focus in the raw text and in the formatted view
- [x] 2.8 Update the unsaved-changes wording in the Help text, and add a Tabs section to `show_help` and a Tabs group, Close tab, Close window, New document (Ctrl+T) and New window rows to `show_shortcuts`; verify both windows against the `help-and-about` spec
- [x] 2.9 Split the banner into working out its state (any document, any time) and drawing it (selected document only), per `design.md`, with its buttons acting on the selected document; verify the scenarios in "Every tab keeps watching its file" and "The banner belongs to the open file" by hand using a second app to change, delete and restore files in tabs that aren't selected, and that the `file_change.rs` unit tests still pass

## 3. Where files open

- [x] 3.1 Write the "which window and tab" decision as a plain function, with unit tests for every case in "Opening files from outside LeafMD", "Empty windows are reused" and "Already open files are brought forward"; verify with `cargo test`
- [x] 3.2 Keep the windows list in most recently used order, and pass it to each `Window`; verify with the standard checks and by hand that a file from the terminal goes to the window used last
- [x] 3.3 Send Open, Recent files, drops and links through the one route, including the "being opened" list, reusing an untouched Untitled tab, and removing the `open-in-new-window` action; verify every scenario in `windows` and `links`, and that a file that can't be opened adds no tab
- [x] 3.4 Send files from the file manager and terminal through the same route, so `leafmd a.md b.md` gives two tabs with b.md selected; verify with the scenarios in "Command line" and by opening the same file twice quickly
- [x] 3.5 Leave files open in the window out of its recent files list; verify the "The open file is left out" and "A file open in another window" scenarios by hand
- [x] 3.6 Update the "Files and Links" text in `show_help` for links, opening files and Close Document; verify it matches the specs and no longer mentions links opening a new window

## 4. Views and find per tab

- [ ] 4.1 Keep the view per `Document`, with new tabs starting in the window's last used view, and new documents in Split view; verify each scenario in "Each tab has its own view" by hand
- [ ] 4.2 Save and load each tab's find state, with a guard so loading doesn't count as typing, and window-wide search options; verify each scenario in "Each tab has its own find bar" by hand, including that the cursor doesn't move when switching back

## 5. New windows and moving tabs

- [ ] 5.1 Add the New Window action (Ctrl+N) and menu item, opening on the start page; verify the "New windows" scenarios by hand
- [ ] 5.2 Add the right-click tab menu with Move Left, Move Right, Move to New Window and Close, acting on the right-clicked tab, with Move Left and Right greyed out at the ends; verify each item by hand on a tab that isn't selected, and that dragging a tab along the tab bar changes the order as in the "Reordering" scenario
- [ ] 5.3 Connect `create-window` and `page-attached`, so dragging or moving a tab updates its window link, find options and toolbar state, and a window left with no tabs closes; verify that a moved tab's formatted view still draws, its links open, its messages appear in the new window, it still saves, and the "Moving a tab that has a banner" and "Changes after moving a tab" scenarios hold. If the page comes back wrong, redraw it after the move as `design.md` says
- [ ] 5.4 Add `--new-window` handling and the `should_detach` exception, with a unit test for the exception; verify with `cargo test` and by running `leafmd --new-window` with LeafMD open and closed
- [ ] 5.5 Add the `New Window` action to `data/io.github.jeremehancock.LeafMD.desktop`; verify with `desktop-file-validate`, `./build-aux/flathub-lint.sh` and, after installing, right-clicking the icon with LeafMD open and with it closed

## 6. Reopening tabs

- [ ] 6.1 Replace `file_to_reopen` in `src/state.rs` with a function returning the list of files and the selected one, plus saving and loading the `tabs` key; verify with unit tests for a missing key (older state file), an empty value, missing files, and a selected file that's gone
- [ ] 6.2 Save the tabs on close under the existing rules, and restore them at startup with the blank page until the selected one is ready; verify every scenario in "Reopening the last file" by hand, and that "No Document Open" never flashes
- [ ] 6.3 Hold off drawing the formatted view for unselected restored tabs until first selected; verify that a heading link, find and the banner still work in a tab selected for the first time

## 7. Everything users read

- [ ] 7.1 Update `README.md`: tabs in the features, links opening a tab, Close Document, the shortcuts table (Ctrl+T, Ctrl+N, Ctrl+W, Ctrl+Q and the tab keys); verify no line still says links open a new window or that Ctrl+W closes the window
- [ ] 7.2 Add tabs to the description in `data/io.github.jeremehancock.LeafMD.metainfo.xml`; verify with `./build-aux/flathub-lint.sh`
- [ ] 7.3 Add tabs to the storefront in `site/index.html`; verify the page reads correctly in a browser
- [ ] 7.4 Add tabs to `ROADMAP.md` as an unticked item, since it isn't there today, for `/ship` to tick; verify the item is present

## 8. Checking the whole thing

- [ ] 8.1 Run the standard checks, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate document-tabs --strict`; verify all pass
- [ ] 8.2 Test narrow windows at 360 and 500 pixels with 8 tabs, including with minimize and maximize buttons on and the find bar open; verify each scenario in "Tabs in a narrow window" that the compact layout scenarios in `windows` still hold, and that the unsaved tabs dialog fits and scrolls at 360 pixels with long file names and 8 unsaved tabs
- [ ] 8.3 Open 10 tabs, note the memory used, and tell the owner the number; verify switching stays quick
- [ ] 8.4 Rebuild and install with `./build-aux/flatpak-install.sh`, then ask the owner to close every LeafMD window, reopen it and try the change; verify with the owner, and don't close their running LeafMD yourself

## Workflow follow-up

- `/ship` handles commits, the version and release notes, pushing, the pull request and archiving, once the owner has tried it.
- At release time, ask the owner whether the screenshots should show tabs. With one document they're still accurate. Release notes should call out that Ctrl+N and Ctrl+W changed.
- On archive, update the Purpose lines in `openspec/specs/windows/spec.md` ("one document per window") and `openspec/specs/recent-files/spec.md` ("reopening the last file"), since archiving doesn't change them.

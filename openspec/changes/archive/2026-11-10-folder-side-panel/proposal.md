# Proposal

## Why

A user asked for a tree of the folder they're working in, so they can move between the markdown files in a project or notes folder without going back to Open each time. Running `leafmd .` in a terminal should open that folder the way VS Code does. The owner also wants the space the tree lives in to be a general side panel that later features (a headings list, for example) can use too.

## What Changes

- **A side panel on the left of each window.** It's closed by default. A new button at the left end of the header bar, the F9 key, or a new "Side Panel" item in the main menu opens and closes it.
- **Once opened, it stays open until the user closes it.** Switching tabs, opening new windows and restarting LeafMD all keep it open. Closing it is remembered the same way.
- **The panel is built to hold more than one page.** For now it has one page, Files. With only one page nothing extra shows; a small switcher appears at the top only once a second page exists.
- **The Files page shows a folder as a tree.** Folders expand and collapse. Every file shows, except hidden ones like `.git`. Files that are clearly not text (images, PDFs, archives, videos) are greyed out and can't be clicked. LeafMD judges this by the file's name only and never reads a file to decide. Clicking any other file opens it in a tab, following the existing tab rules. The tree updates by itself when files are added, removed or renamed.
- **Which folder the tree shows.** Each window can have a chosen folder, set by `leafmd .` (or any folder on the command line) or by a new "Open Folder…" item (Ctrl+Shift+O). The tree shows it while the selected document is inside it. Otherwise, or until one is chosen, the tree shows the folder of the selected tab's document, and changes as tabs change.
- **How it works with tabs.** The panel belongs to the window, not to a tab, so switching tabs never closes it. The selected tab's file is highlighted in the tree, with its folders expanded. Clicking a file that's already open selects its tab instead of opening it twice. Dragging a tab to another window doesn't take the folder with it.
- **`leafmd .` from the terminal** opens that folder with the panel showing, in a new window. A window showing "No Document Open" is reused instead, and a window that already has that folder is brought to the front. Files given on the same command line open as tabs in that window.
- **Folders LeafMD can't see.** When LeafMD can't see a folder (for example `flatpak run` without the short command, from `/tmp`), the panel says so, offers Open Folder… and links to an explanation, instead of showing the wrong contents. A document opened by itself from outside the home folder gets the same message, because LeafMD can't see the files next to it.
- **Narrow windows.** Below 920 pixels wide, the panel slides over the left of the document instead of pushing it over, and tucks away after a file is picked. It never appears there by itself. Widening the window puts it back beside the document if it was open.
- **The chosen folder lasts as long as its window.** Reopening LeafMD from the app grid brings the tabs back, and the tree follows the document.
- **BREAKING (layout):** the compact layout for narrow windows now starts below 680 pixels instead of 640, because the header bar gains a button and the wide layout no longer fits in 640.

New things added to the app:
- Button: side panel toggle, at the left end of the header bar
- Keyboard shortcuts: **F9** (side panel), **Ctrl+Shift+O** (Open Folder…)
- Main menu items: **Open Folder…** (with Open…) and **Side Panel** (a checkbox, in a new section)
- No new settings. The panel's open or closed state is remembered, the way the view and window size are today. A window's folder isn't.

### Out of scope

- Creating, renaming, moving or deleting files from the tree, and right-click menus on it
- Dragging files out of the tree, or dropping them onto it
- Making the panel wider or narrower by dragging its edge (it has a fixed width)
- Showing hidden files, or a setting to choose which files show
- Pages for the panel other than Files (it's only built to allow them)
- Registering LeafMD as an app for folders in the file manager (so "Open With LeafMD" on a folder isn't added)
- Showing which files have unsaved changes in the tree
- A "Close Folder" action. Choosing another folder with Open Folder… replaces it, and a new window starts with none.

## Capabilities

### New Capabilities
- `side-panel`: the panel on the left of the window: opening and closing it, remembering that, how it behaves in narrow windows, and how it holds pages.
- `folder-tree`: the Files page: which folder it shows, what's listed and greyed out, opening files from it, how it follows tabs, and folders LeafMD can't see.

### Modified Capabilities
- `desktop-integration`: the command line accepts folders (`leafmd .`), with rules for which window a folder opens in.
- `windows`: the compact layout starts below 680 pixels instead of 640, and the panel button stays in the header bar.
- `tabs`: the narrow window rule's width moves from 640 to 680 pixels, to match.
- `help-and-about`: the main menu gains Open Folder… and Side Panel; the Help window covers the side panel; the "From a terminal" row mentions `leafmd .`.

## Impact

- **Code:** `src/window.rs` (layout gains a split view, header button, actions, menu, Help and Keyboard Shortcuts), `src/main.rs` (folders on the command line, new shortcuts), `src/opening.rs` (which window a folder goes to), `src/state.rs` (remembering whether the panel is open, and the window's folder, in state.ini). New files: `src/side_panel.rs` (the panel and its pages) and `src/folder_tree.rs` (the Files page).
- **Icons:** new LeafMD icons for the panel button and the tree's folders and files, in `data/resources/icons`.
- **Permissions:** none new. Folders outside the home folder come through the same file-sharing route (the document portal) that single files use today.
- **Docs and listing:** Help, Keyboard Shortcuts, README, metainfo description, leafmd.app, screenshots (the header bar gains a button), docs/DEVELOPMENT.md file table, ROADMAP.md.

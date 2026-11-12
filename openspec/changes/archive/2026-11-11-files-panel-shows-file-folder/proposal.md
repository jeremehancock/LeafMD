# Proposal

## Why

Today the Files panel follows two rules that surprise people:

- It only moves when the selected file isn't inside the folder it already shows, so it never comes back down to a narrower folder. Example: a file on the Desktop is selected and the panel shows the Desktop. Saving another tab's document in the home folder moves the panel to the home folder. Switching back to the Desktop file leaves the panel on the home folder, because the Desktop is inside home.
- A folder picked with Open Folder… only shows while the selected file is inside it. With a file from somewhere else open, picking a folder seems to do nothing.

The owner's expectation is simple: when you're viewing a file, the panel shows the folder that file is in, with its contents and the file highlighted, however things were opened. LeafMD has no real idea of a "project folder", so a picked folder shouldn't act like one.

## What Changes

- **The panel always shows the selected file's own folder.** Switching tabs, opening a file any way (from the panel too) and saving somewhere new all put the panel on the folder of the file now selected. **Changes today's behaviour:** clicking ~/notes/docs/setup.md in the panel moves the panel to ~/notes/docs. Today it stays on ~/notes, and today a picked folder also stays for every file inside it.
- **A picked folder is where to start, not where to stay.** A folder picked with Open Folder… (Ctrl+Shift+O) or `leafmd .` shows while the window has no document, or while a new unsaved document is selected. Once a file is selected, the panel follows the file.
- **Open Folder… with a document open opens a new window.** If the window already has a document open, Open Folder… opens the folder in a new window, the way `leafmd .` already does, so the panel never shows a folder unrelated to the document beside it. If the window is empty, the folder opens in it. A window that already has that folder comes to the front instead, as with `leafmd .`.
- **An up button in the panel.** Because the panel now moves into subfolders, a button at the top of the panel, left of the folder's name, shows the folder above, with the folder you came from expanded and your file still highlighted. Its tooltip names the folder it goes to, like "Show the folder above (notes)". It's a temporary look around: switching tabs, opening a file or saving somewhere new puts the panel back on the selected file's folder. It's greyed out when the folder above can't be listed (above the home folder inside the Flatpak, or above a folder shared from outside home). It shows only on the Files page, and has no keyboard shortcut.
- **Documentation says it plainly.** Help's "Which folder" row, README, leafmd.app and the app store description say the panel always shows the folder of the document you're looking at, and that Open Folder… is for browsing a folder before opening anything, in a new window if one is already in use.
- **A document opened by itself from outside the home folder** (like /tmp/a.md) now shows the panel's existing message that LeafMD was given this one document, with Open Folder… and Learn More, even when the window has a picked folder. Today the picked folder stays showing instead, which is the same mismatch as above.
- Unchanged: a new, unsaved document with no picked folder shows "No Folder Open" (the small fix already on `dev`). `leafmd .` picks its window exactly as today.

One new button (up a folder, on the Files page). No new keyboard shortcuts, menu items or settings. Open Folder… keeps its name and Ctrl+Shift+O.

### Out of scope

- A keyboard shortcut for the up button. Alt+Up, the file manager's shortcut, may clash with the editor; it can be added later.
- A hint on the empty page in a window opened by Open Folder… (considered, and dropped by the owner as more confusing than helpful). The empty page keeps saying "Open a markdown file, or drop one here".
- A tooltip on the panel's title showing the folder's full location (considered, and dropped as not needed: the up button's tooltip names the folder above).
- Remembering which folders were expanded when the panel comes back to a folder. As today, it expands the folders leading to the highlighted file.
- A message saying a new window was opened. The new window appearing, with the folder in its panel, says that already.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `folder-tree`: "Which folder the tree shows" becomes "the selected file's own folder; a picked folder only with no file", and "The tree follows the selected tab" gets its "Switching tabs" scenario updated to match. New requirements: Open Folder… with a document open uses a new window and the up button.

## Impact

- `src/folder_tree.rs`: `folder_to_show` drops its `showing` input. With a file, it always returns the file's own folder. Its tests change to match. A folder inside a picked folder outside the home folder (like /tmp/notes/sub after `leafmd .` in /tmp/notes) must still count as one LeafMD can list.
- `src/window.rs`: `update_files_page` stops passing what the panel shows now; `ask_open_folder` uses this window only if it's empty, else the same window choice as `leafmd .`; the Files page can show a folder above the file's for a while.
- `src/side_panel.rs`: room for a button at the left of the panel's header bar, shown only with the Files page.
- Help (`show_help`), README.md, `site/index.html`, `data/io.github.jeremehancock.LeafMD.metainfo.xml`, ROADMAP.md, the `folder-tree` spec, and end-to-end tests.

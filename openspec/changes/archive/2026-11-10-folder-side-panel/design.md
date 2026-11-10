# Design

## Context

See proposal.md for why. What shapes the approach today:

- Each window (`src/window.rs`) is one `adw::ToolbarView`: the header bar, tab bar, file banner, formatting bar and find bar on top, the documents in the middle, and a bottom bar that only shows in the compact layout.
- The compact layout is an `adw::Breakpoint` at 639sp. A comment there records that the wide header bar needs about 614 pixels with minimize and maximize buttons, so it only just fits in 640. One more button (about 40 pixels) doesn't fit.
- Files from the command line arrive through GTK's "open" signal in `src/main.rs`, already turned into `gio::File`s. A folder arrives the same way. Today it fails when LeafMD tries to read it as text.
- The `leafmd` short command runs `flatpak run --file-forwarding ... @@ "$@" @@`. Tested on the owner's laptop: a folder between the `@@` markers is shared through the document portal (the Flatpak service that hands a sandboxed app access to a file or folder), and inside the home folder `.` stays a plain path.
- Inside the sandbox, `/tmp` and some other folders are the sandbox's own, not the real ones, so a plain path outside the home folder can list the wrong contents without any error.
- `src/state.rs` keeps small settings in `~/.config/leafmd/state.ini`, and saves the tabs of the window closed last.
- The rule for which window a file goes to lives in `src/opening.rs` as a plain function with tests.

## Goals / Non-Goals

**Goals:**
- A panel container that a later feature can add a page to with a few lines, without touching the layout again.
- A tree that stays quick in large folders (a project with `node_modules` or `target`), by never listing a folder until it's expanded and never reading files.
- Keep every existing window, tab and compact-layout rule working, apart from the compact layout's new width.

**Non-Goals:**
- A general file manager (see the proposal's out of scope list).
- Changing how documents are read or saved.

## Decisions

### 1. The panel is an `adw::OverlaySplitView` around the existing layout

The window's content becomes an `adw::OverlaySplitView`: the sidebar is the new panel, and the content is today's `ToolbarView`, unchanged. This is libadwaita's own widget for exactly this (GNOME Files uses it). It already does the two behaviours we need: sitting beside the content, or sliding over it when "collapsed", with Escape and a click on the content closing it in the collapsed state.

The panel gets its own small `ToolbarView` with a header bar, so its title (the folder's name) lines up with the window's header bar. libadwaita splits the window buttons between the two header bars by itself.

*Alternative:* a `gtk::Paned` (a draggable divider). It would give a resizable panel, but no sliding-over mode for narrow windows, and we'd have to build the narrow behaviour by hand. Rejected; resizing is out of scope.

### 2. Widths: panel 240, side by side from 920, compact layout below 680

- The panel is a fixed 240 pixels wide.
- A new breakpoint collapses the split view below 920sp (so the panel slides over). At 920 the document keeps 680 pixels.
- The compact layout's breakpoint moves from 639sp to 679sp, so the header bar with its new button always has room. Raising it keeps one rule ("below this width the buttons move") rather than adding a second compact state for "panel open".
- The exact numbers are checked during implementation against a window with minimize and maximize buttons (the tightest case). If measurement says 680 is too tight, the number moves and the specs move with it.

*Alternative:* move the panel button into the main menu only, keeping 640. Rejected: the owner wants the panel easy to open, and a header button is how GNOME apps do it.

### 3. Remembering "open": one value, changed only by a deliberate choice

state.ini gets `[side-panel] open=true|false`. It changes when the user toggles the panel in a wide window, uses Open Folder…, or runs `leafmd` with a folder. Toggling while the panel is sliding over a narrow window doesn't change it, because there the panel is a temporary overlay that tucks itself away after each use.

Each window has its own "shown" state, starting from the saved value. So toggling in one window changes what new windows and the next start get, but not other windows already open (spec: "Two windows").

When the split view collapses, the window hides the panel. When it expands again, the panel shows if the window's own state says open.

### 4. The panel holds pages through a small `SidePanel` type (`src/side_panel.rs`)

`SidePanel` owns the panel's `ToolbarView`, an `adw::ViewStack` for pages and an `adw::ViewSwitcher` that's hidden while there's only one page. A page is added with a name, a title and a widget, and can set the panel's title (Files sets the folder's name). The window only talks to `SidePanel`, so a later "Headings" page is a new file plus one `add_page` call.

### 5. The tree is a `gtk::ListView` over `gtk::DirectoryList` (`src/folder_tree.rs`)

- `gtk::DirectoryList` lists a folder in the background and, with `monitored` on, keeps itself current when files change (spec: "The tree stays current").
- `gtk::TreeListModel` turns those lists into a tree, creating a folder's child list only when the user expands it. That's what keeps big folders cheap: `node_modules` is never listed unless someone opens it.
- A `gtk::FilterListModel` removes hidden entries (GIO's `standard::is-hidden`, which covers names starting with a dot), and a `gtk::SortListModel` puts folders first, then names alphabetically ignoring case.
- Each row is a `gtk::TreeExpander` (GTK's own indent and arrow, which also gives Right and Left to expand and collapse) holding an icon and the name.
- Single click opens a file (`single-click-activate`), and Enter does the same, as in the spec.

*Alternative:* the older `gtk::TreeView`. It's deprecated in GTK 4.10 and later, so rejected.

### 6. Greying out by name only

`DirectoryList` asks GIO for `standard::fast-content-type`, a type guessed only from the file name, with no reading. A small plain function, `can_open(content_type)`, decides:

- Openable: anything GIO counts as a kind of `text/plain` (which includes markdown, JSON, shell scripts, source code), plus unknown types (`application/octet-stream`, or nothing), such as `LICENSE` or `build`.
- Greyed out: everything else that GIO recognises (`image/*`, `audio/*`, `video/*`, `application/pdf`, zip and other archives, fonts, office documents).

An unknown file that turns out not to be text gets LeafMD's existing "This file isn't plain text" message when clicked. `can_open` has its own tests with real type names.

### 7. Which folder the tree shows: one field per window

Each `Window` gets `folder: RefCell<Option<gio::File>>`, the chosen folder. The Files page asks the window for "the folder to show": the chosen folder if the selected document is inside it (or there's no document), otherwise the folder it's already showing if the selected document is inside it, otherwise the selected document's parent folder, otherwise none. (Testing showed that always using the document's own folder made the tree jump into a subfolder whenever a file there was opened from the tree; the owner chose this instead. At first the chosen folder showed whatever tab was selected, as VS Code's Explorer does, but without VS Code's Open Editors list that left the tree showing a folder unrelated to the tab, so the owner chose to follow documents outside it.) A document LeafMD was given by itself, whose folder can't be listed, leaves the chosen folder showing. The page remembers which folders were expanded in the chosen folder while it shows another, and expands them again on the way back. The rule is `folder_tree::folder_to_show`, with unit tests. The window tells the page to refresh when the chosen folder changes or the selected tab changes. Highlighting the selected file uses the same tab-change signal: the page walks down from the root, expanding each folder on the way to the file, then selects its row.

### 8. Folders on the command line

In `main.rs`'s open handler, each `gio::File` is checked with `query_file_type`. Folders and files are split. For each folder, a new plain function in `opening.rs`, `choose_window_for_folder`, picks the window: one that already has that chosen folder, else an empty one, else a new one (with tests, like `choose_window`). Files from the same command line then open in the last folder's window, using the existing queue. Starting LeafMD with a folder skips reopening the previous tabs, the same way starting with a file does.

### 9. Spotting folders LeafMD can't see

Inside the Flatpak (when `/.flatpak-info` exists), a folder is trusted only if it's inside the home folder or under the document portal's mount (`/run/user/<id>/doc/`). Anything else may be the sandbox's own copy, so the page shows the "can't see this folder" message with Open Folder…. Outside the Flatpak (`cargo run`), every folder is trusted. A document opened alone from outside home has a portal path whose folder only contains that one file, so it hits the same check: its parent folder is the portal's per-file folder (`/run/user/<id>/doc/<doc-id>`), which the page recognises and shows the message. A folder shared whole is one level deeper (`/run/user/<id>/doc/<doc-id>/notes`), so it and its subfolders can be listed even when they weren't chosen in this window, for example when a document from them is selected. (At first any unchosen portal folder was treated as unlistable, which kept the tree from following a document in a shared folder.)

Open Folder… uses `gtk::FileDialog::select_folder`, and the `leafmd` command uses `--file-forwarding`. Both go through the portal, so a folder anywhere is shared properly. No new Flatpak permission is needed.

The panel's title shows the folder's display name (`standard::display-name`), never a `/run/user/...` path.

### 10. The chosen folder isn't saved

A window's chosen folder lives only in memory, for as long as the window is open. Reopening LeafMD from the app grid brings back the tabs but not the folder, so the tree follows the document. It was saved with the tabs at first (`[recent] folder`), but in testing the owner found that left the panel stuck on the folder of the last `leafmd .`, even when none of the reopened documents came from it. To get a folder back, use Open Folder… or run `leafmd .` again.

### 11. Icons

New LeafMD icons, drawn with filled shapes only and listed in `data/resources/resources.gresource.xml`: `leafmd-side-panel-symbolic` (the button), `leafmd-folder-symbolic` and `leafmd-document-symbolic` (tree rows). The expander arrows are GTK's own, which the "Same icons on every distro" rule already allows.

## Risks / Trade-offs

- [Opening files inside a folder shared through the portal] → Tested on 2026-11-09 with the installed Flatpak and `--file-forwarding @@ . @@` from a folder under `/tmp`: the folder arrives as `/run/user/1000/doc/<id>/notes`, and listing it, listing a subfolder, reading files and saving (write a copy, then rename over the file) all work.
- [No change notices for portal-shared folders] → Confirmed in the same test: a file added or renamed from outside the sandbox sent no notice through the portal path, while the same check on a folder inside the home folder did. The owner chose: when the window becomes active again, the Files page re-lists every expanded folder that's under the portal's mount, so changes made in another app show up on return. Folders inside home keep live notices.
- [The 680 and 920 pixel numbers are estimates] → Measured during implementation; the specs are updated to match before the owner tries it.
- [A folder like `node_modules` with tens of thousands of files is slow to list if someone expands it] → It's listed in the background, so the window doesn't freeze; rows appear as they arrive.
- [The window's header bar layout changes, so screenshots and the "wide windows don't change" promise change] → Screenshots are retaken; the spec now says exactly what changed.
- [Portal entries build up: each `leafmd .` outside home adds one to `flatpak documents`] → Same as files today. Not addressed here.

## Migration Plan

No data migration. Old state.ini files have no `[side-panel]` group, so the panel starts closed, as the spec asks. An older LeafMD reading a newer state.ini ignores the new keys.

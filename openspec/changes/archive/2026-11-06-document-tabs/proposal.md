# Proposal

## Why

LeafMD shows one document per window, so following a link or opening a second file fills the desktop with windows or replaces what you were reading. Tabs, working the way they do in gedit, let one window hold everything you're working on, while still allowing a second window when you want one.

## What Changes

**Tabs**
- A LeafMD window can hold several documents, each in its own tab. The tab bar sits under the header bar and only appears when a window has two or more tabs, as in gedit, so someone with one document sees no change.
- In narrow windows the tab bar keeps its own row. Tabs get smaller and then scroll sideways, down to the 360 pixel minimum.
- Tabs can be dragged to reorder them, moved to another LeafMD window, or pulled out into a new window.
- Each tab keeps its own text, undo history, unsaved changes, view (Formatted, Raw or Split), find bar and "changed by another app" banner.

**Where files open**
- Open, Recent files, dropped files and links to text files open in a new tab, which becomes the selected one, and a link to a heading scrolls to it. If the current tab is an empty, untouched document, the file takes its place. **BREAKING:** these no longer replace the current document, and links no longer open a new window.
- A file opened from outside LeafMD (file manager or terminal):
  - LeafMD not running, or a window with no document: opens as it does today.
  - LeafMD open with a document: opens as a new, selected tab in that window.
  - Several windows: an empty window if there is one, otherwise the window used most recently.
- `leafmd a.md b.md` opens both as tabs in one window. **BREAKING:** today each gets its own window.
- A file that's already open is never opened twice. LeafMD switches to its tab, in whichever window it's in.

**Closing**
- **BREAKING:** Ctrl+W closes the current tab instead of the window. Closing the last tab closes the window, so one-document use feels the same as today.
- Ctrl+Q closes the whole window, with all its tabs.
- Close Document in the main menu closes the current tab, and on the last tab leaves the window open on the "No Document Open" page.
- Closing a window with more than one unsaved tab shows one dialog, as in gedit, listing them with tick boxes so you choose which to save.

**Files changed by other apps**
- The "changed by another app" banner keeps working in every tab. Each tab keeps watching its file while it isn't selected, and a change shows in that tab's banner when you select it. Moving a tab to another window keeps its banner.

**Reopening**
- Starting LeafMD from the app grid reopens every saved file from the window closed last, in the same order, with the same tab selected. Never-saved documents aren't restored, which is why closing them still warns.

**New keyboard shortcuts, menu items and options**
- Ctrl+T: new document, in a new tab. The New button and the New menu item do the same. With nothing open, the new document is simply the window's only document, with no tab bar.
- **BREAKING:** Ctrl+N now opens a new window, matching gedit. It used to start a new document.
- Next and previous tab: Ctrl+Tab and Ctrl+Shift+Tab, Ctrl+Page Down and Ctrl+Page Up, and gedit's Ctrl+Alt+Page Down and Ctrl+Alt+Page Up.
- Alt+1 to Alt+9: go to a tab by its position.
- Main menu: "New Window" is the only new item. "New" keeps its name, as it does in gedit.
- Right-clicking a tab offers: Move Left, Move Right, Move to New Window, and Close.
- The app launcher's right-click menu gains a "New Window" item. It's listed in LeafMD's `.desktop` file, which has no such item today.
- A new command line option, `--new-window`, which the launcher item uses.
- No new settings.

**Out of scope**
- Tab groups (gedit's split of one window into side-by-side sets of tabs).
- Close All (gedit's Ctrl+Shift+W) and reopening a closed tab (gedit's Ctrl+Shift+T).
- gedit's setting for when the tab bar shows. LeafMD always uses the automatic behaviour, since it aims to need no settings.
- A separate new tab button or icon, on the tab bar or anywhere else. The existing New button does this.
- A mark on a tab that isn't selected to say its file changed. The banner appears when the tab is selected.
- Restoring never-saved documents, or several windows, when LeafMD starts.
- Remembering each tab's view, cursor or scroll position after LeafMD is closed.
- Stopping Save As from saving over a file that's open in another tab. LeafMD already allows this today across windows, and the other tab shows its usual "changed by another app" banner.

## Capabilities

### New Capabilities
- `tabs`: the tab bar, moving between tabs, closing and rearranging them, and how they fit narrow windows.

### Modified Capabilities
- `windows`: replaces "one document per window" with the rules for where a file opens; adds New Window; Ctrl+W no longer closes the window when other tabs are open.
- `documents`: the New button and Ctrl+T open a tab; opening a file no longer needs an unsaved-changes check; the title and banner follow the current tab; Close Document and the closing dialog change.
- `links`: links to text files open in a new tab, not a new window.
- `recent-files`: choosing a recent file opens a tab; reopening at startup brings back all saved tabs.
- `views`: each tab has its own view; a new tab starts in the last view used.
- `find-replace`: each tab has its own find bar.
- `help-and-about`: the main menu, Help and Keyboard Shortcuts cover tabs and the changed shortcuts.
- `desktop-integration`: the launcher's New Window item, and several files on the command line opening as tabs.

## Impact

- **Code:** most of `src/window.rs` (2850 lines) assumes one document per window. The per-document parts move into a new `src/document.rs`. `src/main.rs` (choosing where a file opens, `--new-window`) and `src/state.rs` (the list of tabs to reopen) also change, and `src/find.rs` gets a small change.
- **Dependencies:** none new. Tabs use libadwaita's `TabView` and `TabBar`, which LeafMD's libadwaita already includes.
- **Data:** the saved state gains a list of tabs. Existing state files keep working: the current last file simply opens as a single tab.
- **Memory:** each tab has its own formatted view, so each open tab uses more memory than a tab in an app with no web view.
- **User-facing text:** Help, Keyboard Shortcuts, README, the app store description, the storefront, and `docs/DEVELOPMENT.md`. `ROADMAP.md` gains this item, since it isn't listed today.
- **Screenshots:** with one document LeafMD looks the same, so the existing four stay accurate. Whether to show tabs in them is decided at release time.

# Proposal

## Why

Once a document has been opened in LeafMD, there's no way back to the "No Document Open" page. No command closes a document while keeping the window open, closing the window just brings the document back at the next start, and Clear Recent Files doesn't help: it keeps a separate record of the last file, and it isn't even offered when the only recent file is the one that's open.

Separately, the Keyboard Shortcuts window has a search box that Help and the Markdown Cheatsheet don't have. The list is short enough not to need one, and the box sits too close to the top of the window.

## What Changes

- **New menu item: Close Document**, in the main menu's first section (New, Open…, Close Document). It closes the document after the usual unsaved-changes check and shows the "No Document Open" page in the same window. Greyed out when no document is open. LeafMD then starts on that page next time too, unless another file is closed later.
- **No new keyboard shortcut.** Ctrl+W and Ctrl+Q still close the window. Close Document is reached from the menu.
- **No new settings.**
- **Clear Recent Files also forgets the last file**, so LeafMD starts on the "No Document Open" page next time. If a file is open when you clear, it stays open, but closing its window won't bring it back at the next start, unless you save it again (saving puts a file back on the list).
- **Clear Recent Files appears whenever the list has anything in it**, including when the only entry is the open file or a file that's currently hidden. The menu then shows the greyed-out "No Recent Files" with Clear Recent Files under it.
- **The Keyboard Shortcuts window has no search box.** It is rebuilt in the same plain style as Help and the Cheatsheet: a titled group per section, one row per shortcut, the keys on the right as plain text like "Ctrl+1". This changes how the keys look, since they are no longer drawn as small key-shaped boxes.
- **The shortcuts list is corrected:** "Close" becomes "Close window" and also lists Ctrl+Q, which was missing.

### Out of scope

- A setting to turn off reopening the last file. Close Document and Clear Recent Files cover this without a preferences window.
- Changing what Ctrl+W or Ctrl+Q do.
- Search in Help or the Cheatsheet.
- What happens when a file is opened from the file manager or a link. A window showing the start page is reused, as it already is.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `documents`: adds Close Document, and the start page's greyed-out actions now include it.
- `recent-files`: Clear Recent Files appears in more cases and also forgets the last file; closing a document or clearing the list changes which file reopens at the next start.
- `help-and-about`: the main menu gains Close Document; the Keyboard Shortcuts window has no search box and lists Ctrl+Q.

## Impact

- **Code:** `src/window.rs` (the new action and menu item, the recent list menu, the shortcuts window, what a closing window records) and `src/state.rs` (telling "nothing to reopen" apart from "not known yet").
- **Saved data:** the existing last-file record gains one new meaning, stored as an empty value. Existing users' saved files keep working, and nothing needs converting.
- **Docs:** Help, README, the app store description if it applies, and the three specs above.
- **Screenshots:** this change doesn't affect them, since they don't show menus or the shortcuts window. The header bar icon work already committed on `dev` does.
- **Dependencies:** none added. The dialog from libadwaita that provides the search box is no longer used.

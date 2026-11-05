# Design

## Context

See proposal.md for why. The details that shape the approach:

- `adw::ShortcutsDialog`, which draws the shortcuts window today, only offers "create" and "add a section". Its search box is built in and has no off switch.
- Help and the Cheatsheet are plain dialogs LeafMD builds itself, from a header bar and a scrolling page of titled groups and rows.
- At startup, `open_last_file` uses the saved `last` value, and falls back to the newest recent file if there isn't one. An empty value counts as "there isn't one" today, so simply blanking `last` would reopen the newest recent file instead of showing the start page.
- A window records its file as `last` when it closes. That is why clearing the list wouldn't be enough while a file is open: closing the window would record the file again.
- The recent menu counts only the files it shows, which is why Clear Recent Files disappears when the open file is the only entry.

## Goals / Non-Goals

**Goals:**

- A way back to the start page that works no matter what's on the recent list.
- Clear Recent Files doing what its name suggests, including while a file is open.
- A shortcuts window with no search box that matches Help and the Cheatsheet.

**Non-Goals:**

- A shared helper for the three dialogs. They already repeat a few lines of setup, and this doesn't need to change that.

## Decisions

### Build the shortcuts window ourselves, rather than add space above the search box

The owner's first choice is no search, with more space as the fallback. Since search can't be switched off, there were three ways to go:

- **Build our own dialog (chosen).** It's the same pattern as Help, it removes the search for real, and it needs no new dependencies.
- **Find the dialog's internal search box and hide it.** Rejected: this relies on libadwaita's private layout, which can change in any GNOME runtime update and would silently bring the search back or break the window.
- **Add space with custom styling.** Rejected for the same reason, and it would keep the search.

Each row is an `adw::ActionRow` with the keys as a dimmed label on its right.

### Show keys as hand-written text

Rows store their keys as text ("Ctrl+1", "Ctrl+W or Ctrl+Q"), written the same way as tooltips and the README. GTK's ready-made key-drawing widget, `gtk::ShortcutLabel`, belongs to the older shortcuts system that GTK has been phasing out, and it can't show "or" between two shortcuts. The cost is that this text is kept in step with `set_shortcuts` in `src/main.rs` by hand, and the spec already requires a new shortcut to be added here.

### No keyboard shortcut for Close Document

Ctrl+W already means "close the window" here and in most apps, and changing it would surprise people. Ctrl+Shift+W is often used for "close all windows". A menu item adds the least, and a shortcut is easy to add later if it's missed.

### An empty `last` value means "start on the start page"

`state.rs` will tell three cases apart: no `last` value (not known, so fall back to the newest recent file, as today), a file (reopen it), and an empty value (reopen nothing). Close Document and Clear Recent Files write the empty value. No conversion is needed, since older saved data never has an empty value.

The decision will be a small function with no GTK in it, so `cargo test` can check it directly: given the saved value and the recent list, which file should open?

### A closing window records its file only if it's on the recent list

This is what makes clearing work while a file is open: the list is empty, so closing the window records nothing and the empty value stays. Saving puts the file back on the list, so it's remembered again from then on. The window reads the saved list as it closes, since another window may have cleared it.

Plain Save (Ctrl+S) didn't add a file to the recent list before, only Save As did, although the existing spec says "opened or saved". It now does, which brings the code in line with that rule and makes the "Saving after clearing" scenario work.

Alternative rejected: a "don't remember this window" flag on each window, which would need passing between windows when one clears the list.

### Close Document reuses the existing pieces

After `confirm_discard` (the existing unsaved-changes check), it empties the text with `set_text("")`, clears the file with `set_file(None)`, closes the find bar, shows the `empty` page, greys out the document actions, refreshes the recent menu so the closed file appears there, and writes the empty `last` value. `close-document` joins `DOCUMENT_ACTIONS`, the list of actions greyed out on the start page.

## Risks / Trade-offs

- **A file pushed off the 10-item recent list isn't recorded when its window closes.** If someone opens 10 other files in other windows, then closes that older window last, a different file reopens, or the start page shows. → This is rare and harmless, so it's accepted.
- **"No Recent Files" above "Clear Recent Files" reads a little oddly** when the open file is the only entry. → Accepted: it's accurate, since nothing else is available to reopen, and the owner can judge it when trying the change.
- **Keys are plain text rather than key-shaped boxes,** so the window looks a little different from other GNOME apps. → This matches Help and the Cheatsheet, and the owner can judge it when trying the change.
- **The hand-written key text could fall out of step with the real shortcuts.** → The spec and the project's "every user-visible change" checklist already require updating this window.

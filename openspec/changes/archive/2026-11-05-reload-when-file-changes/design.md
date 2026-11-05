# Design

## Context

See proposal.md for why. What the code does today:

- `load_file` in `src/window.rs` reads the file and fills the editor. `write_to` saves it using GIO's "write a temporary file, then swap it in" call, and passes no check on what's already there. Nothing watches the file afterwards.
- `set_file` is the one place every path goes through when the window's file changes (open, new, Close Document, Save As), so it's the natural place to start and stop watching.
- The window's top bars are stacked in an `adw::ToolbarView`: header, formatting bar, find bar.
- The Flatpak has `--filesystem=home`. Files outside the home folder reach LeafMD through the document portal (Flatpak's way of granting access to a single file), which is a special filesystem that may not report changes made by other apps.

## Goals / Non-Goals

**Goals:**

- Never overwrite another app's changes without the user knowing, even when the change notice never arrives.
- Never show a banner when nothing really changed.

**Non-Goals:**

- Closing the very small gap between Save's check and its write. See Risks.

## Decisions

### Compare contents, not timestamps

LeafMD keeps a copy of the file's bytes as it last knew them (the "known version"), updated when it opens or saves a file, reloads, or the user chooses Ignore. Every check reads the file and compares it with that copy.

This one rule covers most of the hard cases. LeafMD's own save matches, so no banner. An app that rewrites identical text matches, so no banner. A change that gets undone matches again, so the banner closes.

*Alternative considered:* timestamps or a "we just saved" flag, using GIO's built-in file tags (etags). They're cheaper, but they raise false alarms for identical rewrites, can't tell when a change was undone, and can miss two changes in quick succession. Markdown files are small, so reading them is not a real cost.

The copy holds the bytes on disk rather than the editor's text, because the editor can differ from disk (for example, while there are unsaved changes).

### Keep the comparison logic free of GTK

`src/file_change.rs` has a small function that takes the known version and what's on disk now (contents, or missing), and returns "same", "changed", "deleted" or "unknown". Because it uses no GTK, it can have ordinary unit tests, like `typing.rs` and `state.rs`.

A read that fails for any reason other than "not found" (such as a permissions error) returns "unknown", which changes nothing, because a brief access problem doesn't mean the file changed, and shouldn't close a banner that's still right. An unmounted drive looks like "deleted"; if the file returns unchanged, the banner closes again.

### How checks are triggered

`gio::File::monitor_file` (GTK's way of asking the system to report changes to a file) starts in `set_file`. It watches by path, so apps that swap in a new file keep being noticed.

- **Waiting briefly:** the first event starts a 500 millisecond timer, and the check runs when it ends. Later events during that time don't restart it. A single save can produce several events (deleted, created, changed), so this gives one check, and it stops a delete-then-recreate save from flashing "deleted". Not restarting the timer means an app writing constantly can't hold the check off forever.
- **Returning to the window:** the existing "window became active" handler also runs a check. This is the safety net for network drives and portal files. If starting the monitor fails, LeafMD carries on without it and relies on this.
- **Before saving:** see below.

*Alternative considered:* polling on a timer. It would catch network drives sooner, but it would keep reading the disk while the user isn't doing anything.

### Old checks can't affect a new document

Checks read the file asynchronously, so a new file could be opened while one is running. A counter goes up in `set_file`, and each check notes it when starting and throws away its result if it has changed. Checks are also skipped while a save is in progress, and one runs after the save finishes, so a change that arrived during the save isn't lost.

### Build the banner ourselves

GNOME's ready-made `adw::Banner` allows only one button, and this needs two. So the banner is a `gtk::Revealer` (a container that slides its content in and out) holding a wrapping label and two buttons. It uses a small tint so it looks like GNOME's own banners. It's a new top bar placed directly under the header. Following `find.rs`, the new file only builds the widgets, and `window.rs` decides what they do.

Buttons don't take keyboard focus when the banner appears, so typing isn't interrupted. The label wraps, so the banner fits at the smallest window size.

Ignore sits on the far right, so it never moves. Testing showed why: with Ignore on the left of the other button, starting to type widened that button into "Discard and Reload" and pushed Ignore along, so a click meant for Ignore landed on the button that discards work.

*Alternative considered:* a dialog. It would interrupt typing, and the banner is what the owner asked for.

### What each choice records

- **Ignore** records the version the banner was showing as known (it may be "missing"). It uses the version from the last check rather than reading again, so a newer change arriving at that moment still gets its own banner.
- **Reload** reads the file fresh when clicked, so it always brings in the newest version. If that read fails or isn't plain text, the document and banner stay, and a message explains.
- **Save Again** and **Save Anyway** go through the normal save, which records what was written as known.

The button and wording that depend on unsaved changes are refreshed from the existing `connect_modified_changed` handler.

### Reload keeps your place

Reload uses a new function alongside `set_text`, because `set_text` moves the cursor to the top and should keep doing so when opening a file. The new one notes the cursor's line and column, replaces the text without adding to the undo history, then puts the cursor back as close as it can. The scroll position is restored once GTK has laid out the new text. The formatted view updates through its normal redraw, and doesn't go through `set_file`, because that would reload the whole preview page and lose its scroll position.

Undo history is cleared on reload because older steps describe text that's no longer there.

### Saving checks first

`save` compares the disk with the known version before `write_to`. If they differ and the file isn't missing, it shows the `adw::AlertDialog`. Cancel returns false, the same as a failed save, so the existing "only close if the save worked" logic in `confirm_discard` works with no changes. `save_as` skips this check, because the file dialog already asks before overwriting.

`write_to` records the bytes it actually wrote, not the editor's current text, so typing during a save is still correctly treated as unsaved.

## Risks / Trade-offs

- [Another app could write between Save's check and its write] → That gap is a few milliseconds. Passing GIO's file tag to the write would close it, but adds a second way of detecting changes for a very rare case. Accepted.
- [Portal files outside the home folder may not send change notices] → The check on returning to the window and the check before saving cover these. Testing this on a real file is a task.
- [A file deleted with no unsaved changes won't ask to save when the window closes] → Deliberate. The banner has already told the user, and deleting is usually intentional (for example, switching Git branches). Marking it as unsaved would add a "Save Changes?" question most people don't want.
- [Writing a very large file repeatedly would mean reading it repeatedly] → At most one check every half second, and markdown files are small.
- [Non-atomic saves could be read half-written] → The banner might appear for a partial file, but nothing changes until the user chooses, and Reload reads the newest version when clicked.

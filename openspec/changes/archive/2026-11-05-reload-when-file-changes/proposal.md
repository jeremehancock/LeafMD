# Proposal

## Why

LeafMD doesn't notice when another app changes the file it has open. You keep looking at the old text, and pressing Save overwrites the other app's changes without warning. This is becoming more common, because markdown files are often changed by Git, sync tools like Nextcloud, other editors and AI tools while they're open.

## What Changes

- **A banner across the top of the window** appears when another app changes the open file, in the style of gedit. It says what happened and offers a choice. Nothing changes in your document until you choose. This is the main way the feature appears: there's no reload button or menu item.
- **The banner's choices depend on the situation:**
  - The file changed and you have no unsaved changes: **Ignore** or **Reload**.
  - The file changed and you have unsaved changes: **Ignore** or **Discard and Reload**, so it's clear what reloading costs.
  - The file was deleted or moved: **Ignore** or **Save Again**, which puts your text back where the file was.
- **Reload keeps your place.** The cursor and scroll position stay where they were, rather than jumping to the top.
- **Ignore means "keep what I have".** The banner goes away and won't come back for that version of the file. If the file changes again later, the banner returns.
- **Save checks first.** If the file on disk is different from what LeafMD knows about, and you didn't choose Ignore, Save asks before replacing it. This also protects files where LeafMD couldn't get a change notice, such as some network drives.
- **No false alarms.** LeafMD's own saves don't bring up the banner, and neither does another app rewriting the file with identical content. Apps that save by swapping in a new file work like any other change.
- **Coming back to the window checks again**, so a change that LeafMD wasn't told about still appears when you return to the window.
- **No new keyboard shortcuts, menu items or settings.**

### Out of scope

- Reloading automatically without asking, and any setting to turn that on.
- Showing the differences between the two versions, or merging them.
- Following a file to its new name after it's been renamed. LeafMD can't tell a rename from a delete, so both are treated as deleted or moved.
- Watching files that aren't open, or the folder around them.
- Telling you straight away about changes on drives that don't send change notices. Those appear when you return to the window or press Save.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `documents`: adds the banner, what each of its buttons does, and checking again when you return to the window. Saving now asks first if the file on disk changed without you knowing.

## Impact

- **Code:** a new `src/file_change.rs` (the banner's widgets, and the plain logic that compares versions, which can be unit tested) and `src/window.rs` (watching the file, opening, saving and reloading).
- **Docs:** Help, README, the app store description, the storefront at leafmd.app, and the file table in `docs/DEVELOPMENT.md`.
- **Screenshots:** no change. The banner only appears when a file changes, so the existing screenshots stay accurate.
- **Dependencies and permissions:** none added. GTK's built-in file watching is used, and the Flatpak's permissions don't change.

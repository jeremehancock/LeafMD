# Proposal

## Why

The side panel is always 240 pixels wide. In a project with long file names, or a document with long headings, the names are cut short, and there's no way to see more of them. On a wide screen there's plenty of room to spare. The Files page also gives no way to read a name that's been cut short, while the Headings page already shows the full heading on hover.

This release also fixes a crash: clicking a web link when the desktop has no portal (the service that hands links to the browser for a Flatpak app) closes LeafMD, instead of showing the "Couldn't open the link" message the links spec asks for. It has only been seen on the hidden test desktop, but a crash is worth fixing wherever it shows up, and the fix lets the end-to-end tests check that message.

## What Changes

- **Drag the panel's edge to resize it.** The pointer turns into the usual left-right resize arrows over the line between the panel and the document. Dragging makes the panel anywhere from 200 to 400 pixels wide. It starts at 240, as it is today.
- **The width is remembered**, the same way the panel's open or closed state and its page are: new windows and the next start use the width chosen last. Resizing one window's panel doesn't change other windows already open.
- **The document keeps its room.** When the panel sits beside the document, the document always keeps at least 680 pixels (what the header bar needs). If the window is too narrow for the chosen width, the panel shows narrower, down to 240, and goes back to the chosen width when the window is wide enough again. The remembered width doesn't change when this happens. Below 920 pixels the panel slides over the document as it does now, at 240 pixels, with nothing to drag.
- **Full names on hover in Files.** Hovering a file or folder in the Files page shows its whole name, as Headings already does for headings. Greyed-out files keep their "LeafMD can't open this type of file" tooltip.
- **Web links no longer crash LeafMD when there's no portal.** The "Couldn't open the link:" message shows instead, as the links spec already says. No spec change, since this brings LeafMD back in line with what the spec describes.

New things added to the app:
- The drag handle on the side panel's edge
- No new keyboard shortcuts, menu items or settings. The width is remembered, like whether the panel is open and which page it shows.

### Out of scope

- Resizing with the keyboard. The width only changes by dragging.
- Double-clicking the edge to reset the width. Dragging back to about 240 does the same.
- Resizing while the panel slides over the document in a narrow window.
- Making the panel narrower than 200 or wider than 400 pixels, or hiding it by dragging it closed (F9 and the panel button do that).
- Changing the 920 pixel width at which the panel starts sliding over the document.
- Other roadmap items, including Sharing (Print, Export to PDF and Export to HTML), which is planned as its own release.

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `side-panel`: three new requirements: resizing the panel by dragging its edge (200 to 400 pixels, and 240 with nothing to drag when it slides over the document in a narrow window), the document keeping at least 680 pixels, and remembering the width.
- `folder-tree`: a new requirement, hovering a file or folder shows its full name.
- `help-and-about`: Help's side panel section mentions dragging the edge to resize it.
- `end-to-end-tests`: two new scenarios (46 in all): resizing the panel and the width being remembered, and a web link with no portal showing the message instead of crashing. The tests can now also click and drag with the pointer.

## Impact

- **Code:** `src/window.rs` (the drag handle, applying the width as the window changes size, Help), `src/state.rs` (remembering the width in state.ini, with unit tests for reading it back), `src/folder_tree.rs` (the tooltip), `src/document.rs` (opening web links and files in their usual app in a way that can't crash without a portal).
- **Tests:** a pointer drag helper in `build-aux/test-desktop.sh`, and two new files in `build-aux/end-to-end/`.
- **Dependencies and permissions:** none new.
- **Docs and listing:** Help, README, metainfo description, leafmd.app (only if a short mention fits without unbalancing the feature boxes), docs/DEVELOPMENT.md (no new source files, so only if something there describes the panel's width). Keyboard Shortcuts doesn't change. Screenshots don't need retaking, since the panel starts at the same width.
- **Roadmap:** the resizable panel isn't on ROADMAP.md yet. It's added under "Reading and writing comfort" and ticked when the change is archived.

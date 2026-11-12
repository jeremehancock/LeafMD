# Proposal

## Why

Adding a picture to a document today means typing `![photo](images/photo.png)` by hand and getting the path right. Dropping the image from the file manager onto the raw text should write that link for you. Right now it does something unhelpful instead: LeafMD tries to open the image as a document and says it isn't plain text.

## What Changes

- Dropping one or more image files (PNG, JPEG, GIF, WebP, SVG and other image types) onto the raw text inserts a markdown image link for each one where they're dropped, for example `![photo](images/photo.png)`.
- The path is written relative to the document's folder, so the link keeps working when the folder is moved or shared (`../Pictures/photo.png` for an image in a neighbouring folder). Spaces and other awkward characters in names are written so the link still works (`my%20photo.png`).
- The description in the brackets is the image's name without its extension, so `beach day.jpg` becomes `![beach day](beach%20day.jpg)`. It can be edited afterwards like any text.
- One Ctrl+Z removes everything the drop inserted.
- When a link can't be made, a short message explains why and nothing is inserted:
  - the document has never been saved, so it has no folder to link from yet
  - the image or the document came from outside the home folder, which LeafMD can only reach through a stand-in path that would make a broken link
- Dropping anything that isn't an image (a markdown file, say) onto the raw text opens it, as today. Dropping an image anywhere other than the raw text (the formatted view, the tab bar) also behaves as today.

No new keyboard shortcut, menu item or setting.

**Out of scope**

- Copying the image next to the document, or into an `images/` folder. The link points to the image where it already is.
- Pasting an image from the clipboard (a screenshot, say). There's no file to link to.
- Dragging images from LeafMD's own Files panel or from a web browser.
- Dropping an image onto the formatted view.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `smart-typing`: adds dropping an image onto the raw text to insert an image link, alongside the other typing helpers like pasting a link over text.
- `documents`: "Opening files" says a file dropped anywhere on the window opens. That stays true, except for images dropped onto the raw text, which are now linked instead.

## Impact

- Code: the raw text editor in `src/document.rs` gets its own drop handler for files. It reuses the window's existing way of opening dropped files (`open_file_here` in `src/window.rs`) for anything that isn't an image, and the document portal check in `src/folder_tree.rs` to spot stand-in paths. The work of turning two paths into a link (relative path, encoding, description) goes in a small, testable function.
- No new libraries and no new Flatpak permissions.
- Docs: Help, README, the app store description, the storefront at leafmd.app and the ROADMAP.

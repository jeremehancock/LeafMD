## ADDED Requirements

### Requirement: Dropping an image onto the raw text inserts an image link
When the user drops an image file (such as PNG, JPEG, GIF, WebP or SVG) onto the raw text of a saved document, LeafMD SHALL insert a markdown image link at the place it was dropped, and put the cursor after it. Several images dropped together SHALL each get a link, one per line, in the order they were dropped.

#### Scenario: An image next to the document
- **WHEN** `~/notes/trip.md` is open in Split view and the user drops `~/notes/beach.png` onto the empty line under `Day one:` in the raw text
- **THEN** that line becomes `![beach](beach.png)`, the cursor is after it, and the picture shows in the formatted view

#### Scenario: Dropping beside a short line
- **WHEN** the user drops `~/notes/beach.png` in the empty space to the right of the short line `Day one:`, with more text below it
- **THEN** the link goes at the end of `Day one:`, not at the bottom of the document

#### Scenario: Several images at once
- **WHEN** the user drops `beach.png` and `sunset.jpg` from `~/notes/` onto an empty line in `~/notes/trip.md`
- **THEN** `![beach](beach.png)` and `![sunset](sunset.jpg)` are inserted on two lines, one under the other

### Requirement: Image links use a path relative to the document
The link's address SHALL be the image's path from the document's folder, using `../` to reach folders above it, so the link keeps working if the folders are moved together. Spaces SHALL be written as `%20`, and other characters that would end or break a link (brackets, angle brackets, `%`, `#` and `?`) SHALL be written the same way. The description SHALL be the image's name without its extension, with any square brackets in it escaped.

#### Scenario: Image in a subfolder
- **WHEN** the user drops `~/notes/images/photo.png` onto `~/notes/trip.md`
- **THEN** `![photo](images/photo.png)` is inserted

#### Scenario: Image in a neighbouring folder
- **WHEN** the user drops `~/Pictures/photo.png` onto `~/notes/trip.md`
- **THEN** `![photo](../Pictures/photo.png)` is inserted

#### Scenario: Spaces in the name
- **WHEN** the user drops `~/notes/beach day.jpg` onto `~/notes/trip.md`
- **THEN** `![beach day](beach%20day.jpg)` is inserted and the picture shows in the formatted view

#### Scenario: A name starting with #
- **WHEN** the user drops `~/notes/#00080, tour.jpg` onto `~/notes/trip.md`
- **THEN** `![#00080, tour](%2300080,%20tour.jpg)` is inserted and the picture shows in the formatted view, because a bare `#` would be read as a jump to a heading

### Requirement: Images LeafMD can't link to
When an image is dropped onto the raw text but no working link can be made, LeafMD SHALL insert nothing and show a short message saying why. This SHALL happen when the document has never been saved, and when the image or the document is outside the home folder: either LeafMD can't see the image at the path it was given, or only sees a stand-in path that wouldn't work as a link.

#### Scenario: Unsaved document
- **WHEN** the user presses Ctrl+T and drops `~/notes/beach.png` onto the new document's raw text
- **THEN** nothing is inserted and a message says "Save the document first, so LeafMD knows where to link the image from"

#### Scenario: Image outside the home folder
- **WHEN** `~/notes/trip.md` is open and the user drops an image from a USB drive onto its raw text
- **THEN** nothing is inserted and a message says "LeafMD can only link to images in your home folder"

#### Scenario: Image from /tmp
- **WHEN** `~/notes/trip.md` is open and the user drops `/tmp/screenshot.png` onto its raw text
- **THEN** nothing is inserted and the same "home folder" message is shown, rather than a link the formatted view can't show

### Requirement: Other files dropped onto the raw text still open
Dropping a file that isn't an image onto the raw text SHALL open it, as dropping it anywhere else on the window does. When images and other files are dropped together, the images SHALL be linked and the other files opened.

#### Scenario: Dropping a markdown file onto the raw text
- **WHEN** `~/notes/trip.md` is open and the user drops `~/notes/todo.md` onto its raw text
- **THEN** `todo.md` opens in a new selected tab and nothing is inserted into `trip.md`

### Requirement: One undo for a dropped image
A single Ctrl+Z SHALL remove everything one drop inserted, including every link when several images were dropped together.

#### Scenario: Undoing a drop of two images
- **WHEN** the user has just dropped `beach.png` and `sunset.jpg` onto the raw text, inserting two links, and presses Ctrl+Z
- **THEN** both links are gone and the text is as it was before the drop

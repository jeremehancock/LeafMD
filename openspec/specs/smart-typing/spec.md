# smart-typing Specification

## Purpose
The help LeafMD gives while typing markdown in the raw text, so lists, links, images and code blocks take less typing: continuing and ending lists on Enter, wrapping selected text in a link when a web address is pasted over it, inserting a link to an image dropped onto the text, and suggesting language names for code blocks.

## Requirements

### Requirement: Lists continue on Enter
When the user presses Enter in the raw text with the cursor after the marker of a list item that has text, LeafMD SHALL start a new item on the next line with the same indent and the same kind of marker. Text after the cursor SHALL move into the new item.

#### Scenario: Bullet item
- **WHEN** the line is `- Milk` with the cursor at the end, and the user presses Enter
- **THEN** a new line `- ` appears below with the cursor after the marker, ready to type

#### Scenario: Other bullet characters
- **WHEN** the line is `* Milk` with the cursor at the end, and the user presses Enter
- **THEN** the new line starts with `* `

#### Scenario: Indented item
- **WHEN** the line is `    - Eggs` (four spaces in) with the cursor at the end, and the user presses Enter
- **THEN** the new line is `    - ` with the same four spaces

#### Scenario: Splitting an item
- **WHEN** the line is `- Milk and eggs` with the cursor just before "and", and the user presses Enter
- **THEN** the line becomes `- Milk ` and a new line `- and eggs` appears below, with the cursor before "and"

### Requirement: Numbered items count up
When a numbered list item continues, LeafMD SHALL give the new item the next number and keep the same punctuation after it (a full stop or a closing bracket). Items further down SHALL NOT be renumbered.

#### Scenario: Full stop
- **WHEN** the line is `2. Preheat the oven` with the cursor at the end, and the user presses Enter
- **THEN** the new line starts with `3. `

#### Scenario: Closing bracket
- **WHEN** the line is `9) Serve` with the cursor at the end, and the user presses Enter
- **THEN** the new line starts with `10) `

### Requirement: Checklist items continue unticked
When a checklist item continues, LeafMD SHALL start the new item with an unticked box, whether the item above is ticked or not.

#### Scenario: After a ticked item
- **WHEN** the line is `- [x] Book flights` with the cursor at the end, and the user presses Enter
- **THEN** the new line starts with `- [ ] `

### Requirement: Lists end on an empty item
When the user presses Enter on a list item that has nothing after its marker, LeafMD SHALL remove the marker and its indent, leaving a blank line with the cursor on it, and SHALL NOT add a new line.

#### Scenario: Finishing a list
- **WHEN** the lines are `- Milk` and then `- ` with the cursor at the end of `- `, and the user presses Enter
- **THEN** the second line becomes empty, the cursor stays on it, and `- Milk` is unchanged

#### Scenario: Empty checklist item
- **WHEN** the line is `- [ ] ` with nothing after the box, and the user presses Enter
- **THEN** the line becomes empty

### Requirement: Shift+Enter skips the marker
LeafMD SHALL insert a plain new line, with no list marker, when the user presses Shift+Enter in a list item.

#### Scenario: A line that isn't a new item
- **WHEN** the line is `- Milk` with the cursor at the end, and the user presses Shift+Enter
- **THEN** a new line appears below with no `- ` marker

### Requirement: Enter behaves normally outside list text
LeafMD SHALL insert an ordinary new line when Enter is pressed on a line that isn't a list item, with the cursor before or inside an item's marker, while text is selected, or inside a fenced code block (between two ` ``` ` lines).

#### Scenario: Plain paragraph
- **WHEN** the line is `Shopping list:` with the cursor at the end, and the user presses Enter
- **THEN** a plain new line appears, with no marker

#### Scenario: Cursor before the marker
- **WHEN** the line is `- Milk` with the cursor at the very start, and the user presses Enter
- **THEN** a blank line is inserted above `- Milk`, and no marker is added

#### Scenario: Inside a code block
- **WHEN** the cursor is at the end of `- not a list` on a line between ` ```bash ` and ` ``` `, and the user presses Enter
- **THEN** a plain new line appears, with no marker

### Requirement: Pasting a web address over selected text makes a link
When text on a single line is selected in the raw text and the user pastes a web address (text starting with `http://` or `https://`, with no spaces), LeafMD SHALL replace the selection with `[selected text](address)` and put the cursor after it. Pasting anything else SHALL replace the selection as usual.

#### Scenario: Making a link
- **WHEN** "the docs" is selected in `Read the docs first` and the user pastes `https://example.com/docs`
- **THEN** the line becomes `Read [the docs](https://example.com/docs) first`

#### Scenario: Pasting ordinary text
- **WHEN** "the docs" is selected and the user pastes `the manual`
- **THEN** the line becomes `Read the manual first`

#### Scenario: Nothing selected
- **WHEN** nothing is selected and the user pastes `https://example.com`
- **THEN** the address is inserted as plain text, as before

#### Scenario: Selection over several lines
- **WHEN** text across two lines is selected and the user pastes `https://example.com`
- **THEN** the selection is replaced by the address, as before

### Requirement: One undo per helping hand
LeafMD SHALL let a single Ctrl+Z undo each of these as one step: a continued list item (the new line and its marker), an ended list (the removed marker), or a pasted link (bringing back the selected text).

#### Scenario: Undoing a link
- **WHEN** the user has just pasted `https://example.com` over "the docs", turning it into a link, and presses Ctrl+Z
- **THEN** the line goes back to `Read the docs first`

#### Scenario: Undoing a continued item
- **WHEN** the user has just pressed Enter at the end of `- Milk`, which added `- ` below, and presses Ctrl+Z
- **THEN** the new line and its marker are both gone, leaving just `- Milk`

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

### Requirement: Language suggestions for code blocks
When the user types ` ``` ` at the start of a line in the raw text (after up to three spaces) followed by at least one letter, and the line opens a code block rather than closing one, LeafMD SHALL show a list of the code block languages it colours whose names start with what's typed. The list SHALL narrow as the user types, and SHALL close when nothing matches.

#### Scenario: Typing a language
- **WHEN** the user types ` ```py ` on an empty line
- **THEN** a small list appears under the cursor offering `python`

#### Scenario: Several matches
- **WHEN** the user types ` ```j `
- **THEN** the list offers `java`, `javascript` and `json`, and typing `a` narrows it to `java` and `javascript`

#### Scenario: Closing a block
- **WHEN** the user types ` ```p ` on the line after `print("hi")` inside an open ` ```python ` block
- **THEN** no list appears, because that line closes the block

### Requirement: Picking or ignoring a suggestion
Pressing Enter or clicking a suggestion SHALL replace what was typed after ` ``` ` with the chosen name, and close the list. Escape, moving the cursor away or typing something that matches nothing SHALL close the list without changing the text. Typing ` ``` ` and pressing Enter with no letter after it SHALL start a new line as before, with no list shown.

#### Scenario: Picking with Enter
- **WHEN** the user types ` ```py ` and presses Enter
- **THEN** the line becomes ` ```python ` and the cursor is at its end

#### Scenario: Ignoring the list
- **WHEN** the user types ` ```py `, presses Escape and then Enter
- **THEN** the line stays ` ```py ` and a new line starts below it

#### Scenario: A plain block
- **WHEN** the user types ` ``` ` and presses Enter
- **THEN** a new line starts below, as it did before, and no list appears

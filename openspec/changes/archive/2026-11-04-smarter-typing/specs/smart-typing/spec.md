# Spec Delta

## Purpose

The help LeafMD gives while typing markdown in the raw text, so lists and links take less typing: continuing and ending lists on Enter, and wrapping selected text in a link when a web address is pasted over it.

## ADDED Requirements

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

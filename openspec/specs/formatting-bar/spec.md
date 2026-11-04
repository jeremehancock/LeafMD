# formatting-bar Specification

## Purpose
The row of buttons above the raw text that adds markdown for you (bold, links, lists, headings, tables and so on), and their keyboard shortcuts.

## Requirements

### Requirement: Where the bar appears
LeafMD SHALL show the formatting bar above the raw text in Raw and Split view only, and the formatting shortcuts SHALL only work there. In a very narrow window the buttons SHALL scroll sideways instead of stopping the window from getting smaller. The bar's buttons SHALL use LeafMD's own icons, so they look the same with any icon theme.

#### Scenario: Formatted view
- **WHEN** the user switches to Formatted view
- **THEN** the formatting bar is hidden and Ctrl+B does nothing

### Requirement: Buttons and shortcuts
The bar SHALL have, in order: Bold (Ctrl+B), Italic (Ctrl+I), Strikethrough (Ctrl+Shift+X), Inline code (Ctrl+E), Link (Ctrl+K), a Heading menu (Heading 1, 2 and 3), Bulleted list, Numbered list, Checklist, Quote, Code block, Table, and a Cheatsheet button at the far end.

#### Scenario: Shortcut
- **WHEN** the user selects a word in the raw text and presses Ctrl+I
- **THEN** the word is wrapped in `*` markers

### Requirement: Text styles
Bold, italic, strikethrough and inline code SHALL wrap the selection in their markers (`**`, `*`, `~~`, `` ` ``) and remove them if it already has them. Bold and italic SHALL be able to combine and be removed separately.

#### Scenario: Toggle bold
- **WHEN** "text" in "Some text" is selected and Bold is pressed twice
- **THEN** it becomes "Some **text**" and then "Some text" again, with "text" still selected

#### Scenario: Italic on bold
- **WHEN** Italic is pressed with the "x" in `**x**` selected
- **THEN** it becomes `***x***`

### Requirement: Text styles with nothing selected
With nothing selected, the text styles SHALL apply to the word the cursor is in. When the cursor isn't in a word, they SHALL add an empty pair of markers with the cursor between them, ready to type.

#### Scenario: Cursor in a word
- **WHEN** the cursor is in the middle of "text" and Italic is pressed
- **THEN** the word becomes `*text*`

#### Scenario: Nothing selected, not in a word
- **WHEN** the cursor is after a space and Bold is pressed
- **THEN** `****` is added with the cursor in the middle, and pressing Bold again right away removes it

### Requirement: Text styles over awkward selections
Spaces at the edges of the selection SHALL stay outside the markers, and a selection over several lines SHALL be styled line by line, because markdown styles can't cross a blank line.

#### Scenario: Selection with spaces
- **WHEN** " word " (with spaces) is selected in "a word here" and Bold is pressed
- **THEN** it becomes "a **word** here"

### Requirement: Links
The Link button SHALL turn the selection into `[text](url)` with "url" selected ready to paste over. If the selection is a web address, it SHALL become `[link text](address)` with "link text" selected. With nothing selected, it SHALL insert `[link text](url)` with "link text" selected.

#### Scenario: Selected web address
- **WHEN** `https://example.com` is selected and Link is pressed
- **THEN** it becomes `[link text](https://example.com)` with "link text" selected

### Requirement: Line styles
Headings, bulleted lists, numbered lists, checklists and quotes SHALL apply to every line the selection touches (or the cursor's line), and SHALL turn them back into plain lines if they all already have that style. Blank lines SHALL be skipped, except a blank line on its own, which starts a new item.

#### Scenario: Starting a list on an empty line
- **WHEN** the cursor is on an empty line and Checklist is pressed
- **THEN** the line becomes `- [ ] ` with the cursor after it

### Requirement: Switching line styles
Switching between list kinds SHALL keep each line's text and indent. Numbered lists SHALL be numbered 1, 2, 3, ticked checklist items SHALL stay ticked, and choosing a different heading level SHALL replace the old one.

#### Scenario: Bullets to numbers
- **WHEN** the lines `- a`, `- b`, `- c` are selected and Numbered list is pressed
- **THEN** they become `1. a`, `2. b`, `3. c`

#### Scenario: Heading level change
- **WHEN** the cursor is on `## Title` and Heading 1 is chosen
- **THEN** the line becomes `# Title`, and choosing Heading 1 again makes it plain `Title`

#### Scenario: Quoting paragraphs
- **WHEN** "one", a blank line and "two" are selected and Quote is pressed
- **THEN** they become `> one`, `>` and `> two`

### Requirement: Code blocks and tables
The Code block button SHALL put the lines inside a ``` fence, or take them out if they are already fenced. The Table button SHALL insert a small two-column starter table with "Column 1" selected, replacing the line if it's blank, otherwise going below it, with blank lines added around it where needed so it doesn't merge with a paragraph.

#### Scenario: Table after a paragraph
- **WHEN** the cursor is on the line "Intro" and Table is pressed
- **THEN** a blank line and the starter table are added below "Intro"

### Requirement: One undo per button press
Each formatting button press SHALL be undone by a single Ctrl+Z, and the cursor or selection SHALL move along with the text it was on.

#### Scenario: Undo a list
- **WHEN** the user turns five lines into a bulleted list and presses Ctrl+Z
- **THEN** all five lines go back to how they were

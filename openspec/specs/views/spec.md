# views Specification

## Purpose
The three ways to look at a document (Formatted, Raw and Split), how you switch between them, and how the two sides of Split view stay in step with each other.

## Requirements

### Requirement: Three views
LeafMD SHALL offer three views of the open document: Formatted (the rendered document only, for reading), Raw (the markdown text only, editable) and Split (raw text on the left and formatted on the right).

#### Scenario: Formatted view
- **WHEN** the Formatted view is chosen
- **THEN** only the rendered document is shown, and nothing in it can be edited

#### Scenario: Raw view
- **WHEN** the Raw view is chosen
- **THEN** only the markdown text is shown, with line numbers, and it can be edited

#### Scenario: Split view
- **WHEN** the Split view is chosen
- **THEN** the raw text and the formatted document are shown side by side, divided down the middle of the window

### Requirement: The raw text editor
The raw text SHALL be shown in a monospace font with line numbers, markdown syntax colouring, the current line highlighted, long lines wrapped at the window edge (never scrolling sideways), tabs four spaces wide, and new lines keeping the previous line's indent. Undo (Ctrl+Z) and redo (Ctrl+Shift+Z) SHALL work as in any GNOME text editor.

#### Scenario: A long line
- **WHEN** a paragraph is longer than the window is wide
- **THEN** it wraps onto the next row, keeping one line number

#### Scenario: Indented typing
- **WHEN** the user presses Enter at the end of a line indented by four spaces
- **THEN** the new line starts with the same four spaces

### Requirement: Switching views
LeafMD SHALL let the user switch views with the rounded view switcher in the header bar and with Ctrl+1 (Formatted), Ctrl+2 (Raw) and Ctrl+3 (Split), and the switcher SHALL always show the active view.

#### Scenario: Switching by keyboard
- **WHEN** the user presses Ctrl+3 while in Formatted view
- **THEN** Split view is shown and the switcher's Split button becomes the active one

#### Scenario: No document open
- **WHEN** no document is open
- **THEN** the view switcher is greyed out

### Requirement: The view is remembered
LeafMD SHALL remember the last view used and start in it next time. On the very first run it SHALL start in Formatted view.

#### Scenario: Reopening the app
- **WHEN** the user closes LeafMD while in Raw view and starts it again
- **THEN** it opens in Raw view

### Requirement: New documents open where you can type
LeafMD SHALL switch from Formatted view to Split view when the user starts a new document, and put the cursor in the raw text.

#### Scenario: New document from Formatted view
- **WHEN** the user presses Ctrl+N while in Formatted view
- **THEN** an empty document opens in Split view with the cursor in the raw text

### Requirement: Live updates
LeafMD SHALL update the formatted side of Split view shortly after the user stops typing (about 150 milliseconds), and SHALL keep the formatted view's scroll position when it updates.

#### Scenario: Typing in Split view
- **WHEN** the user types `**hi**` in the raw text
- **THEN** a moment later the formatted side shows "hi" in bold without jumping to the top

### Requirement: Scrolling together in Split view
In Split view, LeafMD SHALL scroll the other side to the matching spot when either side is scrolled, matching blocks (paragraphs, headings, list items and so on) by the lines of raw text they came from. Reaching the bottom of one side SHALL bring the other to its bottom too. The two sides MUST NOT keep nudging each other after one has been scrolled.

#### Scenario: Scrolling the raw text
- **WHEN** the user scrolls the raw text so a heading on line 40 is at the top
- **THEN** the formatted side scrolls so that same heading is at its top

#### Scenario: Only in Split view
- **WHEN** only one view is showing
- **THEN** no scroll syncing happens

### Requirement: Double-click to jump to the raw text
In Split view, LeafMD SHALL select the same word in the raw text when the user double-clicks a word on the formatted side, picking the matching copy if the word appears more than once in that block, and SHALL briefly highlight its line (about 1.2 seconds) in the accent colour. In Formatted view, double-clicking SHALL just select the word as in any reader.

#### Scenario: Jumping to a repeated word
- **WHEN** the user double-clicks the second "the" in a paragraph on the formatted side of Split view
- **THEN** the second "the" in that paragraph's raw text is selected, scrolled into view, and its line flashes

#### Scenario: Formatted view only
- **WHEN** the user double-clicks a word in Formatted view
- **THEN** the word is selected on the page and the raw text is not touched

### Requirement: Follows the system style
LeafMD SHALL follow the system's light or dark style in both the raw text (Adwaita or Adwaita-dark colours) and the formatted view, and SHALL switch straight away when the system style changes.

#### Scenario: Switching to dark mode
- **WHEN** the system switches to dark style while a document is open
- **THEN** the raw text and the formatted view both turn dark without reopening anything

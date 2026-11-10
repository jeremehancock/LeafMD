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
- **THEN** only the markdown text is shown, and it can be edited

#### Scenario: Split view
- **WHEN** the Split view is chosen
- **THEN** the raw text and the formatted document are shown side by side, divided down the middle of the window

### Requirement: The raw text editor
The raw text SHALL be shown in a monospace font with markdown syntax colouring, the current line highlighted, long lines wrapped at the window edge (never scrolling sideways), tabs four spaces wide, and new lines keeping the previous line's indent. Undo (Ctrl+Z) and redo (Ctrl+Shift+Z) SHALL work as in any GNOME text editor.

#### Scenario: A long line
- **WHEN** a paragraph is longer than the window is wide
- **THEN** it wraps onto the next row, and with line numbers on it keeps one line number

#### Scenario: Indented typing
- **WHEN** the user presses Enter at the end of a line indented by four spaces
- **THEN** the new line starts with the same four spaces

### Requirement: Switching views
LeafMD SHALL let the user switch views with the rounded view switcher in the header bar and with Ctrl+1 (Formatted), Ctrl+2 (Raw) and Ctrl+3 (Split), and the switcher SHALL always show the active view. In a window narrower than 640 pixels, the switcher SHALL be in the bottom bar instead of the header bar.

#### Scenario: Switching by keyboard
- **WHEN** the user presses Ctrl+3 while in Formatted view
- **THEN** Split view is shown and the switcher's Split button becomes the active one

#### Scenario: No document open
- **WHEN** no document is open
- **THEN** the view switcher is greyed out

#### Scenario: Switching in a narrow window
- **WHEN** the window is 400 pixels wide and the user clicks Raw on the switcher in the bottom bar
- **THEN** Raw view is shown and the switcher's Raw button becomes the active one

### Requirement: The view is remembered
LeafMD SHALL remember the last view used and start in it next time, and every tab it reopens SHALL start in that view. On the very first run it SHALL start in Formatted view.

#### Scenario: Reopening the app
- **WHEN** the user closes LeafMD while in Raw view and starts it again
- **THEN** it opens in Raw view

#### Scenario: Reopening several tabs
- **WHEN** a window has a tab in Formatted view and a tab in Split view, the Split tab is selected, the user closes the window and starts LeafMD again
- **THEN** both tabs reopen in Split view

### Requirement: New documents open where you can type
LeafMD SHALL switch from Formatted view to Split view in the new tab when the user starts a new document, and put the cursor in the raw text. Other tabs SHALL keep their views.

#### Scenario: New document from Formatted view
- **WHEN** the user presses Ctrl+T while in Formatted view
- **THEN** an empty document opens in a new tab in Split view with the cursor in the raw text

#### Scenario: Going back to the first tab
- **WHEN** notes.md is in Formatted view, the user presses Ctrl+T, and then selects notes.md's tab again
- **THEN** notes.md is still in Formatted view

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
LeafMD SHALL follow the system's light or dark style in both the raw text (GNOME's Adwaita or Adwaita-dark editor colours) and the formatted view, and SHALL switch straight away when the system style changes. The raw text and the formatted view SHALL have exactly the same background colour.

#### Scenario: Switching to dark mode
- **WHEN** the system switches to dark style while a document is open
- **THEN** the raw text and the formatted view both turn dark without reopening anything

#### Scenario: Split view in dark style
- **WHEN** a document is in Split view in dark style
- **THEN** the raw text half and the formatted half are the same shade, with no visible change of colour at the divider except the divider line itself

### Requirement: Each tab has its own view
Each tab SHALL keep its own view, and the view switcher SHALL always show the selected tab's view. Changing the view SHALL only affect the selected tab. A file opening in a new tab SHALL start in the view last chosen in that window.

#### Scenario: Different views side by side
- **WHEN** notes.md is in Formatted view, todo.md is in Raw view, and the user selects notes.md's tab
- **THEN** notes.md shows in Formatted view and the switcher's Formatted button is the active one

#### Scenario: Ctrl+3 in one tab
- **WHEN** two tabs are in Formatted view and the user presses Ctrl+3 in the first
- **THEN** the first is in Split view and the second is still in Formatted view

#### Scenario: A newly opened file
- **WHEN** the user switches notes.md to Raw view and then opens todo.md with Ctrl+O
- **THEN** todo.md opens in a new tab in Raw view

### Requirement: Line numbers are a choice
LeafMD SHALL hide line numbers in the raw text unless the user turns them on with Line Numbers in the main menu. The choice SHALL apply to every window and every tab at once, and SHALL be remembered between runs.

#### Scenario: First run
- **WHEN** LeafMD is started for the first time and a document is shown in Raw view
- **THEN** the raw text has no line numbers

#### Scenario: Turning them on
- **WHEN** two windows are open, each with two tabs, and the user chooses Line Numbers in one window's main menu
- **THEN** the raw text in all four tabs shows line numbers straight away

#### Scenario: Remembered
- **WHEN** the user turns line numbers on, closes LeafMD and starts it again
- **THEN** the raw text shows line numbers

### Requirement: Quiet raw text
The raw text SHALL leave clear room around the text (about 18 pixels at the sides) and a little extra space between lines. Line numbers SHALL be dim enough not to compete with the text, and the current-line highlight SHALL be faint. Syntax colours SHALL stay GNOME's.

#### Scenario: Line numbers beside the text
- **WHEN** line numbers are on and the raw text shows `# Community Garden Plan` on line 1
- **THEN** the "1" is a pale grey, there is a clear gap between it and the "#", and the line number of the line with the cursor is a little darker than the others

### Requirement: A readable column in Raw view
When only the raw text is showing, LeafMD SHALL keep it in a centred column about 80 characters wide in a wider window, with the scroll bar at the window's edge. With line numbers on, the column SHALL widen to make room for them. In Split view the raw text SHALL fill its half.

#### Scenario: A wide window
- **WHEN** a 1400 pixel wide window shows a document in Raw view
- **THEN** the text sits in a centred column, the space either side has the same background as the text, and long paragraphs wrap at the column's edge

#### Scenario: A narrow window
- **WHEN** the window is narrower than the column
- **THEN** the raw text fills the window, as before

#### Scenario: Split view
- **WHEN** the same document is switched to Split view
- **THEN** the raw text fills the left half

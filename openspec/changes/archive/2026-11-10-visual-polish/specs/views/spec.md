# Spec Delta

## MODIFIED Requirements

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

### Requirement: Follows the system style
LeafMD SHALL follow the system's light or dark style in both the raw text (GNOME's Adwaita or Adwaita-dark editor colours) and the formatted view, and SHALL switch straight away when the system style changes. The raw text and the formatted view SHALL have exactly the same background colour.

#### Scenario: Switching to dark mode
- **WHEN** the system switches to dark style while a document is open
- **THEN** the raw text and the formatted view both turn dark without reopening anything

#### Scenario: Split view in dark style
- **WHEN** a document is in Split view in dark style
- **THEN** the raw text half and the formatted half are the same shade, with no visible change of colour at the divider except the divider line itself

## ADDED Requirements

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

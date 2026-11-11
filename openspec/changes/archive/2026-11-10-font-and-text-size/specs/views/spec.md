# Spec Delta

## MODIFIED Requirements

### Requirement: The raw text editor
The raw text SHALL be shown in the Raw Text font from Preferences (GNOME's monospace font until the user picks another), at the window's zoom, with markdown syntax colouring, the current line highlighted, long lines wrapped at the window edge (never scrolling sideways), tabs four spaces wide, and new lines keeping the previous line's indent. Undo (Ctrl+Z) and redo (Ctrl+Shift+Z) SHALL work as in any GNOME text editor.

#### Scenario: A long line
- **WHEN** a paragraph is longer than the window is wide
- **THEN** it wraps onto the next row, and with line numbers on it keeps one line number

#### Scenario: Indented typing
- **WHEN** the user presses Enter at the end of a line indented by four spaces
- **THEN** the new line starts with the same four spaces

#### Scenario: The default font
- **GIVEN** the user has never changed the Raw Text font
- **WHEN** they open notes.md in Raw view
- **THEN** the text is in GNOME's monospace font, as before Preferences existed

### Requirement: A readable column in Raw view
When only the raw text is showing, LeafMD SHALL keep it in a centred column about 80 characters wide in a wider window, with the scroll bar at the window's edge. The column SHALL be measured in the current Raw Text font at the current zoom, so it widens when the text gets bigger. With line numbers on, the column SHALL widen to make room for them. In Split view the raw text SHALL fill its half.

#### Scenario: A wide window
- **WHEN** a 1400 pixel wide window shows a document in Raw view
- **THEN** the text sits in a centred column, the space either side has the same background as the text, and long paragraphs wrap at the column's edge

#### Scenario: A narrow window
- **WHEN** the window is narrower than the column
- **THEN** the raw text fills the window, as before

#### Scenario: Split view
- **WHEN** the same document is switched to Split view
- **THEN** the raw text fills the left half

#### Scenario: Zooming in Raw view
- **WHEN** a 1400 pixel wide window shows a document in Raw view and the user presses Ctrl+plus twice
- **THEN** the column grows wider along with the text, so a line still holds about 80 characters before wrapping

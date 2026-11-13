## MODIFIED Requirements

### Requirement: Preferences window
LeafMD SHALL have a Preferences window, opened with Preferences in the main menu or Ctrl+, (comma). It SHALL hold two groups. First, Appearance, with a Style row offering Follow System, Light and Dark side by side, then a Margins row offering None, Narrow, Normal and Wide side by side, the current choice selected in each. Then Fonts, with a Raw Text row and a Formatted View row, each with a button that shows the font's name and size and opens GNOME's font picker. Preferences SHALL hold nothing else.

#### Scenario: Opening Preferences
- **WHEN** notes.md is open and the user presses Ctrl+,
- **THEN** the Preferences window opens over the document, with an Appearance group showing the Style row and the Margins row, then a Fonts group showing the Raw Text row and the Formatted View row

#### Scenario: From the main menu with no document open
- **WHEN** a window shows the "No Document Open" page and the user chooses Preferences in the main menu
- **THEN** the Preferences window opens

### Requirement: Resetting a font
Each row in the Fonts group SHALL have a Reset button, shown only while that font differs from its starting font. Reset SHALL put it back to the starting font, including following GNOME's monospace font again for the raw text.

#### Scenario: Resetting the raw text
- **WHEN** the Raw Text font is "Fira Code 12" and the user clicks its Reset button
- **THEN** the raw text goes back to GNOME's monospace font, and the Reset button disappears

#### Scenario: Nothing to reset
- **WHEN** neither font has been changed and the user opens Preferences
- **THEN** neither row shows a Reset button

## MODIFIED Requirements

### Requirement: Preferences window
LeafMD SHALL have a Preferences window, opened with Preferences in the main menu or Ctrl+, (comma). It SHALL hold three groups. First, Appearance, with one Style row offering Follow System, Light and Dark side by side, the current one selected. Then Raw Text and Formatted View, each with one Font row whose button shows the font's name and size and opens GNOME's font picker. Preferences SHALL hold nothing else.

#### Scenario: Opening Preferences
- **WHEN** notes.md is open and the user presses Ctrl+,
- **THEN** the Preferences window opens over the document, with an Appearance group showing the Style row, then a Raw Text group and a Formatted View group, each showing one Font row

#### Scenario: From the main menu with no document open
- **WHEN** a window shows the "No Document Open" page and the user chooses Preferences in the main menu
- **THEN** the Preferences window opens

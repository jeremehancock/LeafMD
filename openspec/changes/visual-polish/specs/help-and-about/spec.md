# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Find… and Find and Replace…; Side Panel and Line Numbers; Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it. Side Panel SHALL show a tick while the panel is open, and Line Numbers SHALL show a tick while line numbers are on.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the five sections are shown, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O and Side Panel showing F9

#### Scenario: Side Panel's tick
- **WHEN** the side panel is open and the user opens the main menu
- **THEN** Side Panel has a tick, and choosing it closes the panel

#### Scenario: Line Numbers' tick
- **WHEN** line numbers are off and the user chooses Line Numbers in the main menu, then opens the menu again
- **THEN** line numbers are showing and Line Numbers has a tick

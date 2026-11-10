# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Find… and Find and Replace…; Side Panel; Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it. Side Panel SHALL show a tick while the panel is open.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the five sections are shown, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O and Side Panel showing F9

#### Scenario: Side Panel's tick
- **WHEN** the side panel is open and the user opens the main menu
- **THEN** Side Panel has a tick, and choosing it closes the panel

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel and its folder tree, and files and links. Every user-visible feature SHALL be described there.

#### Scenario: Pressing F1
- **WHEN** the user presses F1
- **THEN** the Help window opens over the document

#### Scenario: Reading about tabs
- **WHEN** the user opens Help and looks for tabs
- **THEN** it explains opening and closing tabs, moving between them, dragging them to another window, and that links and opened files use a new tab

#### Scenario: Reading about the side panel
- **WHEN** the user opens Help and looks for the side panel
- **THEN** it explains opening and closing it with F9, that it stays open until closed, which folder the tree shows, Open Folder…, and why some files are greyed out

### Requirement: Terminal command in Help
The "Files and Links" section of the Help window SHALL have a "From a terminal" row saying that `leafmd notes.md` opens a file and `leafmd .` opens the current folder in the side panel, after a one-time setup, with a Setup link that opens `https://leafmd.app/#terminal` in the browser. The setup commands themselves SHALL NOT be shown in the app, so that the website stays the only place they need updating.

#### Scenario: Finding the setup steps
- **WHEN** the user presses F1, scrolls to "Files and Links" and clicks Setup on the "From a terminal" row
- **THEN** `https://leafmd.app/#terminal` opens in their browser, at the setup section

#### Scenario: A narrow window
- **WHEN** the Help window is 360 pixels wide
- **THEN** the row's text wraps and the Setup link can still be clicked

#### Scenario: Reading about folders
- **WHEN** the user reads the "From a terminal" row
- **THEN** it mentions both `leafmd notes.md` and `leafmd .`

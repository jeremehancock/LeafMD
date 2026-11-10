# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Find… and Find and Replace…; Side Panel, Line Numbers, Distraction Free and Full Screen; Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it. Side Panel SHALL show a tick while the panel is open, Line Numbers while that window shows line numbers, Distraction Free while that window is in it, and Full Screen while the window fills the screen.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the five sections are shown, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O, Side Panel showing F9, Distraction Free showing Shift+Ctrl+D and Full Screen showing F11

#### Scenario: Side Panel's tick
- **WHEN** the side panel is open and the user opens the main menu
- **THEN** Side Panel has a tick, and choosing it closes the panel

#### Scenario: Line Numbers' tick
- **WHEN** line numbers are off and the user chooses Line Numbers in the main menu, then opens the menu again
- **THEN** line numbers are showing and Line Numbers has a tick

#### Scenario: Line Numbers' tick in Distraction Free
- **WHEN** line numbers are on, the user turns Distraction Free on and opens the main menu with F10
- **THEN** Line Numbers has no tick, because that window isn't showing them

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel and its folder tree, Distraction Free and Full Screen, and files and links. Every user-visible feature SHALL be described there.

#### Scenario: Pressing F1
- **WHEN** the user presses F1
- **THEN** the Help window opens over the document

#### Scenario: Reading about tabs
- **WHEN** the user opens Help and looks for tabs
- **THEN** it explains opening and closing tabs, moving between them, dragging them to another window, and that links and opened files use a new tab

#### Scenario: Reading about the side panel
- **WHEN** the user opens Help and looks for the side panel
- **THEN** it explains opening and closing it with F9, that it stays open until closed, which folder the tree shows, Open Folder…, and why some files are greyed out

#### Scenario: Reading about Distraction Free
- **WHEN** the user opens Help and looks for Distraction Free
- **THEN** it explains Ctrl+Shift+D, what gets hidden, bringing the bars back with the pointer at the top edge or F10, leaving with Escape, and that F11 adds full screen

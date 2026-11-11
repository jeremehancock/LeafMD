# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL start with the zoom row (Zoom Out, the zoom percentage, Zoom In), then contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Find… and Find and Replace…; Side Panel, Line Numbers, Distraction Free and Full Screen; Preferences, Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the zoom row is at the top, followed by the five sections, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O, Side Panel showing F9, Distraction Free showing Shift+Ctrl+D, Full Screen showing F11 and Preferences showing Ctrl+,

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel and its folder tree, Distraction Free and Full Screen, fonts and zoom, and files and links. Every user-visible feature SHALL be described there.

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

#### Scenario: Reading about fonts and zoom
- **WHEN** the user opens Help and looks for fonts and zoom
- **THEN** it explains choosing the two fonts in Preferences (Ctrl+,), Reset, zooming with Ctrl+plus, Ctrl+minus, Ctrl+0 and Ctrl with the mouse wheel, that zoom covers the whole window, and that zoom is back to 100% each time LeafMD starts

### Requirement: Keyboard shortcuts window
Keyboard Shortcuts (Ctrl+?) SHALL list every keyboard shortcut, grouped into Views, Files, Tabs, Formatting, Find and Help, with no search box, in the same style as the Help window.

#### Scenario: A new shortcut is added
- **WHEN** a feature adds a keyboard shortcut
- **THEN** it also appears in this window

#### Scenario: Opening the window
- **WHEN** the user presses Ctrl+?
- **THEN** the list starts directly under the title bar with no search box, and each row shows its keys as text, like "Ctrl+1" beside "Formatted view"

#### Scenario: Closing keys
- **WHEN** the user looks under Files and Tabs
- **THEN** "Close tab" shows "Ctrl+W" and "Close window" shows "Ctrl+Q"

#### Scenario: Tab keys
- **WHEN** the user looks under Tabs
- **THEN** it lists next tab, previous tab and going to a tab by position with their keys

#### Scenario: New document and new window keys
- **WHEN** the user looks under Files
- **THEN** "New document" shows "Ctrl+T" and "New window" shows "Ctrl+N"

#### Scenario: Zoom and Preferences keys
- **WHEN** the user looks under Views and Help
- **THEN** Views lists "Zoom in" with "Ctrl+Plus or Ctrl+=", "Zoom out" with "Ctrl+Minus" and "Normal size" with "Ctrl+0", and Help lists "Preferences" with "Ctrl+,"

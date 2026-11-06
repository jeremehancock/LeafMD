# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL contain, in sections: New, New Window, Open… and Close Document; Save and Save As…; Find… and Find and Replace…; Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the four sections are shown, separated by dividers, with New showing Ctrl+T and New Window showing Ctrl+N

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, and files and links. Every user-visible feature SHALL be described there.

#### Scenario: Pressing F1
- **WHEN** the user presses F1
- **THEN** the Help window opens over the document

#### Scenario: Reading about tabs
- **WHEN** the user opens Help and looks for tabs
- **THEN** it explains opening and closing tabs, moving between them, dragging them to another window, and that links and opened files use a new tab

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

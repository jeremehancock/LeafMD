# Spec Delta

## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL start with the zoom row (Zoom Out, the zoom percentage, Zoom In), then contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Print…, Export as PDF… and Export as HTML…; Find… and Find and Replace…; Side Panel, Line Numbers, Distraction Free and Full Screen; Preferences, Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the zoom row is at the top, followed by the six sections, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O, Print… showing Ctrl+P, Side Panel showing F9, Distraction Free showing Shift+Ctrl+D, Full Screen showing F11 and Preferences showing Ctrl+,

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel with its folder tree and headings, the word count, Distraction Free and Full Screen, fonts and zoom, printing and exporting, and files and links. Every user-visible feature SHALL be described there.

#### Scenario: Pressing F1
- **WHEN** the user presses F1
- **THEN** the Help window opens over the document

#### Scenario: Reading about tabs
- **WHEN** the user opens Help and looks for tabs
- **THEN** it explains opening and closing tabs, moving between them, dragging them to another window, and that links and opened files use a new tab

#### Scenario: Reading about the side panel
- **WHEN** the user opens Help and looks for the side panel
- **THEN** it explains opening and closing it with F9, that it stays open until closed, dragging its edge to make it wider or narrower, which folder the tree shows, Open Folder…, why some files are greyed out, and switching between Files and Headings at the bottom of the panel

#### Scenario: Reading about Distraction Free
- **WHEN** the user opens Help and looks for Distraction Free
- **THEN** it explains Ctrl+Shift+D, what gets hidden, bringing the bars back with the pointer at the top edge or F10, leaving with Escape, and that F11 adds full screen

#### Scenario: Reading about fonts and zoom
- **WHEN** the user opens Help and looks for fonts and zoom
- **THEN** it explains choosing the two fonts in Preferences (Ctrl+,), Reset, zooming with Ctrl+plus, Ctrl+minus, Ctrl+0 and Ctrl with the mouse wheel, that zoom covers the whole window, and that zoom is back to 100% each time LeafMD starts

#### Scenario: Reading about headings and the word count
- **WHEN** the user opens Help and looks for headings or the word count
- **THEN** it explains that the Headings page lists the document's headings, that clicking one jumps to it, that the section you're reading is highlighted, what the word count in the bottom-right corner counts, the 200 words a minute reading time, and that selecting text counts the selection

#### Scenario: Reading about printing and exporting
- **WHEN** the user opens Help and looks for printing or exporting
- **THEN** it explains that Ctrl+P prints the formatted document whichever view is showing, that pages and PDFs are always light, that Export as PDF… and Export as HTML… are in the main menu, that the web page packs in pictures from the computer and follows the reader's light or dark style, and that unsaved changes are included

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

#### Scenario: Print key
- **WHEN** the user looks under Files
- **THEN** "Print" shows "Ctrl+P"

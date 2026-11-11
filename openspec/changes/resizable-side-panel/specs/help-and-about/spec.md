# Spec Delta

## MODIFIED Requirements

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel with its folder tree and headings, the word count, Distraction Free and Full Screen, fonts and zoom, and files and links. Every user-visible feature SHALL be described there.

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

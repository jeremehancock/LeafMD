# help-and-about Specification

## Purpose
The built-in help: the Help window, the list of keyboard shortcuts, the markdown cheatsheet, and the About window, all reached from the main menu.

## Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL start with the zoom row (Zoom Out, the zoom percentage, Zoom In), then contain, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Find… and Find and Replace…; Side Panel, Line Numbers, Distraction Free and Full Screen; Preferences, Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD. Each item with a keyboard shortcut SHALL show it.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the zoom row is at the top, followed by the five sections, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O, Side Panel showing F9, Distraction Free showing Shift+Ctrl+D, Full Screen showing F11 and Preferences showing Ctrl+,

### Requirement: Ticks in the main menu
Side Panel SHALL show a tick while the panel is open, Line Numbers while that window shows line numbers, Distraction Free while that window is in it, and Full Screen while the window fills the screen.

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

### Requirement: Markdown cheatsheet
The Markdown Cheatsheet (Shift+F1, the Cheatsheet button on the formatting bar, or the main menu) SHALL show the most common markdown next to what it looks like, in six sections: Text, Headings, Lists, Links and Images, Blocks, and Paragraphs and More.

#### Scenario: Looking up bold
- **WHEN** the user opens the cheatsheet
- **THEN** `**Bold text**` is shown next to "Bold text" in bold

### Requirement: About window
About LeafMD SHALL show the app's name, icon, version and description, the developer (Jereme Hancock) with a link, an AI disclosure saying the project was created with the help of AI, a "Built With" section listing Rust, GTK, libadwaita, GtkSourceView and WebKitGTK with the versions LeafMD is running with plus comrak and ammonia, the GPL 3.0 license, a Website link to `https://leafmd.app/`, a GitHub link and a link for reporting issues.

#### Scenario: Checking library versions
- **WHEN** the user opens About and goes to the credits
- **THEN** the AI disclosure appears right under the developer, and the Built With section shows the running GTK version

#### Scenario: Visiting the website
- **WHEN** the user opens About and clicks Website
- **THEN** `https://leafmd.app/` opens in their web browser

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

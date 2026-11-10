# help-and-about Specification

## Purpose
The built-in help: the Help window, the list of keyboard shortcuts, the markdown cheatsheet, and the About window, all reached from the main menu.

## Requirements

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

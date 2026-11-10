# windows Specification

## Purpose
How LeafMD uses windows: where a file opens, new windows, a single running app shared by all of them, and never opening the same file twice.

## Requirements

### Requirement: Empty windows are reused
When opening a file from outside LeafMD, LeafMD SHALL use a window that is showing the "No Document Open" page, if there is one, instead of adding a tab or opening another window. A link SHALL always open in the window it was clicked in.

#### Scenario: Start page already open
- **WHEN** a LeafMD window shows "No Document Open" and the user double-clicks a markdown file in the file manager
- **THEN** the file opens in that window

#### Scenario: A link with an empty window elsewhere
- **WHEN** one window has README.md open, another shows "No Document Open", and the user clicks a link to LICENSE in README.md
- **THEN** LICENSE opens as a new tab beside README.md, and the empty window stays empty

### Requirement: Already open files are brought forward
LeafMD SHALL NOT open a file a second time, however the user asks to open it. It SHALL select the file's existing tab and bring its window to the front. If a link names a heading, that tab SHALL scroll to it.

#### Scenario: Link to an open file
- **WHEN** the user clicks a link to `guide.md#setup` and `guide.md` is already open in another window
- **THEN** that window comes to the front with guide.md's tab selected, scrolled to the "setup" heading

#### Scenario: Link to a file in another tab
- **WHEN** README.md and guide.md are tabs in one window, and the user clicks a link to guide.md in README.md
- **THEN** the guide.md tab is selected and no new tab opens

#### Scenario: Opening an open file from the file manager
- **WHEN** notes.md is a tab in a LeafMD window, todo.md is selected, and the user double-clicks notes.md in the file manager
- **THEN** that window comes to the front with notes.md selected, and unsaved changes in it are untouched

#### Scenario: Opening the same file twice quickly
- **WHEN** the user double-clicks notes.md in the file manager twice in a row, with LeafMD not running
- **THEN** one tab shows notes.md

### Requirement: Starting LeafMD again
Starting LeafMD from the app grid while it is already running SHALL bring an existing window to the front instead of starting a second copy of the app.

#### Scenario: Clicking the app icon again
- **WHEN** LeafMD is open and the user clicks its icon in the app grid
- **THEN** the existing window comes to the front

### Requirement: Smallest window size
A LeafMD window SHALL be usable down to 360 by 300 pixels (phone-sized screens) and SHALL NOT shrink further.

#### Scenario: Narrow window
- **WHEN** the user makes the window 360 pixels wide in Raw view
- **THEN** everything still fits, with the formatting bar scrolling sideways if needed

#### Scenario: Any view, with or without a document
- **WHEN** the window is dragged as narrow as it will go in Formatted view, in Split view, or on the "No Document Open" page
- **THEN** it reaches 360 pixels wide, the same as in Raw view

#### Scenario: Extra window buttons
- **WHEN** the desktop adds minimize and maximize buttons to the title bar and the window is 360 pixels wide
- **THEN** every button in the header bar is still fully visible

#### Scenario: Not smaller than the limit
- **WHEN** the user tries to drag the window to 300 by 200 pixels
- **THEN** it stops at 360 by 300 pixels

### Requirement: Closing
Ctrl+W SHALL close the current tab, and close the window when it's the last tab or there are none. Ctrl+Q SHALL close the current window with all its tabs. Both SHALL go through the unsaved-changes check first.

#### Scenario: Ctrl+Q
- **WHEN** the user presses Ctrl+Q in a window with three tabs and no unsaved changes
- **THEN** the current window closes with all three tabs, and other LeafMD windows stay open

#### Scenario: Ctrl+W with several tabs
- **WHEN** the user presses Ctrl+W in a window with three tabs and no unsaved changes
- **THEN** only the selected tab closes

#### Scenario: Ctrl+W on the start page
- **WHEN** the window shows "No Document Open" and the user presses Ctrl+W
- **THEN** the window closes

### Requirement: Compact layout in narrow windows
When a LeafMD window is narrower than 680 pixels, it SHALL use a compact layout: the New button, the Save button, the magnifying glass button and the view switcher SHALL move from the header bar to a bar along the bottom of the window, in that order. The side panel button, Open, its recent files arrow, the title, the main menu and the window's own buttons SHALL stay in the header bar. At 680 pixels or wider there SHALL be no bottom bar.

#### Scenario: Dragging a window narrower
- **WHEN** a window 800 pixels wide with notes.md open is dragged to 500 pixels wide
- **THEN** New, Save, the magnifying glass and the view switcher move to a bar along the bottom, and the header bar still shows the side panel button, Open, "notes.md", the main menu and the close button

#### Scenario: Dragging it wide again
- **WHEN** that window is dragged back to 800 pixels wide
- **THEN** the bottom bar is gone and the header bar looks exactly as it did before

#### Scenario: Opening at a narrow size
- **WHEN** LeafMD was last closed at 500 pixels wide and is opened again
- **THEN** it opens in the compact layout

#### Scenario: Nothing is lost while moved
- **WHEN** the window is 500 pixels wide, the find bar is open, the view is Split and the document has unsaved changes
- **THEN** the find bar stays open, the magnifying glass stays pressed in, the view switcher shows Split, the text and the cursor stay where they were, and the dot before the file name stays

#### Scenario: Shortcuts still work
- **WHEN** the window is 500 pixels wide and the user presses Ctrl+S, Ctrl+N, Ctrl+F or Ctrl+3
- **THEN** each does the same thing it does in a wide window

#### Scenario: Wide windows don't change
- **WHEN** the window is 1000 by 700 pixels and the side panel is closed
- **THEN** it looks exactly as it did before the compact layout existed, apart from the side panel button at the left end of the header bar

#### Scenario: Just above the limit with extra window buttons
- **WHEN** the desktop adds minimize and maximize buttons to the title bar and the window is 680 pixels wide
- **THEN** every header bar button is fully visible and there is no bottom bar

### Requirement: Opening files inside LeafMD uses a new tab
LeafMD SHALL open a file chosen with Open or Recent files, dropped on a window, or reached through a link, in a new selected tab in that same window. If the selected tab is an empty new document with nothing typed in it, the file SHALL take its place instead of adding a tab.

#### Scenario: Opening a second file
- **WHEN** notes.md is open and the user presses Ctrl+O and chooses todo.md
- **THEN** todo.md opens in a new tab and is selected, and notes.md stays open in its tab, without any question about unsaved changes

#### Scenario: Replacing an untouched new document
- **WHEN** the user presses Ctrl+T, types nothing, and then drops notes.md on the window
- **THEN** notes.md takes the place of the empty Untitled tab, so no empty tab is left behind

#### Scenario: A new document with text in it
- **WHEN** the user presses Ctrl+T, types "hello" and then drops notes.md on the window
- **THEN** notes.md opens in a new tab and the Untitled tab keeps "hello"

### Requirement: Opening files from outside LeafMD
A file opened from outside LeafMD SHALL open in a new window if none is open. Otherwise it SHALL use a window showing the "No Document Open" page if there is one, and failing that, open as a new selected tab in the window used most recently. That window SHALL come to the front.

#### Scenario: LeafMD not running
- **WHEN** LeafMD isn't running and the user double-clicks notes.md in the file manager
- **THEN** LeafMD starts with notes.md in one window, and doesn't also reopen earlier tabs

#### Scenario: LeafMD open with a document
- **WHEN** notes.md is open and the user double-clicks todo.md in the file manager
- **THEN** that window comes to the front with two tabs showing, and todo.md is the selected one

#### Scenario: Two windows with documents
- **WHEN** two LeafMD windows have documents, the second was used most recently, and the user runs `leafmd todo.md`
- **THEN** todo.md opens as a new tab in the second window

#### Scenario: A file that can't be opened
- **WHEN** notes.md is open and the user double-clicks a file that isn't valid UTF-8 text
- **THEN** no tab is added, notes.md stays selected, and a message says "This file isn't plain text, so LeafMD can't open it"

### Requirement: New windows
LeafMD SHALL open a new window showing the "No Document Open" page when the user presses Ctrl+N or chooses New Window. It SHALL NOT reopen earlier tabs, and SHALL use the view last chosen and the remembered window size.

#### Scenario: From the main menu
- **WHEN** notes.md is open and the user chooses New Window from the main menu
- **THEN** a second window opens on the "No Document Open" page, and notes.md stays open in the first

#### Scenario: Opening a file after that
- **WHEN** that empty window is open and the user double-clicks todo.md in the file manager
- **THEN** todo.md opens in the empty window

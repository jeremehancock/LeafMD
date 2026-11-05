# windows Specification

## Purpose
How LeafMD uses windows: one document per window, a single running app shared by all of them, and never opening the same file twice.

## Requirements

### Requirement: One document per window
LeafMD SHALL show one document per window. Opening another file with Open, Recent files or by dropping it SHALL replace the document in the same window (after the unsaved-changes check). Opening a file from outside LeafMD or from a link SHALL use its own window.

#### Scenario: Opening from the file manager while a file is open
- **WHEN** a document is open and the user double-clicks a different markdown file in the file manager
- **THEN** that file opens in a new LeafMD window and the first document stays open

### Requirement: Empty windows are reused
When opening a file from outside LeafMD or from a link, LeafMD SHALL use a window that is showing the "No Document Open" page, if there is one, instead of opening another window.

#### Scenario: Start page already open
- **WHEN** a LeafMD window shows "No Document Open" and the user double-clicks a markdown file in the file manager
- **THEN** the file opens in that window

### Requirement: Already open files are brought forward
LeafMD SHALL bring a file's existing window to the front instead of opening it a second time, when the file is opened from outside LeafMD or from a link. If the link names a heading, that window SHALL scroll to it.

#### Scenario: Link to an open file
- **WHEN** the user clicks a link to `guide.md#setup` and `guide.md` is already open in another window
- **THEN** that window comes to the front and scrolls to the "setup" heading

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
Ctrl+W and Ctrl+Q SHALL close the current window, going through the unsaved-changes check first.

#### Scenario: Ctrl+Q
- **WHEN** the user presses Ctrl+Q with no unsaved changes
- **THEN** the current window closes

### Requirement: Compact layout in narrow windows
When a LeafMD window is narrower than 640 pixels, it SHALL use a compact layout: the New button, the Save button, the magnifying glass button and the view switcher SHALL move from the header bar to a bar along the bottom of the window, in that order. Open, its recent files arrow, the title, the main menu and the window's own buttons SHALL stay in the header bar. At 640 pixels or wider there SHALL be no bottom bar.

#### Scenario: Dragging a window narrower
- **WHEN** a window 800 pixels wide with notes.md open is dragged to 500 pixels wide
- **THEN** New, Save, the magnifying glass and the view switcher move to a bar along the bottom, and the header bar still shows Open, "notes.md", the main menu and the close button

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
- **WHEN** the window is 1000 by 700 pixels
- **THEN** it looks exactly as it did before the compact layout existed

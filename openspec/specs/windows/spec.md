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

### Requirement: Closing
Ctrl+W and Ctrl+Q SHALL close the current window, going through the unsaved-changes check first.

#### Scenario: Ctrl+Q
- **WHEN** the user presses Ctrl+Q with no unsaved changes
- **THEN** the current window closes

# Spec Delta

## MODIFIED Requirements

### Requirement: Clearing the list
LeafMD SHALL offer "Clear Recent Files" at the bottom of the list whenever it has files in it, even if none of them are showing because the only one is the open file or the others are missing. Clearing SHALL empty the list and forget the last file.

#### Scenario: Clearing
- **WHEN** the user chooses Clear Recent Files
- **THEN** the list shows "No Recent Files"

#### Scenario: The only recent file is the open one
- **WHEN** notes.md is open, it is the only file on the list, and the user opens the arrow next to Open
- **THEN** the menu shows a greyed-out "No Recent Files" with Clear Recent Files under it

#### Scenario: Nothing on the list
- **WHEN** the list has just been cleared and the user opens the arrow next to Open
- **THEN** the menu shows only "No Recent Files"

#### Scenario: Clearing while a file is open
- **WHEN** notes.md is open, the user chooses Clear Recent Files, closes the window and starts LeafMD again
- **THEN** notes.md stays open until the window is closed, and LeafMD then starts on the "No Document Open" page

#### Scenario: Saving after clearing
- **WHEN** notes.md is open, the user chooses Clear Recent Files, saves notes.md with Ctrl+S, closes the window and starts LeafMD again
- **THEN** notes.md reopens, because saving put it back on the list

### Requirement: Reopening the last file
When started without a file (from the app grid), LeafMD SHALL reopen the file that was showing in the window closed last, if it still exists. If that isn't known, it SHALL use the newest recent file. A window with no saved file (the start page or a never-saved new document) SHALL NOT change which file reopens. Close Document and Clear Recent Files SHALL leave LeafMD with no file to reopen, until another file on the recent list is closed.

#### Scenario: Following a link and closing both windows
- **WHEN** the user opens README.md, clicks a link that opens LICENSE in a new window, closes LICENSE and then README.md, and starts LeafMD again
- **THEN** README.md reopens

#### Scenario: Last file was deleted
- **WHEN** the last file no longer exists
- **THEN** LeafMD starts on the "No Document Open" page

#### Scenario: Starting after Close Document
- **WHEN** the user chooses Close Document on notes.md, closes the window and starts LeafMD again
- **THEN** LeafMD starts on the "No Document Open" page, even though notes.md is still on the recent list

#### Scenario: A file closed after Close Document
- **WHEN** the user chooses Close Document in one window, then closes a second window showing todo.md last, and starts LeafMD again
- **THEN** todo.md reopens

# Spec Delta

## MODIFIED Requirements

### Requirement: Recent files list
LeafMD SHALL keep a list of the 10 most recently opened or saved files, newest first, shown under the arrow next to the Open button. Each entry SHALL show the file name with its folder in brackets, for example "README.md (LeafMD)", and choosing one SHALL open it in a new selected tab, or select its tab if it's already open in another window.

#### Scenario: Opening a file moves it to the top
- **WHEN** the user opens a file that is already third on the list
- **THEN** it moves to the top of the list

#### Scenario: The open file is left out
- **WHEN** notes.md and todo.md are tabs in a window and the user opens the list in that window
- **THEN** neither is listed, since both are already open there

#### Scenario: A file open in another window
- **WHEN** notes.md is open in one window, and the user chooses it from the list in a second window
- **THEN** the first window comes to the front with notes.md's tab selected

#### Scenario: Empty list
- **WHEN** there are no recent files to show
- **THEN** the list shows a greyed-out "No Recent Files"

### Requirement: Reopening the last file
When started without a file (from the app grid), LeafMD SHALL reopen the saved files that are on the recent list from the window closed last, in the same order, with the same tab selected, skipping any that no longer exist. Failing that it SHALL use the newest recent file. Never-saved documents SHALL NOT be restored, and a window with no saved file SHALL NOT change what reopens. Close Document on the last tab and Clear Recent Files SHALL leave nothing to reopen.

#### Scenario: Reopening several tabs
- **WHEN** a window has README.md, LICENSE and notes.md in that order, LICENSE is selected, the user presses Ctrl+Q and starts LeafMD again
- **THEN** one window opens with the same three tabs in the same order, and LICENSE is selected

#### Scenario: Following a link and closing both windows
- **WHEN** the user opens README.md, clicks a link that opens LICENSE in a new tab, moves LICENSE to a new window, closes LICENSE's window and then README.md's, and starts LeafMD again
- **THEN** README.md reopens

#### Scenario: Following a link and closing tabs one at a time
- **WHEN** the user opens README.md, clicks a link that opens LICENSE in a new tab, presses Ctrl+W to close LICENSE, Ctrl+W again to close README.md, and starts LeafMD again
- **THEN** README.md reopens by itself

#### Scenario: Two windows
- **WHEN** one window has a.md and b.md open, a second has c.md, the user closes the first and then the second, and starts LeafMD again
- **THEN** only c.md reopens, and a.md and b.md are on the recent list

#### Scenario: Last file was deleted
- **WHEN** the last file no longer exists
- **THEN** LeafMD starts on the "No Document Open" page

#### Scenario: One of several tabs was deleted
- **WHEN** a window was closed with a.md, b.md and c.md with b.md selected, b.md has since been deleted, and LeafMD starts
- **THEN** a.md and c.md reopen, in that order, and one of them is selected

#### Scenario: More tabs than the list holds
- **WHEN** a window has 12 saved files open, the user closes it and starts LeafMD again
- **THEN** the 10 on the recent list reopen, and the two opened longest ago don't, because only files on the list reopen

#### Scenario: A never-saved tab
- **WHEN** a window has notes.md and an empty Untitled tab, Untitled is selected, the user presses Ctrl+Q and starts LeafMD again
- **THEN** notes.md reopens by itself and is selected

#### Scenario: Starting after Close Document
- **WHEN** the user chooses Close Document on notes.md, the only tab, closes the window and starts LeafMD again
- **THEN** LeafMD starts on the "No Document Open" page, even though notes.md is still on the recent list

#### Scenario: A file closed after Close Document
- **WHEN** the user chooses Close Document in one window, then closes a second window showing todo.md last, and starts LeafMD again
- **THEN** todo.md reopens

#### Scenario: Files that aren't on the list after clearing
- **WHEN** notes.md and todo.md are open, the user chooses Clear Recent Files, saves only todo.md, closes the window and starts LeafMD again
- **THEN** todo.md reopens by itself, because saving put it back on the list

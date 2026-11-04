# recent-files Specification

## Purpose
The list of recently opened files under the arrow next to Open, and reopening the last file when LeafMD starts.

## Requirements

### Requirement: Recent files list
LeafMD SHALL keep a list of the 10 most recently opened or saved files, newest first, shown under the arrow next to the Open button. Each entry SHALL show the file name with its folder in brackets, for example "README.md (LeafMD)", and choosing one SHALL open it after the unsaved-changes check.

#### Scenario: Opening a file moves it to the top
- **WHEN** the user opens a file that is already third on the list
- **THEN** it moves to the top of the list

#### Scenario: The open file is left out
- **WHEN** the user opens the list while a file is open
- **THEN** that file isn't listed, since it's already open

#### Scenario: Empty list
- **WHEN** there are no recent files to show
- **THEN** the list shows a greyed-out "No Recent Files"

### Requirement: Missing files are hidden but kept
LeafMD SHALL hide files that don't exist right now from the list, but keep them on it, because they may be on a drive that is only unplugged. A recent file that fails to open SHALL be removed from the list.

#### Scenario: Unplugged drive
- **WHEN** a recent file is on a USB drive that isn't plugged in
- **THEN** it doesn't appear in the list, and reappears once the drive is plugged back in

#### Scenario: File deleted
- **WHEN** a file on the list is deleted in the file manager and the user switches back to LeafMD
- **THEN** it no longer appears in the list

### Requirement: Clearing the list
LeafMD SHALL offer "Clear Recent Files" at the bottom of the list whenever it has files in it, which empties the list.

#### Scenario: Clearing
- **WHEN** the user chooses Clear Recent Files
- **THEN** the list shows "No Recent Files"

### Requirement: Shared between windows
All LeafMD windows SHALL share one recent files list, and each window SHALL pick up changes made by the others when it is switched to.

#### Scenario: Two windows
- **WHEN** the user opens a file in one window and then switches to another LeafMD window
- **THEN** that file is in the second window's list

### Requirement: Reopening the last file
When started without a file (from the app grid), LeafMD SHALL reopen the file that was showing in the window closed last, if it still exists. If that isn't known, it SHALL use the newest recent file. A window with no saved file (the start page or a never-saved new document) SHALL NOT change which file reopens.

#### Scenario: Following a link and closing both windows
- **WHEN** the user opens README.md, clicks a link that opens LICENSE in a new window, closes LICENSE and then README.md, and starts LeafMD again
- **THEN** README.md reopens

#### Scenario: Last file was deleted
- **WHEN** the last file no longer exists
- **THEN** LeafMD starts on the "No Document Open" page

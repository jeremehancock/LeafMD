# Spec Delta

## ADDED Requirements

### Requirement: Folders on the command line
`leafmd` SHALL accept a folder (`leafmd .`, `leafmd ~/notes`), opening the side panel with that folder as the window's chosen folder, even if the panel was closed, and remembering the panel as open. A window that already has that folder SHALL come to the front. Otherwise a window showing "No Document Open" with no folder chosen SHALL be used, or failing that a new window. Files on the same command line SHALL open as tabs in that window.

#### Scenario: LeafMD not running
- **WHEN** LeafMD isn't running and the user runs `leafmd .` in ~/notes
- **THEN** one window opens on the "No Document Open" page with the panel showing ~/notes, and earlier tabs aren't reopened

#### Scenario: LeafMD open with documents
- **WHEN** todo.md is open and the user runs `leafmd .` in ~/notes
- **THEN** a new window opens with the panel showing ~/notes, and todo.md's window is left as it was

#### Scenario: The folder is already open
- **WHEN** a window already shows ~/notes as its chosen folder and the user runs `leafmd .` in ~/notes again
- **THEN** that window comes to the front, and no new window opens

#### Scenario: A folder and a file
- **WHEN** the user runs `leafmd . README.md` in ~/notes
- **THEN** one window shows ~/notes in the panel and README.md in a tab

#### Scenario: A folder outside the home folder
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd .` in /tmp/notes
- **THEN** the panel lists the files in /tmp/notes

#### Scenario: Without the short command
- **WHEN** the user runs `flatpak run io.github.jeremehancock.LeafMD .` in /tmp/notes
- **THEN** the panel says "LeafMD hasn't been given access to this folder" and offers Open Folder…, instead of listing a different, empty /tmp/notes

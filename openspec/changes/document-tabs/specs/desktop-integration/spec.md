# Spec Delta

## ADDED Requirements

### Requirement: New Window from the app launcher
LeafMD SHALL list a "New Window" item for the desktop to show when its app icon is right-clicked. Choosing it SHALL open a new window on the "No Document Open" page, whether or not LeafMD is already running, the same as `leafmd --new-window`. Existing windows SHALL stay as they are.

#### Scenario: LeafMD is already open
- **WHEN** notes.md is open and the user right-clicks LeafMD's icon and chooses New Window
- **THEN** a second window opens on the "No Document Open" page, and notes.md stays open in the first

#### Scenario: LeafMD isn't running
- **WHEN** LeafMD isn't running and the user right-clicks its icon in the app grid and chooses New Window
- **THEN** LeafMD starts with one window on the "No Document Open" page, and doesn't reopen earlier tabs

#### Scenario: From a terminal
- **WHEN** the user runs `leafmd --new-window` in a terminal
- **THEN** a new window opens and the prompt comes back straight away

## MODIFIED Requirements

### Requirement: Command line
`leafmd` SHALL accept files to open (`leafmd notes.md todo.md`), opening them as tabs following the rules for files from outside LeafMD, with the last one selected, and with no files SHALL behave like starting from the app grid. On a Flatpak install, the short name `leafmd` SHALL be available after the one-time setup described on the storefront. The full `flatpak run io.github.jeremehancock.LeafMD` command SHALL accept the same arguments.

#### Scenario: Two files
- **WHEN** LeafMD isn't running and the user runs `leafmd a.md b.md`
- **THEN** one window opens with both files as tabs, and b.md is selected

#### Scenario: Two files with a document already open
- **WHEN** notes.md is open and the user runs `leafmd a.md b.md`
- **THEN** a.md and b.md open as new tabs beside notes.md, and b.md is selected

#### Scenario: A file outside the home folder
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd /tmp/outside.md`
- **THEN** the file opens, even though LeafMD can't browse `/tmp` itself

#### Scenario: A file name with spaces
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd 'my notes.md'` in the folder holding that file
- **THEN** the file opens, with "my notes.md" as its name

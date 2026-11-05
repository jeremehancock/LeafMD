# Spec Delta

## MODIFIED Requirements

### Requirement: Command line
`leafmd` SHALL accept files to open (`leafmd notes.md todo.md`), opening each in its own window, and with no files SHALL behave like starting from the app grid. On a Flatpak install, the short name `leafmd` SHALL be available after the one-time setup described on the storefront. The full `flatpak run io.github.jeremehancock.LeafMD` command SHALL accept the same arguments.

#### Scenario: Two files
- **WHEN** the user runs `leafmd a.md b.md`
- **THEN** both files open, each in its own window

#### Scenario: A file outside the home folder
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd /tmp/outside.md`
- **THEN** the file opens, even though LeafMD can't browse `/tmp` itself

#### Scenario: A file name with spaces
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd 'my notes.md'` in the folder holding that file
- **THEN** the file opens, with "my notes.md" as its name

## ADDED Requirements

### Requirement: Giving the terminal back
When started from a terminal, LeafMD SHALL return the prompt straight away and keep running on its own, so closing the terminal doesn't close LeafMD. Started any other way, such as from the app grid or the file manager, it SHALL behave as before.

#### Scenario: Starting LeafMD for the first time
- **GIVEN** no LeafMD window is open
- **WHEN** the user runs `leafmd notes.md` in a terminal
- **THEN** the prompt comes back straight away, and notes.md opens in a window

#### Scenario: The long command
- **GIVEN** no LeafMD window is open
- **WHEN** the user runs `flatpak run io.github.jeremehancock.LeafMD notes.md` in a terminal
- **THEN** the prompt comes back straight away, and notes.md opens in a window

#### Scenario: Closing the terminal
- **GIVEN** LeafMD was started from a terminal
- **WHEN** the user closes that terminal
- **THEN** LeafMD stays open, along with any unsaved changes

#### Scenario: A file that doesn't exist
- **WHEN** the user runs `leafmd missing.md` in a terminal
- **THEN** LeafMD's window shows that the file could not be opened, and the terminal shows no error

#### Scenario: Output sent somewhere else
- **WHEN** the user runs `flatpak run io.github.jeremehancock.LeafMD > log.txt` in a terminal
- **THEN** LeafMD stays attached and writes its messages to log.txt, because the user asked to keep them

### Requirement: Staying attached to the terminal
`--foreground` SHALL keep LeafMD attached to the terminal it was started from, showing its messages there until it closes. Any other command line option, such as `--help`, SHALL also print to the terminal rather than being lost.

#### Scenario: Seeing what went wrong
- **GIVEN** no LeafMD window is open
- **WHEN** the user runs `leafmd --foreground notes.md`
- **THEN** notes.md opens, messages appear in the terminal, and the prompt comes back when the last LeafMD window closes

#### Scenario: Asking for help
- **WHEN** the user runs `leafmd --help`
- **THEN** the list of options, including `--foreground`, is printed in the terminal

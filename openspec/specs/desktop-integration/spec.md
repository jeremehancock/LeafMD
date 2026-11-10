# desktop-integration Specification

## Purpose
How LeafMD fits into the desktop: its app grid entry, the file types it opens, starting it from the command line, and the Flatpak permissions it asks for and why.

## Requirements

### Requirement: App grid entry
LeafMD SHALL appear in the app grid as "LeafMD" (described as a Markdown Viewer) with its own icon, in the Utility and Text Editor categories, and SHALL be found by searching for markdown, md, viewer, editor, notes or readme.

#### Scenario: Searching the app grid
- **WHEN** the user types "readme" in the GNOME app search
- **THEN** LeafMD is offered

### Requirement: Same icons on every distro
LeafMD SHALL look the same whatever icon theme the system has chosen: its buttons and window controls SHALL use LeafMD's own icons, never the system's icon theme. Small icons GTK adds by itself, like the arrow beside Open, SHALL use the Adwaita icons from the GNOME runtime.

#### Scenario: A system with a different icon theme
- **WHEN** LeafMD runs on a system set to another icon theme, such as Papirus or Ubuntu's Yaru
- **THEN** the main menu button shows three lines, and every other icon matches what LeafMD shows on any other distro

#### Scenario: Maximizing a window
- **WHEN** the window isn't maximized
- **THEN** the maximize button shows two arrows pointing out to opposite corners, and once the user maximizes the window it shows a rounded square

### Requirement: Markdown files open in LeafMD
LeafMD SHALL register itself for markdown files (`text/markdown` and `text/x-markdown`), so it can be chosen to open them from the file manager, and SHALL accept several files at once.

#### Scenario: Open With
- **WHEN** the user right-clicks a `.md` file and chooses Open With
- **THEN** LeafMD is in the list

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

### Requirement: Flatpak permissions
The Flatpak SHALL ask only for what LeafMD needs: the home folder (so images and links next to a markdown file work in the formatted view), the network (so images hosted online, like README badges, load), and the graphics card (so the formatted view draws faster), plus the usual display access. Any new permission MUST be explained in the README's Permissions table.

#### Scenario: Online image
- **WHEN** a document shows a badge image from a website
- **THEN** it loads in the formatted view

### Requirement: App store listing
The app store description SHALL list LeafMD's main features and stay in step with them, and the app SHALL be described as working with keyboard and mouse on screens at least 360 pixels wide. The listing SHALL give `https://leafmd.app/` as LeafMD's homepage, SHALL have a light and a dark brand colour, and SHALL show screenshots of the formatted, raw and split views in light style (formatted first), plus the split view in dark style, each with a one-sentence caption.

#### Scenario: A feature is added
- **WHEN** a new user-visible feature ships
- **THEN** the description in the metainfo file mentions it

#### Scenario: The interface changes before a release
- **GIVEN** a change since the screenshots were last taken adds a button to the formatting bar
- **WHEN** that version is released
- **THEN** the screenshots in the listing and on `https://leafmd.app/` show the new button, unless the owner chose to release with the old ones

#### Scenario: Viewing LeafMD in GNOME Software
- **WHEN** someone opens LeafMD's page in GNOME Software
- **THEN** it shows the formatted view screenshot first, with the other three after it
- **AND** the Website link opens `https://leafmd.app/`

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

#### Scenario: From a folder outside the home folder
- **GIVEN** the user has done the one-time setup
- **WHEN** the user runs `leafmd --new-window` with the terminal in `/tmp`
- **THEN** a new window opens, rather than Flatpak treating the option as a file

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

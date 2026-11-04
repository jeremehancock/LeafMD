# desktop-integration Specification

## Purpose
How LeafMD fits into the desktop: its app grid entry, the file types it opens, starting it from the command line, and the Flatpak permissions it asks for and why.

## Requirements

### Requirement: App grid entry
LeafMD SHALL appear in the app grid as "LeafMD" (described as a Markdown Viewer) with its own icon, in the Utility and Text Editor categories, and SHALL be found by searching for markdown, md, viewer, editor, notes or readme.

#### Scenario: Searching the app grid
- **WHEN** the user types "readme" in the GNOME app search
- **THEN** LeafMD is offered

### Requirement: Markdown files open in LeafMD
LeafMD SHALL register itself for markdown files (`text/markdown` and `text/x-markdown`), so it can be chosen to open them from the file manager, and SHALL accept several files at once.

#### Scenario: Open With
- **WHEN** the user right-clicks a `.md` file and chooses Open With
- **THEN** LeafMD is in the list

### Requirement: Command line
`leafmd` SHALL accept files to open (`leafmd notes.md todo.md`), opening each in its own window, and with no files SHALL behave like starting from the app grid.

#### Scenario: Two files
- **WHEN** the user runs `leafmd a.md b.md`
- **THEN** both files open, each in its own window

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

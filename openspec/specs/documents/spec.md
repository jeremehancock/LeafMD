# documents Specification

## Purpose
Starting, opening and saving documents, keeping unsaved changes safe, and showing which file is open.

## Requirements

### Requirement: Start page
LeafMD SHALL show a "No Document Open" page with an "Open File…" button when no document is open, and the actions that need a document (Save, Save As, Close Document, the view switcher, find) SHALL be greyed out.

#### Scenario: Nothing to reopen
- **WHEN** LeafMD starts and there is no previous file to reopen
- **THEN** the "No Document Open" page is shown

### Requirement: Open, New and Save buttons
The left of the header bar SHALL have an Open button with a small arrow beside it that lists recent files, followed by a New button. A Save button SHALL sit just to the left of the view switcher. Open and Save SHALL be words rather than icons, because icons for those actions vary a lot between icon themes. New SHALL use LeafMD's own new document icon. Open and its arrow SHALL look raised, sharing one background, as Save does.

#### Scenario: Hovering the buttons
- **WHEN** the user hovers over Open, New or Save
- **THEN** a tooltip names the action and its shortcut, like "Save (Ctrl+S)" or "New document (Ctrl+N)"

#### Scenario: Save icon on any distro
- **WHEN** LeafMD runs with any system icon theme
- **THEN** the Save button shows the word "Save", and the New button shows the same new document icon

#### Scenario: Clicking New with unsaved changes
- **WHEN** the user has typed in notes.md without saving and clicks the New button
- **THEN** LeafMD asks "Save Changes?", the same as pressing Ctrl+N

#### Scenario: The start page
- **WHEN** the "No Document Open" page is showing
- **THEN** the New button can be clicked and the Save button is greyed out

### Requirement: Opening files
LeafMD SHALL open a markdown or text file chosen with Open (Ctrl+O or the Open button), dropped anywhere on the window, double-clicked in the file manager, or given on the command line (`leafmd notes.md`). The Open dialog SHALL offer Markdown (.md, .markdown, .mdown, .mkd), Plain text and All files filters.

#### Scenario: Dropping a file
- **WHEN** the user drops a markdown file onto the window
- **THEN** that file opens in the window, after the unsaved-changes check

#### Scenario: Not plain text
- **WHEN** the user opens a file that isn't valid UTF-8 text
- **THEN** the file is not opened and a message says "This file isn't plain text, so LeafMD can't open it"

#### Scenario: Can't be read
- **WHEN** a file can't be read (for example it was deleted)
- **THEN** a message says the file could not be opened, and it is removed from Recent files

### Requirement: A freshly opened file
A file that has just been opened SHALL start with the cursor and scroll position at the top, marked as having no unsaved changes, and opening it SHALL NOT be something Ctrl+Z can undo.

#### Scenario: Undo straight after opening
- **WHEN** the user opens a file and presses Ctrl+Z before typing anything
- **THEN** nothing changes, rather than the text disappearing

### Requirement: Opening without a flash
While a file opens in an empty window, LeafMD SHALL show a blank page instead of "No Document Open", and SHALL reveal the document once the formatted view has drawn it, or after one second at most.

#### Scenario: Starting with the last file
- **WHEN** LeafMD starts and reopens the last file
- **THEN** the window never shows "No Document Open" first

### Requirement: New documents
LeafMD SHALL start an empty, unsaved document called "Untitled" when the user chooses New (Ctrl+N), after the unsaved-changes check.

#### Scenario: Saving a new document
- **WHEN** the user saves a new document
- **THEN** a save dialog opens, suggesting the name "Untitled.md"

### Requirement: Saving
LeafMD SHALL save to the open file with Save (Ctrl+S), and ask for a location with Save As (Ctrl+Shift+S). Saving MUST write to a temporary file first and then swap it in, so a crash or full disk part-way through can't leave a half-written file. A "Saved" message SHALL confirm success, and a failure SHALL show what went wrong.

#### Scenario: Typing while saving
- **WHEN** the user types while a save is still being written
- **THEN** the document is still marked as having unsaved changes afterwards

#### Scenario: Save fails
- **WHEN** the file can't be written
- **THEN** a message says "Could not save:" followed by the reason, and the document stays marked as unsaved

### Requirement: Unsaved changes are never lost silently
LeafMD SHALL show a dot (•) before the file name while there are unsaved changes, and SHALL ask "Save Changes?" with Cancel, Discard and Save before closing a window, starting a new document, or opening another file in the same window with unsaved changes. Save SHALL be the default answer, and the action SHALL only go ahead if the save worked.

#### Scenario: Closing with changes
- **WHEN** the user presses Ctrl+W with unsaved changes and chooses Cancel
- **THEN** the window stays open with the changes intact

#### Scenario: Save fails during close
- **WHEN** the user chooses Save when closing but the save fails
- **THEN** the window stays open

### Requirement: The title shows the file
LeafMD SHALL show the open file's name as the window title, with its folder underneath (shortened to `~` for the home folder), and the window's name SHALL be "file name - LeafMD".

#### Scenario: A file in the home folder
- **WHEN** `~/notes/todo.md` is open with unsaved changes
- **THEN** the title reads "• todo.md" with "~/notes" underneath

### Requirement: The window size is remembered
LeafMD SHALL remember the window's size and whether it was maximized, and use them for new windows.

#### Scenario: Reopening after resizing
- **WHEN** the user maximizes the window, closes LeafMD and starts it again
- **THEN** the window opens maximized

### Requirement: Closing a document
LeafMD SHALL offer Close Document in the main menu. After the unsaved-changes check, it SHALL close the document and show the "No Document Open" page in the same window, leaving the window open. It SHALL work for saved files and for new, never-saved documents.

#### Scenario: Closing a saved file
- **WHEN** notes.md is open with no unsaved changes and the user chooses Close Document
- **THEN** the window stays open and shows the "No Document Open" page, and its title says "LeafMD"

#### Scenario: Unsaved changes
- **WHEN** the user types in notes.md without saving and chooses Close Document
- **THEN** LeafMD asks "Save Changes?" before closing it, and Cancel leaves the document open and unchanged

#### Scenario: Closing a new document
- **WHEN** the user presses Ctrl+N, types nothing, and chooses Close Document
- **THEN** the "No Document Open" page is shown, and nothing is asked because there is nothing to save

#### Scenario: Getting the file back
- **WHEN** the user closes notes.md with Close Document and opens the arrow next to Open
- **THEN** notes.md is on the recent files list, and choosing it opens it again

#### Scenario: Ctrl+W still closes the window
- **WHEN** a document is open and the user presses Ctrl+W
- **THEN** the whole window closes, as it did before

# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Start page
LeafMD SHALL show a "No Document Open" page with an "Open File…" button when no document is open, and the actions that need a document (Save, Save As, Close Document, the view switcher, find) SHALL be greyed out.

#### Scenario: Nothing to reopen
- **WHEN** LeafMD starts and there is no previous file to reopen
- **THEN** the "No Document Open" page is shown

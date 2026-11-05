# Spec Delta

## ADDED Requirements

### Requirement: Banner when another app changes the file
When the contents of the open file on disk differ from the version LeafMD last opened, saved or ignored, LeafMD SHALL show a banner across the top of the window, under the header bar, in every view. The document's text SHALL NOT change until the user chooses. A new, never-saved document SHALL never show the banner.

#### Scenario: Another app edits the file
- **WHEN** notes.md is open with no unsaved changes and another app adds a line to it
- **THEN** a banner says "“notes.md” was changed by another app" with Ignore and Reload buttons, and the text in LeafMD stays as it was

#### Scenario: An app that saves by swapping in a new file
- **WHEN** notes.md is open and an editor saves it by writing a new file and renaming it over notes.md
- **THEN** one banner says the file was changed, and it does not say the file was deleted

#### Scenario: Only the formatted view is showing
- **WHEN** only the formatted view is showing and another app changes the file
- **THEN** the banner appears above the formatted view

#### Scenario: Typing isn't interrupted
- **WHEN** the user is typing in the raw text and the banner appears
- **THEN** the keyboard stays in the raw text and the next letters typed go into the document

#### Scenario: The file changes several times
- **WHEN** the banner is showing and another app changes the file again
- **THEN** one banner is still showing, and Reload brings in the newest version

#### Scenario: The change is undone
- **WHEN** the banner is showing and the other app puts the file back exactly as LeafMD last knew it
- **THEN** the banner goes away by itself

### Requirement: No banner for changes that don't matter
LeafMD SHALL NOT show the banner when the file's contents on disk match the version it last opened, saved or ignored.

#### Scenario: LeafMD's own save
- **WHEN** the user presses Ctrl+S
- **THEN** the "Saved" message appears and no banner appears

#### Scenario: Same contents rewritten
- **WHEN** another app updates the file's modified time, or rewrites it with exactly the same text
- **THEN** no banner appears

### Requirement: Reloading the file
Reload SHALL replace the document's text with the newest version of the file, mark it as having no unsaved changes, close the banner, and keep the cursor and scroll position as close as possible to where they were. Reloading SHALL clear the undo history, as opening a file does.

#### Scenario: Reading a long document
- **WHEN** the user is halfway down a long document, with the cursor on line 200, and chooses Reload
- **THEN** the view stays around the same place and the cursor stays on line 200, rather than jumping to the top

#### Scenario: The file got shorter
- **WHEN** the cursor is on line 200 and the new version only has 50 lines
- **THEN** the cursor moves to the end of the document

#### Scenario: Undo after reloading
- **WHEN** the user chooses Reload and then presses Ctrl+Z
- **THEN** nothing changes

#### Scenario: The new version isn't plain text
- **WHEN** another app replaced the file with one that isn't valid UTF-8 text and the user chooses Reload
- **THEN** a message says "This file isn't plain text, so LeafMD can't open it", the document's text stays, and the banner stays

### Requirement: Reloading with unsaved changes
When the document has unsaved changes, the banner SHALL say so, and its reload button SHALL read "Discard and Reload" and be marked as destructive (shown in red). The banner's words and button SHALL update if unsaved changes appear or go away while it's showing.

#### Scenario: Both versions changed
- **WHEN** the user has typed in notes.md without saving and another app changes the file
- **THEN** the banner says "“notes.md” was changed by another app, and you have unsaved changes" with Ignore and Discard and Reload buttons

#### Scenario: Keeping unsaved changes
- **WHEN** that banner is showing and the user keeps typing
- **THEN** the typing goes into the document and nothing is lost

#### Scenario: Choosing to discard
- **WHEN** that banner is showing and the user chooses Discard and Reload
- **THEN** the unsaved changes are gone, the other app's version shows, and the dot leaves the title

#### Scenario: Starting to type while the banner shows
- **WHEN** the banner shows Reload, and the user then types a letter
- **THEN** the button changes to Discard and Reload and the banner mentions the unsaved changes

### Requirement: Ignoring a change
Ignore SHALL close the banner and leave the document exactly as it is, including whether it has unsaved changes. LeafMD SHALL treat the version on disk at that moment as seen: it SHALL NOT mention that version again, and saving afterwards SHALL replace it without asking.

#### Scenario: Ignoring and then saving
- **WHEN** the user chooses Ignore and then presses Ctrl+S
- **THEN** LeafMD saves its own text over the other app's version without asking, because the user has already decided

#### Scenario: Another change after ignoring
- **WHEN** the user chooses Ignore and another app then changes the file again
- **THEN** the banner appears again

### Requirement: The file is deleted or moved
When the open file no longer exists, LeafMD SHALL keep the document open with its text unchanged and show a banner saying "“notes.md” was deleted or moved by another app", with Ignore and Save Again buttons. Save Again SHALL write the text back to the same place. If the file comes back, LeafMD SHALL treat it as any other change.

#### Scenario: Saving it again
- **WHEN** notes.md is deleted and the user chooses Save Again
- **THEN** notes.md exists again with the text shown in LeafMD, the "Saved" message appears, and the banner closes

#### Scenario: Pressing Ctrl+S instead
- **WHEN** the deleted-file banner is showing and the user presses Ctrl+S
- **THEN** the file is written back without asking, because there is nothing to overwrite, and the banner closes

#### Scenario: Ignoring a deleted file
- **WHEN** notes.md is deleted, the user chooses Ignore, and the document has no unsaved changes
- **THEN** the banner closes, the text stays on screen, and closing the window doesn't ask to save

#### Scenario: Renaming the file in the file manager
- **WHEN** notes.md is renamed to ideas.md while it's open
- **THEN** the banner says it was deleted or moved, and LeafMD still shows the name notes.md

#### Scenario: A brief disappearance
- **WHEN** another app deletes notes.md and creates it again within a moment, with the same text
- **THEN** no banner appears

### Requirement: Checking when returning to the window
LeafMD SHALL check the open file again each time its window becomes the active window, so changes it wasn't notified of still appear.

#### Scenario: A file on a network drive
- **WHEN** a file on a drive that sends no change notices is changed elsewhere, and the user switches back to LeafMD
- **THEN** the banner appears

#### Scenario: A file outside the home folder
- **WHEN** a file outside the home folder is open, is changed by another app, and the user switches back to LeafMD
- **THEN** the banner appears

### Requirement: The banner belongs to the open file
LeafMD SHALL close the banner, and stop checking the old file, when the document is closed, or when the window starts a new document or opens or saves as a different file. It SHALL NOT show a banner about a file that's no longer open.

#### Scenario: Opening another file
- **WHEN** the banner is showing for notes.md and the user opens todo.md in the same window
- **THEN** the banner closes, and later changes to notes.md don't bring it back

#### Scenario: Save As
- **WHEN** the banner is showing for notes.md and the user saves the document as copy.md
- **THEN** the banner closes, and LeafMD now watches copy.md

### Requirement: Saving over a change made elsewhere
Before saving to the open file, LeafMD SHALL check whether the file on disk differs from the version it last opened, saved or ignored. If so, it SHALL ask before replacing it, with Cancel and Save Anyway, where Cancel is the default. Save Anyway SHALL be marked as destructive. Cancel SHALL leave the file and the document as they were. If the check can't read the file, LeafMD SHALL go ahead and let the save report any problem.

#### Scenario: Pressing Ctrl+S while the banner shows
- **WHEN** the banner says notes.md was changed, and the user presses Ctrl+S
- **THEN** LeafMD asks "Replace the Other Version?" and the file isn't touched unless the user chooses Save Anyway

#### Scenario: A change LeafMD hadn't noticed yet
- **WHEN** a file on a drive that sends no change notices was changed elsewhere, and the user presses Ctrl+S
- **THEN** LeafMD asks "Replace the Other Version?" instead of overwriting it

#### Scenario: Saving while closing
- **WHEN** the user closes the window, chooses Save at "Save Changes?", and then chooses Cancel at "Replace the Other Version?"
- **THEN** the window stays open with the unsaved changes intact

## MODIFIED Requirements

### Requirement: Saving
LeafMD SHALL save to the open file with Save (Ctrl+S), and ask for a location with Save As (Ctrl+Shift+S). Saving MUST write to a temporary file first and then swap it in, so a crash or full disk part-way through can't leave a half-written file. A "Saved" message SHALL confirm success, and a failure SHALL show what went wrong. Save SHALL first ask before replacing a version of the file that was changed elsewhere (see "Saving over a change made elsewhere").

#### Scenario: Typing while saving
- **WHEN** the user types while a save is still being written
- **THEN** the document is still marked as having unsaved changes afterwards

#### Scenario: Save fails
- **WHEN** the file can't be written
- **THEN** a message says "Could not save:" followed by the reason, and the document stays marked as unsaved

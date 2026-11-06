# Spec Delta

## MODIFIED Requirements

### Requirement: Open, New and Save buttons
The left of the header bar SHALL have an Open button with a small arrow beside it that lists recent files, followed by a New button that opens a new tab. A Save button SHALL sit just to the left of the view switcher. Below 640 pixels wide, New and Save SHALL be in the bottom bar. Open and Save SHALL be words rather than icons, because icons for those actions vary a lot between icon themes. New SHALL use LeafMD's own new document icon. Open and its arrow SHALL look raised, sharing one background, as Save does.

#### Scenario: Hovering the buttons
- **WHEN** the user hovers over Open, New or Save
- **THEN** a tooltip names the action and its shortcut, like "Save (Ctrl+S)" or "New document (Ctrl+T)"

#### Scenario: Save icon on any distro
- **WHEN** LeafMD runs with any system icon theme
- **THEN** the Save button shows the word "Save", and the New button shows the same new document icon

#### Scenario: Clicking New with unsaved changes
- **WHEN** the user has typed in notes.md without saving and clicks the New button
- **THEN** a new Untitled tab opens without asking anything, and notes.md keeps its unsaved changes in its own tab

#### Scenario: The start page
- **WHEN** the "No Document Open" page is showing
- **THEN** the New button can be clicked and the Save button is greyed out

#### Scenario: In the bottom bar
- **WHEN** the window is 500 pixels wide, the "No Document Open" page is showing, and the user clicks New in the bottom bar
- **THEN** a new document opens, and Save, which was greyed out in the bottom bar, can now be clicked

### Requirement: Opening files
LeafMD SHALL open a markdown or text file chosen with Open (Ctrl+O or the Open button), dropped anywhere on the window, double-clicked in the file manager, or given on the command line (`leafmd notes.md`). The Open dialog SHALL offer Markdown (.md, .markdown, .mdown, .mkd), Plain text and All files filters.

#### Scenario: Dropping a file
- **WHEN** the user drops a markdown file onto the window while another document is open
- **THEN** that file opens in a new selected tab in the window, and nothing is asked about unsaved changes because the other document stays open

#### Scenario: Not plain text
- **WHEN** the user opens a file that isn't valid UTF-8 text
- **THEN** the file is not opened and a message says "This file isn't plain text, so LeafMD can't open it"

#### Scenario: Can't be read
- **WHEN** a file can't be read (for example it was deleted)
- **THEN** a message says the file could not be opened, no tab is added, and it is removed from Recent files

### Requirement: New documents
LeafMD SHALL start an empty, unsaved document called "Untitled" in a new selected tab when the user clicks the New button, chooses New from the main menu or presses Ctrl+T. Nothing SHALL be asked, because no other document is replaced.

#### Scenario: Saving a new document
- **WHEN** the user saves a new document
- **THEN** a save dialog opens, suggesting the name "Untitled.md"

#### Scenario: A new tab beside a document
- **WHEN** notes.md is open and the user presses Ctrl+T
- **THEN** the tab bar appears with notes.md and Untitled, and Untitled is selected

#### Scenario: Several new documents
- **WHEN** the user presses Ctrl+T twice
- **THEN** two tabs called Untitled open, each with its own text

### Requirement: Unsaved changes are never lost silently
LeafMD SHALL show a dot (•) before the file name while there are unsaved changes, and SHALL ask "Save changes to document “notes.md” before closing?" with Close without Saving, Cancel and Save, side by side as in gedit, before closing a tab or window with unsaved changes. Only Close without Saving SHALL have a colour (red). Save SHALL be the default answer, and the action SHALL only go ahead if the save worked.

#### Scenario: Closing with changes
- **WHEN** the user presses Ctrl+W on the only tab, with unsaved changes, and chooses Cancel
- **THEN** the window stays open with the changes intact

#### Scenario: Save fails during close
- **WHEN** the user chooses Save when closing but the save fails
- **THEN** the window stays open

#### Scenario: A never-saved document
- **WHEN** the user closes an Untitled document with text in it
- **THEN** the last button reads "Save As…" and opens a save dialog

#### Scenario: The buttons in a narrow window
- **WHEN** the window is 360 pixels wide and LeafMD asks about unsaved changes
- **THEN** the buttons stack, because they don't fit side by side, and all three are fully visible

#### Scenario: Opening doesn't ask
- **WHEN** the user has unsaved changes in notes.md and opens todo.md from Recent files
- **THEN** nothing is asked, todo.md opens in a new tab, and notes.md keeps its changes

### Requirement: The title shows the file
LeafMD SHALL show the selected tab's file name as the window title, with its folder underneath (shortened to `~` for the home folder), and the window's name SHALL be "file name - LeafMD". The title SHALL change as soon as another tab is selected.

#### Scenario: A file in the home folder
- **WHEN** `~/notes/todo.md` is open with unsaved changes
- **THEN** the title reads "• todo.md" with "~/notes" underneath

#### Scenario: Switching tabs
- **WHEN** `~/notes/todo.md` and `~/work/plan.md` are tabs in a window and the user selects plan.md
- **THEN** the title reads "plan.md" with "~/work" underneath, and the window's name is "plan.md - LeafMD"

### Requirement: Closing a document
LeafMD SHALL offer Close Document in the main menu. After the unsaved-changes check, it SHALL close the selected tab. If that was the last tab, it SHALL show the "No Document Open" page in the same window, leaving the window open. It SHALL work for saved files and for new, never-saved documents.

#### Scenario: Closing a saved file
- **WHEN** notes.md is the only document, with no unsaved changes, and the user chooses Close Document
- **THEN** the window stays open and shows the "No Document Open" page, and its title says "LeafMD"

#### Scenario: Closing one of several
- **WHEN** notes.md and todo.md are open, todo.md is selected, and the user chooses Close Document
- **THEN** todo.md's tab closes and notes.md is selected

#### Scenario: Unsaved changes
- **WHEN** the user types in notes.md without saving and chooses Close Document
- **THEN** LeafMD asks whether to save before closing it, and Cancel leaves the document open and unchanged

#### Scenario: Closing a new document
- **WHEN** the user presses Ctrl+T on the start page, types nothing, and chooses Close Document
- **THEN** the "No Document Open" page is shown, and nothing is asked because there is nothing to save

#### Scenario: Getting the file back
- **WHEN** the user closes notes.md with Close Document and opens the arrow next to Open
- **THEN** notes.md is on the recent files list, and choosing it opens it again

#### Scenario: Ctrl+W still closes the window
- **WHEN** one document is open and the user presses Ctrl+W
- **THEN** the whole window closes, as it did before

### Requirement: Checking when returning to the window
LeafMD SHALL check the selected tab's file again each time its window becomes the active window, and each time a tab is selected, so changes it wasn't notified of still appear.

#### Scenario: A file on a network drive
- **WHEN** a file on a drive that sends no change notices is changed elsewhere, and the user switches back to LeafMD
- **THEN** the banner appears

#### Scenario: A file outside the home folder
- **WHEN** a file outside the home folder is open, is changed by another app, and the user switches back to LeafMD
- **THEN** the banner appears

#### Scenario: Switching to a tab
- **WHEN** a file on a drive that sends no change notices is open in a tab, is changed elsewhere, and the user selects that tab
- **THEN** the banner appears

### Requirement: The banner belongs to the open file
LeafMD SHALL give each tab its own banner, showing only the selected tab's. It SHALL close a tab's banner, and stop checking the old file, when the tab is closed or saved as a different file. It SHALL NOT show a banner about a file that isn't in the selected tab.

#### Scenario: Opening another file
- **WHEN** the banner is showing for notes.md and the user opens todo.md
- **THEN** todo.md's tab shows no banner, and selecting notes.md's tab shows its banner again

#### Scenario: A change in a tab that isn't selected
- **WHEN** todo.md is selected and another app changes notes.md, which is open in another tab
- **THEN** no banner shows over todo.md, and selecting notes.md's tab shows "“notes.md” was changed by another app"

#### Scenario: Closing a document with a banner
- **WHEN** the banner is showing for notes.md and the user closes its tab
- **THEN** the banner is gone, and later changes to notes.md don't bring it back

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
- **WHEN** the user closes the window, chooses Save when asked about unsaved changes, and then chooses Cancel at "Replace the Other Version?"
- **THEN** the window stays open with the unsaved changes intact

## ADDED Requirements

### Requirement: Every tab keeps watching its file
Each tab SHALL keep watching its own file while it isn't selected, and all the banner rules SHALL apply to it, including its unsaved changes and whether a change was ignored. Its banner SHALL be up to date when the tab is selected. A banner's buttons SHALL only act on the selected tab's document.

#### Scenario: Two files changed
- **WHEN** notes.md and todo.md are tabs and another app changes both
- **THEN** selecting either tab shows a banner naming that file, and choosing Reload there leaves the other tab and its banner as they were

#### Scenario: Deleted while not selected
- **WHEN** notes.md is deleted while todo.md is selected, and the user selects notes.md's tab
- **THEN** the banner says "“notes.md” was deleted or moved by another app", with Ignore and Save Again buttons

#### Scenario: Changed back while not selected
- **WHEN** another app changes notes.md while todo.md is selected, then puts it back exactly as it was, and the user selects notes.md's tab
- **THEN** no banner shows

#### Scenario: Unsaved changes in a tab that isn't selected
- **WHEN** notes.md has unsaved changes, todo.md is selected, another app changes notes.md, and the user selects notes.md's tab
- **THEN** the banner mentions the unsaved changes and offers Discard and Reload, and the typed text is still there

#### Scenario: Ignoring stays with its tab
- **WHEN** the user chooses Ignore in notes.md's banner, switches to todo.md and back, with no further change
- **THEN** no banner shows over notes.md

#### Scenario: Typing while another tab's file changes
- **WHEN** the user is typing in todo.md and another app changes notes.md
- **THEN** no banner appears, and the next letters typed go into todo.md

#### Scenario: Moving a tab that has a banner
- **WHEN** notes.md's banner is showing and the user moves its tab to a new window
- **THEN** the new window shows the banner, the first window shows the banner for its own selected tab or none, and Reload works in the new window

#### Scenario: Changes after moving a tab
- **WHEN** notes.md's tab has moved to another window and another app then changes notes.md
- **THEN** the banner appears in the window it moved to

### Requirement: Closing a window with several unsaved tabs
When a window closes with two or more unsaved tabs, LeafMD SHALL ask once, in a single dialog listing each with a tick box, its name and its folder, all ticked to start. The buttons SHALL be Close without Saving, Cancel and Save, with Save as the default. Save SHALL save the ticked documents and discard the rest, and SHALL be greyed out when none are ticked. The window SHALL only close if every save worked.

#### Scenario: Two unsaved tabs
- **WHEN** notes.md and todo.md have unsaved changes and the user presses Ctrl+Q
- **THEN** one dialog says "There are 2 documents with unsaved changes. Save changes before closing?" and lists both, ticked, and Cancel leaves the window and both tabs as they were

#### Scenario: Saving some of them
- **WHEN** that dialog is showing, the user unticks todo.md and chooses Save
- **THEN** notes.md is saved, todo.md's changes are discarded, and the window closes

#### Scenario: Closing without saving
- **WHEN** that dialog is showing and the user chooses Close without Saving
- **THEN** the window closes without saving either file

#### Scenario: Nothing ticked
- **WHEN** the user unticks every document in that dialog
- **THEN** Save is greyed out, and Cancel and Close without Saving can still be chosen

#### Scenario: Two files with the same name
- **WHEN** `~/a/README.md` and `~/b/README.md` both have unsaved changes and the dialog is showing
- **THEN** each row shows its folder, so the two can be told apart

#### Scenario: A never-saved document
- **WHEN** notes.md and an Untitled tab are both ticked and the user chooses Save
- **THEN** notes.md is saved, the Untitled tab is selected, and a save dialog asks where to save it

#### Scenario: Stopping part-way
- **WHEN** the user chooses Save with todo.md unticked, and then cancels the save dialog for a ticked Untitled tab
- **THEN** the window stays open, documents already saved stay saved, and nothing is discarded, including todo.md and the Untitled tab

#### Scenario: A ticked file changed by another app
- **WHEN** notes.md is ticked, another app has changed it, and the user chooses Save
- **THEN** notes.md's tab is selected and LeafMD asks "Replace the Other Version?", and Cancel keeps the window open with nothing discarded

#### Scenario: Only one unsaved tab
- **WHEN** three tabs are open, only notes.md has unsaved changes, and the user presses Ctrl+Q
- **THEN** LeafMD asks the usual "Save changes to document “notes.md” before closing?" with Close without Saving, Cancel and Save

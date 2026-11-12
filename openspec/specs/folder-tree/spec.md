# folder-tree Specification

## Purpose
The Files page of the side panel: a tree of a folder's contents for moving between documents, which folder it shows, what it lists, and how it works with tabs and folders LeafMD can't see.

## Requirements

### Requirement: Which folder the tree shows
Each window SHALL have its own chosen folder, set with `leafmd` or Open Folder… (Ctrl+Shift+O). While the selected document has a file, LeafMD SHALL show the folder it's in (or one above, after the up button), even inside the chosen folder, moving back when another tab is selected, a file is opened or it's saved somewhere new. With no document, or a new unsaved one, the tree SHALL show the chosen folder. With neither, the panel SHALL say "No Folder Open" and offer an Open Folder… button.

#### Scenario: Following the document
- **GIVEN** no folder has been chosen in the window
- **WHEN** ~/notes/ideas.md and ~/work/plan.md are open, and the user selects plan.md's tab
- **THEN** the tree switches from the contents of ~/notes to the contents of ~/work

#### Scenario: Opening a file in a subfolder
- **GIVEN** the tree shows ~/notes because README.md is selected
- **WHEN** the user expands docs in the tree and clicks setup.md, then switches back to README.md's tab
- **THEN** setup.md opens and the tree shows the contents of ~/notes/docs with setup.md highlighted, then ~/notes again with README.md highlighted

#### Scenario: Going back to a file in a narrower folder
- **GIVEN** no folder has been chosen in the window, ~/Desktop/todo.md is selected, and the tree shows ~/Desktop
- **WHEN** the user switches to an Untitled tab, saves it as ~/plan.md with Ctrl+S, then switches back to todo.md's tab
- **THEN** the tree shows the home folder with plan.md highlighted, then ~/Desktop again with todo.md highlighted

#### Scenario: Saving somewhere new
- **GIVEN** the tree shows ~/notes because ~/notes/ideas.md is selected
- **WHEN** the user saves ideas.md as ~/work/ideas.md with Save As (Ctrl+Shift+S)
- **THEN** the tree shows ~/work with ideas.md highlighted

#### Scenario: Starting from a chosen folder
- **GIVEN** the user ran `leafmd .` in ~/notes, so the window shows "No Document Open" and the tree shows ~/notes
- **WHEN** they expand docs in the tree and click setup.md
- **THEN** setup.md opens and the tree shows the contents of ~/notes/docs with setup.md highlighted

#### Scenario: A document outside the chosen folder
- **GIVEN** the user ran `leafmd .` in ~/notes and opened ~/notes/docs/setup.md from the tree
- **WHEN** they open ~/work/plan.md with Ctrl+O, then switch back to setup.md's tab
- **THEN** the tree shows ~/work with plan.md highlighted, then ~/notes/docs again with setup.md highlighted

#### Scenario: A document shared by itself
- **GIVEN** the user ran `leafmd .` in ~/notes
- **WHEN** they open /tmp/a.md, which LeafMD is given by itself
- **THEN** the panel says LeafMD was given this one document, not the folder it's in, with an Open Folder… button and a Learn More link, instead of showing ~/notes

#### Scenario: Nothing to show
- **WHEN** the window shows "No Document Open" with no chosen folder and the user presses F9
- **THEN** the panel says "No Folder Open" with an Open Folder… button

#### Scenario: A new document
- **GIVEN** no folder has been chosen in the window, and the tree shows ~/notes because ideas.md is selected
- **WHEN** the user presses Ctrl+T, then saves the new document as ~/work/plan.md
- **THEN** the panel says "No Folder Open" while the new document is unsaved, then shows ~/work with plan.md highlighted

#### Scenario: A new document with a chosen folder
- **GIVEN** the user ran `leafmd .` in ~/notes and opened ~/notes/docs/setup.md from the tree
- **WHEN** the user presses Ctrl+T
- **THEN** the tree shows ~/notes while the new document is unsaved

#### Scenario: Open Folder opens the panel
- **WHEN** the panel is closed and the user presses Ctrl+Shift+O and picks ~/notes
- **THEN** the panel opens showing ~/notes, and is still open after restarting, like opening it with F9

### Requirement: What the tree lists
The tree SHALL list every folder and file in the folder, folders first, then files, each in alphabetical order ignoring case. Hidden files and folders (such as `.git` or `.env`) SHALL NOT be listed. Folders SHALL expand and collapse with a click, or the Right and Left keys.

#### Scenario: A project folder
- **WHEN** the tree shows a folder holding `.git`, `docs`, `README.md`, `LICENSE` and `logo.png`
- **THEN** it lists docs, then LICENSE, logo.png and README.md, and no `.git`

#### Scenario: Expanding a folder
- **WHEN** the user clicks docs in the tree
- **THEN** it expands to show the files in docs, indented underneath it

### Requirement: Files LeafMD can't open are greyed out
Files that are clearly not text, judged only by their names (images, audio, video, PDFs, archives, fonts, office documents), SHALL be greyed out, and clicking them SHALL do nothing. Hovering one SHALL say "LeafMD can't open this type of file". Any other file, including one with no extension, SHALL show normally. LeafMD SHALL NOT read a file's contents to decide.

#### Scenario: An image
- **WHEN** the user clicks logo.png in the tree
- **THEN** nothing opens, and logo.png stays greyed out

#### Scenario: Text files of other kinds
- **WHEN** the tree shows LICENSE, notes.txt and main.rs
- **THEN** none of them are greyed out, and clicking LICENSE opens it in a tab

#### Scenario: A file with no extension that isn't text
- **WHEN** the user clicks a program called `build` with no extension
- **THEN** a message says "This file isn't plain text, so LeafMD can't open it", as it does for any file that isn't text

#### Scenario: Empty files
- **WHEN** the tree shows an empty plan.md and an empty photo.png
- **THEN** plan.md shows normally and opens when clicked, and photo.png is greyed out

#### Scenario: Saving a new document into the folder showing
- **GIVEN** the user ran `leafmd .` in ~/notes, and the tree shows ~/notes
- **WHEN** they press Ctrl+T, type "Plan", and save it as ~/notes/plan.md
- **THEN** plan.md appears in the tree, not greyed out, and opens when clicked

### Requirement: Opening files from the tree
Clicking a file in the tree, or selecting it and pressing Enter, SHALL open it following the rules for opening files inside LeafMD: a new selected tab in that window, or the tab that already has it.

#### Scenario: A file that isn't open yet
- **WHEN** README.md is open and the user clicks todo.md in the tree
- **THEN** todo.md opens in a new tab and is selected, and README.md stays open in its tab

#### Scenario: A file open in another tab
- **WHEN** README.md and todo.md are tabs, README.md is selected, and the user clicks todo.md in the tree
- **THEN** todo.md's tab is selected, and no new tab opens

#### Scenario: A file open in another window
- **WHEN** todo.md is open in another LeafMD window and the user clicks it in the tree
- **THEN** that window comes to the front with todo.md's tab selected

### Requirement: The tree follows the selected tab
When the selected tab's file is inside the tree's folder, the tree SHALL highlight it and expand the folders leading to it, so it's in view. Switching tabs SHALL move the highlight. The panel belongs to the window, so switching tabs SHALL NOT close it, and a tab dragged to another window SHALL NOT take the folder with it.

#### Scenario: Switching tabs
- **WHEN** the tree shows ~/notes, and the user switches from ~/notes/README.md to ~/notes/ideas.md
- **THEN** ideas.md is highlighted in the tree instead of README.md

#### Scenario: A file the tree can't show
- **WHEN** the user selects a tab with /tmp/a.md, which LeafMD was given by itself
- **THEN** the panel says LeafMD was given this one document, and nothing is highlighted

#### Scenario: Moving a tab to a new window
- **WHEN** the window's chosen folder is ~/notes and the user chooses Move to New Window on a tab
- **THEN** the original window keeps ~/notes as its chosen folder, and the new window has no chosen folder, so its tree shows the moved document's folder

### Requirement: The tree stays current
The tree SHALL update by itself when files or folders in it are added, removed or renamed, keeping expanded folders expanded. For a folder outside the home folder, where LeafMD isn't told about changes, the tree SHALL catch up when the user switches back to the LeafMD window.

#### Scenario: A new file
- **WHEN** the tree shows ~/notes and another app saves a new file ~/notes/ideas.md
- **THEN** ideas.md appears in the tree in its alphabetical place, without the user doing anything

#### Scenario: A new file outside the home folder
- **GIVEN** the tree shows /tmp/notes, opened with `leafmd .`
- **WHEN** the user switches to a text editor, saves /tmp/notes/ideas.md, and switches back to LeafMD
- **THEN** ideas.md is in the tree

### Requirement: A chosen folder lasts as long as its window
A folder chosen with Open Folder… or `leafmd` SHALL belong to its window only while that window is open. It SHALL NOT be saved, so when LeafMD is started from the app grid and reopens the tabs from the window closed last, the tree SHALL follow the selected document.

#### Scenario: Restarting
- **GIVEN** the user ran `leafmd .` in ~/notes and opened ~/work/plan.md in that window
- **WHEN** they close the window with Ctrl+Q and start LeafMD again from the app grid
- **THEN** the window opens with plan.md, and the tree shows ~/work, not ~/notes

### Requirement: Folders LeafMD can't see
When LeafMD can't see a folder's real contents, the panel SHALL say why and offer Open Folder… and a Learn More link to the README, instead of listing the wrong contents. This applies to a folder outside the home folder given without the `leafmd` short command, and to the folder of a document opened by itself from outside the home folder. Folders chosen with Open Folder… or given to `leafmd`, and the folders inside them, SHALL always be listed, wherever they are.

#### Scenario: A document from outside the home folder
- **GIVEN** no folder has been chosen in the window
- **WHEN** the user runs `leafmd /tmp/notes/a.md` and opens the panel
- **THEN** the panel says LeafMD was given this one document, not the folder it's in, with an Open Folder… button and a Learn More link

#### Scenario: Choosing the folder
- **WHEN** the user then clicks Open Folder… and picks /tmp/notes
- **THEN** the tree lists the files in /tmp/notes

#### Scenario: A subfolder of a chosen folder outside the home folder
- **GIVEN** the user ran `leafmd .` in /tmp/notes
- **WHEN** they expand drafts in the tree and click /tmp/notes/drafts/b.md
- **THEN** the tree lists the files in /tmp/notes/drafts with b.md highlighted

### Requirement: Full names on hover
Hovering a file or folder in the tree that LeafMD can open SHALL show its whole name, so a name cut short to fit the panel can still be read. Greyed-out files SHALL keep saying "LeafMD can't open this type of file" instead.

#### Scenario: A long file name
- **WHEN** the tree shows `2026-11-meeting-notes-for-the-garden-committee.md`, cut short in the middle to fit, and the user hovers over it
- **THEN** a tooltip shows `2026-11-meeting-notes-for-the-garden-committee.md` in full

#### Scenario: A folder
- **WHEN** the user hovers over the folder `drafts` in the tree
- **THEN** a tooltip shows `drafts`

#### Scenario: A greyed-out file
- **WHEN** the user hovers over logo.png
- **THEN** the tooltip says "LeafMD can't open this type of file"

### Requirement: Open Folder… with a document open
When the window has a document open, Open Folder… SHALL open the chosen folder in a new window, so the panel never shows a folder unrelated to the document beside it. A window that already has that folder chosen SHALL come to the front instead. When the window has no document open, Open Folder… SHALL use it.

#### Scenario: A document is open
- **GIVEN** ~/work/plan.md is open
- **WHEN** the user presses Ctrl+Shift+O and picks ~/notes
- **THEN** a new window opens on "No Document Open" with the panel showing ~/notes, and plan.md's window is left as it was

#### Scenario: The window is empty
- **GIVEN** the window shows "No Document Open"
- **WHEN** the user presses Ctrl+Shift+O and picks ~/notes
- **THEN** the panel in that window shows ~/notes, and no new window opens

#### Scenario: The folder is already open
- **GIVEN** another window already has ~/notes as its chosen folder, and ~/work/plan.md is open in this one
- **WHEN** the user presses Ctrl+Shift+O and picks ~/notes
- **THEN** the other window comes to the front, and no new window opens

### Requirement: Going up a folder
On the Files page, a button at the left of the panel's header SHALL show the folder above the one showing, with the folder it came from expanded and the selected document's file highlighted. Its tooltip SHALL name that folder. It SHALL be greyed out when there's no folder showing or the folder above can't be listed. Switching tabs, opening a file or saving somewhere new SHALL put the panel back on the selected file's folder.

#### Scenario: Going up
- **GIVEN** ~/notes/docs/setup.md is selected and the tree shows ~/notes/docs
- **WHEN** the user hovers the up button, then clicks it
- **THEN** the tooltip says "Show the folder above (notes)", and the tree shows ~/notes with docs expanded and setup.md highlighted

#### Scenario: Going up twice
- **GIVEN** the user went up from ~/notes/docs to ~/notes
- **WHEN** they click the up button again
- **THEN** the tree shows the home folder with notes and docs expanded and setup.md highlighted

#### Scenario: Back to the file's folder
- **GIVEN** the user went up from ~/notes/docs to ~/notes, and ~/work/plan.md is open in another tab
- **WHEN** they switch to plan.md's tab, then back to setup.md's tab
- **THEN** the tree shows ~/work, then ~/notes/docs again

#### Scenario: Nowhere further up
- **WHEN** the tree shows the home folder in the installed Flatpak
- **THEN** the up button is greyed out, because LeafMD can't list the folder above the home folder

#### Scenario: Headings
- **WHEN** the panel shows Headings
- **THEN** the up button isn't shown

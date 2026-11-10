# Spec Delta

## Purpose

The Files page of the side panel: a tree of a folder's contents for moving between documents, which folder it shows, what it lists, and how it works with tabs and folders LeafMD can't see.

## ADDED Requirements

### Requirement: Which folder the tree shows
Each window SHALL have its own chosen folder, set from the command line or with Open Folder… (Ctrl+Shift+O). The tree SHALL show it while the selected document is inside it. Otherwise the tree SHALL follow the selected document: it SHALL show the document's folder, and only move to another folder when the selected document isn't inside the one it shows. With neither, the panel SHALL say "No Folder Open" and offer an Open Folder… button.

#### Scenario: Following the document
- **GIVEN** no folder has been chosen in the window
- **WHEN** ~/notes/ideas.md and ~/work/plan.md are open, and the user selects plan.md's tab
- **THEN** the tree switches from the contents of ~/notes to the contents of ~/work

#### Scenario: Opening a file in a subfolder
- **GIVEN** no folder has been chosen in the window, and the tree shows ~/notes because README.md is selected
- **WHEN** the user expands docs in the tree and clicks setup.md
- **THEN** setup.md opens, and the tree still shows ~/notes with docs expanded and setup.md highlighted

#### Scenario: A document outside the chosen folder
- **GIVEN** the user ran `leafmd .` in ~/notes, expanded docs and opened ~/notes/docs/setup.md
- **WHEN** they open ~/work/plan.md with Ctrl+O, then switch back to setup.md's tab
- **THEN** the tree shows ~/work with plan.md highlighted, then ~/notes again with docs expanded and setup.md highlighted

#### Scenario: A document shared by itself
- **GIVEN** the user ran `leafmd .` in ~/notes
- **WHEN** they open /tmp/a.md, which LeafMD is given by itself
- **THEN** the tree stays on ~/notes, instead of saying it can't see /tmp

#### Scenario: Nothing to show
- **WHEN** the window shows "No Document Open" with no chosen folder and the user presses F9
- **THEN** the panel says "No Folder Open" with an Open Folder… button

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
- **WHEN** the tree shows ~/notes, and the user switches from ~/notes/README.md to ~/notes/docs/setup.md
- **THEN** docs is expanded in the tree and setup.md is highlighted instead of README.md

#### Scenario: A file the tree can't show
- **WHEN** the tree shows the chosen folder ~/notes and the user selects a tab with /tmp/a.md, which LeafMD was given by itself
- **THEN** nothing in the tree is highlighted

#### Scenario: Moving a tab to a new window
- **WHEN** the tree shows the chosen folder ~/notes and the user chooses Move to New Window on a tab
- **THEN** the original window keeps ~/notes, and the new window has no chosen folder, so its tree follows the moved document

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
When LeafMD can't see a folder's real contents, the panel SHALL say why and offer Open Folder… and a Learn More link to the README, instead of listing the wrong contents. This applies to a folder outside the home folder given without the `leafmd` short command, and to the folder of a document opened by itself from outside the home folder. Folders chosen with Open Folder… or given to `leafmd` SHALL always be listed, wherever they are.

#### Scenario: A document from outside the home folder
- **GIVEN** no folder has been chosen in the window
- **WHEN** the user runs `leafmd /tmp/notes/a.md` and opens the panel
- **THEN** the panel says LeafMD was given this one document, not the folder it's in, with an Open Folder… button and a Learn More link

#### Scenario: Choosing the folder
- **WHEN** the user then clicks Open Folder… and picks /tmp/notes
- **THEN** the tree lists the files in /tmp/notes

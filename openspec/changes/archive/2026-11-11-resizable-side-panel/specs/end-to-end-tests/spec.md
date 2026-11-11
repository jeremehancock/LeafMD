# Spec Delta

## MODIFIED Requirements

### Requirement: One command runs the tests
LeafMD SHALL have an end-to-end test run started with `./build-aux/end-to-end-tests.sh`. It SHALL test the installed Flatpak, not a development build, and SHALL print each scenario's name with "passed" or "failed", then a total. It SHALL finish with success only when every scenario passed.

#### Scenario: Everything passes
- **WHEN** the owner runs `./build-aux/end-to-end-tests.sh` after `./build-aux/flatpak-install.sh`
- **THEN** each scenario is listed with "passed", the last line reads "46 of 46 passed", and the command finishes with success

#### Scenario: LeafMD isn't installed
- **WHEN** the owner runs it on a computer without LeafMD's Flatpak installed
- **THEN** it says to run `./build-aux/flatpak-install.sh` first, and stops without running anything

### Requirement: The tests act like a person
The tests SHALL drive LeafMD only the way a person could: starting it with files, pressing keys, typing text, clicking and dragging with the pointer, and closing windows. They SHALL check results a person could also check: what is in a file on disk, the window's title, how many windows are open and what LeafMD saved for next time.

#### Scenario: Checking a save
- **WHEN** a scenario types "eggs" into an open file and presses Ctrl+S
- **THEN** the test passes only if the file on disk now contains "eggs"

### Requirement: What the tests cover
The test run SHALL include at least 46 scenarios, covering typing, text and line styles, find and replace, saving and the unsaved-changes questions, files another app changed or deleted, tabs and windows, what LeafMD remembers, the side panel with its folder tree, headings and width, web links, Distraction Free and full screen, and zoom. Each scenario SHALL be a file in `build-aux/end-to-end/` that starts by naming the spec it follows.

#### Scenario: Seeing what's covered
- **WHEN** the owner looks in `build-aux/end-to-end/`
- **THEN** there are 46 files, such as `close-unsaved-tabs.sh`, and each starts with a comment like "documents spec, Closing a window with several unsaved tabs"

#### Scenario: Unsaved changes are kept
- **WHEN** the unsaved-guard scenario types "draft" into notes.md and presses Ctrl+Q without saving
- **THEN** the window is still open, and notes.md on disk doesn't contain "draft"

#### Scenario: A list continues
- **WHEN** the lists-continue scenario types "- milk", presses Enter, types "eggs" and saves
- **THEN** the file holds "- milk" and "- eggs" on separate lines

#### Scenario: Bold and undo
- **WHEN** the bold scenario selects the word "plan", presses Ctrl+B and saves, then presses Ctrl+Z once and saves again
- **THEN** the file first has "**plan**", then plain "plan"

#### Scenario: Replace All and undo
- **WHEN** the replace-all scenario opens a file with "cat" written 4 times, presses Ctrl+H, replaces all with "dog", saves, then presses Ctrl+Z once and saves again
- **THEN** the file first has "dog" 4 times and no "cat", then "cat" 4 times and no "dog"

#### Scenario: Tabs
- **WHEN** the tabs scenario starts LeafMD with a.md, then starts it again with b.md
- **THEN** there is still one window, its title shows b.md, and after Ctrl+Page Up the title shows a.md

#### Scenario: Reopening the last files
- **WHEN** the reopen-last scenario opens a.md and b.md as tabs, selects a.md, presses Ctrl+Q and starts LeafMD with no file
- **THEN** one window opens with the title showing a.md, and after Ctrl+Page Down it shows b.md

#### Scenario: Reloading a changed file
- **WHEN** the reload scenario has notes.md open with no unsaved changes, the test writes "Changed elsewhere" into notes.md from outside LeafMD, clicks Reload in the banner that appears, then types "!" at the end of the document and saves
- **THEN** the file holds "Changed elsewhere!", showing LeafMD brought in the change before the typing

#### Scenario: Zoom
- **WHEN** the zoom scenario opens notes.md, presses Ctrl+plus twice, Ctrl+minus once, then Ctrl+N
- **THEN** the first window's zoom is 110%, the new window's is 100%, and after Ctrl+0 in the first window it is 100% too

#### Scenario: Zooming with the mouse wheel
- **WHEN** the zoom scenario turns the mouse wheel up twice with Ctrl held over the raw text, then down once with Ctrl held over the formatted view, then down once without Ctrl
- **THEN** the zoom is 120%, then 110%, and stays 110%

#### Scenario: Zoom isn't remembered
- **WHEN** the zoom-not-remembered scenario presses Ctrl+plus three times, presses Ctrl+Q and starts LeafMD again
- **THEN** the window's zoom is 100%, and nothing about zoom was saved for next time

#### Scenario: Jumping to a heading
- **WHEN** the jump-to-heading scenario opens a document in Raw view with the headings "Garden Plan", "Seeds" and "Beds", opens the Headings page, clicks "Beds", types "Raised " and saves
- **THEN** the file holds "Raised ## Beds"

#### Scenario: The panel's page is remembered
- **WHEN** the headings-page-remembered scenario opens the side panel, switches it to Headings, presses Ctrl+Q and starts LeafMD again
- **THEN** the panel is open on Headings

#### Scenario: Escape leaves full screen first
- **WHEN** the escape-full-screen-first scenario turns on Distraction Free with Ctrl+Shift+D and full screen with F11, then presses Escape
- **THEN** full screen is off and Distraction Free is still on, and after a second Escape Distraction Free is off too

#### Scenario: Resizing the side panel
- **WHEN** the side-panel-width scenario opens the panel in a 1200 pixel window, drags the line between the panel and the document 80 pixels to the right, presses Ctrl+Q and starts LeafMD again, drags the panel to 400, then narrows the window bit by bit to 1000 pixels
- **THEN** LeafMD saved a panel width of 320 for next time, the panel opens 320 pixels wide, and in the narrowed window the panel is 320 pixels wide again so the document keeps 680

#### Scenario: A web link with nothing to open it
- **WHEN** the web-link-no-portal scenario opens a document in Formatted view whose first line is a link to `https://example.com`, on the hidden desktop, which has no portal to hand links to a browser, and clicks the link
- **THEN** LeafMD is still running with its window open, and its messages say it couldn't open the link


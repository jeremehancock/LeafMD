# end-to-end-tests Specification

## Purpose
Lets LeafMD's owner check that the installed app still behaves the way the specs say, automatically, without trying every feature by hand after each change.

## Requirements

### Requirement: One command runs the tests
LeafMD SHALL have an end-to-end test run started with `./build-aux/end-to-end-tests.sh`. It SHALL test the installed Flatpak, not a development build, and SHALL print each scenario's name with "passed" or "failed", then a total. It SHALL finish with success only when every scenario passed.

#### Scenario: Everything passes
- **WHEN** the owner runs `./build-aux/end-to-end-tests.sh` after `./build-aux/flatpak-install.sh`
- **THEN** each scenario is listed with "passed", the last line reads "46 of 46 passed", and the command finishes with success

#### Scenario: LeafMD isn't installed
- **WHEN** the owner runs it on a computer without LeafMD's Flatpak installed
- **THEN** it says to run `./build-aux/flatpak-install.sh` first, and stops without running anything

### Requirement: Running some of the scenarios
The test run SHALL accept scenario names, and then run only those.

#### Scenario: One scenario
- **WHEN** the owner runs `./build-aux/end-to-end-tests.sh lists-continue`
- **THEN** only the list continuation scenario runs, and the last line reads "1 of 1 passed"

#### Scenario: A name that doesn't exist
- **WHEN** the owner runs `./build-aux/end-to-end-tests.sh lists-contine`
- **THEN** it says there is no scenario by that name, lists the ones there are, and runs nothing

### Requirement: The tests act like a person
The tests SHALL drive LeafMD only the way a person could: starting it with files, pressing keys, typing text, clicking and dragging with the pointer, and closing windows. They SHALL check results a person could also check: what is in a file on disk, the window's title, how many windows are open and what LeafMD saved for next time.

#### Scenario: Checking a save
- **WHEN** a scenario types "eggs" into an open file and presses Ctrl+S
- **THEN** the test passes only if the file on disk now contains "eggs"

### Requirement: The tests never touch the real desktop
The test run SHALL run LeafMD on a hidden desktop of its own, with its own home folder, and SHALL leave the owner's real desktop, files, LeafMD settings, recent files and running LeafMD windows untouched. It SHALL never open a file dialog. When it finishes, it SHALL check that the real LeafMD settings didn't change, the real document portal (the Flatpak service that hands files to apps) didn't restart, and Flatpak apps still start, and SHALL warn and fail if any of those went wrong.

#### Scenario: LeafMD already open with unsaved work
- **WHEN** the owner has LeafMD open with an unsaved document and runs the tests
- **THEN** their LeafMD window stays open with the unsaved text still there, and no test window appears on their screen

#### Scenario: Something got out
- **WHEN** a run finishes and the real document portal has restarted during it
- **THEN** the run fails with a warning saying so, and how to fix it: `systemctl --user restart xdg-document-portal.service`

### Requirement: Evidence when a scenario fails
When a scenario fails, the test run SHALL say which check failed and what it found instead, and SHALL keep a picture of the hidden desktop and LeafMD's messages from that scenario in `target/end-to-end/`, naming the folder in its output. A failing scenario SHALL NOT stop the others from running.

#### Scenario: A save that didn't happen
- **WHEN** the save scenario finds "- milk" in the file but not "- eggs"
- **THEN** the output says "save: failed: the file doesn't contain '- eggs'", the remaining scenarios still run, and `target/end-to-end/save/` holds `screen.png` and `leafmd.log`

### Requirement: Each scenario starts fresh
Each scenario SHALL start with a new, empty home folder and fresh copies of its sample files, so one scenario's saved settings, recent files or edits can't change another's result.

#### Scenario: Recent files from an earlier scenario
- **WHEN** the tabs scenario has opened a.md and b.md, and the next scenario starts LeafMD without a file
- **THEN** LeafMD shows its start page, not a.md and b.md

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

### Requirement: Run before every hand test
`/ship` SHALL run the full test run after installing a new build and before asking the owner to try it. If any scenario fails, `/ship` SHALL stop and report the failure instead of asking the owner to test. The list `/ship` gives the owner to try SHALL leave out what the passing scenarios already checked.

#### Scenario: A failure during /ship
- **WHEN** `/ship` installs a build and the bold scenario fails
- **THEN** `/ship` reports the failure and where its picture is, and doesn't ask the owner to try the build

#### Scenario: A passing run during /ship
- **WHEN** every scenario passes for a change that added word count
- **THEN** the owner's list covers trying the word count, and says that saving, lists, tabs and the other tested behaviour were checked automatically

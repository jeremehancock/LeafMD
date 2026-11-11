# side-panel Specification

## Purpose
The panel on the left of each LeafMD window: how it opens and closes, how that is remembered, how it behaves in narrow windows, and how it holds pages such as Files.

## Requirements

### Requirement: Opening and closing the side panel
Each LeafMD window SHALL have a side panel on its left, closed by default. The panel button at the left end of the header bar, F9, and Side Panel in the main menu SHALL each open it when closed and close it when open. The button SHALL stay pressed in, and the menu item ticked, while the panel shows.

#### Scenario: First use
- **GIVEN** the user has never opened the side panel
- **WHEN** they start LeafMD with notes.md
- **THEN** no panel shows, and the panel button at the left of the header bar isn't pressed in

#### Scenario: Pressing F9
- **WHEN** the panel is closed in a window 1200 pixels wide and the user presses F9
- **THEN** the panel appears on the left, beside the document, and the panel button is pressed in

#### Scenario: Closing it with the button
- **WHEN** the panel is open and the user clicks the panel button
- **THEN** the panel closes and the document fills the window again

### Requirement: The side panel stays open until closed
Once the user opens the side panel, LeafMD SHALL keep it open until the user closes it: through tab changes, in new windows, and after restarting. Closing it SHALL be remembered the same way. Opening or closing it in one window SHALL NOT change other windows already open.

#### Scenario: Switching tabs
- **WHEN** the panel is open and the user presses Ctrl+Tab to move from notes.md to todo.md
- **THEN** the panel stays open

#### Scenario: A new window
- **WHEN** the panel is open and the user presses Ctrl+N
- **THEN** the new window opens with its panel open

#### Scenario: Restarting
- **WHEN** the panel is open, the user presses Ctrl+Q and starts LeafMD again from the app grid
- **THEN** the window opens with its panel open

#### Scenario: Closing it is remembered too
- **WHEN** the user closes the panel, presses Ctrl+Q and starts LeafMD again
- **THEN** the window opens with no panel

#### Scenario: Two windows
- **WHEN** two windows have their panels open and the user presses F9 in the first
- **THEN** only the first window's panel closes

### Requirement: The side panel in narrow windows
When a window is narrower than 920 pixels, the side panel SHALL slide over the left of the document instead of sitting beside it, and SHALL only appear when the user asks (or from the command line). There, it SHALL tuck away when the user picks a file or a heading, presses Escape or clicks the document, without changing whether it's remembered as open. Widening the window SHALL put it back beside the document if it's remembered as open.

#### Scenario: Opening it in a narrow window
- **WHEN** the window is 700 pixels wide and the user presses F9
- **THEN** the panel slides out over the left of the document, and the document doesn't move or shrink

#### Scenario: Picking a file
- **WHEN** the panel is over the document in a 700 pixel window and the user clicks todo.md in it
- **THEN** todo.md opens in a tab and the panel tucks away

#### Scenario: Picking a heading
- **WHEN** the Headings page is over the document in a 700 pixel window and the user clicks "Beds" in it
- **THEN** the document scrolls to the Beds heading and the panel tucks away

#### Scenario: Dragging the window narrower and wider
- **WHEN** the panel is open beside the document in a 1200 pixel window and the user drags the window to 700 pixels and back to 1200
- **THEN** the panel hides at 700 pixels and is back beside the document at 1200

#### Scenario: Starting in a narrow window
- **GIVEN** the panel was open when LeafMD last closed, at 700 pixels wide
- **WHEN** the user starts LeafMD again
- **THEN** the document shows with no panel over it, and F9 slides the panel out

### Requirement: Pages in the side panel
The side panel SHALL show one page at a time, Files or Headings, with a title at the top saying what it shows (for Files, the folder's name; for Headings, "Headings"). A switcher along the bottom of the panel SHALL show both pages, each with an icon and its name, and clicking one SHALL show it.

#### Scenario: Opening the panel
- **WHEN** notes.md in the folder ~/notes is open, the panel is on Files and the user opens the panel
- **THEN** the top of the panel says "notes", and the bottom shows Files and Headings with Files selected

#### Scenario: Switching to Headings
- **WHEN** the panel shows Files and the user clicks Headings at the bottom of the panel
- **THEN** the panel lists the document's headings, and its title says "Headings"

### Requirement: The side panel remembers its page
LeafMD SHALL open the side panel on the page the user last chose, in new windows and after restarting, starting on Files the first time. Choosing a page in one window SHALL NOT change other windows already open.

#### Scenario: First use
- **GIVEN** the user has never chosen a page
- **WHEN** they press F9
- **THEN** the panel shows Files

#### Scenario: Restarting
- **WHEN** the panel shows Headings, the user presses Ctrl+Q and starts LeafMD again
- **THEN** the panel opens on Headings

#### Scenario: Two windows
- **WHEN** two windows show Files and the user switches the first to Headings
- **THEN** the second window still shows Files, and a new window opened with Ctrl+N shows Headings

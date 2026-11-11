# Spec Delta

## MODIFIED Requirements

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

## ADDED Requirements

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

# headings Specification

## Purpose
The Headings page of the side panel: a list of the selected document's headings for seeing its outline and jumping to a section, which keeps up with typing, tabs and scrolling.

## Requirements

### Requirement: What the Headings page lists
The Headings page SHALL list the selected document's markdown headings in order, both `#` headings and text underlined with `===` or `---`. Each SHALL be indented one step per level below the top one, and SHALL show its text as it reads, without markdown marks. LeafMD SHALL NOT list `#` lines inside code blocks, empty headings, or headings written as HTML tags.

#### Scenario: A short outline
- **WHEN** notes.md holds `# Garden Plan`, `## Seeds`, `### Tomatoes` and `## Beds`, and the user opens the Headings page
- **THEN** it lists "Garden Plan", then "Seeds" one step in, "Tomatoes" two steps in, and "Beds" one step in

#### Scenario: Formatting in a heading
- **WHEN** a heading reads `## **Install** [LeafMD](https://leafmd.app)`
- **THEN** the list shows "Install LeafMD"

#### Scenario: A comment in a code block
- **WHEN** a code block holds the line `# install the tools`
- **THEN** it isn't in the list

#### Scenario: A document starting at level 2
- **WHEN** a document's headings are `## Intro` and `### Details`, with no `#` heading
- **THEN** "Intro" is not indented and "Details" is one step in

### Requirement: The Headings page keeps up
LeafMD SHALL update the Headings page shortly after the user stops typing (about 150 milliseconds), and SHALL switch it to the selected tab's document when tabs change.

#### Scenario: Adding a heading
- **WHEN** the Headings page is showing and the user types `## Watering` on a new line under "Beds"
- **THEN** "Watering" appears in the list after "Beds" a moment after they stop typing

#### Scenario: Switching tabs
- **WHEN** notes.md and todo.md are open and the user selects todo.md's tab
- **THEN** the Headings page lists todo.md's headings

### Requirement: Nothing to list
When the selected document has no headings, the Headings page SHALL say "No Headings" and explain that lines starting with `#` appear here. With no document open, it SHALL say "No Document Open".

#### Scenario: A document without headings
- **WHEN** shopping.md holds only a list and the user opens the Headings page
- **THEN** the page says "No Headings", and that lines starting with # appear here

#### Scenario: No document
- **WHEN** the window shows "No Document Open" and the user opens the Headings page
- **THEN** the page says "No Document Open"

### Requirement: Jumping to a heading
Clicking a heading in the list, or selecting it and pressing Enter, SHALL scroll the document so that heading is at the top, in whichever view is showing (both sides in Split view). When the raw text is showing, LeafMD SHALL also put the cursor at the start of the heading's line.

#### Scenario: Formatted view
- **WHEN** a long notes.md is in Formatted view and the user clicks "Beds" in the list
- **THEN** the formatted document scrolls so the Beds heading is at the top

#### Scenario: Raw view
- **WHEN** notes.md is in Raw view, the user clicks "Beds" in the list and types "Raised "
- **THEN** the line reads `Raised ## Beds`, because the cursor was at the start of the heading's line

#### Scenario: Split view
- **WHEN** notes.md is in Split view and the user clicks "Tomatoes" in the list
- **THEN** both sides scroll so the Tomatoes heading is at the top

### Requirement: The Headings page follows your place
As the document scrolls, the Headings page SHALL highlight the heading of the section at the top of the document, and SHALL scroll its list to keep that heading in view. Above the first heading, nothing SHALL be highlighted.

#### Scenario: Scrolling down
- **WHEN** the user scrolls notes.md until text under "Seeds" is at the top
- **THEN** "Seeds" is highlighted in the list, and "Garden Plan" is not

#### Scenario: Back at the top
- **WHEN** notes.md starts with a paragraph before its first heading and the user scrolls back to the very top
- **THEN** nothing in the list is highlighted

#### Scenario: After a jump
- **WHEN** the user clicks "Beds" in the list
- **THEN** "Beds" is highlighted

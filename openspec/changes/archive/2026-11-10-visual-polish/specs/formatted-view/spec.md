# Spec Delta

## MODIFIED Requirements

### Requirement: Readable page styles
The formatted view SHALL show the document in a centred column at most 44em wide (about 85 characters of text), in GNOME's own fonts (Adwaita Sans, with code in Adwaita Mono, the same font as the raw text), with light and dark colours from GNOME's palette matching the system style.

#### Scenario: Wide window
- **WHEN** the window is very wide
- **THEN** the text stays in a centred column instead of stretching across the window

#### Scenario: Code matches the raw text
- **WHEN** the document contains `` `PROG` `` and the user looks at Split view
- **THEN** "PROG" is in the same monospace font on both sides

## ADDED Requirements

### Requirement: Follows the accent colour
The formatted view SHALL use the accent colour chosen in GNOME Settings for links, ticked checkboxes, the bar beside quotes and selected text, and SHALL change straight away when the accent colour changes. Links SHALL use GNOME's readable version of the accent for text (darker in light style, lighter in dark style).

#### Scenario: A purple accent
- **WHEN** the accent colour in GNOME Settings is purple and the document contains `[project page](https://example.org)` and `- [x] Test the soil`
- **THEN** the link text is purple and the ticked checkbox is filled purple with a white tick

#### Scenario: Changing the accent while LeafMD is open
- **WHEN** the user changes the accent colour from blue to green in GNOME Settings while a document is shown
- **THEN** its links and ticked checkboxes turn green without reopening anything

### Requirement: GNOME-style document parts
The formatted view SHALL draw its parts the way GNOME apps draw theirs: checkboxes as rounded squares (an outline when empty, filled with the accent and a white tick when ticked), tables and code blocks as rounded cards with thin lines, and headings heavy and without underlines.

#### Scenario: Checklist
- **WHEN** the document contains `- [x] Get permission from the city` and `- [ ] Order the raised beds`
- **THEN** the first shows a filled rounded checkbox with a white tick, the second an empty rounded outline, both clearly visible (not faded)

#### Scenario: Table
- **WHEN** the document contains a table with columns Item, Amount (right-aligned) and Cost (right-aligned)
- **THEN** it is drawn as a card with rounded corners, a tinted header row and thin lines between rows only, "Item" sits at the left of its column, the amounts and their header sit at the right, and digits line up in columns

#### Scenario: Headings
- **WHEN** the document starts with `# Community Garden Plan` followed later by `## Budget`
- **THEN** both are bold with no line under them, and there is clear space above "Budget"

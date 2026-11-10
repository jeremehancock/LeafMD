# formatted-view Specification

## Purpose
How markdown is turned into the formatted document: which markdown is supported, how images are found, and how the view is kept safe from anything harmful inside a document.

## Requirements

### Requirement: GitHub-style markdown
LeafMD SHALL render CommonMark plus the GitHub-style extras: tables, task lists (shown as ticked or unticked checkboxes that can't be clicked, in place of the bullet), strikethrough, footnotes, and plain web addresses turned into links automatically.

#### Scenario: Task list
- **WHEN** the document contains `- [x] done` and `- [ ] todo`
- **THEN** the formatted view shows a ticked and an unticked checkbox, with no bullet beside them

#### Scenario: Checklist and plain items in one list
- **WHEN** the document contains `- [ ] todo` followed by `- plain`
- **THEN** "todo" has only a checkbox and "plain" has its bullet

#### Scenario: Bare web address
- **WHEN** the document contains `https://example.com` on its own
- **THEN** it is shown as a clickable link

### Requirement: Raw HTML is allowed but made safe
LeafMD SHALL show HTML written inside a document (many READMEs use it for things like centred images), but MUST remove anything unsafe from it first, such as scripts, event handlers (like `onerror`) and iframes. Scripts inside documents MUST never run.

#### Scenario: Image with an event handler
- **WHEN** the document contains `<img src="a.png" onerror="alert(2)">`
- **THEN** the image is shown and the `onerror` part is removed

#### Scenario: Script tag
- **WHEN** the document contains `<script>alert(1)</script>`
- **THEN** nothing runs and the script isn't in the page

### Requirement: LeafMD's own scripts are walled off
LeafMD's helper script in the formatted view (used for scrolling, double-click and find) MUST run in a private area of the page that the document's content can't reach, and only that script SHALL be able to send messages to the app.

#### Scenario: A document tries to talk to the app
- **WHEN** a document contains HTML that tries to call LeafMD's helper functions
- **THEN** nothing happens, because document scripts are switched off and can't see the helper anyway

### Requirement: Images
LeafMD SHALL show images stored next to the file (relative paths like `images/photo.png` are resolved from the document's folder) and images online. Images SHALL never be wider than the page.

#### Scenario: Relative image
- **WHEN** `~/notes/todo.md` contains `![](img/cat.png)`
- **THEN** the formatted view shows `~/notes/img/cat.png`

### Requirement: Readable page styles
The formatted view SHALL show the document in a centred column at most 44em wide (about 85 characters of text), in GNOME's own fonts (Adwaita Sans, with code in Adwaita Mono, the same font as the raw text), with light and dark colours from GNOME's palette matching the system style.

#### Scenario: Wide window
- **WHEN** the window is very wide
- **THEN** the text stays in a centred column instead of stretching across the window

#### Scenario: Code matches the raw text
- **WHEN** the document contains `` `PROG` `` and the user looks at Split view
- **THEN** "PROG" is in the same monospace font on both sides

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

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
The formatted view SHALL show the document in a centred column at most 46em wide (about 90 characters of text), in the system font, with code in a monospace font, and with light and dark colours matching the system style.

#### Scenario: Wide window
- **WHEN** the window is very wide
- **THEN** the text stays in a centred column instead of stretching across the window

# links Specification

## Purpose
What happens when a link in the formatted view is clicked: web pages, other files, and headings in the same document.

## Requirements

### Requirement: Web links open in the browser
LeafMD SHALL open clicked web links in the user's default browser, through the desktop's portal when running as a Flatpak. If that fails, a message SHALL say "Couldn't open the link:" and why.

#### Scenario: Clicking a web link
- **WHEN** the user clicks a link to `https://example.com`
- **THEN** it opens in the default browser and the formatted view stays on the document

### Requirement: Links to text files open in LeafMD
LeafMD SHALL open a clicked link to a text file on this computer (markdown, `.txt`, `LICENSE` and so on) in a new selected tab in the same window, so the current document stays open. Relative links are resolved from the open document's folder. If the link names a heading (`other.md#install`), the new tab SHALL scroll to that heading once it opens.

#### Scenario: Link to another markdown file
- **WHEN** the user clicks `[Guide](docs/guide.md#setup)`
- **THEN** `docs/guide.md` next to the current file opens in a new tab, selected and scrolled to the "setup" heading

#### Scenario: The current document stays as it was
- **WHEN** the user has unsaved changes in README.md and clicks a link to LICENSE
- **THEN** nothing is asked, LICENSE opens in a new tab, and README.md keeps its changes and its scroll position in its own tab

### Requirement: Links to other files open in their usual app
LeafMD SHALL open a clicked link to a file that isn't text (an image, a PDF, a folder) in that file's usual app.

#### Scenario: Link to a PDF
- **WHEN** the user clicks a link to `manual.pdf`
- **THEN** it opens in the default PDF viewer

### Requirement: Missing linked files
LeafMD SHALL show a message saying "“name” doesn't exist" when a clicked link points to a file that doesn't exist.

#### Scenario: Broken link
- **WHEN** the user clicks a link to `missing.md` and there is no such file
- **THEN** the message "“missing.md” doesn't exist" appears and nothing else happens

### Requirement: Heading links in the same document
LeafMD SHALL scroll to the heading when a link to a heading in the same document is clicked, like `[Install](#install)`. Every heading SHALL get an id made from its text, GitHub style.

#### Scenario: Table of contents link
- **WHEN** the user clicks `[Install](#install)` in a document with an "Install" heading
- **THEN** the formatted view scrolls to that heading

### Requirement: The formatted view never leaves the document
The formatted view MUST NOT navigate away from LeafMD's own page. Every other attempt to change the page SHALL be blocked, and a file dropped onto the formatted view SHALL be opened as a document instead.

#### Scenario: Dropping a file on the formatted side
- **WHEN** the user drops a markdown file onto the formatted view
- **THEN** it opens as a document in LeafMD, rather than being shown as a web page

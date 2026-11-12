## MODIFIED Requirements

### Requirement: Opening files
LeafMD SHALL open a markdown or text file chosen with Open (Ctrl+O or the Open button), dropped anywhere on the window, double-clicked in the file manager, or given on the command line (`leafmd notes.md`). The one exception is an image dropped onto the raw text, which inserts an image link instead (see smart-typing). The Open dialog SHALL offer Markdown (.md, .markdown, .mdown, .mkd), Plain text and All files filters.

#### Scenario: Dropping a file
- **WHEN** the user drops a markdown file onto the window while another document is open
- **THEN** that file opens in a new selected tab in the window, and nothing is asked about unsaved changes because the other document stays open

#### Scenario: Dropping an image onto the raw text
- **WHEN** the user drops `photo.png` onto the raw text of a saved document
- **THEN** LeafMD doesn't try to open it, and inserts an image link instead

#### Scenario: Not plain text
- **WHEN** the user opens a file that isn't valid UTF-8 text
- **THEN** the file is not opened and a message says "This file isn't plain text, so LeafMD can't open it"

#### Scenario: Can't be read
- **WHEN** a file can't be read (for example it was deleted)
- **THEN** a message says the file could not be opened, no tab is added, and it is removed from Recent files

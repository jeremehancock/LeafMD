## MODIFIED Requirements

### Requirement: Help window
Help (F1) SHALL open a window explaining, in plain language, the three views, editing, the formatting bar, find and replace, tabs, the side panel with its folder tree and headings, the word count, Distraction Free and Full Screen, fonts and zoom, spell checking, printing and exporting, and files and links. Every user-visible feature SHALL be described there.

#### Scenario: Pressing F1
- **WHEN** the user presses F1
- **THEN** the Help window opens over the document

#### Scenario: Reading about tabs
- **WHEN** the user opens Help and looks for tabs
- **THEN** it explains opening and closing tabs, moving between them, dragging them to another window, and that links and opened files use a new tab

#### Scenario: Reading about the side panel
- **WHEN** the user opens Help and looks for the side panel
- **THEN** it explains opening and closing it with F9, that it stays open until closed, dragging its edge to make it wider or narrower, which folder the tree shows, Open Folder…, why some files are greyed out, and switching between Files and Headings at the bottom of the panel

#### Scenario: Reading about Distraction Free
- **WHEN** the user opens Help and looks for Distraction Free
- **THEN** it explains Ctrl+Shift+D, what gets hidden, bringing the bars back with the pointer at the top edge or F10, leaving with Escape, and that F11 adds full screen

#### Scenario: Reading about fonts and zoom
- **WHEN** the user opens Help and looks for fonts and zoom
- **THEN** it explains choosing the two fonts in Preferences (Ctrl+,), Reset, zooming with Ctrl+plus, Ctrl+minus, Ctrl+0 and Ctrl with the mouse wheel, that zoom covers the whole window, and that zoom is back to 100% each time LeafMD starts

#### Scenario: Reading about headings and the word count
- **WHEN** the user opens Help and looks for headings or the word count
- **THEN** it explains that the Headings page lists the document's headings, that clicking one jumps to it, that the section you're reading is highlighted, what the word count in the bottom-right corner counts, the 200 words a minute reading time, and that selecting text counts the selection

#### Scenario: Reading about printing and exporting
- **WHEN** the user opens Help and looks for printing or exporting
- **THEN** it explains that Ctrl+P prints the formatted document whichever view is showing, that pages and PDFs are always light, that Export as PDF… and Export as HTML… are in the main menu, that the web page packs in pictures from the computer and follows the reader's light or dark style, and that unsaved changes are included

#### Scenario: Reading about spell checking
- **WHEN** the user opens Help and looks for spelling
- **THEN** it explains that misspelt words in the raw text are underlined, that code and addresses aren't checked, that right-clicking a word offers suggestions, Add to Dictionary and Ignore, that Check Spelling in the main menu turns it off everywhere, and that Languages in the right-click menu changes the language

#### Scenario: Reading about code blocks
- **WHEN** the user opens Help and looks for code blocks
- **THEN** it explains writing a language after the opening ``` (such as ```python) to colour the code, the common short names like `sh`, `py`, `js` and `rs`, the list of suggestions that appears while typing one, and that a block with no language or an unknown one stays one colour

### Requirement: Markdown cheatsheet
The Markdown Cheatsheet (Shift+F1, the Cheatsheet button on the formatting bar, or the main menu) SHALL show the most common markdown next to what it looks like, in six sections: Text, Headings, Lists, Links and Images, Blocks, and Paragraphs and More.

#### Scenario: Looking up bold
- **WHEN** the user opens the cheatsheet
- **THEN** `**Bold text**` is shown next to "Bold text" in bold

#### Scenario: Looking up a code block
- **WHEN** the user opens the cheatsheet and looks at Blocks
- **THEN** the code block example starts with ```python, showing where a language goes

### Requirement: About window
About LeafMD SHALL show the app's name, icon, version and description, the developer (Jereme Hancock) with a link, an AI disclosure saying the project was created with the help of AI, a "Built With" section listing Rust, GTK, libadwaita, GtkSourceView and WebKitGTK with the versions LeafMD is running with, libspelling with the version LeafMD was built with, plus comrak, ammonia and syntect, the GPL 3.0 license, a Website link to `https://leafmd.app/`, a GitHub link and a link for reporting issues.

#### Scenario: Checking library versions
- **WHEN** the user opens About and goes to the credits
- **THEN** the AI disclosure appears right under the developer, and the Built With section shows the running GTK version

#### Scenario: Visiting the website
- **WHEN** the user opens About and clicks Website
- **THEN** `https://leafmd.app/` opens in their web browser

#### Scenario: Who colours the code
- **WHEN** the user opens About and goes to the credits
- **THEN** Built With lists syntect, saying it colours code blocks, with a link to its website

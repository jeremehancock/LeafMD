## MODIFIED Requirements

### Requirement: Main menu
The main menu (the button at the right of the header bar) SHALL start with the zoom row (Zoom Out, the percentage, Zoom In), then, in sections: New, New Window, Open…, Open Folder… and Close Document; Save and Save As…; Print…, Export as PDF… and Export as HTML…; Find… and Find and Replace…; Side Panel, Line Numbers, Check Spelling, Distraction Free and Full Screen; Preferences, Help, Markdown Cheatsheet, Keyboard Shortcuts, Support Development and About LeafMD. Each item with a keyboard shortcut SHALL show it.

#### Scenario: Opening the menu
- **WHEN** the user clicks the main menu button
- **THEN** the zoom row is at the top, followed by the six sections, separated by dividers, with New showing Ctrl+T, New Window showing Ctrl+N, Open Folder… showing Ctrl+Shift+O, Print… showing Ctrl+P, Side Panel showing F9, Distraction Free showing Shift+Ctrl+D, Full Screen showing F11 and Preferences showing Ctrl+,

#### Scenario: Where Support Development sits
- **WHEN** the user opens the main menu and looks at the last section
- **THEN** Support Development is just above About LeafMD, with no keyboard shortcut beside it

### Requirement: About window
About LeafMD SHALL show the app's name, icon, version and description, the developer (Jereme Hancock) with a link, an AI disclosure (created with the help of AI), a "Built With" section (Rust, GTK, libadwaita, GtkSourceView and WebKitGTK at the versions running, libspelling at the version built with, plus comrak, ammonia and syntect), the GPL 3.0 license, a Website link to `https://leafmd.app/`, a GitHub link, a Support Development link to `https://www.buymeacoffee.com/jeremehancock` and a link for reporting issues.

#### Scenario: Checking library versions
- **WHEN** the user opens About and goes to the credits
- **THEN** the AI disclosure appears right under the developer, and the Built With section shows the running GTK version

#### Scenario: Visiting the website
- **WHEN** the user opens About and clicks Website
- **THEN** `https://leafmd.app/` opens in their web browser

#### Scenario: Who colours the code
- **WHEN** the user opens About and goes to the credits
- **THEN** Built With lists syntect, saying it colours code blocks, with a link to its website

#### Scenario: Supporting from About
- **WHEN** the user opens About and clicks Support Development, next to Website and GitHub
- **THEN** `https://www.buymeacoffee.com/jeremehancock` opens in their web browser

## ADDED Requirements

### Requirement: Support window
Support Development in the main menu SHALL open a small window over the document with a heart, the heading "Support Development", a short paragraph saying LeafMD is free and asking for a few dollars toward its development, and one "Buy me a coffee" button. LeafMD SHALL never show this window, or any other request for money, unless the user asks for it.

#### Scenario: Buying a coffee
- **WHEN** the user chooses Support Development in the main menu and clicks Buy me a coffee
- **THEN** `https://www.buymeacoffee.com/jeremehancock` opens in their web browser, and the support window closes

#### Scenario: Changing their mind
- **WHEN** the support window is open and the user presses Esc or clicks its close button
- **THEN** it closes, nothing opens in the browser, and the document is just as it was

#### Scenario: Never asked unprompted
- **WHEN** the user starts LeafMD for the hundredth time
- **THEN** no support window or request for money appears

#### Scenario: No browser available
- **WHEN** the user clicks Buy me a coffee and the desktop can't open web links
- **THEN** LeafMD keeps running and says the link couldn't be opened, the same way it does for a link in a document

### Requirement: Support in Help
The Help window SHALL end with a "Support" section saying that LeafMD is free and that Support Development in the main menu is how to help pay for its development.

#### Scenario: Looking for a way to donate
- **WHEN** the user presses F1 and scrolls to the end of Help
- **THEN** the Support section mentions Support Development in the main menu

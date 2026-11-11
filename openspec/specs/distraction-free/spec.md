# distraction-free Specification

## Purpose
Distraction Free hides everything in a LeafMD window except the document, so the user can read or write with nothing else on screen, while keeping every control a pointer move or key press away.

## Requirements

### Requirement: Turning Distraction Free on and off
LeafMD SHALL turn Distraction Free on and off for the current window with Ctrl+Shift+D or Distraction Free in the main menu, which SHALL show a tick while it's on. It SHALL affect only that window, and every window SHALL open with it off.

#### Scenario: Turning it on
- **WHEN** a window shows notes.md in Split view and the user presses Ctrl+Shift+D
- **THEN** the header bar, tab bar and formatting bar disappear and the document fills the window

#### Scenario: Another window
- **WHEN** two windows are open and the user turns Distraction Free on in the first
- **THEN** the second window still shows its bars

#### Scenario: Not remembered
- **WHEN** the user closes LeafMD while in Distraction Free and starts it again
- **THEN** the window opens with all its bars showing

### Requirement: What Distraction Free hides
In Distraction Free, LeafMD SHALL hide the header bar, the tab bar, the formatting bar, the bottom bar of a narrow window, the side panel, the word count and the line numbers. The find bar and the banner about a file changed by another app SHALL still appear when needed.

#### Scenario: The side panel
- **WHEN** the side panel is open and the user turns Distraction Free on
- **THEN** the panel is hidden, and turning Distraction Free off brings it back

#### Scenario: The word count
- **WHEN** the bottom-right corner says "1,240 words · 7 min read" and the user presses Ctrl+Shift+D
- **THEN** the word count disappears, and pressing Ctrl+Shift+D again brings it back

#### Scenario: Finding text
- **WHEN** the user presses Ctrl+F in Distraction Free
- **THEN** the find bar appears above the document, as it does normally

#### Scenario: The file changes
- **WHEN** another app saves notes.md while it's open in Distraction Free
- **THEN** the banner saying the file changed appears above the document

### Requirement: The bars come back when wanted
In Distraction Free, LeafMD SHALL slide the hidden bars in over the document when the pointer reaches the window's top or bottom edge, and slide them away when it moves back over the document, except while one of their menus is open.

#### Scenario: Reaching for the menu
- **WHEN** the user moves the pointer to the top edge of a Distraction Free window
- **THEN** the header bar and formatting bar slide in over the top of the document, with a solid background

#### Scenario: Moving back to the text
- **WHEN** the bars are showing and the user moves the pointer down over the document
- **THEN** the bars slide away

#### Scenario: Using a menu
- **WHEN** the user opens the main menu from the slid-in header bar and moves down to About LeafMD
- **THEN** the menu and the bars stay until the user chooses something or closes the menu

### Requirement: F10 in Distraction Free
In Distraction Free, F10 SHALL slide the bars in and open the main menu. When that menu closes, the bars SHALL slide away again.

#### Scenario: Opening the menu by keyboard
- **WHEN** the user presses F10 in Distraction Free
- **THEN** the bars slide in and the main menu opens, with Distraction Free ticked

#### Scenario: Closing it again
- **WHEN** the main menu was opened with F10 in Distraction Free and the user presses Escape
- **THEN** the menu closes, the bars slide away, and the window is still in Distraction Free

### Requirement: Line numbers in Distraction Free
Turning Distraction Free on SHALL hide the window's line numbers. Choosing Line Numbers in the menu while in Distraction Free SHALL show them and also turn the Line Numbers setting on. Leaving SHALL show them or not according to the setting.

#### Scenario: Hidden on entering
- **WHEN** line numbers are on and the user turns Distraction Free on
- **THEN** the line numbers disappear, and Line Numbers in that window's menu has no tick

#### Scenario: Asking for them
- **WHEN** the user chooses Line Numbers from the menu in Distraction Free
- **THEN** line numbers show in that window, and still show after leaving Distraction Free

#### Scenario: A new tab
- **WHEN** line numbers are on and the user presses Ctrl+T in a Distraction Free window
- **THEN** the new document has no line numbers

### Requirement: Leaving with Escape
Escape SHALL leave Full Screen first, and a second Escape SHALL leave Distraction Free. Anything that already uses Escape SHALL come first: an open find bar closes, a side panel sliding over the document tucks away, and an open menu closes.

#### Scenario: Plain Escape
- **WHEN** the window is in Distraction Free with the cursor in the raw text and the user presses Escape
- **THEN** the bars come back and Distraction Free is off

#### Scenario: Find bar open
- **WHEN** the find bar is open in Distraction Free and the user presses Escape twice
- **THEN** the first press closes the find bar and the second leaves Distraction Free

#### Scenario: Both on
- **WHEN** the window is in Distraction Free and Full Screen and the user presses Escape
- **THEN** the window leaves full screen with the bars still hidden, and a second Escape brings the bars back and leaves Distraction Free

## MODIFIED Requirements

### Requirement: Smallest window size
A LeafMD window SHALL be usable down to 360 by 300 pixels (phone-sized screens) and SHALL NOT shrink further.

#### Scenario: Narrow window
- **WHEN** the user makes the window 360 pixels wide in Raw view
- **THEN** everything still fits, with the formatting bar scrolling sideways if needed

#### Scenario: Any view, with or without a document
- **WHEN** the window is dragged as narrow as it will go in Formatted view, in Split view, or on the "No Document Open" page
- **THEN** it reaches 360 pixels wide, the same as in Raw view

#### Scenario: Extra window buttons
- **WHEN** the desktop adds minimize and maximize buttons to the title bar and the window is 360 pixels wide
- **THEN** every button in the header bar is still fully visible

#### Scenario: Not smaller than the limit
- **WHEN** the user tries to drag the window to 300 by 200 pixels
- **THEN** it stops at 360 by 300 pixels

## ADDED Requirements

### Requirement: Compact layout in narrow windows
When a LeafMD window is narrower than 640 pixels, it SHALL use a compact layout: the New button, the Save button, the magnifying glass button and the view switcher SHALL move from the header bar to a bar along the bottom of the window, in that order. Open, its recent files arrow, the title, the main menu and the window's own buttons SHALL stay in the header bar. At 640 pixels or wider there SHALL be no bottom bar.

#### Scenario: Dragging a window narrower
- **WHEN** a window 800 pixels wide with notes.md open is dragged to 500 pixels wide
- **THEN** New, Save, the magnifying glass and the view switcher move to a bar along the bottom, and the header bar still shows Open, "notes.md", the main menu and the close button

#### Scenario: Dragging it wide again
- **WHEN** that window is dragged back to 800 pixels wide
- **THEN** the bottom bar is gone and the header bar looks exactly as it did before

#### Scenario: Opening at a narrow size
- **WHEN** LeafMD was last closed at 500 pixels wide and is opened again
- **THEN** it opens in the compact layout

#### Scenario: Nothing is lost while moved
- **WHEN** the window is 500 pixels wide, the find bar is open, the view is Split and the document has unsaved changes
- **THEN** the find bar stays open, the magnifying glass stays pressed in, the view switcher shows Split, the text and the cursor stay where they were, and the dot before the file name stays

#### Scenario: Shortcuts still work
- **WHEN** the window is 500 pixels wide and the user presses Ctrl+S, Ctrl+N, Ctrl+F or Ctrl+3
- **THEN** each does the same thing it does in a wide window

#### Scenario: Wide windows don't change
- **WHEN** the window is 1000 by 700 pixels
- **THEN** it looks exactly as it did before the compact layout existed

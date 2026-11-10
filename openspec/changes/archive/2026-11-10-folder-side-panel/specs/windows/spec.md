# Spec Delta

## MODIFIED Requirements

### Requirement: Compact layout in narrow windows
When a LeafMD window is narrower than 680 pixels, it SHALL use a compact layout: the New button, the Save button, the magnifying glass button and the view switcher SHALL move from the header bar to a bar along the bottom of the window, in that order. The side panel button, Open, its recent files arrow, the title, the main menu and the window's own buttons SHALL stay in the header bar. At 680 pixels or wider there SHALL be no bottom bar.

#### Scenario: Dragging a window narrower
- **WHEN** a window 800 pixels wide with notes.md open is dragged to 500 pixels wide
- **THEN** New, Save, the magnifying glass and the view switcher move to a bar along the bottom, and the header bar still shows the side panel button, Open, "notes.md", the main menu and the close button

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
- **WHEN** the window is 1000 by 700 pixels and the side panel is closed
- **THEN** it looks exactly as it did before the compact layout existed, apart from the side panel button at the left end of the header bar

#### Scenario: Just above the limit with extra window buttons
- **WHEN** the desktop adds minimize and maximize buttons to the title bar and the window is 680 pixels wide
- **THEN** every header bar button is fully visible and there is no bottom bar

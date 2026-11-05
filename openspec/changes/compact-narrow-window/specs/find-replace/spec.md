## MODIFIED Requirements

### Requirement: Opening the find bar
LeafMD SHALL show a find bar under the toolbar when the user presses Ctrl+F, clicks the magnifying glass button in the header bar (or in the bottom bar, in a window narrower than 640 pixels), or chooses Find… from the main menu. The cursor SHALL go into the find box with its text selected. If a single line of up to 200 characters is selected in the raw text, the find box SHALL start with that text. The magnifying glass button SHALL stay pressed in while the bar is open, and clicking it again SHALL close the bar.

#### Scenario: Searching for the selected word
- **WHEN** "install" is selected in the raw text and the user presses Ctrl+F
- **THEN** the find bar opens with "install" in the find box and its matches highlighted

#### Scenario: No document open
- **WHEN** no document is open
- **THEN** the magnifying glass button and the Find menu items are greyed out

#### Scenario: From the bottom bar
- **WHEN** the window is 500 pixels wide and the user clicks the magnifying glass in the bottom bar
- **THEN** the find bar opens under the formatting bar, at the top of the window, and the magnifying glass stays pressed in

## ADDED Requirements

### Requirement: Find bar in a narrow window
In a window narrower than 640 pixels, the find bar SHALL fit the window's width without cutting off or squashing anything. The find box and the close button SHALL have the first row to themselves, with the count, the arrows, the replace button and the options button on the row below. The replace row SHALL stay as one row, with its text box shrinking to fit. A hidden find bar SHALL NOT stop the window from getting narrower.

#### Scenario: Searching at the smallest size
- **WHEN** the window is 360 pixels wide and the user presses Ctrl+F and types "garden"
- **THEN** the find box fills the first row beside the close button, and the row below reads like "3 of 12" with the arrows, the replace button and the options button all fully visible

#### Scenario: Longest count
- **WHEN** the window is 360 pixels wide and the count reads "No matches"
- **THEN** the whole of "No matches" shows, and no button is pushed out of view

#### Scenario: Replacing at the smallest size
- **WHEN** the window is 360 pixels wide in Raw view and the user presses Ctrl+H
- **THEN** a row appears with the "Replace with" box and the Replace and Replace All buttons, all fully visible and with their words in full

#### Scenario: Wide windows keep one row
- **WHEN** the window is 1000 pixels wide and the find bar is open
- **THEN** the find box, count, arrows and buttons are on one row, as they were before the compact layout existed

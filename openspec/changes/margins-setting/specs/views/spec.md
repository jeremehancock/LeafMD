## MODIFIED Requirements

### Requirement: A readable column in Raw view
LeafMD SHALL keep the raw text in a centred column whose width comes from the Margins setting: about 140 characters for Narrow, 100 for Normal and 80 for Wide, or the whole width for None, with the scroll bar at the edge of the window or Split half. The column SHALL be measured in the current Raw Text font at the current zoom, and SHALL widen to make room for line numbers. In Split view the raw half SHALL use the same column; when the window or half is narrower than the column, the raw text SHALL fill it.

#### Scenario: A wide window
- **WHEN** Margins is Normal and a 1400 pixel wide window shows a document in Raw view
- **THEN** the text sits in a centred column, the space either side has the same background as the text, and long paragraphs wrap after about 100 characters

#### Scenario: No margins
- **WHEN** Margins is None and a 1920 pixel wide window shows a long paragraph in Raw view
- **THEN** the paragraph runs across the whole window, with only the small gap at each side

#### Scenario: A narrow window
- **WHEN** the window is narrower than the column
- **THEN** the raw text fills the window, as before

#### Scenario: Split view
- **WHEN** Margins is Normal and a 1400 pixel wide window shows the same document in Split view, divided down the middle
- **THEN** the raw text fills the left half, because the half is narrower than the column

#### Scenario: Dragging the Split divider
- **WHEN** Margins is Wide, a 1920 pixel wide window is in Split view, and the user drags the divider far to the right so the raw half is about 1500 pixels wide
- **THEN** the raw text stays in a centred column of about 80 characters within its half, with matching space either side, instead of stretching across it

#### Scenario: Zooming in Raw view
- **WHEN** a 1400 pixel wide window shows a document in Raw view and the user presses Ctrl+plus twice
- **THEN** the column grows wider along with the text, so a line holds the same number of characters before wrapping

## ADDED Requirements

### Requirement: Choosing the margins
LeafMD SHALL let the user choose the margins in Preferences: None, Narrow, Normal or Wide. Until the user picks one, the margins SHALL be Normal. A choice SHALL show straight away in every view of every open tab in every window, with no Apply button, and SHALL change how wide a line can get, not the text's size.

#### Scenario: First use
- **GIVEN** the user has never changed the margins
- **WHEN** they open Preferences
- **THEN** the Margins row shows Normal

#### Scenario: Choosing Wide
- **WHEN** two windows are open, one in Raw view and one in Formatted view, on a 1400 pixel wide screen, and the user clicks Wide next to Margins in Preferences
- **THEN** the text in both windows narrows at once to about 80 to 85 characters per line, with wider space either side, and the text is the same size as before

#### Scenario: Choosing Narrow
- **WHEN** a 1920 pixel wide window shows a long paragraph in Formatted view and the user clicks Narrow next to Margins
- **THEN** the paragraph's lines grow to about 140 characters and it takes fewer lines

### Requirement: The margins are remembered
LeafMD SHALL remember the margins chosen in Preferences and use them next time it starts.

#### Scenario: Restarting
- **WHEN** the user chooses Wide next to Margins, presses Ctrl+Q and starts LeafMD again
- **THEN** the text is in the Wide column, and Preferences shows Wide

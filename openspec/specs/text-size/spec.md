# text-size Specification

## Purpose
How big the text is and in which font: the Preferences window with the raw text and formatted view fonts, how they're remembered and reset, and zoom (its keys, the mouse wheel, the row in the main menu, which tabs and windows it covers, and that it isn't remembered).

## Requirements

### Requirement: Preferences window
LeafMD SHALL have a Preferences window, opened with Preferences in the main menu or Ctrl+, (comma). It SHALL hold two groups, Raw Text and Formatted View, each with one Font row whose button shows the font's name and size and opens GNOME's font picker. Preferences SHALL hold nothing else.

#### Scenario: Opening Preferences
- **WHEN** notes.md is open and the user presses Ctrl+,
- **THEN** the Preferences window opens over the document, with a Raw Text group and a Formatted View group, each showing one Font row

#### Scenario: From the main menu with no document open
- **WHEN** a window shows the "No Document Open" page and the user chooses Preferences in the main menu
- **THEN** the Preferences window opens

### Requirement: Starting fonts
Until the user changes them, the raw text SHALL use GNOME's monospace font (the one set in GNOME Settings, followed if it changes there) and the formatted view SHALL use Adwaita Sans at 12 points, the size it had before Preferences existed.

#### Scenario: First use
- **GIVEN** the user has never changed a font
- **WHEN** they open Preferences
- **THEN** the Raw Text font row shows GNOME's monospace font, such as "Adwaita Mono Regular 11", and the Formatted View row shows "Adwaita Sans Regular 12"

#### Scenario: Changing GNOME's monospace font
- **GIVEN** the raw text font has never been changed in LeafMD
- **WHEN** the user sets GNOME's monospace font to "Source Code Pro 13" in GNOME Settings
- **THEN** LeafMD's raw text shows in Source Code Pro 13

### Requirement: Choosing a font
LeafMD SHALL let the user pick any installed font, its style (such as Italic or Bold) and any size for each of the two groups, and SHALL show the font exactly as picked. A choice SHALL show straight away in the raw text or formatted view of every open tab in every window, with no Apply button. The formatted view's headings, code and tables SHALL keep their sizes relative to the chosen size, and its code SHALL stay in an upright monospace font.

#### Scenario: A bigger formatted view
- **WHEN** two windows are open and the user sets the Formatted View font to "Adwaita Sans 16"
- **THEN** the formatted text in every tab of both windows is bigger at once, the headings are still bigger than the text, and `code` is still monospace

#### Scenario: An italic font
- **WHEN** the user sets the Raw Text font to "Hack Italic 11"
- **THEN** the raw text shows in Hack Italic 11, and the Font row shows "Hack Italic 11"

#### Scenario: A different raw text font
- **WHEN** the user sets the Raw Text font to "Fira Code 12"
- **THEN** the raw text of every open document shows in Fira Code 12, and the formatted view doesn't change

### Requirement: Fonts are remembered
LeafMD SHALL remember both fonts after it is closed and started again.

#### Scenario: Restarting
- **WHEN** the user sets the Raw Text font to "Fira Code 12", presses Ctrl+Q and starts LeafMD again
- **THEN** the raw text shows in Fira Code 12, and Preferences shows "Fira Code 12"

### Requirement: Resetting a font
Each Font row SHALL have a Reset button, shown only while that font differs from its starting font. Reset SHALL put it back to the starting font, including following GNOME's monospace font again for the raw text.

#### Scenario: Resetting the raw text
- **WHEN** the Raw Text font is "Fira Code 12" and the user clicks its Reset button
- **THEN** the raw text goes back to GNOME's monospace font, and the Reset button disappears

#### Scenario: Nothing to reset
- **WHEN** neither font has been changed and the user opens Preferences
- **THEN** neither row shows a Reset button

### Requirement: Zooming
Ctrl+plus (or Ctrl+=) SHALL make the text bigger, Ctrl+minus smaller, and Ctrl+0 back to normal. Holding Ctrl and turning the mouse wheel over the document SHALL zoom in (wheel up) and out (wheel down). Zoom SHALL go in steps of 10%, from 50% to 300%, and SHALL change both the raw text and the formatted view, multiplying the Preferences sizes.

#### Scenario: Zooming in
- **WHEN** notes.md is open in Split view and the user presses Ctrl+plus twice
- **THEN** both the raw text and the formatted view are 120% of their normal size

#### Scenario: Back to normal
- **WHEN** the window is at 120% and the user presses Ctrl+0
- **THEN** both views are back to the sizes set in Preferences

#### Scenario: The smallest size
- **WHEN** the window is at 50% and the user presses Ctrl+minus
- **THEN** nothing changes

#### Scenario: Zoom with a chosen font
- **WHEN** the Formatted View font is "Adwaita Sans 14" and the user presses Ctrl+plus
- **THEN** the formatted text shows at 110% of 14 points

### Requirement: The zoom row in the main menu
The main menu SHALL start with a zoom row: a Zoom Out button, the current zoom as a percentage, and a Zoom In button. Clicking the percentage SHALL set zoom back to 100%. The buttons SHALL be greyed out at the smallest and largest sizes.

#### Scenario: Seeing the zoom
- **WHEN** the user has pressed Ctrl+plus once and opens the main menu
- **THEN** the zoom row shows "110%"

#### Scenario: Resetting from the menu
- **WHEN** the zoom row shows "130%" and the user clicks it
- **THEN** both views go back to normal size, and the row shows "100%"

#### Scenario: At the largest size
- **WHEN** the window is at 300% and the user opens the main menu
- **THEN** the Zoom In button is greyed out

### Requirement: Zoom belongs to a window
Zoom SHALL apply to every tab in the window it was used in, including tabs opened or dragged into that window later, and SHALL NOT change other windows. A new window SHALL start at 100%.

#### Scenario: Switching tabs
- **WHEN** a window has notes.md and todo.md open, and the user presses Ctrl+plus on notes.md then Ctrl+Page Down
- **THEN** todo.md is also at 110%

#### Scenario: A new window
- **WHEN** the first window is at 120% and the user presses Ctrl+N
- **THEN** the new window is at 100%, and the first stays at 120%

#### Scenario: Dragging a tab to another window
- **WHEN** a tab is dragged from a window at 100% into a window at 150%
- **THEN** the tab shows at 150%

### Requirement: Zoom isn't remembered
LeafMD SHALL NOT remember zoom. Each time LeafMD starts, every window SHALL be at 100%, showing the Preferences sizes.

#### Scenario: Restarting
- **WHEN** the window is at 140% and the user presses Ctrl+Q and starts LeafMD again
- **THEN** the window is at 100%

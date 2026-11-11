# Spec Delta

## ADDED Requirements

### Requirement: Resizing the side panel
While the side panel sits beside the document, dragging the line between them SHALL make the panel wider or narrower, from 200 to 400 pixels. Over that line the pointer SHALL show left-right resize arrows. The panel SHALL start at 240 pixels until the user resizes it. When the panel slides over the document in a narrow window, it SHALL be 240 pixels wide with nothing to drag.

#### Scenario: Making it wider
- **WHEN** the panel is open beside the document in a 1200 pixel window, at 240 pixels, and the user drags the line between them 100 pixels to the right
- **THEN** the panel is 340 pixels wide, and the document is 100 pixels narrower

#### Scenario: Dragging past the limits
- **WHEN** the user drags the line far to the right, then far to the left
- **THEN** the panel stops growing at 400 pixels, and stops shrinking at 200 pixels, staying open

#### Scenario: Hovering the line
- **WHEN** the panel is open beside the document and the pointer rests on the line between them
- **THEN** the pointer shows left-right resize arrows

#### Scenario: A narrow window
- **GIVEN** the user had widened the panel to 380 pixels
- **WHEN** the window is 700 pixels wide and the user presses F9
- **THEN** the panel slides over the document at 240 pixels, and its edge can't be dragged

### Requirement: The document keeps its room
While the side panel sits beside the document, the document SHALL keep at least 680 pixels. When the window is too narrow for the panel's chosen width, the panel SHALL show narrower, as narrow as 240 pixels, and SHALL return to the chosen width when the window is wide enough again, without changing the width that's remembered.

#### Scenario: Narrowing the window
- **GIVEN** the panel is 400 pixels wide beside the document in a 1200 pixel window
- **WHEN** the user drags the window to 1000 pixels wide
- **THEN** the panel is 320 pixels wide and the document 680

#### Scenario: Widening it again
- **WHEN** the user then drags the same window back to 1200 pixels
- **THEN** the panel is 400 pixels wide again

### Requirement: The side panel remembers its width
LeafMD SHALL open the side panel at the width the user last dragged it to, in new windows and after restarting. Resizing the panel in one window SHALL NOT change other windows already open.

#### Scenario: Restarting
- **WHEN** the user drags the panel to 320 pixels, presses Ctrl+Q and starts LeafMD again in a 1200 pixel window
- **THEN** the panel opens 320 pixels wide

#### Scenario: Two windows
- **WHEN** two windows have their panels open at 240 pixels and the user drags the first one's to 320
- **THEN** the second window's panel stays at 240, and a new window opened with Ctrl+N has its panel at 320

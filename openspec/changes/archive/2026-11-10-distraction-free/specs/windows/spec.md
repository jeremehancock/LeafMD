# Spec Delta

## ADDED Requirements

### Requirement: Full screen
LeafMD SHALL fill the screen with the current window when the user presses F11 or chooses Full Screen in the main menu, which SHALL show a tick while the window is full screen. Pressing F11 again, choosing the item again or pressing Escape SHALL leave full screen. Full Screen and Distraction Free SHALL be separate: turning one on SHALL NOT turn on the other.

#### Scenario: Going full screen
- **WHEN** the user presses F11
- **THEN** the window fills the screen with its header bar still showing, and Full Screen in the main menu has a tick

#### Scenario: Leaving with Escape
- **WHEN** the window is full screen, the find bar is closed and the user presses Escape
- **THEN** the window goes back to its previous size

#### Scenario: Separate from Distraction Free
- **WHEN** the user turns Distraction Free on
- **THEN** the window stays the same size and is not full screen

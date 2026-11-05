## MODIFIED Requirements

### Requirement: Switching views
LeafMD SHALL let the user switch views with the rounded view switcher in the header bar and with Ctrl+1 (Formatted), Ctrl+2 (Raw) and Ctrl+3 (Split), and the switcher SHALL always show the active view. In a window narrower than 640 pixels, the switcher SHALL be in the bottom bar instead of the header bar.

#### Scenario: Switching by keyboard
- **WHEN** the user presses Ctrl+3 while in Formatted view
- **THEN** Split view is shown and the switcher's Split button becomes the active one

#### Scenario: No document open
- **WHEN** no document is open
- **THEN** the view switcher is greyed out

#### Scenario: Switching in a narrow window
- **WHEN** the window is 400 pixels wide and the user clicks Raw on the switcher in the bottom bar
- **THEN** Raw view is shown and the switcher's Raw button becomes the active one

# Spec Delta

## MODIFIED Requirements

### Requirement: Tabs in a narrow window
In a window narrower than 680 pixels, the tab bar SHALL stay on its own row under the header bar. Tabs SHALL get narrower as more are opened and then scroll sideways, and the tab bar SHALL NOT stop the window from reaching 360 pixels wide.

#### Scenario: Several tabs at the smallest size
- **WHEN** the window is 360 pixels wide and 8 documents are open
- **THEN** the header bar still shows the side panel button, Open, the title, the main menu and the window's buttons, the tab bar scrolls sideways, and nothing is cut off

#### Scenario: The selected tab stays in view
- **WHEN** the window is 360 pixels wide with 8 tabs open and the user presses Alt+8
- **THEN** the tab bar scrolls so the eighth tab is fully visible

#### Scenario: Bottom bar works with tabs
- **WHEN** the window is 500 pixels wide with two tabs and the user clicks New in the bottom bar
- **THEN** a third tab opens and is selected

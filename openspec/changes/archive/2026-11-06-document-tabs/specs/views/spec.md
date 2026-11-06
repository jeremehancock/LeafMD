# Spec Delta

## MODIFIED Requirements

### Requirement: The view is remembered
LeafMD SHALL remember the last view used and start in it next time, and every tab it reopens SHALL start in that view. On the very first run it SHALL start in Formatted view.

#### Scenario: Reopening the app
- **WHEN** the user closes LeafMD while in Raw view and starts it again
- **THEN** it opens in Raw view

#### Scenario: Reopening several tabs
- **WHEN** a window has a tab in Formatted view and a tab in Split view, the Split tab is selected, the user closes the window and starts LeafMD again
- **THEN** both tabs reopen in Split view

### Requirement: New documents open where you can type
LeafMD SHALL switch from Formatted view to Split view in the new tab when the user starts a new document, and put the cursor in the raw text. Other tabs SHALL keep their views.

#### Scenario: New document from Formatted view
- **WHEN** the user presses Ctrl+T while in Formatted view
- **THEN** an empty document opens in a new tab in Split view with the cursor in the raw text

#### Scenario: Going back to the first tab
- **WHEN** notes.md is in Formatted view, the user presses Ctrl+T, and then selects notes.md's tab again
- **THEN** notes.md is still in Formatted view

## ADDED Requirements

### Requirement: Each tab has its own view
Each tab SHALL keep its own view, and the view switcher SHALL always show the selected tab's view. Changing the view SHALL only affect the selected tab. A file opening in a new tab SHALL start in the view last chosen in that window.

#### Scenario: Different views side by side
- **WHEN** notes.md is in Formatted view, todo.md is in Raw view, and the user selects notes.md's tab
- **THEN** notes.md shows in Formatted view and the switcher's Formatted button is the active one

#### Scenario: Ctrl+3 in one tab
- **WHEN** two tabs are in Formatted view and the user presses Ctrl+3 in the first
- **THEN** the first is in Split view and the second is still in Formatted view

#### Scenario: A newly opened file
- **WHEN** the user switches notes.md to Raw view and then opens todo.md with Ctrl+O
- **THEN** todo.md opens in a new tab in Raw view

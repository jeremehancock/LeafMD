# Spec Delta

## ADDED Requirements

### Requirement: Each tab has its own find bar
Each tab SHALL remember whether its find bar is open, what is in the find and replace boxes, and whether the replace row is showing, and these SHALL come back when the tab is selected. Searching SHALL only look in the selected tab. Match Case and Whole Words Only SHALL apply to every tab in the window.

#### Scenario: Find bar open in one tab
- **WHEN** the find bar is open in notes.md with "garden" in it and the user selects todo.md's tab, where it was never opened
- **THEN** no find bar shows over todo.md and the magnifying glass isn't pressed in

#### Scenario: Coming back
- **WHEN** the user then selects notes.md's tab again
- **THEN** the find bar is open with "garden" in it, its matches are highlighted, and the magnifying glass is pressed in

#### Scenario: Replace All stays in its tab
- **WHEN** notes.md and todo.md both contain "cat" and the user chooses Replace All in notes.md
- **THEN** only notes.md changes

#### Scenario: Search options
- **WHEN** the user ticks Match Case in notes.md and then searches in todo.md
- **THEN** Match Case is ticked there too

#### Scenario: Moving a tab to another window
- **WHEN** notes.md's find bar is open with "garden" and the user moves the tab to a new window
- **THEN** the find bar is open with "garden" in the new window

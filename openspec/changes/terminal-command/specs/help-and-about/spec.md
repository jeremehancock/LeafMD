# Spec Delta

## ADDED Requirements

### Requirement: Terminal command in Help
The "Files and Links" section of the Help window SHALL have a "From a terminal" row saying that `leafmd notes.md` opens a file after a one-time setup, with a Setup link that opens `https://leafmd.app/#terminal` in the browser. The setup commands themselves SHALL NOT be shown in the app, so that the website stays the only place they need updating.

#### Scenario: Finding the setup steps
- **WHEN** the user presses F1, scrolls to "Files and Links" and clicks Setup on the "From a terminal" row
- **THEN** `https://leafmd.app/#terminal` opens in their browser, at the setup section

#### Scenario: A narrow window
- **WHEN** the Help window is 360 pixels wide
- **THEN** the row's text wraps and the Setup link can still be clicked

## MODIFIED Requirements

### Requirement: Open, New and Save buttons
The left of the header bar SHALL have an Open button with a small arrow beside it that lists recent files, followed by a New button. A Save button SHALL sit just to the left of the view switcher. In a window narrower than 640 pixels, New and Save SHALL be in the bottom bar instead (see the compact layout in the windows spec), and Open SHALL stay in the header bar. Open and Save SHALL be words rather than icons, because icons for those actions vary a lot between icon themes. New SHALL use LeafMD's own new document icon. Open and its arrow SHALL look raised, sharing one background, as Save does.

#### Scenario: Hovering the buttons
- **WHEN** the user hovers over Open, New or Save
- **THEN** a tooltip names the action and its shortcut, like "Save (Ctrl+S)" or "New document (Ctrl+N)"

#### Scenario: Save icon on any distro
- **WHEN** LeafMD runs with any system icon theme
- **THEN** the Save button shows the word "Save", and the New button shows the same new document icon

#### Scenario: Clicking New with unsaved changes
- **WHEN** the user has typed in notes.md without saving and clicks the New button
- **THEN** LeafMD asks "Save Changes?", the same as pressing Ctrl+N

#### Scenario: The start page
- **WHEN** the "No Document Open" page is showing
- **THEN** the New button can be clicked and the Save button is greyed out

#### Scenario: In the bottom bar
- **WHEN** the window is 500 pixels wide, the "No Document Open" page is showing, and the user clicks New in the bottom bar
- **THEN** a new document opens, and Save, which was greyed out in the bottom bar, can now be clicked

# Spec Delta

## ADDED Requirements

### Requirement: Full names on hover
Hovering a file or folder in the tree that LeafMD can open SHALL show its whole name, so a name cut short to fit the panel can still be read. Greyed-out files SHALL keep saying "LeafMD can't open this type of file" instead.

#### Scenario: A long file name
- **WHEN** the tree shows `2026-11-meeting-notes-for-the-garden-committee.md`, cut short in the middle to fit, and the user hovers over it
- **THEN** a tooltip shows `2026-11-meeting-notes-for-the-garden-committee.md` in full

#### Scenario: A folder
- **WHEN** the user hovers over the folder `drafts` in the tree
- **THEN** a tooltip shows `drafts`

#### Scenario: A greyed-out file
- **WHEN** the user hovers over logo.png
- **THEN** the tooltip says "LeafMD can't open this type of file"

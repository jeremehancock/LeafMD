## MODIFIED Requirements

### Requirement: Readable page styles
The formatted view SHALL show the document in a centred column whose width comes from the Margins setting (about 140 characters of text for Narrow, 100 for Normal and 85 for Wide, or the whole width for None), in GNOME's own fonts (Adwaita Sans, with code in Adwaita Mono, the same font as the raw text), with light and dark colours from GNOME's palette matching the system style. The column SHALL be measured in the formatted view's font at the current zoom, and SHALL be the same in Formatted view and the formatted half of Split view.

#### Scenario: Wide window
- **WHEN** Margins is Normal and the window is very wide
- **THEN** the text stays in a centred column instead of stretching across the window

#### Scenario: No margins
- **WHEN** Margins is None and a 1920 pixel wide window shows a long paragraph in Formatted view
- **THEN** the paragraph runs across the whole window, with only the page's usual padding at each side

#### Scenario: Wide margins
- **WHEN** Margins is Wide and a 1920 pixel wide window shows a long paragraph in Formatted view
- **THEN** its lines hold about 85 characters, the same as before the Margins setting existed

#### Scenario: Dragging the Split divider
- **WHEN** Margins is Normal, a 1920 pixel wide window is in Split view, and the user drags the divider far to the left so the formatted half is about 1500 pixels wide
- **THEN** the formatted text stays in a centred column of about 100 characters within its half

#### Scenario: Code matches the raw text
- **WHEN** the document contains `` `PROG` `` and the user looks at Split view
- **THEN** "PROG" is in the same monospace font on both sides

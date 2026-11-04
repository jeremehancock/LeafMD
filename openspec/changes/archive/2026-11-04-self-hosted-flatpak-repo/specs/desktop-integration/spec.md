# Spec Delta

## MODIFIED Requirements

### Requirement: App store listing
The app store description SHALL list LeafMD's main features and stay in step with them, and the app SHALL be described as working with keyboard and mouse on screens at least 360 pixels wide. The listing SHALL give `https://leafmd.app/` as LeafMD's homepage, SHALL have a light and a dark brand colour, and SHALL show screenshots of the formatted, raw and split views in light style (formatted first), plus the split view in dark style, each with a one-sentence caption.

#### Scenario: A feature is added
- **WHEN** a new user-visible feature ships
- **THEN** the description in the metainfo file mentions it

#### Scenario: The interface changes before a release
- **GIVEN** a change since the screenshots were last taken adds a button to the formatting bar
- **WHEN** that version is released
- **THEN** the screenshots in the listing and on `https://leafmd.app/` show the new button, unless the owner chose to release with the old ones

#### Scenario: Viewing LeafMD in GNOME Software
- **WHEN** someone opens LeafMD's page in GNOME Software
- **THEN** it shows the formatted view screenshot first, with the other three after it
- **AND** the Website link opens `https://leafmd.app/`

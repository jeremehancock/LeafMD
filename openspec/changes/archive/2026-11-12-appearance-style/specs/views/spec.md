## MODIFIED Requirements

### Requirement: Follows the system style
LeafMD SHALL show the style chosen in Preferences (Follow System, Light or Dark) in its windows, the raw text (GNOME's Adwaita or Adwaita-dark editor colours) and the formatted view. With Follow System, it SHALL follow the system's light or dark style and switch straight away when the system style changes. The raw text and the formatted view SHALL have exactly the same background colour.

#### Scenario: Switching to dark mode
- **GIVEN** Style is Follow System
- **WHEN** the system switches to dark style while a document is open
- **THEN** the raw text and the formatted view both turn dark without reopening anything

#### Scenario: Split view in dark style
- **WHEN** a document is in Split view in dark style
- **THEN** the raw text half and the formatted half are the same shade, with no visible change of colour at the divider except the divider line itself

#### Scenario: Dark on a light desktop
- **GIVEN** the system style is light and notes.md is open in Split view
- **WHEN** the user picks Dark in Preferences
- **THEN** the header bar, the raw text and the formatted view all turn dark straight away, in every LeafMD window, while other apps stay light

#### Scenario: Light ignores the system
- **GIVEN** Style is Light
- **WHEN** the system switches to dark style
- **THEN** LeafMD stays light

#### Scenario: Back to following the system
- **GIVEN** Style is Dark and the system style is light
- **WHEN** the user picks Follow System
- **THEN** LeafMD turns light straight away

## ADDED Requirements

### Requirement: The style is remembered
LeafMD SHALL remember the style chosen in Preferences and start in it next time. Until the user picks one, the style SHALL be Follow System.

#### Scenario: First use
- **GIVEN** the user has never changed the style
- **WHEN** they open Preferences
- **THEN** the Style row shows Follow System

#### Scenario: Dark after a restart
- **WHEN** the user picks Dark, quits LeafMD with Ctrl+Q and opens notes.md again
- **THEN** LeafMD opens in dark style and the Style row in Preferences shows Dark

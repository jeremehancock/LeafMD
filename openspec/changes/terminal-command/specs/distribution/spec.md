# Spec Delta

## ADDED Requirements

### Requirement: Terminal command setup
The storefront SHALL have a section at `https://leafmd.app/#terminal`, always visible, that explains the one-time setup for the `leafmd` command. It SHALL give a copyable command for `/usr/local/bin` (needs the admin password) and one for `~/.local/bin` (doesn't). The README SHALL give the same commands.

#### Scenario: Setting up with the admin password
- **GIVEN** the computer has no `/usr/local/bin/leafmd`
- **WHEN** someone pastes the main command into a terminal and enters their password
- **THEN** typing `leafmd notes.md` opens notes.md in LeafMD

#### Scenario: Setting up without the admin password
- **GIVEN** the computer has no `~/.local/bin/leafmd`
- **WHEN** someone pastes the second command into a terminal
- **THEN** the script is added to `~/.local/bin`, and the section explains what to do if `leafmd` still isn't found

#### Scenario: A leafmd already exists
- **GIVEN** `/usr/local/bin/leafmd` already exists
- **WHEN** someone pastes the main command
- **THEN** the terminal shows an error and the existing file is unchanged

#### Scenario: Using a shell other than bash
- **WHEN** someone pastes either command into zsh or fish
- **THEN** it does the same as in bash

#### Scenario: On a phone
- **WHEN** someone opens `https://leafmd.app/#terminal` on a 360-pixel-wide screen
- **THEN** the commands wrap or scroll inside their own boxes, and the page doesn't scroll sideways

#### Scenario: Removing it
- **WHEN** someone wants to undo the setup
- **THEN** the section tells them which file to delete

### Requirement: Script works with every version
The storefront and the README SHALL say that the `leafmd` script works with every version of LeafMD, and works best with the latest version, which doesn't hold the terminal. They SHALL NOT present any version as a requirement.

#### Scenario: Reading the setup section with an older LeafMD installed
- **GIVEN** someone has an older LeafMD that hasn't updated yet
- **WHEN** they read the setup section
- **THEN** it's clear they can set up the script now, that files will open normally, and that updating LeafMD frees the terminal

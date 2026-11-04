# Spec Delta

## MODIFIED Requirements

### Requirement: Published only on release
The repository SHALL change only when a release is made (a merge into `main` whose newest `<release>` isn't marked `type="development"` and has no `v<version>` tag yet), and SHALL then hold that version and no older ones. Republishing the site between releases SHALL leave the repository exactly as that release published it.

#### Scenario: Unreleased work
- **WHEN** a commit is pushed to `dev`, or a merge into `main` carries a version marked `type="development"`
- **THEN** the repository and the storefront stay as they were

#### Scenario: A release
- **WHEN** a merge into `main` makes the 0.2.0 release
- **THEN** the repository offers 0.2.0, and 0.1.0 is no longer in it

## ADDED Requirements

### Requirement: Help when Flatpak isn't set up
Under the storefront's Install button, LeafMD SHALL show everyone the line "On Ubuntu? Set up Flatpak first.", linking to the Ubuntu setup steps. The install section SHALL say that a file opening in a text editor means Flatpak isn't set up yet, and link to `https://flatpak.org/setup/` for other systems. This SHALL work with scripts turned off.

#### Scenario: Chrome on Ubuntu
- **WHEN** someone using Chrome on Ubuntu visits `https://leafmd.app/`
- **THEN** they see "On Ubuntu? Set up Flatpak first." under the Install button, and clicking it takes them to the Ubuntu setup steps

#### Scenario: The file opened in a text editor
- **WHEN** someone whose `LeafMD.flatpakref` opened in a text editor reads the install section
- **THEN** it tells them Flatpak isn't set up yet and links to `https://flatpak.org/setup/`

### Requirement: Ubuntu setup steps
The install section SHALL have a "Setting up Flatpak on Ubuntu" box with the commands `sudo apt install flatpak gnome-software-plugin-flatpak` and `flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo`, each with a Copy button when scripts are on, and a note to restart afterwards. The box SHALL start closed and open when clicked, except for visitors whose browser says it runs on Ubuntu, for whom it SHALL start open.

#### Scenario: Firefox on Ubuntu
- **WHEN** someone using Firefox on Ubuntu visits `https://leafmd.app/`
- **THEN** the "Setting up Flatpak on Ubuntu" box is already open, and the line under the Install button says they're on Ubuntu

#### Scenario: Another system
- **WHEN** someone using Firefox on Fedora visits `https://leafmd.app/`
- **THEN** the box is closed, and opens when they click "Setting up Flatpak on Ubuntu"

#### Scenario: Copying a command
- **WHEN** someone clicks Copy next to `sudo apt install flatpak gnome-software-plugin-flatpak`
- **THEN** exactly that command is on their clipboard

### Requirement: Republishing the site
The owner SHALL be able to republish leafmd.app between releases with a manual run of the GitHub build. A republish SHALL publish the storefront's text and design from `main`, with the version, icon, screenshots and Flatpak repository of the newest release, without building the app again.

#### Scenario: A wording fix
- **GIVEN** 0.1.0 is the newest release, and a fix to the storefront's wording has been merged into `main`
- **WHEN** the owner republishes leafmd.app
- **THEN** `https://leafmd.app/` shows the new wording and still says "Version 0.1.0"
- **AND** `flatpak update` on a computer with LeafMD 0.1.0 installed finds nothing to update

#### Scenario: Unreleased app changes on main
- **GIVEN** `main` has app changes for 0.2.0, still marked `type="development"`
- **WHEN** the owner republishes leafmd.app
- **THEN** the repository still offers 0.1.0, and the storefront's screenshots and version are 0.1.0's

### Requirement: Sharp main screenshot
On a window wide enough for it, the storefront SHALL show its large screenshot at its exact size, so the app's text in it isn't blurred by resizing.

#### Scenario: A laptop screen
- **WHEN** someone views `https://leafmd.app/` in a window 1280 pixels wide
- **THEN** the large screenshot is shown 1050 pixels wide, the size it was taken at

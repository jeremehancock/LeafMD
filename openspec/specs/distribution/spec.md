# distribution Specification

## Purpose
How people find, get and keep LeafMD up to date without building it: the storefront at leafmd.app, its own signed Flatpak repository with a one-click install, the file on each GitHub release, and the Flathub checks every build must pass.

## Requirements

### Requirement: One-click install
LeafMD SHALL be installable from `https://leafmd.app/LeafMD.flatpakref`. Opening that file SHALL show LeafMD ready to install in GNOME Software (or any app store that understands Flatpak), and installing it SHALL also fetch the GNOME runtime from Flathub, adding Flathub first if it isn't set up.

#### Scenario: Installing from GNOME Software
- **WHEN** someone downloads `LeafMD.flatpakref` in Firefox and opens it
- **THEN** GNOME Software shows LeafMD with an Install button
- **AND** after clicking Install, LeafMD appears in the app grid and opens normally

#### Scenario: Installing from the terminal
- **WHEN** someone runs `flatpak install https://leafmd.app/LeafMD.flatpakref`
- **THEN** LeafMD and the `org.gnome.Platform` runtime it needs are installed

#### Scenario: Flathub not set up yet
- **WHEN** someone without Flathub opens `LeafMD.flatpakref`
- **THEN** Flatpak offers to add Flathub for the runtime, and installing then succeeds

### Requirement: Storefront
LeafMD SHALL have a one-page storefront at `https://leafmd.app/` showing its icon, name and summary, an Install button that opens `LeafMD.flatpakref`, the screenshots, the main features, and links to GitHub, issues and the GPL 3.0 license, with the same AI disclosure line the About window shows.

#### Scenario: First visit
- **WHEN** someone visits `https://leafmd.app/` on a laptop
- **THEN** they see LeafMD's icon, "LeafMD", "Read and edit markdown your way" and an Install button without scrolling
- **AND** scrolling down shows the screenshots, the features, and "This project was created with the help of AI." in the footer

#### Scenario: Other addresses
- **WHEN** someone visits `https://www.leafmd.app/` or `https://jeremehancock.github.io/LeafMD/`
- **THEN** they end up at `https://leafmd.app/`

### Requirement: Every way to install
The storefront SHALL explain each way to install LeafMD: the Install button for GNOME Software, the command `flatpak install https://leafmd.app/LeafMD.flatpakref`, and the `.flatpak` file on the GitHub releases page. It SHALL say LeafMD needs a Linux computer with Flatpak, on x86_64.

#### Scenario: Installing from a terminal
- **WHEN** someone reads the storefront's install section
- **THEN** it shows `flatpak install https://leafmd.app/LeafMD.flatpakref` as text they can select and copy

### Requirement: Current version on the storefront
The storefront SHALL show the version the repository offers and that release's notes, taken from the newest release in the app store description, so the site and the repository never disagree.

#### Scenario: After a release
- **WHEN** 0.2.0 is released
- **THEN** the storefront says "Version 0.2.0" with 0.2.0's release notes

### Requirement: Storefront look and behaviour
The storefront SHALL follow the visitor's light or dark style, fit a 360-pixel-wide phone screen without sideways scrolling, and work fully with scripts turned off. It SHALL load nothing from other websites and SHALL use no tracking or cookies.

#### Scenario: Dark style on a phone
- **WHEN** someone opens `https://leafmd.app/` on a phone set to dark style
- **THEN** the page is dark, everything fits the screen width, and the Install button and screenshots are readable

#### Scenario: Nothing from elsewhere
- **WHEN** the page loads with the browser's network panel open
- **THEN** every request goes to `leafmd.app`

### Requirement: Link previews
The storefront SHALL give link previews a title, the summary and a screenshot, so posting `https://leafmd.app/` in a chat app shows a card for LeafMD.

#### Scenario: Sharing the link
- **WHEN** someone pastes `https://leafmd.app/` into a Matrix or Discord message
- **THEN** a preview card shows "LeafMD", "Read and edit markdown your way" and a screenshot

### Requirement: Updates
Once installed from the repository, LeafMD SHALL receive each new release as an ordinary Flatpak update, shown in GNOME Software's Updates and applied by `flatpak update`.

#### Scenario: A new version is released
- **GIVEN** LeafMD 0.1.0 is installed from the repository
- **WHEN** 0.2.0 is released
- **THEN** `flatpak update` offers `io.github.jeremehancock.LeafMD`, and afterwards Help, About shows 0.2.0

### Requirement: Published only on release
The repository SHALL change only when a release is made (a merge into `main` whose newest `<release>` isn't marked `type="development"` and has no `v<version>` tag yet), and SHALL then hold that version and no older ones.

#### Scenario: Unreleased work
- **WHEN** a commit is pushed to `dev`, or a merge into `main` carries a version marked `type="development"`
- **THEN** the repository and the storefront stay as they were

#### Scenario: A release
- **WHEN** a merge into `main` makes the 0.2.0 release
- **THEN** the repository offers 0.2.0, and 0.1.0 is no longer in it

### Requirement: Signed updates
Everything in the repository SHALL be signed with LeafMD's signing key, and both `LeafMD.flatpakref` and the GitHub release file SHALL carry that key's public half, so Flatpak installs and updates LeafMD only if it was signed by that key.

#### Scenario: A tampered update
- **WHEN** a copy of LeafMD that isn't signed with LeafMD's key is put in the repository
- **THEN** `flatpak update` refuses it with a signature error and keeps the installed version

### Requirement: GitHub release file updates too
The `LeafMD-<version>.flatpak` file attached to each GitHub release SHALL install LeafMD with the repository set as its source, so installs from the file receive later releases as updates.

#### Scenario: Installing from the file, then updating
- **GIVEN** someone installed `LeafMD-0.1.0.flatpak` by opening it in GNOME Software
- **WHEN** 0.2.0 is released
- **THEN** GNOME Software offers the update to 0.2.0

### Requirement: Flathub checks
Every build SHALL pass Flathub's checker (`flatpak-builder-lint`) on the build recipe, the app store description and the finished build, and any problem SHALL fail the build. The only problems allowed SHALL be those in a written exceptions list, each with its reason.

#### Scenario: A problem Flathub would reject
- **WHEN** a change makes the app store description invalid, for example an empty `<summary>`
- **THEN** the GitHub build fails and names the problem

#### Scenario: Allowed exception
- **WHEN** the finished build is checked
- **THEN** `appstream-screenshots-not-mirrored-in-ostree` (Flathub's servers keeping their own copy of the screenshots) is allowed because the exceptions list names it, and the build passes
- **AND** removing a screenshot from the metainfo file fails the build with `metainfo-missing-screenshots`

#### Scenario: Before the first release
- **GIVEN** LeafMD has no `v...` release tag yet, so `https://leafmd.app/` isn't live
- **WHEN** a build is checked
- **THEN** the screenshots and homepage on `leafmd.app` being unreachable are allowed, every other check still applies, and the build passes
- **AND** once `v0.1.0` exists, an unreachable screenshot or homepage fails the build

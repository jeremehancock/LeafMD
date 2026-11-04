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
The storefront SHALL show the version the repository offers, taken from the newest release in the app store description so the site and the repository never disagree, as a link to that version's release notes on GitHub.

#### Scenario: After a release
- **WHEN** 0.2.0 is released
- **THEN** the storefront's hero says "Version 0.2.0"
- **AND** clicking it opens `https://github.com/jeremehancock/LeafMD/releases/tag/v0.2.0`

### Requirement: Storefront look and behaviour
The storefront SHALL follow the visitor's light or dark style, fit a 360-pixel-wide phone screen without sideways scrolling, and work fully with scripts turned off. It SHALL load nothing from other websites and SHALL use no tracking or cookies.

#### Scenario: Dark style on a phone
- **WHEN** someone opens `https://leafmd.app/` on a phone set to dark style
- **THEN** the page is dark, everything fits the screen width, and the Install button and screenshots are readable

#### Scenario: Nothing from elsewhere
- **WHEN** the page loads with the browser's network panel open
- **THEN** every request goes to `leafmd.app`

### Requirement: Larger screenshots
On a screen at least 700 pixels wide, clicking one of the storefront's small screenshots SHALL show it full size over the page with its caption, previous and next buttons, and SHALL close with Esc, the close button or a click outside the picture. On a narrower screen, or with scripts turned off, clicking SHALL open the picture by itself.

#### Scenario: Looking closer on a laptop
- **WHEN** someone clicks the raw view screenshot on a 1280-pixel-wide window
- **THEN** it opens full size over the page with "Edit markdown with the formatting bar" under it
- **AND** the next button shows the next screenshot, and Esc closes the viewer

#### Scenario: On a phone
- **WHEN** someone taps the raw view screenshot on a 390-pixel-wide phone
- **THEN** the browser opens `screenshots/raw.png` by itself, where it can be zoomed

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
The repository SHALL change only when a release is made (a merge into `main` whose newest `<release>` isn't marked `type="development"` and has no `v<version>` tag yet), and SHALL then hold that version and no older ones. Republishing the site between releases SHALL leave the repository exactly as that release published it.

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

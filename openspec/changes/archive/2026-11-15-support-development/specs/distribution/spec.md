## ADDED Requirements

### Requirement: Support section on the storefront
The storefront SHALL end, after "Early days" and before the footer, with a "Support development" section reached at `https://leafmd.app/#support`: a heart, a short paragraph saying LeafMD is free and asking for a few dollars toward its development, and a "Buy me a coffee" button opening `https://www.buymeacoffee.com/jeremehancock` in a new tab. The top bar (on screens at least 600 pixels wide) and the footer SHALL link to it. The button SHALL be drawn by the page itself.

#### Scenario: Supporting from the website
- **WHEN** someone scrolls to the bottom of `https://leafmd.app/` and clicks Buy me a coffee
- **THEN** `https://www.buymeacoffee.com/jeremehancock` opens in a new tab, and the storefront stays open in the original tab

#### Scenario: From the top bar
- **WHEN** someone on a 1280-pixel-wide window clicks "Support" in the top bar, after Terminal
- **THEN** the page moves to the Support development section, and "Support" is highlighted in the top bar

#### Scenario: Top bar on a narrow screen
- **WHEN** someone opens `https://leafmd.app/` in a 560-pixel-wide window
- **THEN** the top bar shows Features, Install, Terminal and GitHub without "Support", and everything fits

#### Scenario: From the footer
- **WHEN** someone clicks "Support development" in the footer
- **THEN** the page moves to the Support development section

#### Scenario: Still nothing from elsewhere
- **WHEN** the page loads with the browser's network panel open
- **THEN** the support section's heart and button load nothing from buymeacoffee.com or anywhere else

### Requirement: Donation link in app stores
LeafMD's app store description SHALL carry a donation link to `https://www.buymeacoffee.com/jeremehancock`, so app stores that show one (such as GNOME Software) offer it on LeafMD's page.

#### Scenario: GNOME Software
- **WHEN** someone opens LeafMD's page in GNOME Software after installing from leafmd.app
- **THEN** its links include one to donate, which opens `https://www.buymeacoffee.com/jeremehancock`

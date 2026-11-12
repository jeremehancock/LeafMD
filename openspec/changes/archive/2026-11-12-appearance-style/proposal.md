# Proposal

## Why

LeafMD always follows the desktop's light or dark style, with no way to choose otherwise. Some people want their editor dark while the rest of the desktop is light (or the other way round), for example for writing at night or reading in bright light, and GNOME apps such as Text Editor let them pick.

## What Changes

- A new **Appearance** group at the top of Preferences, with one **Style** row offering three choices side by side: **Follow System**, **Light** and **Dark**.
- Follow System is the starting choice and works exactly as LeafMD does today.
- Choosing Light or Dark changes every LeafMD window at once: the header bar and other controls, the raw text colours, and the formatted view. It changes straight away, without reopening anything.
- The choice is remembered, so LeafMD starts in it next time.
- New setting: the Style row. No new keyboard shortcut and no new menu item (it lives in Preferences only).

Out of scope:

- Printing and Export as PDF stay light, since they're meant for paper. Export as HTML keeps following the reader's own light or dark setting, not LeafMD's.
- Windows that belong to the desktop rather than LeafMD (the file chooser and the Print dialog) keep the desktop's style.
- No other themes, colour choices or custom colours. The accent colour still comes from GNOME Settings.
- No shortcut or main menu item for switching style.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `views`: "Follows the system style" becomes "follows the chosen style": the system's by default, or always light or always dark when the user picks that in Preferences. A new requirement covers remembering the choice.
- `text-size`: the "Preferences window" requirement says Preferences holds only the two font groups. It changes to also hold the Appearance group with the Style row, shown first.

## Impact

- `src/preferences.rs`: the new Appearance group and Style row.
- `src/main.rs`: an app-wide "style" choice, applied when LeafMD starts and whenever it changes. Everything that already follows light and dark (raw text colours, formatted view, spelling underline) keeps working because it reads the same light-or-dark answer.
- `src/state.rs`: remembering the choice in LeafMD's settings file.
- An end-to-end scenario checking the choice is remembered.
- Help, README, the app store description, leafmd.app and ROADMAP.md mention it. The screenshots don't change, since they're taken with Follow System.

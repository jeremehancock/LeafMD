# Proposal

## Why

Starting a new document takes a trip to the main menu unless you know Ctrl+N, even though it's one of the most basic things to do. Save also sits among the file buttons on the left, next to Open, when the owner wants it on the right, close to the view switcher.

## What Changes

- **New button: a new document icon** in the header bar, in the spot where the Save icon is now (just right of Open). It does the same as New in the main menu and Ctrl+N, including the unsaved-changes check. Its tooltip reads "New document (Ctrl+N)".
- **The Save button moves** to just left of the view switcher.
- **Save becomes a word instead of an icon**, matching Open. LeafMD's floppy disk icon is no longer used, so it is removed.
- **Open and its recent files arrow look raised**, with one background across both parts like gedit's Open button, so they match the Save button.
- **New maximize and restore icons on the window's title bar**, drawn by LeafMD to look like gedit's on the owner's desktop: two corner arrows normally, and a rounded square while maximized. Adwaita's versions look small next to minimize and close. They look the same on every distro.
- **LeafMD draws the rest of its everyday icons too:** the magnifying glass, main menu, the find bar's arrows and three dots, the start page's page, and minimize and close. They keep the same shapes as before, but a GNOME update can no longer change how they look. Small icons GTK adds by itself, like the arrow beside Open, still come from Adwaita.
- **The smallest window gets a little wider.** The header bar needs more room, so with minimize, maximize and close buttons the smallest window grows by about 40 pixels. Nothing hides or gets cut off.
- **No new keyboard shortcuts, menu items or settings.** Ctrl+S, Ctrl+N and the main menu are unchanged.

The header bar then reads, left to right: Open, New, the document title, Save, the view switcher, Find, the main menu.

### Out of scope

- Changing what New or Save do.
- Moving any other header bar button, or changing what it does.
- Replacing the small icons GTK adds by itself.
- Hiding header bar buttons in narrow windows.
- Making windows shrink to 360 pixels. The `windows` spec promises this, but it was already not true before this change (the window stopped at about 566 pixels in testing). That needs its own decision.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `documents`: the "Open and Save buttons" requirement now covers the New icon, Save's new place and the word in place of the floppy disk icon.
- `desktop-integration`: "Same icons on every distro" now says which icons LeafMD draws itself, including maximize and restore.

## Impact

- **Code:** the header bar section of `src/window.rs`, and the icon theme choice in `src/main.rs`.
- **Packaging:** a small new icon theme in `data/icon-theme/`, installed by the Flatpak manifest.
- **Icons:** adds `leafmd-new-symbolic.svg` and removes `leafmd-save-symbolic.svg`, in `data/resources/icons` and `data/resources/resources.gresource.xml`.
- **Docs:** Help if it describes these buttons, README if it does, and the icon list in `docs/DEVELOPMENT.md`. The Keyboard Shortcuts window doesn't change.
- **Screenshots:** the header bar appears in them, so the owner needs to retake them, following `data/screenshots/README.md`.
- **Dependencies:** none.

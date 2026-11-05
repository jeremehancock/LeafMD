# Proposal

## Why

The `windows` spec and the app store listing both promise that LeafMD works down to 360 by 300 pixels, but on plain GNOME it stops at 566 to 606 pixels wide. So it can't be used as a narrow panel next to another window, and the promise is false. Measured limits: the find bar needs 606 pixels even while it's hidden, and the header bar needs 530.

## What Changes

- Windows narrower than about 640 pixels switch to a compact layout, and switch back as soon as they're wide enough again.
- In the compact layout, the header bar keeps Open and its recent files arrow, the document's title, the main menu and the window's own buttons. New, Save, Find and the view switcher move to a new bar along the bottom of the window. They work exactly as they do in the header bar.
- The find bar stops reserving a fixed width. In the compact layout its contents wrap onto an extra row, so the find box is wider than it is today, and nothing is cut off or squashed.
- With those two fixed, LeafMD keeps its existing promise: it shrinks to 360 by 300 pixels and no further.
- Windows 640 pixels or wider look exactly as they do today, so the screenshots don't change.

This adds no keyboard shortcut, menu item or setting. The switch is automatic.

## Out of Scope

- A phone version: touch controls, or builds for phones and other ARM devices.
- LeafMD's other windows (Help, Keyboard Shortcuts, Markdown Cheatsheet, About), which aren't part of the size promise.
- Changing how anything works, or adding or removing any command.
- A way to choose the compact layout on a wide window.
- Fitting everything at once at the very smallest size. In a 360 by 300 window with the reload banner, find and replace all open, the window can grow a little taller so that the text stays usable.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `windows`: "Smallest window size" becomes true and gains checks that say what fits, and a new rule describes when the compact layout applies and what moves.
- `documents`: "Open, New and Save buttons" allows for New and Save being in the bottom bar in a narrow window.
- `views`: "Switching views" allows for the view switcher being in the bottom bar in a narrow window.
- `find-replace`: "Opening the find bar" allows for the magnifying glass being in the bottom bar, and a new rule describes how the find bar fits a narrow window.

## Impact

- **Code:** `src/window.rs` (the header bar, the new bottom bar and the switch between layouts) and `src/find.rs` (how the find bar is laid out).
- **Dependencies:** none. libadwaita, already used, has what's needed.
- **App store listing:** the `display_length` of 360 in the metainfo file stays, and becomes accurate.
- **Docs:** Help and the README gain a short note about the compact layout. Keyboard Shortcuts doesn't change, because no shortcut does.
- **Screenshots:** no change, because they're taken at 1000 pixels wide. `build-aux/take-screenshots.sh` can confirm this.
- **Roadmap:** this isn't on `ROADMAP.md`, so there's nothing to tick.

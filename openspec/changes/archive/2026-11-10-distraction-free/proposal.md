# Proposal

## Why

ROADMAP.md's "Reading and writing comfort" section promises a distraction-free mode: one switch that leaves just your document. With the visual polish making the document itself look good, it's the natural next step. The owner tried this in a scratch build and made the choices below along the way.

This change builds on `visual-polish` (it uses that change's Line Numbers setting and centred Raw column), so `visual-polish` is archived first.

## What Changes

- **Distraction Free, per window.** Ctrl+Shift+D, or Distraction Free in the main menu, hides the header bar, the tab bar, the formatting bar, the bottom bar in narrow windows, the side panel and the line numbers. Only the document is left. Other windows aren't affected.
- **The bars come back when wanted.** Moving the pointer to the window's top or bottom edge slides the bars in over the document. They slide away when the pointer moves back over the text, but stay while one of their menus is open. F10 brings them in and opens the main menu, and they slide away again when that menu closes.
- **Bars that only appear when needed stay.** The find bar (Ctrl+F) and the banner that says another app changed the file still show up in Distraction Free.
- **Line numbers in Distraction Free.** They're hidden when it's turned on. Choosing Line Numbers in the menu there shows them, and also turns on the normal setting. The Line Numbers tick now shows whether *this window* is showing line numbers.
- **Escape leaves.** Escape leaves Distraction Free and Full Screen together, once nothing else wants it: the find bar closes first, and a side panel sliding over the document tucks away first.
- **Always starts off.** Every window opens with Distraction Free off, even if it was on when LeafMD was closed. This differs from ROADMAP.md, which said "remembered": a window opening with its bars missing would look broken.
- **Full Screen (F11).** A separate item, Full Screen in the main menu or F11, fills the screen. The owner chose to keep it separate from Distraction Free, so a distraction-free window can sit beside other windows. Using both gives just the text on the whole screen.
- **Leaving puts everything back,** including the side panel if it was open.

New things added to the app:
- Keyboard shortcuts: **Ctrl+Shift+D** (Distraction Free), **F11** (Full Screen). Escape gains a new use, and F10 works in Distraction Free.
- Main menu items: **Distraction Free** and **Full Screen** (checkboxes, in the section with Side Panel and Line Numbers)
- No new settings. Nothing about Distraction Free is remembered.

### Out of scope

- Remembering Distraction Free between runs, or starting in it
- Turning on Full Screen with Distraction Free, or the other way round
- Hiding GNOME's own top bar without Full Screen
- Dimming everything but the current paragraph, typewriter scrolling, or other "focus" effects some writing apps have
- A setting for which parts are hidden

## Capabilities

### New Capabilities
- `distraction-free`: hiding everything but the document in one window, bringing the bars back with the pointer or F10, what still appears there, line numbers there, and leaving it.

### Modified Capabilities
- `windows`: windows gain Full Screen (F11, and leaving it with Escape).
- `help-and-about`: the main menu gains Distraction Free and Full Screen, and Line Numbers' tick follows the window. The Help window describes Distraction Free and Full Screen.

## Impact

- **Code:** `src/window.rs` (the window's bars split into two layers, one hidden by Distraction Free and one always kept; the actions, menu items, Escape and F10 handling, the pointer watching, line numbers per window; Help and Keyboard Shortcuts), `src/main.rs` (shortcuts, and the Line Numbers setting changing through the window's menu item), `src/document.rs` (line numbers settable per document).
- **Permissions and dependencies:** none new.
- **Docs and listing:** Help, Keyboard Shortcuts, README, metainfo description, leafmd.app, ROADMAP.md (tick the item and correct "remembered").

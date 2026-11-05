# Design

## Context

See proposal.md for why. The header bar is built near the top of `Window::new` in `src/window.rs`. `pack_start` adds buttons from the left edge inwards, and `pack_end` adds them from the right edge inwards (the menu, then Find, then the view switcher). The `windows` spec says windows work at 360 pixels wide, but measuring showed the committed code already stops at about 566.

## Decisions

### Save goes after the view switcher in `pack_end`

Because `pack_end` works inwards from the right, adding Save straight after the view switcher places it just to the left of the switcher, with no other layout work. Both New and Save use the existing `win.new` and `win.save` actions, so they get the unsaved-changes check and greying-out on the start page with no new code.

### Draw LeafMD's own new document icon

The icon is a page with a plus sign, drawn with filled shapes only, because GTK doesn't paint outlined lines in these icons. *Alternative considered:* Adwaita's `document-new-symbolic`. It needs no new file, but the Save icon had to be replaced precisely because Adwaita's version looked wrong, and a future runtime could change or remove an icon we don't own.

### Remove the floppy disk icon

Nothing else uses it once Save is a word. Keeping it would leave a dead file in the app. It can be restored from Git history if that ever changes.

### Replace maximize and restore with a small icon theme

GTK draws the window's buttons itself and asks for its icons by name, so LeafMD can't simply set a different icon on them. Bundling icons with the same names in `data/resources/` doesn't work: testing showed GTK still picks Adwaita's, because bundled icons only count as a last resort. So LeafMD gets a tiny theme named "LeafMD" that inherits from Adwaita. Icons drawn in it win, and everything else falls through to Adwaita. *Alternatives considered:* covering GTK's icon with a CSS background image, which depends on how Adwaita styles these buttons and could break with any runtime update; and copying Papirus's files, which would bring in its license. The icons are drawn fresh.

The theme is installed in the Flatpak. Running with `cargo run` doesn't install it, so GTK falls back to Adwaita there.

The owner then asked for LeafMD's other everyday icons to be drawn too, so they live in the same theme under their existing names. That means no code changes where they're used, and GTK's own uses of those names (like the window's close button) pick them up as well. The small icons GTK adds by itself stay with Adwaita, because there are many of them and they rarely stand out.

### Don't hide anything in narrow windows

The original plan hid New in narrow windows, to keep the promise in the `windows` spec that windows work at 360 pixels. Measuring showed that can't work: the committed code already stops at about 566 pixels, and the new header needs about 490 pixels even without New (close button only). So hiding New wouldn't reach 360.

The owner chose to always show both buttons. GTK keeps the window at least as wide as its contents, so nothing gets cut off. `adw::Breakpoint`, libadwaita's way of changing layout below a chosen width, would have turned that protection off.

## Risks / Trade-offs

- [The smallest window grows, by about 40 pixels with minimize, maximize and close buttons (from about 566 to about 604)] → Accepted by the owner. These are measurements in a test session using the system's fonts, so the installed Flatpak may differ slightly.
- [The `windows` spec still promises 360 pixels, which isn't true] → It wasn't true before this change either. Left for a separate decision.
- [The document title gets less room, since the header bar has one more button and a wider Save] → The title already shortens itself with "…" when space is tight, and its full text is available in the window's name.
- [People used to the Save icon on the left will need to find it in its new place] → It's the owner's choice, the word makes it easy to spot, and Ctrl+S doesn't change.
- [Existing screenshots go out of date] → A task reminds the owner to retake them before release, because only the owner can.

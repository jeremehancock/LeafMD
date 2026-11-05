# Design

## Context

See proposal.md for why. All measurements below are on plain GNOME, from a throwaway copy of LeafMD that printed how much width each part of the window asks for.

| Part | Narrowest width it allows |
|---|---|
| Find bar, in Raw and Split view (hidden or not) | 606 |
| Find bar, in Formatted view or with no document (no replace button) | 566 |
| Header bar | 530 (left buttons 128, title 74, right buttons 316) |
| Reload banner | 156 |
| "No Document Open" page | 171 |
| Formatting bar and document area | under 60 (they scroll) |

GTK adds up what the contents need and won't let a window get smaller than that, so `width_request(360)` in `src/window.rs` has no effect today. A hidden find bar counts because GTK's slide-down bar still measures what's inside it, whether or not it's showing.

## Goals / Non-Goals

**Goals:**

- Reach 360 by 300 in every view, including on desktops that add minimize and maximize buttons.
- Keep windows 640 pixels or wider pixel-for-pixel the same as today.
- Keep each button as one thing, so moving it can't lose its state.

**Non-Goals:**

- Fixing the size of LeafMD's other windows.
- A tidier layout for wide windows.

## Decisions

### Switch layouts with a libadwaita breakpoint

A breakpoint is libadwaita's built-in rule of the form "when the window is narrower than this, change these things". LeafMD already uses `adw::ApplicationWindow`, which supports them, and the runtime's libadwaita is new enough (LeafMD already needs 1.7 or later for `adw::ToggleGroup`).

Adding one has a side effect that's essential here: a window with a breakpoint no longer takes its minimum size from its contents, only from `width_request` and `height_request`, so the 360 by 300 that's already there starts to work. That's also the risk, covered below: if the wide layout needs more width than the breakpoint allows for, its contents get clipped rather than the window refusing to shrink.

Alternative: measuring the window's width and reacting to it by hand. It would mean rebuilding what libadwaita provides, and tends to flicker.

### Use 640 as the width, measured in text-scaled units

The proposal discussion said about 620, and the wide layout needs 606 on plain GNOME and about 614 on the owner's desktop, which has extra window buttons. 620 leaves almost no margin, so this uses 640.

The condition is written as `max-width: 639sp`, because `max-width` includes the number it names, and a window of exactly 640 stays wide. `sp` units grow with GNOME's Large Text setting, which also makes the wide layout's words wider, so the switch keeps happening before anything gets clipped. At normal text size, 640sp is 640 pixels.

Half of a 1366 pixel laptop screen (683) stays in the wide layout, and half of a 1280 pixel screen (640) does too, because the rule is "narrower than".

Alternative: a plain 640 pixels. It's simpler but would clip the wide layout with Large Text on.

### Move the real buttons, rather than having two copies

When the breakpoint applies, its `apply` signal takes New, Save, the magnifying glass and the view switcher out of the header bar and puts them in the bottom bar. Its `unapply` signal puts them back. They go back in their original order, so the header bar ends up the same as before: `pack_end` fills from the right, and the menu never moves, so re-adding the magnifying glass, switcher and Save in that order restores it. New was the last thing added on the left, so it returns to the same spot.

There's only ever one of each button, so its pressed state, greyed-out state, active view and every existing signal connection carry on with nothing extra to keep in step. That's what makes the "Nothing is lost while moved" scenario simple.

Testing found one thing this doesn't cover: moving the buttons makes the window forget which part has typing focus, so typing stopped dead after crossing the width. Both signals now note what has focus before moving anything, and give it back afterwards.

Alternative: two copies of each, showing one at a time. Save and New would be easy, since they follow an action. The switcher and the magnifying glass would each need code to keep both copies in step, which is more to get wrong. If moving widgets during the signal turns out to misbehave, this is the fallback.

### The bottom bar

A `gtk::ActionBar` (GTK's standard bar for this), added with `toolbar_view.add_bottom_bar`, and shown only by the breakpoint. New, Save and the magnifying glass go at the start, and the switcher at the end.

Widths at 360: New 40, Save 80, magnifying glass 40 and switcher about 116 come to about 276 plus spacing, so they fit. The header bar keeps Open about 88, title 74, menu 40 and close 40, about 254 plus spacing. With minimize and maximize added (about 80 more) that's about 334, which still fits.

### The find bar wraps instead of squeezing

ghostwriter, another markdown editor, lets its find bar get squeezed, and at 391 pixels its "Find:" label is cut to "Fin" with a very small box. The spec rules that out.

`src/find.rs` builds the find row with everything in one line. It becomes a box holding the find box and a second box with the count, arrows, replace button and options button. Breakpoint setters (small "set this property" rules with no code) then make these changes:

- `find_row` changes to vertical, which puts the second box on its own row. This changes nothing on a wide window, because one row in a box looks the same as before if the spacing is kept.
- The find box's `width-chars` drops from 26 to a small number and it expands to fill its row, so it ends up wider than today.
- The replace box's `width-chars` drops too, so it shrinks beside its two buttons.
- The `SizeGroup` that keeps the two text boxes the same width is switched off, because they no longer line up in the compact layout.
- The bar's own close button is hidden, and a second one beside the find box is shown. The bar's own button keeps a whole column to itself and sits centred across the rows, which in testing left the replace box too narrow to show "Replace with". There's no way to move it from outside the bar, so a plain button that closes the bar takes over.

The count's reserved width stays at 11 characters: it needs about 100 pixels, and with the arrows, replace button and options button comes to about 268, so it fits and "No matches" shows in full.

Replace stays one row, because a row with the box at about 110 and the buttons at about 80 and 105 fits in 360, and because every extra row takes height from the document.

### Height at the smallest size

With the find bar and replace row open at 360 by 300, measured, the window stays at 300 and a small strip of the document remains. That's tight but workable, and the window can simply be made taller. If the reload banner is showing as well, the contents may need more than 300 (not measured). The proposal lists this as out of scope.

## Risks / Trade-offs

- [The breakpoint clips the wide layout if it ever needs more than 640sp, say after adding another header button] → Note the limit in a code comment beside the breakpoint, and check the narrowest wide-layout window after any header change. A window 640 to 650 pixels wide is the place to look.
- [Moving widgets inside the signal could misbehave, for example by flickering, or by dropping keyboard focus from the editor] → Checked during implementation, including typing straight after crossing the width. The fallback is the two-copies approach above.
- [Dragging across 640 pixels moves buttons around under the mouse] → This is normal for GNOME apps that adapt to their width, and it only happens once per crossing. Nothing happens on a wide window.
- [A second layout to keep working] → Any new header button has to be added to both the header bar and the move logic. Say so in a comment there, and add the narrow window checks to the owner's test list when the header changes.
- [The split view is cramped at 360, with about 180 pixels per side] → Acceptable, since it still works and the user can switch to one view. The switcher is right there.
- [Screenshots could change by accident] → `build-aux/take-screenshots.sh` run into a temporary folder should give the same pictures as the ones in `data/screenshots/`.

## Migration Plan

None. No saved data changes, and a window that was saved at any width opens correctly. To undo this, revert the commit.

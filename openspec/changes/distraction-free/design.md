# Design

## Context

A LeafMD window is an `adw::OverlaySplitView` (side panel plus everything else) holding an `adw::ToolbarView`, libadwaita's container that stacks bars above and below a content area. All of the window's bars were top bars of that one ToolbarView: header bar, tab bar, file-changed banner, formatting bar and find bar, plus a bottom bar in narrow windows. The code was built and tried by the owner in a scratch build before this design, so the decisions below describe what's there. It depends on `visual-polish` for the Line Numbers setting.

## Goals / Non-Goals

**Goals:**
- Hide the window's controls without moving or recreating them, so their state (pressed buttons, the active view, greyed-out items) stays right.
- Keep the find bar and banner working exactly as now.

**Non-Goals:**
- Animations beyond libadwaita's own sliding.

## Decisions

**Two layers of bars.** The outer ToolbarView keeps the header bar, tab bar and formatting bar on top and the narrow window's bottom bar. Its content is a second ToolbarView holding the banner and find bar, with the document below. Distraction Free hides only the outer layer's bars, using ToolbarView's built-in `reveal-top-bars`/`reveal-bottom-bars` (they slide) and `extend-content-to-*-edge` (so the document fills the space and the bars come back *over* it, rather than pushing the text down each time). One visible side effect: the banner now sits under the formatting bar instead of above it. *Alternative:* hiding each bar's widget separately. Rejected: the tab bar hides and shows itself (with two or more tabs), and the formatting bar is hidden in Formatted view, so separate rules would fight.

**Two small style rules** (`load_window_style` in window.rs). The bars normally have no background of their own, which is fine above the document but see-through over it, so in Distraction Free they get GNOME's header bar colour and a soft shadow. And libadwaita leaves a 6 pixel gap under any top bar area, even an empty one, which would show as a strip between the bars and the document, so the inner layer resets it.

**Watching the pointer.** A motion controller on the outer ToolbarView, in the capture phase (so it hears movement before the formatted view's web page, which could keep it), reveals the bars within 6 pixels of the top or bottom edge, and hides them once the pointer is more than 24 pixels past them. They don't hide while the main menu is open or keyboard focus is inside any popover (that covers the Open and heading dropdowns too).

**F10.** F10 normally works by pressing the menu button, which can't be pressed while hidden. In Distraction Free, LeafMD's own key handler (the existing capture-phase one that also handles Escape) catches F10, reveals the bars, and opens the menu on the first frame after the bars have finished sliding in. While that's happening, pointer movement is ignored, because GTK reports the content moving under a still pointer as movement, which would otherwise hide the bars at once. The menu closing hides them again.

**Escape order.** The same key handler checks, in order: a side panel sliding over the document tucks away (existing rule), then if the find bar isn't open and the window is in Distraction Free or full screen, both are turned off. The find bar's own Escape and menus' Escape are handled before this handler sees the key (menus are separate surfaces, and the find bar check stops it), so they keep working.

**Line numbers belong to the window in the menu, and the setting stays shared.** The Line Numbers menu item becomes a window action whose tick shows what that window shows. The shared setting (from `visual-polish`) stays an app action, changed with `change_action_state`, which updates every window. A window in Distraction Free shows line numbers only if they've been asked for there (`distraction_free_numbers`, reset each time it's turned on). Choosing the item flips what the window shows and sets the shared setting to match. New tabs and tabs dragged in follow the window's rule.

**Not remembered.** Distraction Free is a plain field on the window, starting false, and never saved.

**Full screen** is GTK's own `fullscreened` property. The menu tick follows the window's notifications, so leaving full screen any way (Escape, F11, the desktop) keeps it right.

## Risks / Trade-offs

- [Someone presses Escape out of habit while writing and leaves Distraction Free] → Ctrl+Shift+D puts it back. The owner accepted this.
- [The pointer can't reach the top edge, for example on a second monitor above] → F10 and Ctrl+Shift+D still work, and Escape leaves.
- [libadwaita changes its top bar spacing or style names] → the 6 pixel reset would stop matching. It's isolated in one style rule, with a comment saying why.
- [The banner moves below the formatting bar] → a small visible change in every window, not just Distraction Free. It sits next to the document it's about, which reads naturally.

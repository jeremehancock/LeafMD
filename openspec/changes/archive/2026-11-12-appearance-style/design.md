# Design

## Context

LeafMD already asks libadwaita (GNOME's app toolkit) one question everywhere it needs to know whether to be light or dark: the style manager's "is it dark?" answer. The raw text colours and the formatted view (`Document::update_style`), the spelling underline colour (`raw_view.rs`) and every window's "redraw when it changes" hook (`window.rs`) all use that answer. Today the style manager simply follows the desktop.

The style manager can also be told to use light or dark whatever the desktop says. When told, its "is it dark?" answer changes and it announces the change, so everything above follows without being touched.

## Goals / Non-Goals

**Goals:**
- One switch, applied in one place, that every window and tab obeys.
- No change at all for people who never open the Style row.

**Non-Goals:**
- Changing how printing, PDF or HTML export choose their colours (see proposal.md, Out of scope).

## Decisions

**Tell the style manager, don't repaint by hand.** The choice sets libadwaita's colour scheme to "prefer light" (Follow System, libadwaita's own default), "force light" or "force dark". The alternative, passing LeafMD's own light-or-dark flag to each place that draws, would touch every one of them and easily miss the header bar and dialogs, which libadwaita draws itself.

Follow System uses "prefer light" rather than "default" because "default" means "whatever the app's settings say" and LeafMD has none. "Prefer light" is what libadwaita does by default today, so nothing changes for anyone who never picks.

**An app-wide action holds the choice.** A stateful action named `style` with the values `system`, `light` and `dark`, set up in `main.rs` beside `spell-check`. Its state is loaded from the settings file at start, applied to the style manager before the first window opens (so a dark choice doesn't flash light at start), and saved whenever it changes. The end-to-end tests can read and change app actions already (`app_action_state`, `activate_app_action`), so this makes the scenario easy, and leaves room for a menu item later without a second copy of the logic.

**A toggle group in a row for the choice.** The Style row uses libadwaita's toggle group (three buttons joined together, one selected), which LeafMD's libadwaita version (1.8) has. All three choices are visible at once, which suits three short words better than a drop-down. Considered: a drop-down row (one more click, hides the options), and the coloured circles GNOME Text Editor shows in its main menu (needs custom drawing, and the request was for Preferences).

**Saved in the existing preferences group** of LeafMD's settings file, as `style=system|light|dark`. Anything unreadable counts as `system`.

**Preferences gets taller** to fit the new group (its fixed content height is 320 now, chosen for two rows).

## Risks / Trade-offs

- [The desktop's own windows (file chooser, Print dialog) stay in the desktop's style, so a dark LeafMD can open a light file chooser.] → Expected and named in the proposal; it's how other GNOME apps behave too.
- [The formatted view is a web page; if it isn't ready yet when the style changes, it could miss the change.] → `update_style` already handles this for the system style change (it sets the theme when the page finishes loading), and a forced change arrives through the same announcement.
- [A dark choice could flash light while LeafMD starts.] → The scheme is set before the first window is built.

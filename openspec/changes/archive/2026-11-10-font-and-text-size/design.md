# Design: Font and text size, with zoom

## Context

See proposal.md for why. What the code does today:

- **Raw text:** `src/document.rs` turns on GtkSourceView's monospace setting, which uses GNOME's monospace font. There is no other font code.
- **Formatted view:** `src/preview.html` sets the page's font in its own styles: `16px/1.7 "Adwaita Sans", ...`. Headings, code and tables are sized in `em`, which means "relative to the text size", so they already grow and shrink with it. The text column is `44em` wide, so it widens with the text too.
- **Raw-only column:** `column_width()` in `src/document.rs` measures ten "0" characters in the editor's current font and works out an 80 character column from that. It already follows the font, as long as it's measured again after a change.
- **Saved settings:** `src/state.rs` keeps everything LeafMD remembers in `~/.config/leafmd/state.ini`, grouped into sections (`[window]`, `[editor]`, ...).
- **Styles:** `load_window_style()` in `src/window.rs` already adds one app-wide stylesheet (a set of CSS rules GTK applies to every window) for Distraction Free. The same approach works here.

## Goals / Non-Goals

**Goals:**
- One rule for "how big is the text": the Preferences font times the window's zoom, used for both views.
- No extra libraries, and no GTK functions that GTK has marked as on their way out (clippy would flag them).
- The end-to-end tests can read the zoom level the same way they read the view or side panel state.

**Non-Goals:**
- Zooming pictures in the formatted view. Zoom changes the text size, not the whole page like a web browser's zoom. Pictures stay their size, which keeps the document's layout steady and works the same in both views.

## Decisions

### 1. Zoom changes the font size, not the page

Zoom is applied by multiplying the Preferences size: at 120% with a 12 point font, both views simply use 14.4 points. This is the same thing that happens when the user picks a bigger font in Preferences.

- *Why:* there's then only one way text size changes, the raw-only column re-measures itself for free, and headings and code in the formatted view keep their proportions because they're in `em`.
- *Alternative:* WebKit's own page zoom for the formatted view. It would also enlarge pictures and the page's padding, and the raw text would still need the font-size approach, so the two views would behave differently at the same zoom. Rejected.

### 2. Raw text font through one app-wide stylesheet

LeafMD keeps one extra stylesheet for the whole app, rebuilt whenever a font preference changes (or GNOME's monospace font changes). For every zoom step it holds a rule like:

```css
.leafmd-zoom-120 .leafmd-raw { font-family: "Fira Code"; font-size: 14.4pt; }
```

Each raw editor gets the `leafmd-raw` style name, and each window gets `leafmd-zoom-<percent>` for its current zoom. Zooming swaps the window's style name. That's 26 small rules (50% to 300% in steps of 10).

- *Why:* GTK's way of giving one widget its own stylesheet is marked as deprecated (being removed in a later GTK), so clippy with `-D warnings` would reject it. A single shared stylesheet with names per zoom step does the same job with supported calls, and it's how the Distraction Free styles already work.
- *Alternative:* a text style ("tag") over the whole raw text. It has to be reapplied after every edit and undo, and it doesn't change the line numbers. Rejected.

### 3. Following GNOME's monospace font

While the raw text font hasn't been changed, LeafMD reads GNOME's monospace font from libadwaita's style manager (`monospace-font-name`, available in the libadwaita LeafMD already uses) and listens for it changing, rebuilding the stylesheet when it does. Inside the Flatpak this setting reaches LeafMD through the settings portal, the same way light and dark mode do.

### 4. Formatted view font through WebKit's font settings

`preview.html` changes its `body` font to `1em/1.7 sans-serif`, and LeafMD sets each formatted view's WebKit settings: the sans-serif font family to the chosen family, and the default font size to the chosen size times zoom (converted from points to pixels with WebKit's own helper, so 12 points stays 16 pixels). Code blocks keep their own `font-family` line in the page, so they stay monospace. WebKit's settings have no weight or slant, so a chosen style such as Italic or Bold goes to the page as two CSS variables (`--leafmd-weight`, `--leafmd-style`) that the page's text uses, set through LeafMD's private script area once the page has loaded. Code is kept upright.

- *Why:* no script has to run in the page, and a font set this way is in place before the page first draws, so there's no flash of the old size when a tab opens.
- *Alternative:* sending the font to the page's helper script (`preview.js`) as a CSS variable. It works, but the first frame shows the default size until the script runs. Rejected.

### 5. Where preferences are saved

A new `[preferences]` section in `state.ini`, with `raw-font` and `formatted-font` holding a font description string such as `Fira Code 12` or `Hack Italic 11`. Only the family, weight, slant and size are kept; rarer details like a condensed width are dropped, so what's saved is what both views can show. A missing key means "the starting font". Reset removes the key, so the raw text goes back to following GNOME. Zoom is never written.

### 6. Applying a change everywhere

When the user picks a font, LeafMD saves it, rebuilds the raw text stylesheet, and walks every open window and each of its documents to update the formatted view's WebKit settings and re-measure the raw-only column. Nothing is cached per document, so new tabs pick up the current fonts when they're created.

### 7. Zoom as a window action

Each window gets a `zoom` action. Its state is the percentage as text (`"100"`, `"110"`, ...), so the end-to-end tests read it with the existing `action_state` helper. Activating it with `in`, `out` or `reset` changes the state, swaps the window's `leafmd-zoom-<percent>` style name and updates each document's formatted view.

- Keys (set in `src/main.rs`): `<Control>plus`, `<Control>equal`, `<Control>KP_Add` for in; `<Control>minus`, `<Control>KP_Subtract` for out; `<Control>0`, `<Control>KP_0` for reset; `<Control>comma` for Preferences.
- Ctrl with the mouse wheel: a scroll watcher on each document's raw editor and formatted view, listening before the views do, that zooms when Ctrl is held and lets plain scrolling through.
- A document joining a window (new tab, opened file or a dragged tab) takes that window's zoom when it's attached.

### 8. The zoom row in the menu

The main menu (a `GMenu`) gets a first section holding one custom item, filled in with a small box: a Zoom Out button, a flat button showing the percentage (which runs `zoom` with `reset`), and a Zoom In button. GTK's popover menus support placing a widget like this in a menu. The buttons use the action's `out` and `in` targets, and are greyed out at 50% and 300% by turning the window's matching actions off.

### 9. The Preferences window

A new `src/preferences.rs` builds an `AdwPreferencesDialog` (libadwaita's standard settings window) with two groups. Each has a "Font" row with GTK's font button (which opens GNOME's font picker) and a flat Reset button beside it, shown only when that font is saved in `state.ini`. Only one Preferences window opens at a time.

## Risks / Trade-offs

- **[Risk] WebKit may not redraw a page that's already shown when its font settings change.** → Mitigation: after changing them, call `render_now()` for that document, which replaces the page's content and forces a fresh layout. Check this early in implementation.
- **[Risk] A very large font or zoom makes the 80 character column wider than the window.** → It already falls back to filling the window ("A narrow window" in the views spec).
- **[Trade-off] The raw text can be set to a font that isn't monospace.** GNOME Text Editor allows this too, and some people like it. Tables in the raw text won't line up, but nothing breaks.
- **[Risk] Zoom keys clash with something in the views.** GtkSourceView and WebKit have no built-in Ctrl+plus, minus or 0, and the app's shortcuts are checked before the views see the keys, as Ctrl+B already shows.
- **[Trade-off] Fonts and zoom can't be tested end to end** beyond the zoom level itself, because the tests can't see how text looks and the font picker is a dialog. The font picker, Reset, live updates and following GNOME's monospace font stay hand checks.

## Migration Plan

Nothing to move. Old `state.ini` files have no `[preferences]` section, so they get the starting fonts, which look the same as today.

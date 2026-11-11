# Proposal: Font and text size, with zoom

## Why

LeafMD's text is fixed today: the raw text uses GNOME's monospace font and the formatted view uses Adwaita Sans at 16 pixels. People who find that too small, too large, or just prefer another font have no way to change it. This adds two controls: a lasting choice of font and size in a small Preferences window, and quick zoom keys for a moment's change.

These are planned as one release because both change the same text size. Zoom works on top of whatever Preferences says, so Ctrl+0 ("back to normal") means "back to your Preferences size", and building them together avoids reworking one for the other.

## What Changes

- **New Preferences window**, opened with **Preferences** in the main menu or **Ctrl+,** (comma, GNOME's usual key for this). It has two groups:
  - **Raw Text:** a font picker (any installed font, any size). It starts on GNOME's monospace font, and while it's left alone it keeps following that setting in GNOME Settings.
  - **Formatted View:** a font picker. It starts on Adwaita Sans at the size LeafMD uses today.
  - Each picker has a **Reset** button, shown only when its font has been changed, that puts it back to the starting font.
  - Changes show straight away in every open window and tab, with no Apply button, and are remembered after LeafMD restarts.
- **Zoom**, for a quick change without opening Preferences:
  - **Ctrl+plus** (or Ctrl+=) makes the text bigger, **Ctrl+minus** smaller, **Ctrl+0** back to normal. Holding Ctrl and turning the mouse wheel zooms too.
  - It changes both the raw text and the formatted view, in every tab of that window. Other windows stay as they are.
  - A new window starts at normal size. Zoom isn't remembered: starting LeafMD again shows your Preferences size.
  - **New zoom row at the top of the main menu**, like GNOME Text Editor's: a minus button, the current size as a percentage (clicking it resets to 100%), and a plus button. It shows how far you've zoomed and makes zoom easy to find.
- **New keyboard shortcuts:** Ctrl+, (Preferences), Ctrl+plus and Ctrl+= (zoom in), Ctrl+minus (zoom out), Ctrl+0 (normal size).
- **New menu items:** the zoom row and Preferences in the main menu.
- **New settings:** the raw text font and the formatted view font.
- **Wording change:** the About window, the app store description and leafmd.app currently say there are "no settings to tune". That changes to say LeafMD is great out of the box and keeps its few options out of the way: "It is ready to use the moment you open it, and the few options it has stay out of your way, so you can focus on reading and writing." leafmd.app gets a shorter line: "Great out of the box. The few options stay out of the way, so you can focus on reading and writing."

## Out of scope

- Other settings in Preferences (themes, colours, line spacing, column width, tab width). Preferences only holds the two fonts.
- A separate font for code blocks in the formatted view. They keep Adwaita Mono, sized to match the formatted view's text.
- Remembering zoom, or zooming each tab separately.
- Changing the size of LeafMD's own buttons, menus, tabs or side panel. Only the document text changes.
- Changing the formatted view's colours or page layout, apart from the text column growing with bigger text.

## Capabilities

### New Capabilities
- `text-size`: the Preferences window with the raw text and formatted view fonts, how they're remembered and reset, and zoom (keys, mouse wheel, the menu row, which tabs and windows it applies to, and that it's not remembered).

### Modified Capabilities
- `views`: "The raw text editor" now uses the chosen raw text font (still monospace by default), and "A readable column in Raw view" sizes its 80 character column from that font and the zoom.
- `help-and-about`: the main menu gains the zoom row and Preferences; Help explains fonts and zoom; Keyboard Shortcuts lists the new keys. (The About text's wording also changes, but the spec doesn't quote it.)
- `end-to-end-tests`: two new scenarios (zoom, and zoom not being remembered), so the minimum count goes from 39 to 41.

## Impact

- `src/window.rs`: the main menu's zoom row and Preferences item, the zoom action per window, the Preferences window, Help, Keyboard Shortcuts and the About text.
- `src/document.rs`: applying a font and zoom to one document's raw text and formatted view.
- `src/preview.html`: the formatted view's font comes from LeafMD instead of being fixed in the page's styles.
- `src/state.rs`: remembering the two fonts in `~/.config/leafmd/state.ini`.
- `src/main.rs`: the new shortcuts.
- A new `src/preferences.rs` for the Preferences window, listed in `docs/DEVELOPMENT.md`.
- README, the metainfo description, `site/index.html`, ROADMAP.md, and two new files in `build-aux/end-to-end/`.
- No new libraries: GTK already has a font picker, and the formatted view (WebKitGTK) has zoom built in.

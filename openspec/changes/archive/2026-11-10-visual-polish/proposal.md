# Proposal

## Why

LeafMD's features are in a good place, and the owner wants it to feel beautiful without looking any less like a GNOME app. Right now the window around the document already looks like GNOME, but the document itself doesn't. The formatted view is styled like GitHub (always-blue links, faded browser checkboxes, spreadsheet-style tables, underlined headings), and its colours and code font don't quite match the raw text beside it. The raw text is also cramped: it starts 8 pixels from the line numbers, and the grey current-line band is one of the most noticeable things on screen.

The owner approved a before and after mockup of the formatted view, then tried this whole change in a scratch build before this proposal was written.

## What Changes

- **The formatted view follows the desktop's accent colour.** Links, ticked checkboxes and the bar beside quotes use the accent colour chosen in GNOME Settings, and change straight away when it changes. Links use GNOME's darker or lighter version of the accent so they stay readable in both light and dark styles.
- **A GNOME look for the formatted view.** Checkboxes are drawn like GNOME's own (a rounded outline, or filled with the accent and a white tick). Tables are a rounded card with a tinted header row and thin lines between rows, and numbers line up in columns. Header cells line up with their column instead of being centred. Headings lose their underlines and become heavier and slightly tighter, with more space above each section. Code blocks match the table card. Quotes get a softer, thinner bar. Selected text is tinted with the accent.
- **Matching fonts and colours.** Text uses GNOME's own fonts (Adwaita Sans, and Adwaita Mono for code, the same font as the raw text). Backgrounds and text colours come from GNOME's current palette, so the two halves of Split view are exactly the same shade in dark mode (today they're slightly different).
- **A calmer raw text view.** More room around the text (18 pixels at the sides instead of 8), a little more space between lines, dimmer line numbers, and a much fainter current-line highlight. Syntax colours are unchanged (GNOME's Adwaita editor colours).
- **A centred column in Raw view.** When only the raw text is showing, it stays in a centred column about 80 characters wide, like the formatted view already does. The scroll bar stays at the window's edge. In Split view it fills its half as before.
- **BREAKING (default): line numbers are off by default.** A new **Line Numbers** item in the main menu turns them on and off. It's one setting for every window, remembered between runs, and it shows a tick while line numbers are showing.

New things added to the app:
- Main menu item: **Line Numbers** (a checkbox, in the section with Side Panel)
- Setting: whether line numbers show (remembered, like the side panel being open)
- No new keyboard shortcuts

### Out of scope

- Distraction Free mode and Full Screen. They're a separate proposal, `distraction-free`, which builds on this one.
- Changing the header bar's buttons (moving New into a menu was discussed and not chosen)
- The "No Document Open" page
- Choosing fonts or text size (that's the roadmap's Preferences item)
- Syntax colours in the raw text, and colours for code inside code blocks in the formatted view
- A keyboard shortcut for Line Numbers

## Capabilities

### New Capabilities

None.

### Modified Capabilities
- `views`: the raw text no longer always has line numbers (off by default, with a menu toggle), sits in a centred column in Raw view, and uses quieter colours. "Follows the system style" grows to cover the accent colour.
- `formatted-view`: "Readable page styles" changes: GNOME fonts and palette, the accent colour, GNOME-style checkboxes, tables, headings, code and quotes, and a column of 44em instead of 46em.
- `help-and-about`: the main menu gains Line Numbers.

## Impact

- **Code:** `src/preview.html` (the formatted view's stylesheet), `src/document.rs` (the raw text's spacing, colours, centred column and line numbers; sending the accent colour to the formatted view), `src/window.rs` (the menu item, following accent colour changes), `src/main.rs` (the shared Line Numbers setting; telling the editor where LeafMD's colour files are), `src/state.rs` (remembering the setting in state.ini).
- **New files:** `data/resources/styles/leafmd.xml` and `leafmd-dark.xml`, the raw text's colours (GNOME's Adwaita editor colours with quieter line numbers and current line), listed in `data/resources/resources.gresource.xml`.
- **Permissions and dependencies:** none new.
- **Docs and listing:** Help, README, metainfo description, leafmd.app, screenshots (every view looks different), docs/DEVELOPMENT.md (the new styles folder), ROADMAP.md.

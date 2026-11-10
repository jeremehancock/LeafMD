# Design

## Context

The formatted view is a small web page (`src/preview.html`) with hand-written colours. LeafMD tells it light or dark by setting `data-theme` on the page (in `Document::update_style`). The raw text is GNOME's text editor widget (GtkSourceView), coloured by a "style scheme", a small XML file of colours. LeafMD used GNOME's own Adwaita schemes. All of this was built and tried by the owner in a scratch build before this design was written, so the decisions below describe what's there.

## Goals / Non-Goals

**Goals:**
- Take every colour and font from GNOME, so LeafMD keeps matching GNOME as it changes, instead of copying values that drift.
- Keep the formatted view's page free of scripts from the document (nothing here adds any).

**Non-Goals:**
- Any change to how markdown becomes HTML (`src/render.rs` is untouched).

## Decisions

**The accent colour is sent to the page, not read by it.** The page can't see GNOME's settings. LeafMD reads the accent from libadwaita (GNOME's widget library) and sets two CSS variables on the page, `--accent` (for fills, like a ticked box) and `--link` (libadwaita's "standalone" version of the accent, which is adjusted to stay readable as text on the current background). It does this in the same place it already sets light or dark, and the window listens for accent changes the way it listens for dark mode. The page starts with GNOME blue in case anything goes wrong. *Alternative:* a fixed link colour per accent in the stylesheet. Rejected: it would duplicate GNOME's table of accents and its readability adjustments.

**The raw text's colours are GNOME's, with three overrides.** Two small scheme files, `leafmd` and `leafmd-dark`, name GNOME's Adwaita schemes as their parent and only change the line numbers, the current line's number and the current line's band. They're built into the program (in the resource bundle with the icons), and GtkSourceView is told to look there at startup. If they ever fail to load, LeafMD falls back to GNOME's own schemes. *Alternative:* restyling with GTK CSS. Rejected: GtkSourceView takes these colours from its scheme, not CSS.

**Backgrounds match by using the same numbers GNOME's editor scheme uses.** The formatted view's page and the web view behind it use `#ffffff` and `#1d1d20`, the values in GNOME's Adwaita and Adwaita-dark editor schemes, which in turn match libadwaita's own view background. Previously the dark page used `#1e1e1e`.

**The Raw view column uses libadwaita's ClampScrollable.** This is a wrapper that keeps a scrolling widget's content to a maximum width while letting the scroll bar sit at the window's edge, and that moves the line numbers along with the text. The wrapper gets GNOME's "view" style, so the space either side is the same colour as the text. Its maximum is set when the view changes: about 80 characters of the editor's font plus margins (plus room for line numbers when they're on) in Raw view, and effectively unlimited in Split view. *Alternative:* growing the editor's own left and right margins with the window. Rejected: the line numbers would stay at the window's edge, far from the text.

**Line Numbers is an application-wide setting.** It's stored in state.ini under `[editor]`, like the side panel's open state, and held in an app-level action so every window's menu item shows the same tick. Each new document reads it when it's created. Turning it on or off updates every document in every window.

**Line spacing.** Three pixels below each line and between the rows of a wrapped line, so wrapped paragraphs and separate lines are spaced the same.

## Risks / Trade-offs

- [Adwaita Sans or Adwaita Mono not installed (older GNOME, or another desktop)] → the page lists fallbacks (the system font, Source Code Pro, DejaVu Sans Mono), so it looks like before rather than broken.
- [People who relied on line numbers being on] → it's one menu item to bring them back, and it's remembered. The README and Help say where it is.
- [A future GNOME changes its background colours] → the two hex values in `update_style` and the page would need updating to match. They're next to each other and commented.
- [The checkbox is drawn by CSS (the browser's own drawing is switched off)] → it's still a real, disabled checkbox underneath, so it stays impossible to click, as the spec requires.

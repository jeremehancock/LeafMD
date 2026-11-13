# Design

## Context

See proposal.md for why. The column width is set in four separate places today, and each needs the setting:

- **Raw text:** `Document::column_width` in `src/document.rs` measures ten characters ("0000000000") in the raw text font at the document's zoom and multiplies by 8, giving 80 characters, plus the 18px side gaps and room for line numbers. A clamp around the editor (a container that stops its child getting wider than a set size and centres it) uses that width. `apply_view` only turns the clamp on in Raw view. In Split view it sets it to "unlimited", which is why the raw half stretches when the divider is dragged.
- **Formatted view:** `src/preview.css` has `#content { max-width: 44em }`. An "em" is the width of the letter M at the current font size, so the column already grows with the font and with zoom.
- **Exported web page:** `src/export.rs` builds the page from the same `preview.css` plus a little CSS for the font (`Look::css`), so it also gets 44em.
- **Paper:** `@media print` in `preview.css` turns the column limit off, and the page margins decide the line length. Export as PDF sets 20mm on a `gtk::PageSetup`. Print hands WebKit's `PrintOperation` no page setup, so the Print dialog's margins are used.

The Style setting (`state::load_style` and `save_style`, the app-wide stateful `style` action in `src/main.rs`, and the toggle row in `src/preferences.rs`) is a complete example of a remembered three-way choice to copy.

## Goals / Non-Goals

**Goals:**
- One remembered choice, read in one place, giving every number (raw characters, formatted column, PDF margin).
- A change shows straight away in every open document, with no reload of the page and no lost scroll position.
- Wide reproduces today's look exactly, so it's an honest "old look" option.

**Non-Goals:**
- No new way of measuring the column. Raw keeps counting characters in its font, and formatted keeps using em.

## Decisions

**1. One `Margins` type in `src/state.rs` holds every number.** It's an enum with None, Narrow, Normal and Wide, saved as `margins=none|narrow|normal|wide` in the settings file's `preferences` group (missing or unknown means Normal). It has small methods for each place that needs a number:

| | Raw characters | Formatted column | PDF |
|---|---|---|---|
| None | no limit | no limit | 5mm |
| Narrow | 140 | 72em | 10mm |
| Normal | 100 | 52em | 15mm |
| Wide | 80 | 44em | 20mm |

The enum's variant for None is called `FullWidth` in the code, so it can't be mistaken for Rust's own `None`; it's saved and shown as "none" and "None". The em values are chosen so the formatted view holds about the same number of characters as the raw text (44em gives about 85 characters in Adwaita Sans today, so about 1.93 characters per em). *Alternative:* separate constants in each file. Rejected, because the numbers would drift apart.

**2. An app-wide stateful `margins` action, like `style`.** Preferences changes the action. The action saves the choice and tells every window to tell every document. This also makes the end-to-end test simple, since the existing `set_app_action` and `expect_app_action_state` helpers work on it unchanged. *Alternative:* a GSettings key. Rejected, because LeafMD keeps all its settings in its own key file.

**3. The Margins row sits in the Appearance group, under Style, as the same kind of toggle row** (four labelled buttons side by side, via `adw::ToggleGroup`). It looks and works like the row above it. The two font groups become one Fonts group with a Raw Text row and a Formatted View row (the owner's request), so Preferences has two groups of two rows. The dialog's `content_height` fits that.

**4. Raw text: `column_width` multiplies by the chosen characters ÷ 10 instead of 8 (or gives the existing "unlimited" width for None), and the clamp is used in Split view too.** `apply_view` and `apply_fonts` stop treating Split as "unlimited", so they treat Raw and Split the same. A new `Document::apply_margins` re-measures and sets the clamp. The clamp already fills the space when the window or half is narrower, so nothing extra is needed for the narrow-window case. *Alternative:* keep Split unlimited. Rejected by the owner, since dragging the divider shows the difference.

**5. Formatted view: a CSS variable, set the same way as the font weight.** `preview.css` becomes `#content { max-width: var(--leafmd-column, 52em) }`. `Document` sets `--leafmd-column` (an em width, or `none` for None) with `run_js` once the page has loaded (beside `--leafmd-weight` in `apply_fonts`) and again whenever the setting changes. Changing a variable re-flows the page in place, so the scroll position and Split view's scroll matching keep working. *Alternative:* reload the page. Rejected, because it flashes and loses the scroll position.

**6. Exported web page: `Look` carries the column width**, and `Look::css` adds `#content { max-width: <em> }`. A `max-width` only caps the column, so in a browser window narrower than that the text still fills the width (inside `body`'s side padding) with no sideways scrolling. The print block's `max-width: none` stays, so paper is decided by the page margins alone.

**7. PDF: one helper builds the `gtk::PageSetup`.** It uses the locale's usual paper (as PDF does today) and the chosen margin on all four sides.

**8. Print is left alone.** The first build also handed Print our page setup, and the owner found that choosing a paper in the Print dialog replaces its margins (the dialog rebuilds the page setup from the paper's defaults). The planned fallback, `@page { margin }` in the print CSS, was tested with WebKit 6 on this computer: WebKit ignores it and uses only the page setup's margins. The remaining fix, having the dialog only choose the printer and paper and then making the pages ourselves (GTK's `PrintDialog` with `print_file`), would lose the dialog's Preview, and isn't what markdown apps usually do. The owner chose to leave Print as it was.

## Risks / Trade-offs

- [People may expect Margins to change printouts too.] → Help and the README say it covers the screen, PDFs and web pages, and that a PDF can be printed for a printout with the same margins.
- [Everyone's default look changes, and the screenshots change with it.] → The owner agreed. Wide gives back today's look. Retake the screenshots with `build-aux/take-screenshots.sh` and check them as `data/screenshots/README.md` describes.
- [On a laptop screen the column is often wider than the window, so None, Narrow and Normal can look the same there.] → Expected. The setting only matters when there's spare room, and the spec says the text fills a narrower window.
- [Raw text with hard line breaks at about 80 characters looks no different at Normal or Narrow, just with more space to the right of each line.] → Expected. Wide is today's look for anyone who prefers it.
- [The formatted view's character count per em depends on the font. A user's own wide or narrow font gives more or fewer characters than the table says.] → Acceptable. The specs say "about", and the same is true of today's 44em.

## Migration Plan

Nothing to migrate. Existing settings files have no `margins` key, so they start at Normal. Going back to an older LeafMD ignores the unknown key.

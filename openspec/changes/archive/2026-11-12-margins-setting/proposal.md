# Proposal

## Why

LeafMD keeps text in a fairly slim centred column (about 80 characters in the raw text, about 85 in the formatted view), with the rest of a wide window left as margin. That suits careful reading, but on a big screen many people would rather see more text per line, and there's no way to choose. PDFs have the same gap: they always get 20mm margins.

## What Changes

- A new **Margins** row in the Appearance group of Preferences, under Style, with four choices side by side: **None**, **Narrow**, **Normal** and **Wide**. Narrow, Normal and Wide are the names word processors use for page margins.
- The choice sets how wide a line of text can get, in every view, and how big the page margins are in a PDF:

  | Margins | Raw text line | Formatted view line | Export as PDF |
  |---|---|---|---|
  | None | fills the window | fills the window | 5mm |
  | Narrow | about 140 characters | about 140 characters | 10mm |
  | Normal | about 100 characters | about 100 characters | 15mm |
  | Wide | about 80 characters (today) | about 85 characters (today) | 20mm (today) |

- **None** keeps only the small gap that stops text touching the window's edge, like GNOME Text Editor, for people who want no empty space on a big screen.
- **Normal is the new starting choice.** This changes the default look for everyone: text gets about 20% wider per line on a wide window, and PDF margins go from 20mm to 15mm. Choosing Wide gives back exactly today's look. (The owner agreed to changing the default, since LeafMD is still early days.)
- **Split view follows the setting too.** Today the raw half of Split view fills its half however wide it gets, while the formatted half stops at its column. With this change both halves stop at the chosen column, which matters when the divider is dragged to make one side wide. When a half (or the window) is narrower than the column, the text fills it, as today.
- **Export as HTML** keeps the chosen line width in the exported web page, so it matches what was on screen.
- The choice shows straight away in every tab of every window, and is remembered.
- The column still grows with zoom, so a line holds the same number of characters at any zoom.
- **Preferences is regrouped.** Appearance holds Style and Margins. The two font groups (Raw Text and Formatted View, one row each) become one **Fonts** group with a Raw Text row and a Formatted View row, each still with its font button and Reset.
- New setting: the Margins row. No new keyboard shortcut and no new menu item.

Out of scope:

- **Printing.** Print keeps using the margins the system's Print dialog gives it, as today. Choosing a paper in that dialog replaces any margins LeafMD asks for, and WebKit (which draws the pages) ignores margins set in the page's own style, so the only way round it would be for LeafMD to make the pages itself after the dialog closes, which loses the dialog's Preview. Markdown apps such as Obsidian and Typora likewise keep margin choices to PDF export. To print with the chosen margins, export a PDF and print that.
- No custom sizes or slider: four choices only.
- No separate settings for each view, for screen and PDF, or for each tab or document.
- No change to the space above and below the text, or to the small gap that stops text touching the column's edge.
- Narrow windows don't change: when the window is smaller than the column, the text fills it, as today.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `views`: "A readable column in Raw view" changes so the column width comes from the Margins setting (none, 140, 100 or 80 characters) and applies to the raw half of Split view too, instead of the raw half always filling its half. New requirements cover choosing the margins and remembering the choice.
- `formatted-view`: "Readable page styles" changes from a fixed 44em column to one whose width comes from the Margins setting.
- `text-size`: the "Preferences window" requirement changes to the new layout (Appearance with Style and Margins, then one Fonts group with Raw Text and Formatted View rows), and "Resetting a font" follows the renamed rows.
- `print-and-export`: PDFs use the page margins from the Margins setting, and the exported web page keeps the chosen line width. Printing doesn't change.

## Impact

- `src/preferences.rs`: the Margins row in the Appearance group, and one Fonts group in place of two.
- `src/main.rs`: an app-wide "margins" choice, like the existing "style" one, telling every open document when it changes.
- `src/state.rs`: remembering the choice in LeafMD's settings file.
- `src/document.rs`: the raw text's column width comes from the setting, and is used in Split view too; the formatted view is told its column width.
- `src/preview.css`: the column width becomes adjustable instead of a fixed 44em.
- `src/export.rs`: page margins for PDF, and the line width in the exported web page.
- An end-to-end scenario checking the choice is remembered.
- Help, README, the app store description, leafmd.app and ROADMAP.md mention it. The screenshots change, since the default line width does.

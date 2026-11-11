# Proposal

## Why

LeafMD shows a nicely formatted document, but there's no way to get it out of the app. To hand someone a copy on paper, as a PDF, or as a web page, people have to copy the text into another program and format it again. Sharing is the last unticked section of the roadmap, and its three parts belong together: all three start from the formatted document, and Print and Export as PDF share most of their work.

This release also fixes a small tab bar annoyance noticed while testing tabs: making a window narrower can leave the selected tab scrolled out of sight in the tab bar, until another tab is picked.

## What Changes

- **Print (Ctrl+P).** Opens the system's Print dialog for the selected document. What prints is the formatted document, whichever view is showing (Formatted, Raw or Split). Pages are always light (dark text on white paper), even when LeafMD is in dark style, and use the font and size from Preferences, not the window's zoom. Code blocks wrap their long lines instead of being cut off at the page edge, and headings, code blocks, table rows and images aren't split across pages where it can be avoided.
- **Export as PDF.** Asks where to save, suggesting the document's name with `.pdf` (so `notes.md` becomes `notes.pdf`, next to it), then saves the formatted document as a PDF that looks the same as printing it would. A short message confirms it was saved.
- **Export as HTML.** Asks where to save, suggesting `notes.html`, then saves a single web page that opens in any browser and looks like LeafMD's formatted view. Pictures stored on the computer are packed inside the file, so it still shows them after being emailed or moved. The page follows the reader's own light or dark setting in their browser, and contains no scripts.
- **What gets used** is the text as it is in the tab right now, including unsaved changes. An untitled document is offered as `Untitled.pdf` or `Untitled.html`.
- **The selected tab comes back into view** in the tab bar when a window is made narrower, so it's never left scrolled out of sight.

New things added to the app:
- One new keyboard shortcut: **Ctrl+P** for Print.
- Three new main menu items, in a new section after Save and Save As…: **Print…**, **Export as PDF…** and **Export as HTML…**. They're greyed out when no document is open.
- No new settings.

### Out of scope

- Page headers, footers and page numbers on printed pages and PDFs.
- Choosing what the PDF looks like separately from printing (paper size, margins, orientation). Export as PDF uses the usual paper size for the computer's language and region; Print uses whatever is picked in the Print dialog.
- Printing or exporting the raw markdown text.
- Exporting to other formats (Word, ODT, EPUB and so on).
- Packing online pictures into the HTML file. They stay as links to where they are online, so they need an internet connection to show.
- Turning links to other markdown files into links to their exported web pages. They're left as written.
- Exporting several documents, or a whole folder, at once.
- A keyboard shortcut for either export.
- Other roadmap items, such as Flathub and translations.

## Capabilities

### New Capabilities
- `print-and-export`: printing the formatted document, exporting it as a PDF, and exporting it as a self-contained HTML page: what they produce, how they look, the suggested file names, and the messages shown when they work or fail.

### Modified Capabilities
- `help-and-about`: the main menu gains a section with Print…, Export as PDF… and Export as HTML…; the Help window explains printing and exporting; Keyboard Shortcuts lists Ctrl+P under Files.
- `tabs`: "Tabs in a narrow window" gains a scenario: making a window narrower keeps the selected tab in view in the tab bar.
- `end-to-end-tests`: one new scenario (47 in all): Print and the exports are greyed out with no document open.

## Impact

- **Code:** `src/window.rs` (the menu section, the three actions, Help and Keyboard Shortcuts), `src/main.rs` (Ctrl+P), `src/render.rs` (building the export page), a new `src/export.rs` for printing and exporting, `src/preview.html` (styles for printed pages). The tab bar fix is in `src/window.rs`.
- **Tests:** unit tests (`cargo test`) for the exported HTML page: pictures packed in, no scripts, no line tags, light and dark styles, the page title. One new end-to-end scenario checks that Print and the exports are greyed out with no document. Printing and exporting themselves go through dialogs, which the end-to-end tests never open, and the tab bar fix is about what can be seen, which they can't check.
- **Dependencies and permissions:** none new. Printing goes through the desktop's print portal (the service that shows the Print dialog for Flatpak apps and talks to printers on their behalf), so LeafMD still needs no printer access of its own.
- **Docs and listing:** Help, Keyboard Shortcuts, README (features and shortcuts table), the app store description in the metainfo file, leafmd.app (`site/index.html`), and docs/DEVELOPMENT.md for the new `src/export.rs`. Screenshots don't change, since the window looks the same apart from the menu.
- **Roadmap:** ticks Print, Export to PDF and Export to HTML under "Sharing".

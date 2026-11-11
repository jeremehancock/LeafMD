# Proposal

## Why

In a long document there's no quick way to see how it's organised or to get to a particular section: you scroll, or use Find. Writers also have no way to see how long a document is getting. These are the last two items in "Reading and writing comfort" on the roadmap. They're planned as one release because both keep track of what's in the document as you type, and the side panel was built so a second page like Headings could be added easily.

## What Changes

- **A Headings page in the side panel.** It lists the selected document's headings in order, indented to match their level (a `##` heading sits one step in from a `#` heading). The text shows as it reads, without markdown marks, so `## **Install** it` shows as "Install it". It updates shortly after you stop typing, and follows the selected tab.
- **Clicking a heading jumps to it.** The document scrolls so the heading is at the top, in whichever view is showing (both sides in Split view). When the raw text is showing, the cursor moves to the start of the heading's line, so you can start typing there. In a narrow window, where the panel slides over the document, it tucks away after a heading is picked, the same as after picking a file.
- **The Headings page follows your place.** As you scroll, the heading of the section you're reading is highlighted, and the list scrolls to keep it in view.
- **A way to switch pages.** Now that the panel has two pages, the small page switcher it was built with appears along its bottom: Files and Headings, each with an icon. The page you were last on is remembered, in new windows and after restarting, the same way the panel's open or closed state is.
- **Word count in the bottom-right corner.** A small, faint label floats in the bottom-right corner of the document, saying for example "1,240 words · 7 min read". It counts the words a reader would see, so markdown marks like `**`, `#` and link addresses aren't counted. Reading time assumes 200 words a minute. It updates as you type and shows in all three views.
- **Counting a selection.** While text is selected, in the raw text or the formatted view, the label shows the selection too, for example "35 of 1,240 words". In Split view, the selection made most recently counts.
- **Distraction Free hides the word count**, along with the bars and the side panel, so only the document is left.
- **Escape steps back one mode at a time.** With Distraction Free and Full Screen both on, the first Escape leaves Full Screen and the second leaves Distraction Free. Until now one Escape left both. (Added after the owner tried the first build.)

New things added to the app:
- Side panel page: **Headings**, and the page switcher at the bottom of the panel (Files, Headings)
- The word count label in the document's bottom-right corner
- No new keyboard shortcuts, menu items or settings. Which page the panel shows is remembered, like whether it's open.

### Out of scope

- A keyboard shortcut for the Headings page (F9 opens the panel, and the switcher can be reached with Tab). One could be added later if it's missed.
- Collapsing and expanding sections in the Headings list, or dragging headings to reorder sections
- Headings written as HTML tags (`<h2>Install</h2>`) inside the markdown. Only markdown headings (`#` lines, and text underlined with `===` or `---`) are listed.
- Character, sentence or paragraph counts, a word goal, or clicking the label to change what it shows
- Counting words in languages written without spaces (such as Chinese or Japanese) one character at a time. They're counted by the same rule as other text, so a long run of characters counts as one word.
- Hiding the word count, or a setting for reading speed

## Capabilities

### New Capabilities
- `headings`: the Headings page of the side panel: what it lists and how, jumping to a heading, following your place as you scroll, and what it shows with no document or no headings.
- `word-count`: the word count label: where it shows, what counts as a word, reading time, counting a selection, and when it's hidden.

### Modified Capabilities
- `side-panel`: "Pages in the side panel" now has two pages, a switcher along the bottom, and remembers the last page; the narrow-window rule adds tucking away after picking a heading.
- `distraction-free`: "What Distraction Free hides" adds the word count, and "Leaving with Escape" now leaves Full Screen before Distraction Free.
- `help-and-about`: the Help window covers the Headings page and the word count.
- `end-to-end-tests`: three new scenarios (44 in all), and the coverage list mentions headings.

## Impact

- **Code:** `src/side_panel.rs` (a way to choose and report the showing page), `src/window.rs` (building the Headings page, the word count label, remembering the page, Help), `src/state.rs` (remembering the panel's page in state.ini), `src/document.rs` (telling the window when the text, selection or scroll position changes; jumping to a line), `src/preview.js` (a new message reporting the formatted view's selected text; the line at its top is already reported). New files: `src/headings.rs` (finding headings in the text, and the Headings page) and `src/word_count.rs` (counting words and building the label's text), both with unit tests.
- **Icons:** a new LeafMD icon for the Headings page in the switcher, in `data/resources/icons`.
- **Dependencies and permissions:** none new. Headings are found with comrak, the markdown library LeafMD already uses.
- **Escape:** the Escape handling in `src/window.rs`, Help's Distraction Free rows and the Keyboard Shortcuts row for Escape.
- **Docs and listing:** Help, README, metainfo description, leafmd.app, screenshots (the word count label shows in every window), docs/DEVELOPMENT.md file table, ROADMAP.md. Keyboard Shortcuts doesn't change.

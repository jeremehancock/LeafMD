# Design

## Context

See proposal.md for why. The pieces this builds on, as they are today:

- **The side panel** (`src/side_panel.rs`) already holds pages in a stack, with a page switcher along its bottom (`adw::ViewSwitcherBar`) that only appears once there are two pages. Files is added in `src/window.rs` with `side_panel.add_page(...)`. Whether the panel is open is saved in `state.ini` under `[side-panel]` (`src/state.rs`).
- **Each tab is a `Document`** (`src/document.rs`). Typing fires the text buffer's `changed` signal, which waits about 150 milliseconds (`schedule_render`) and then redraws the formatted view. That redraw is skipped while the formatted view is hidden, so in Raw view nothing currently runs after typing stops.
- **Scrolling is already measured in source lines.** `scroll_editor_to_line(line)` scrolls the raw text to a line, the formatted view's `scrollToLine(line)` (in `src/preview.js`) does the same there, and the formatted view posts a `scroll` message with `lineAtTop()` whenever it scrolls. Today that message is ignored outside Split view.
- **Markdown is parsed by comrak**, which can give every block, including headings, the line it starts on.
- **The window shows one document at a time** in a tab view, inside a toast overlay, inside the inner bars (find bar, file banner). The narrow window's bottom bar sits outside that.

## Goals / Non-Goals

**Goals:**
- One place works out a document's headings and word count, from the same text, after the same pause.
- Headings and word count are plain functions of the text, unit-tested without GTK.
- Jumping and highlighting reuse the existing "scroll to a source line" code instead of new scrolling code.

**Non-Goals:**
- Changing how synced scrolling in Split view works.
- Matching the formatted view's heading ids (used by `#links`). The Headings page goes by line numbers, which are simpler and always unique.

## Decisions

### 1. Two small modules, one pause

`src/headings.rs` turns markdown into a list of `Heading { level, text, line }`. `src/word_count.rs` turns markdown into a word count, and also builds the label's wording ("1,240 words · 7 min read", "35 of 1,240 words", "1 word"). Both parse with comrak, with the same extensions the formatted view uses (tables, strikethrough, task lists, footnotes), so they agree with what the reader sees.

`Document` gets a second job after the 150 millisecond pause: besides redrawing the formatted view (when it's visible), it always works out the headings and word count, keeps them on the document, and, if it's the selected tab, tells the window to refresh the Headings page and the label. Switching tabs refreshes both from the newly selected document's stored values, without parsing again.

*Alternative considered:* get the headings from the formatted view's HTML in JavaScript. Rejected because the formatted view isn't drawn in Raw view, and it would need a round trip through WebKit for every change.

*Alternative considered:* one parse shared by both modules. Parsing even a very long document takes a few milliseconds and only happens after typing pauses, so two simple functions are worth more than saving that. Can be merged later if it ever shows up.

### 2. What counts as a word

`word_count` walks comrak's parsed document and collects the text a reader sees: plain text, inline code and code blocks. It skips link addresses (the link's own text still counts), images entirely (address and description), raw HTML, list markers and checkboxes (which aren't text in the parsed document anyway). It then counts runs of letters and digits, where an apostrophe (`'` or `’`) or hyphen between two letters or digits stays inside the word. So "don't" and "well-known" are one word each, and a lone "-" is none.

Text from separate blocks is joined with a space, so the last word of one paragraph and the first of the next never merge.

**Selections** are counted by running the same function on the selected markdown. A selection that starts in the middle of `**bold**` may parse a little differently from the whole, but the count of real words comes out the same in nearly every case. The selection count is worked out when the selection changes (the buffer's `mark-set` signal for the cursor or selection end), but no more than once per frame, so dragging across a long document stays smooth.

### 2b. Selections in the formatted view

The formatted view's script (`src/preview.js`) listens for the page's `selectionchange` event and, at most once per frame, posts a `selection` message with the selected text (`window.getSelection().toString()`, empty when nothing is selected). That text has no markdown marks in it, and list numbers, bullets and checkboxes aren't part of it, so counting it with `count_words` gives the same answer as selecting the matching raw text.

The document keeps both counts (raw and formatted) and which of the two changed most recently. The label shows the most recent one whose side is showing: in Formatted view only the formatted count can show, in Raw view only the raw one, and in Split view whichever was made last. A cleared selection on the most recent side goes back to the whole document.

*Alternative considered:* map the formatted selection back to raw lines with the source positions and count those. Rejected: more work for the same number, and partial blocks don't map cleanly.

### 3. Reading time and wording

Reading time is `ceil(words / 200)` minutes. Numbers are grouped with commas ("1,240") by a small helper, since LeafMD isn't translated yet; when translations come, this becomes the locale's grouping. The separator is a middle dot with spaces (" · "), as GNOME apps use.

### 4. The Headings page

A `gtk::ListBox` with GNOME's `navigation-sidebar` style (the same look as the Files tree), one row per heading, inside a scrolled window. Each row's left margin is 12 pixels per level below the document's top level, where "top level" is the smallest level present. So a document whose headings start at `##` isn't pushed in. Long headings are cut off with "…", and hovering shows the full text.

A `ListBox` is used rather than a `ListView` (which Files uses) because heading lists are short and fully rebuilt after each pause; a `ListView` pays off for thousands of rows. Rebuilding keeps the list's scroll position, and the highlighted row is then set again.

Empty states use `adw::StatusPage`: "No Headings" with "Lines starting with # appear here", and "No Document Open".

The page's title at the top of the panel is "Headings". The Files page keeps setting the folder name. When the page changes, the window sets the title for whichever page is showing.

A new LeafMD icon, `leafmd-headings-symbolic` (a few lines of decreasing length, indented, drawn as filled shapes), goes in `data/resources/icons` and the resource list. The Files page's existing `leafmd-folder-symbolic` stays.

### 5. Jumping to a heading

Activating a row calls `Document::jump_to_line(line)`:
- **Raw or Split:** puts the cursor at the start of that line, scrolls the raw text so the line is at the top (`scroll_editor_to_line`), and gives the raw text keyboard focus, so typing goes there.
- **Formatted or Split:** runs `scrollToLine(line)` in the formatted view.
- In Split view both are moved directly, with the "ignore the next scroll" timers set on both sides, so the synced scrolling doesn't fight the jump.
- **Formatted only:** keyboard focus goes to the formatted view.

In a narrow window the panel then tucks away, using the same code as picking a file.

*Alternative considered:* scroll to the heading by its HTML id, as `#links` do. Rejected because ids can repeat ("Notes" twice), aren't there in Raw view, and the line number is already known.

### 6. Following your place

The window needs "the line at the top of the document" for the selected tab:
- **Raw and Split:** from the raw text's scroll position, using the same calculation as `sync_preview_to_editor` (moved into a small shared function).
- **Formatted:** from the formatted view's `scroll` message, which already carries `line`. `handle_preview_message` now passes that line on to the window in every view, and still only syncs the raw text in Split view.

The highlighted heading is the last one whose line is at or above that line (with a small allowance, so a heading exactly at the top counts). Before the first heading, no row is selected. Updates happen at most once per frame. The list scrolls only when the highlighted row would be out of sight, so it doesn't jump around while reading.

### 7. Which page shows, and remembering it

A new window action, `win.side-panel-page`, holds the page name as text (`"files"` or `"headings"`). Clicking the switcher changes the stack's visible page; the window listens for that, updates the action's state and saves it in `state.ini` as `[side-panel] page=headings`. New windows read it, starting on `files` when it's missing or unknown. Other open windows aren't changed, matching how the panel's open state already works.

The action is mainly there so the end-to-end tests can read and set the page (their `action_state` and `activate_action` helpers already work with window actions). It isn't put in a menu or given a shortcut.

`SidePanel` gains `set_visible_page(name)`, `visible_page()` and a way to be told when the page changes.

### 8. The word count label

A `gtk::Label` in a `gtk::Overlay` placed around the window's tab view (inside the toast overlay), aligned to the bottom-right with about 12 pixels of margin, plus room for the scroll bar on the right. One label per window, refreshed from the selected document.

- Style: small text, dimmed, on a rounded, slightly see-through background in the window's colour, so it stays readable over text without drawing the eye. Set in LeafMD's window CSS.
- It ignores the pointer (`can_target = false`), so clicks and selections reach the text under it.
- Hidden when no document is open, and in Distraction Free (the code that hides the bars also hides it).
- So the last line can scroll clear of it, the raw text's bottom margin grows from 16 to about 48 pixels, and the formatted page gets matching padding at its bottom in `preview.html`.

In a narrow window the bottom bar is outside the overlay, so the label sits just above it with no extra work.

*Alternative considered:* a status bar along the bottom of the window. Rejected: GNOME apps avoid them, and it would take a full row from the document for one short line of text.

### 9. Escape leaves one mode at a time

The window's existing Escape handler (the capture-phase key handler in `setup_panel_signals`) keeps its order: a slid-out side panel first, then the find bar (by not handling Escape while it's open). After that it now checks Full Screen on its own: if the window is full screen, Escape leaves only that. Otherwise, if Distraction Free is on, Escape leaves it. The bars stay hidden after leaving full screen, because Distraction Free is still on.

## Risks / Trade-offs

- **[The label covers a corner of the text]** While scrolling, a word in the bottom-right corner can sit under the label for a moment. → It's small and see-through, clicks pass through it, and the extra bottom margin means nothing is stuck under it at the end of the document.
- **[Counts differ from other apps]** Every word counter has its own rules (hyphens, numbers, code). → The rules are written down in the word-count spec and Help, and unit tests pin them.
- **[Highlighting by line is approximate in the formatted view]** `lineAtTop()` is worked out from block positions, so on a page with a tall image the highlight may change slightly early or late. → Good enough to show "where am I"; the jump itself goes to the exact heading.
- **[Larger raw bottom margin]** Typing at the very end now has a little more space below it than before. → It matches what the label needs, and many editors leave room at the end.
- **[Jump and synced scrolling in Split view fight]** → Both sides are set directly, with the existing "ignore the next scroll" timers on both.

## Migration Plan

Nothing to migrate. `state.ini` gains one optional key; older versions ignore it, and a missing value means Files.

## Open Questions

- The exact look of the label (size, opacity, how see-through its background is) is best settled by trying it on the owner's desktop in light and dark style. It doesn't change the spec.

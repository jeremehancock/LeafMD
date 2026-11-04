# Design

## Context

The raw text is a GtkSourceView editor (`source_view` and `buffer` in `src/window.rs`). It already copies the previous line's indent on Enter (its "auto indent" option), and the formatting bar's text rules live in `src/format.rs` as plain functions with unit tests, so they can be tested without opening a window. `format.rs` already has `parse_line`, which splits a line into its indent, its list or heading marker and the rest.

Find and replace and the formatting bar group their edits between `begin_user_action` and `end_user_action`, which is how GTK knows a set of changes is one step for Ctrl+Z. This change does the same.

## Goals / Non-Goals

**Goals:**
- Keep the decisions (what the next marker is, is this a code block, is this a web address) in plain functions with tests, and keep the GTK code in `window.rs` thin.

**Non-Goals:**
- No change to how the formatting bar or find and replace work.

## Decisions

### Catch Enter with a key controller on the editor

A `gtk::EventControllerKey` added to `source_view` in the capture phase (so it sees the key before the editor does) looks at Return and the keypad Enter. Without Shift, Ctrl or Alt held, it asks `typing::on_enter` what to do with the current line and the cursor position in it. If the answer is "nothing special", the key goes on to the editor as normal, so its own indent copying and everything else keep working. Otherwise the controller makes the edit inside one user action, moves the cursor, and stops the key.

Shift+Enter is simply let through: the editor already inserts a plain new line for it.

*Alternative considered:* listening to the buffer's `insert-text` signal and adding the marker after a newline is inserted. That would also fire for pasted text containing new lines and for undo and redo, which would add markers nobody asked for. Catching the key only reacts to someone pressing Enter.

An input method (used for typing Chinese, Japanese and so on) shows unfinished text while it composes, and Enter is how the user accepts it. Because this controller sees keys before the editor does, it would otherwise take that Enter. The editor reports when composing starts and stops (its `preedit-changed` signal, "preedit" being the unfinished text), so the window keeps a flag and leaves Enter alone while composing.

### The plain-text rules in a new `src/typing.rs`

`typing::on_enter(line, cursor)` takes the whole current line and the cursor's position in it (in characters, like `format.rs`) and returns one of:

- `Continue { insert }`: insert a new line plus this marker at the cursor (the text after the cursor ends up after the marker, which is the "split" behaviour for free)
- `EndList`: replace the whole line with nothing
- `Normal`: do nothing special

It works out the marker from the line itself: the indent, then the bullet character (`-`, `*`, `+`), or the number plus one with the same `.` or `)`, or `- [ ] ` for checklist items (keeping the bullet character the item used). Headings count as "Normal". It uses `format::parse_line` (made `pub(crate)`, meaning visible to the rest of LeafMD but not outside it) to find the marker, so the formatting bar and Enter agree on what a list item is.

`typing::in_code_block(text_before_line)` counts the fence lines (lines starting with ` ``` ` or `~~~`, after up to three spaces) before the current line. An odd count means the cursor is inside a code block. The window passes it the text from the start of the document to the start of the current line. That is a scan of the document on every Enter in a list item, which is fast enough for any markdown file a person writes by hand.

`typing::is_web_address(text)` says whether pasted text is a link: after trimming spaces and new lines from the ends, it starts with `http://` or `https://`, has something after that, and has no spaces inside.

*Alternative considered:* adding these to `format.rs`. That file is about the formatting bar's buttons; a separate file keeps each one easier to read, and both are small.

### Catch paste with the editor's `paste-clipboard` signal

GtkTextView emits `paste-clipboard` for Ctrl+V, Shift+Insert and the right-click menu's Paste. When text on a single line is selected, the handler stops the normal paste and reads the clipboard's text (this is asynchronous: the clipboard may belong to another app, so the text arrives a moment later). When it arrives:

- if it is a web address and the same selection is still there, replace the selection with `[selection](address)` in one user action and put the cursor after it
- otherwise call the buffer's normal `paste_clipboard`, which pastes exactly as before (including non-text clipboard content being ignored)

With no selection, or a selection over several lines, the handler does nothing and the normal paste runs untouched. Middle-click pasting (the "primary selection" on Linux) doesn't go through this signal and is unchanged.

*Alternative considered:* letting the paste happen and then rewriting it. That would make undo take two steps and briefly flash the replaced text.

## Risks / Trade-offs

- [A user doesn't want a marker on the next line] → Pressing Enter again on the empty item removes it, and Shift+Enter skips it. Both are listed in Help.
- [A line like `2024. was a good year` looks like a numbered item] → Markdown itself treats it as one, so LeafMD continuing it matches what the formatted view shows.
- [Clipboard read is asynchronous, so the selection could change before the text arrives] → The handler remembers the selection's start and end positions and only makes the link if they still match; otherwise it falls back to the normal paste.
- [Undo leaves the cursor somewhere different from where it started] → GTK's undo puts the cursor at the undone change, which is the usual GNOME behaviour, so the spec only promises the text goes back.

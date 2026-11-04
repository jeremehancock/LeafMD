# Proposal

## Why

Writing lists in the raw text is fiddly today: after every item you have to type `- ` or the next number yourself, and adding a link means typing the brackets and parentheses around the address by hand. Most markdown editors take care of both, so people expect it. This is section 3 of ROADMAP.md, "Smarter typing".

## What Changes

- **Lists continue on Enter.** Pressing Enter in a list item in the raw text starts a new item underneath with the same indent and the same kind of marker:
  - `- `, `* ` and `+ ` bullets continue with the same bullet
  - numbered items count up (`2. ` after `1. `, and `3) ` after `2) `)
  - checklist items continue with an unticked box (`- [ ] `), even after a ticked one
  - Enter in the middle of an item splits it, and the text after the cursor moves into the new item
- **Lists end on an empty item.** Pressing Enter on an item that has only its marker (like `- ` or `3. ` with nothing after it) removes the marker and leaves a blank line, which ends the list.
- **Shift+Enter** starts a plain new line inside a list, without adding a marker. This is the way out when you want a line that isn't a new item. (Shift+Enter already inserts a new line today; it just keeps doing that.)
- **Paste a link over text.** With some text selected on one line, pasting a web address (starting with `http://` or `https://`) turns the selection into `[selected text](address)` instead of replacing it. Pasting anything else replaces the selection as it does now.
- One Ctrl+Z undoes each of these in one step: the new item, the removed marker, or the link.
- None of this happens inside a fenced code block (between ` ``` ` lines), where the text is code, not markdown.

New keyboard shortcuts: Shift+Enter (new line without a list marker), listed in Keyboard Shortcuts. No new menu items or settings.

**Out of scope** (could come later, each would need its own decision):

- Renumbering the items below when an item is added or removed in the middle of a numbered list
- Tab and Shift+Tab to indent or outdent a list item
- Continuing block quotes (`> `) on Enter
- An empty indented item moving one level out instead of ending the list
- Turning a pasted address into a link when nothing is selected
- Any of this in the formatted view, which isn't editable

## Capabilities

### New Capabilities

- `smart-typing`: help LeafMD gives while typing in the raw text: continuing and ending lists on Enter, and turning a pasted web address into a link around the selected text.

### Modified Capabilities

None. The raw text editor requirement in `views` ("new lines keeping the previous line's indent") still holds.

## Impact

- `src/window.rs`: catch Enter and paste in the raw text editor and apply the new behaviour, grouped so one undo reverses it.
- A new `src/typing.rs` holding the plain-text rules (what the next list item looks like, whether the cursor is in a code block, whether pasted text is a web address), with unit tests, in the same style as `src/format.rs`. It reuses the list-marker reading in `src/format.rs`.
- Help, Keyboard Shortcuts, README.md, the metainfo description and ROADMAP.md.
- No new dependencies.

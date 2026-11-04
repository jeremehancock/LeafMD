# find-replace Specification

## Purpose
Finding text in the open document, in whichever view is showing, and replacing it in the raw text.

## Requirements

### Requirement: Opening the find bar
LeafMD SHALL show a find bar under the toolbar when the user presses Ctrl+F, clicks the magnifying glass button in the header bar, or chooses Find… from the main menu. The cursor SHALL go into the find box with its text selected. If a single line of up to 200 characters is selected in the raw text, the find box SHALL start with that text. The magnifying glass button SHALL stay pressed in while the bar is open, and clicking it again SHALL close the bar.

#### Scenario: Searching for the selected word
- **WHEN** "install" is selected in the raw text and the user presses Ctrl+F
- **THEN** the find bar opens with "install" in the find box and its matches highlighted

#### Scenario: No document open
- **WHEN** no document is open
- **THEN** the magnifying glass button and the Find menu items are greyed out

### Requirement: Highlighting and counting matches
LeafMD SHALL highlight every match while the find bar is open and show a count beside the find box: "3 of 12" when a match is selected, "12 matches" (or "1 match") when none is, and "No matches" when nothing matches, with the find box turning red. Typing in the find box SHALL jump to the first match from the cursor onward, so typing more letters keeps the same match where it can.

#### Scenario: Count follows the selection
- **WHEN** the user presses Enter twice after typing a word with 12 matches
- **THEN** the count reads "3 of 12"

#### Scenario: Nothing matches
- **WHEN** the user types text that isn't in the document
- **THEN** the count reads "No matches" and the find box turns red

### Requirement: Moving between matches
Enter, F3, the down arrow button and Ctrl+G SHALL go to the next match, and Shift+Enter, Shift+F3, the up arrow button and Ctrl+Shift+G SHALL go to the previous one, wrapping round at the ends of the document. The selected match SHALL be scrolled into view with some room around it. F3 with the bar closed SHALL open it.

#### Scenario: Wrapping
- **WHEN** the last match is selected and the user presses Enter
- **THEN** the first match is selected

### Requirement: Searching the view that is showing
In Raw and Split view LeafMD SHALL search the raw text. In Formatted view it SHALL search the formatted text instead, highlighting matches on the page (yellow, with the selected match in orange, in darker shades in dark mode). Switching views with the bar open SHALL search the newly shown text, and only one side SHALL show highlights at a time. Text in separate blocks, like the end of one paragraph and the start of the next, SHALL NOT count as one match in the formatted view.

#### Scenario: Formatted view
- **WHEN** the user searches for "world" in Formatted view and one copy is partly bold (`hello **wor**ld`)
- **THEN** that copy counts as a match and is highlighted on the page

#### Scenario: Switching to Split view
- **WHEN** the find bar is open in Formatted view and the user switches to Split view
- **THEN** the page highlights disappear and the raw text's matches are highlighted instead

### Requirement: The replace row
LeafMD SHALL offer replacing in Raw and Split view only. Ctrl+H, Find and Replace… in the main menu, or the replace button in the find bar SHALL show a second row with a "Replace with" box, a Replace button and a Replace All button.

#### Scenario: Formatted view
- **WHEN** the find bar is open in Formatted view
- **THEN** the replace button and the replace row are hidden

### Requirement: Replacing one match
Replace (or Enter in the replace box) SHALL replace the selected match and go to the next one, or just go to the next match if none is selected yet, so the user sees what will change. The replacement text SHALL be used exactly as typed.

#### Scenario: Stepping through
- **WHEN** a match is selected and the user presses Replace
- **THEN** that match is replaced and the next match is selected

### Requirement: Replacing all matches
Replace All SHALL replace every match, show how many were replaced, and MUST be undone with a single Ctrl+Z.

#### Scenario: Replace All and undo
- **WHEN** the user replaces all 4 copies of "cat" with "dog" and then presses Ctrl+Z once
- **THEN** a message says "Replaced 4 matches", and after Ctrl+Z all 4 say "cat" again

### Requirement: Search options
The find bar SHALL have a small options menu with two tick boxes, Match Case and Whole Words Only, both off by default. Changing one SHALL search again straight away. Whole Words Only SHALL not match the word inside a longer word, where letters, digits and underscores count as part of a word.

#### Scenario: Whole words
- **WHEN** Whole Words Only is ticked and the user searches for "cat"
- **THEN** "cat" matches but "concat" and "cat_food" don't

### Requirement: Closing the find bar
Escape (in either box) or the bar's close button SHALL close the find bar, remove all highlights, and put the cursor back in the text that is showing.

#### Scenario: Escape
- **WHEN** the user presses Escape in the find box
- **THEN** the bar closes, the highlights disappear, and typing goes into the raw text again

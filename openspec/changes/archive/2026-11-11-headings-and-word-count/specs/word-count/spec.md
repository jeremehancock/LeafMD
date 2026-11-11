# Spec Delta

## Purpose

The word count label in the corner of each document: what it counts, the reading time it gives, how it counts a selection, and when it's hidden.

## ADDED Requirements

### Requirement: The word count label
While a document is open, LeafMD SHALL show a small, faint label in the bottom-right corner of the document area with the selected tab's word count and reading time, such as "1,240 words · 7 min read", in all three views. It SHALL NOT stop clicks reaching the document under it, and the document's last line SHALL be able to scroll clear of it.

#### Scenario: Opening a document
- **WHEN** the user opens notes.md, which has 1,240 words
- **THEN** the bottom-right corner says "1,240 words · 7 min read"

#### Scenario: Switching tabs
- **WHEN** notes.md has 1,240 words, todo.md has 85, and the user selects todo.md's tab
- **THEN** the label says "85 words · 1 min read"

#### Scenario: No document
- **WHEN** the window shows "No Document Open"
- **THEN** there is no word count label

#### Scenario: Reading the end of a document
- **WHEN** the user scrolls to the bottom of notes.md
- **THEN** the last line sits above the label, not under it

### Requirement: What counts as a word
LeafMD SHALL count the words a reader would see in the formatted document: markdown marks, link and image addresses, image descriptions and HTML tags SHALL NOT count, while text in code SHALL. A word SHALL be a run of letters or digits, which may include apostrophes and hyphens inside it.

#### Scenario: Formatting marks
- **WHEN** a document holds only `## Read the **whole** [guide](https://leafmd.app/guide)`
- **THEN** the label says "4 words"

#### Scenario: Apostrophes, hyphens and numbers
- **WHEN** a document holds only `Don't skip the well-known 3 steps - really.`
- **THEN** the label says "7 words", since the lone hyphen isn't a word

#### Scenario: Lists and checkboxes
- **WHEN** a document holds `- [ ] buy seeds` and `1. water daily`
- **THEN** the label says "4 words"

### Requirement: Reading time
LeafMD SHALL give the reading time at 200 words a minute, rounded up to whole minutes, as "N min read". An empty document SHALL show "0 words" with no reading time, and one word SHALL show "1 word".

#### Scenario: A long document
- **WHEN** a document has 1,240 words
- **THEN** the label says "1,240 words · 7 min read"

#### Scenario: An empty document
- **WHEN** the user presses Ctrl+T
- **THEN** the new tab's label says "0 words"

#### Scenario: One word
- **WHEN** the user types "Hello" in a new document
- **THEN** the label says "1 word · 1 min read"

### Requirement: The count keeps up with typing
LeafMD SHALL update the word count shortly after the user stops typing (about 150 milliseconds), and when the document is reloaded because another app changed it.

#### Scenario: Typing a sentence
- **WHEN** a document has 10 words and the user types " and two more"
- **THEN** a moment after they stop, the label says "13 words · 1 min read"

### Requirement: Counting a selection
While text is selected in the raw text or the formatted view, the label SHALL show the selection's words out of the document's, such as "35 of 1,240 words", counted by the same rules. A selection SHALL only count while its side is showing, and in Split view the selection made most recently SHALL count. Clearing it SHALL go back to the whole document.

#### Scenario: Selecting a paragraph
- **WHEN** the user selects a 35-word paragraph in the raw text of a 1,240-word document
- **THEN** the label says "35 of 1,240 words"

#### Scenario: Clicking elsewhere
- **WHEN** a selection is showing "35 of 1,240 words" and the user clicks somewhere in the text without dragging
- **THEN** the label says "1,240 words · 7 min read" again

#### Scenario: Selecting in Formatted view
- **WHEN** the user selects a 35-word paragraph in Formatted view of a 1,240-word document
- **THEN** the label says "35 of 1,240 words"

#### Scenario: Both sides of Split view
- **WHEN** 10 words are selected in the raw text of Split view, and the user then selects a 35-word paragraph on the formatted side
- **THEN** the label says "35 of 1,240 words"

#### Scenario: Changing view
- **WHEN** 35 words are selected in Raw view and the user presses Ctrl+1
- **THEN** the label says "1,240 words · 7 min read", because the raw text isn't showing

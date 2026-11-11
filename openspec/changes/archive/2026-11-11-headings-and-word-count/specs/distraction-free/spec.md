# Spec Delta

## MODIFIED Requirements

### Requirement: What Distraction Free hides
In Distraction Free, LeafMD SHALL hide the header bar, the tab bar, the formatting bar, the bottom bar of a narrow window, the side panel, the word count and the line numbers. The find bar and the banner about a file changed by another app SHALL still appear when needed.

#### Scenario: The side panel
- **WHEN** the side panel is open and the user turns Distraction Free on
- **THEN** the panel is hidden, and turning Distraction Free off brings it back

#### Scenario: The word count
- **WHEN** the bottom-right corner says "1,240 words · 7 min read" and the user presses Ctrl+Shift+D
- **THEN** the word count disappears, and pressing Ctrl+Shift+D again brings it back

#### Scenario: Finding text
- **WHEN** the user presses Ctrl+F in Distraction Free
- **THEN** the find bar appears above the document, as it does normally

#### Scenario: The file changes
- **WHEN** another app saves notes.md while it's open in Distraction Free
- **THEN** the banner saying the file changed appears above the document

### Requirement: Leaving with Escape
Escape SHALL leave Full Screen first, and a second Escape SHALL leave Distraction Free. Anything that already uses Escape SHALL come first: an open find bar closes, a side panel sliding over the document tucks away, and an open menu closes.

#### Scenario: Plain Escape
- **WHEN** the window is in Distraction Free with the cursor in the raw text and the user presses Escape
- **THEN** the bars come back and Distraction Free is off

#### Scenario: Find bar open
- **WHEN** the find bar is open in Distraction Free and the user presses Escape twice
- **THEN** the first press closes the find bar and the second leaves Distraction Free

#### Scenario: Both on
- **WHEN** the window is in Distraction Free and Full Screen and the user presses Escape
- **THEN** the window leaves full screen with the bars still hidden, and a second Escape brings the bars back and leaves Distraction Free

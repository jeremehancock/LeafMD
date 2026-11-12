## ADDED Requirements

### Requirement: Language suggestions for code blocks
When the user types ` ``` ` at the start of a line in the raw text (after up to three spaces) followed by at least one letter, and the line opens a code block rather than closing one, LeafMD SHALL show a list of the code block languages it colours whose names start with what's typed. The list SHALL narrow as the user types, and SHALL close when nothing matches.

#### Scenario: Typing a language
- **WHEN** the user types ` ```py ` on an empty line
- **THEN** a small list appears under the cursor offering `python`

#### Scenario: Several matches
- **WHEN** the user types ` ```j `
- **THEN** the list offers `java`, `javascript` and `json`, and typing `a` narrows it to `java` and `javascript`

#### Scenario: Closing a block
- **WHEN** the user types ` ```p ` on the line after `print("hi")` inside an open ` ```python ` block
- **THEN** no list appears, because that line closes the block

### Requirement: Picking or ignoring a suggestion
Pressing Enter or clicking a suggestion SHALL replace what was typed after ` ``` ` with the chosen name, and close the list. Escape, moving the cursor away or typing something that matches nothing SHALL close the list without changing the text. Typing ` ``` ` and pressing Enter with no letter after it SHALL start a new line as before, with no list shown.

#### Scenario: Picking with Enter
- **WHEN** the user types ` ```py ` and presses Enter
- **THEN** the line becomes ` ```python ` and the cursor is at its end

#### Scenario: Ignoring the list
- **WHEN** the user types ` ```py `, presses Escape and then Enter
- **THEN** the line stays ` ```py ` and a new line starts below it

#### Scenario: A plain block
- **WHEN** the user types ` ``` ` and presses Enter
- **THEN** a new line starts below, as it did before, and no list appears

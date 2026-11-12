## ADDED Requirements

### Requirement: Coloured code blocks
The formatted view SHALL colour the code in a fenced code block that names a language LeafMD knows, showing keywords, strings, comments, numbers and names in different colours. Common languages SHALL be known, including Bash and shell, C, C++, CSS, Go, HTML, Java, JavaScript, JSON, Python, Ruby, Rust, SQL and YAML, by their usual short names too (`sh`, `js`, `py`, `rs`, `yml`).

#### Scenario: A Rust block
- **WHEN** the document contains a block starting ` ```rust ` with `fn main() { println!("hi"); } // greet`
- **THEN** `fn` is in one colour, `"hi"` in another and `// greet` in a third, and the text itself is unchanged

#### Scenario: A short language name
- **WHEN** a block starts ` ```sh ` and contains `echo "done" # finished`
- **THEN** it's coloured as shell code, with `"done"` and `# finished` in their own colours

### Requirement: Code blocks that stay one colour
A fenced block with no language or a language LeafMD doesn't know, an indented code block, and inline code SHALL be shown in one colour, as before.

#### Scenario: No language named
- **WHEN** a block starts with plain ` ``` ` and contains `fn main() {}`
- **THEN** it's shown in one colour

#### Scenario: Unknown language
- **WHEN** a block starts ` ```typescript ` and contains `let x: number = 1;`
- **THEN** it's shown in one colour, the same as a block with no language

### Requirement: Code colours follow the style
Code colours SHALL be easy to read on the page's background in both light and dark style, and SHALL change straight away when the style changes between light and dark.

#### Scenario: Switching to dark
- **WHEN** a document with a ` ```python ` block is showing and the user picks Dark in Preferences
- **THEN** the page turns dark and the code's colours change to their dark versions, still readable, without reopening anything

### Requirement: Coloured code still works like code
Colouring SHALL NOT change what a code block does: double-clicking it in Split view SHALL still jump to the same place in the raw text, scrolling SHALL stay in step, Find SHALL still find text inside it, even when a match spans several colours, and selecting and copying SHALL copy plain text.

#### Scenario: Finding across colours
- **WHEN** a ` ```rust ` block contains `let total = add(2, 3);` and the user searches the formatted view for `add(2`
- **THEN** the match is found and highlighted

#### Scenario: Double-clicking a coloured block
- **WHEN** a ` ```bash ` block contains `cargo build` and `cargo test`, and the user double-clicks `test` in the formatted side of Split view
- **THEN** the raw text selects `test` in that block, as it would in an uncoloured one

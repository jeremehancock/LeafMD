# Spell Check

## Purpose

Catching typos while writing: LeafMD underlines misspelt words in the raw text, skips code and addresses, offers corrections on right-click, and lets the user turn it off or change its language.

## ADDED Requirements

### Requirement: Misspelt words are underlined
LeafMD SHALL underline each misspelt word in the raw text (Raw and Split view) with a red wavy line, and SHALL update the underlines as the user types, without saving. The formatted view SHALL never show spelling underlines.

#### Scenario: A typo in Raw view
- **GIVEN** spell checking is on in English (US)
- **WHEN** the user types `The garden needs mroe water.` in the raw text
- **THEN** `mroe` has a red wavy underline and no other word does

#### Scenario: Fixing the typo
- **WHEN** the user then changes `mroe` to `more`
- **THEN** the underline disappears

#### Scenario: The formatted view stays clean
- **WHEN** the same document is shown in Split view
- **THEN** `mroe` is underlined on the raw side only, and the formatted side shows the sentence as normal

### Requirement: Code and addresses are not checked
LeafMD SHALL NOT underline words inside inline code, fenced code blocks, code indented by 8 or more spaces (or two tabs), automatic links in angle brackets, or the addresses of links and images. The visible text of a link and an image's description SHALL still be checked. Text indented by 4 spaces SHALL be checked, because it may be a list item's continued paragraph.

#### Scenario: Inline code
- **WHEN** the raw text contains `` Run `cargo bild` first ``
- **THEN** `bild` is not underlined

#### Scenario: A fenced code block
- **WHEN** the raw text contains a block that starts with ```` ```rust ````, holds the line `let resullt = 2;` and ends with ```` ``` ````
- **THEN** nothing inside the block is underlined

#### Scenario: Text indented by 4 spaces
- **WHEN** the raw text contains a line indented by 4 spaces reading `indentd code here`
- **THEN** `indentd` is underlined

#### Scenario: A link
- **WHEN** the raw text contains `[Garden plann](https://example.com/gardn-plan)`
- **THEN** `plann` is underlined, and nothing in `https://example.com/gardn-plan` is

### Requirement: Suggestions on right-click
Right-clicking a misspelt word in the raw text SHALL show suggested spellings in the menu, in a section below the usual Cut, Copy, Paste, Undo and Select All. Choosing one SHALL replace the word, and a single Ctrl+Z SHALL undo the replacement. When there are no suggestions, the menu SHALL say so.

#### Scenario: Choosing a suggestion
- **WHEN** the user right-clicks `mroe` and chooses `more`
- **THEN** the text reads `The garden needs more water.` and the underline is gone

#### Scenario: Undoing it
- **WHEN** the user then presses Ctrl+Z
- **THEN** the text reads `The garden needs mroe water.` again

#### Scenario: A correctly spelt word
- **WHEN** the user right-clicks `garden`
- **THEN** the menu has no suggestions, Add to Dictionary or Ignore, only the usual items, Check Spelling and Languages

### Requirement: Add to Dictionary and Ignore
The right-click menu on a misspelt word SHALL offer Add to Dictionary, which stops the word being underlined in every document from then on, including after LeafMD restarts, and Ignore, which stops it being underlined in every document until LeafMD closes.

#### Scenario: Adding a name
- **WHEN** the user right-clicks `LeafMD` (underlined) and chooses Add to Dictionary, then closes LeafMD and opens another document containing `LeafMD`
- **THEN** `LeafMD` is not underlined in either document

#### Scenario: Ignoring a word
- **WHEN** the user right-clicks `Flatpak` (underlined) and chooses Ignore
- **THEN** `Flatpak` is no longer underlined in any open tab, and is underlined again the next time LeafMD starts

### Requirement: Turning spell checking on and off
Check Spelling in the main menu and in the raw text's right-click menu SHALL turn spell checking on or off, with a tick while it's on. The choice SHALL apply to every window and tab at once and SHALL be remembered between runs. Spell checking SHALL be on the first time LeafMD runs.

#### Scenario: First run
- **WHEN** LeafMD is started for the first time and the user types `mroe` in Raw view
- **THEN** `mroe` is underlined, and Check Spelling in the main menu has a tick

#### Scenario: Turning it off
- **WHEN** two windows are open, each with two tabs containing typos, and the user chooses Check Spelling in one window's main menu
- **THEN** the underlines disappear in all four tabs, and Check Spelling has no tick in either window's main menu or right-click menu

#### Scenario: Remembered
- **WHEN** the user turns spell checking off, closes LeafMD and starts it again
- **THEN** spell checking is still off

### Requirement: Choosing the language
Spell checking SHALL start in the system's language, or English (US) when no dictionary for that language is available. The Languages submenu in the raw text's right-click menu SHALL list the languages with dictionaries on this computer, mark the one in use, and switch to the one chosen. The choice SHALL apply to every document and SHALL be remembered between runs, unless its dictionary is no longer there.

#### Scenario: System in British English
- **GIVEN** the system language is English (UK) and the user has never chosen a language
- **WHEN** the user types `colour` and `color`
- **THEN** `color` is underlined and `colour` is not

#### Scenario: Switching to British English
- **GIVEN** spell checking is in English (US)
- **WHEN** the user right-clicks in the raw text, opens Languages and chooses English (United Kingdom)
- **THEN** `colour` is no longer underlined and `color` now is, in every open tab

#### Scenario: Language remembered
- **WHEN** the user has chosen English (United Kingdom), closes LeafMD and starts it again
- **THEN** spell checking is still in English (United Kingdom)

#### Scenario: Only languages the computer has
- **GIVEN** the system is set up only for English
- **WHEN** the user opens Languages
- **THEN** it lists the English variants, and other languages appear only once the system is set up for them (Flatpak then adds their dictionaries)

#### Scenario: A remembered language that's gone
- **GIVEN** the user chose a language whose dictionary has since been removed
- **WHEN** LeafMD starts
- **THEN** spell checking uses the system's language, or English (US)

#### Scenario: No dictionary for the system language
- **GIVEN** the system language has no dictionary in LeafMD and the user has never chosen a language
- **WHEN** the user types `mroe`
- **THEN** it is checked in English (US), and `mroe` is underlined

### Requirement: Spell checking never changes the file
Spell checking SHALL NOT change the document's text unless the user chooses a suggestion, and SHALL NOT mark a document as having unsaved changes by underlining, adding to the dictionary, ignoring, switching language or turning spell checking on or off.

#### Scenario: Turning it off with a saved document
- **WHEN** a saved document with typos is open and the user turns Check Spelling off and on again
- **THEN** the tab shows no unsaved-changes dot and the file is unchanged

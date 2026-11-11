# Proposal

## Why

LeafMD is for writing as well as reading, but a typo in the raw text goes unnoticed until someone else reads the document. Every other GNOME text editor underlines misspelt words, so people expect it. Spell checking has sat under "Ideas, not decided" on the roadmap, and the owner has now asked for it.

The parts it needs are mostly already there: the GNOME runtime LeafMD runs on includes a spell-checking engine (Enchant, which uses Hunspell dictionaries) and the English dictionaries. Dictionaries for other languages come in the runtime's language packs, which Flatpak installs for the languages each computer is set up for. The markdown colouring LeafMD already uses marks code and link addresses as "don't spell check", so the checker can skip them without LeafMD having to work them out itself.

## What Changes

- **Misspelt words are underlined** in the raw text (Raw and Split view) with a red wavy line, as you type. Code (inline code, fenced code blocks and code indented 8 or more spaces), web addresses in angle brackets and link and image addresses are not checked. Text indented 4 spaces is checked, since it may be a list item's paragraph (the owner accepted this after building showed it).
- **Right-clicking a misspelt word** offers a few suggested spellings in the menu, below the usual Cut, Copy and Paste (GTK always adds them there; the owner chose to keep it). Choosing one replaces the word, and Ctrl+Z undoes it. The same menu has **Add to Dictionary** (never underline this word again, in any document) and **Ignore** (stop underlining it until LeafMD closes).
- **Turning it off:** a new **Check Spelling** item in the main menu, in the section with Line Numbers, with a tick while spell checking is on. The right-click menu has the same item. Like Line Numbers, the choice applies to every window and tab at once and is remembered.
- **Language:** spell checking starts in the language your system uses, or English (US) when LeafMD has no dictionary for it. A **Languages** submenu in the right-click menu switches it. The choice applies to every document and is remembered.
- **On by default**, as in GNOME Text Editor. This is a choice for the owner to confirm (see Decisions below).
- Help, the README, the app store description and leafmd.app say spell checking is there.

New things added to the app:
- Menu items: **Check Spelling** (main menu); **Add to Dictionary**, **Ignore**, **Check Spelling** and **Languages** (right-click menu in the raw text), plus the suggestions.
- Settings remembered: whether spell checking is on, and its language.
- No new keyboard shortcut.

### Decisions for the owner

1. **On or off at first.** Proposed: on, like GNOME Text Editor. The other choice is off until turned on, which is quieter for people who mostly read.
2. **One language for everything**, rather than a language per document. Proposed: one, because it's simpler and most people write in one language. Someone writing in two languages switches from the right-click menu.

## Capabilities

### New Capabilities
- `spell-check`: underlining misspelt words in the raw text, what's skipped, the right-click suggestions, Add to Dictionary and Ignore, turning it on and off, and choosing the language.

### Modified Capabilities
- `help-and-about`: the main menu gains Check Spelling (with a tick while it's on) next to Line Numbers, and Help gets a Spelling row.
- `end-to-end-tests`: two new scenarios (choosing a suggestion, and the setting being remembered), so the run has at least 49.

## Out of scope

- Spell checking in the formatted view (it's for reading, so there's nothing to correct there), in the find and replace boxes, and in file names.
- Grammar or style checking.
- A list of your own dictionary words to edit or remove inside LeafMD. Words added to the dictionary are kept by the spell checker in LeafMD's own settings folder, and can only be removed by editing that file.
- A different language per document or per tab, or detecting the language automatically.
- Downloading dictionaries. Only the languages whose dictionaries are on the computer are offered: the English ones always, and others when the system is set up for them.
- A keyboard shortcut for Check Spelling, and moving to the next or previous misspelt word.
- Spell checking headings' anchor names, HTML tags or other markdown that the markdown colouring doesn't already mark as code or an address. If those turn out to be underlined often, that's a follow-up.

## Impact

- **New dependency:** libspelling, GNOME's spell-checking add-on for the GtkSourceView editor LeafMD uses for raw text (the same one GNOME Text Editor uses), and its Rust bindings (the `libspelling` crate, which matches the GTK versions LeafMD already uses). The GNOME runtime doesn't include libspelling, so the Flatpak build recipe gains a step that builds it from its released source. `build-aux/sdk-check.sh` and the GitHub build need it too. Flathub's checks (`build-aux/flathub-lint.sh`) must still pass.
- **Code:** `src/document.rs` (connecting the checker to each document's raw text and its right-click menu), `src/window.rs` (the Check Spelling menu item and keeping every window in step), `src/state.rs` (remembering the choice and the language), Help in `src/window.rs`.
- **Docs and site:** README, `data/io.github.jeremehancock.LeafMD.metainfo.xml`, `site/index.html`, `docs/DEVELOPMENT.md` (the new library and how to build without the Flatpak), ROADMAP.md (moving spell checking from Ideas to done when archived).
- **Size:** libspelling is small. The dictionaries are already in the runtime, so the download barely grows.

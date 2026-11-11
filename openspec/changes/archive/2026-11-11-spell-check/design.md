# Design

## Context

The raw text is a GtkSourceView editor (`sourceview5::View` with a `sourceview5::Buffer`, made in `Document::new` in `src/document.rs`). Its markdown colouring comes from GtkSourceView's own `markdown.lang`, which already tags inline code, fenced and indented code blocks, `<automatic links>`, link and image addresses and reference labels with the class `no-spell-check`. (Checked in the GNOME 51 runtime's copy.)

The GNOME 51 runtime ships Enchant 2 (the spell-checking engine), its Hunspell back end, and the English Hunspell dictionaries in `/usr/share/hunspell`. Every other language's dictionary there is a link into the runtime's language pack (`/usr/share/locale/<lang>/hunspell`), which Flatpak installs only for the languages the computer is set up for. (Found while building: on an English-only system, Spanish is listed in the folder but has no dictionary.) It does not ship libspelling. The GNOME 51 SDK has everything libspelling needs to build: GTK 4, GtkSourceView 5, Enchant 2 and ICU (a library for splitting text into words in any language).

Line Numbers already shows the pattern for an app-wide, remembered on/off choice: an app-level stateful action set up in `src/main.rs` that saves to `state.ini` (`[editor] line-numbers`) and tells every window to update.

The Flatpak build is offline: Rust crates come from `vendor/`, and the recipe is `build-aux/io.github.jeremehancock.LeafMD.json`. `build-aux/sdk-check.sh` runs clippy and the tests inside the GNOME SDK, and the GitHub build runs it too.

## Goals / Non-Goals

**Goals:**
- Underlines, suggestions, Add to Dictionary, Ignore, on/off and language, as in the spec, with as little code of LeafMD's own as possible.
- Behave like GNOME Text Editor, which uses the same library, so it feels familiar.
- Keep the Flatpak buildable offline, Flathub's checks passing, and the download size about the same.

**Non-Goals:**
- Writing LeafMD's own spell checker or word splitter.
- Teaching the checker about markdown beyond what `markdown.lang` marks (see Risks).
- Shipping extra dictionaries.

## Decisions

### Use libspelling, through the `libspelling` crate

libspelling connects a spell checker to a GtkSourceView buffer: it underlines misspelt words as the text changes, checks only the visible area first so long documents stay fast, skips text in `no-spell-check` contexts, and provides a ready-made right-click menu (suggestions, Add to Dictionary, Ignore, Check Spelling, Languages) with the actions behind it. The `libspelling` crate 0.5 (by the gtk-rs maintainers) depends on `gtk4` 0.11 and `sourceview5` 0.11, the versions LeafMD already uses, and needs libspelling 0.1 or newer.

Alternatives: talking to Enchant directly and drawing underlines ourselves (far more code: word splitting, idle checking, menus, all already solved), or WebKit's spell checking (only applies to editable web content, and the raw text isn't a web view).

### Build libspelling in the Flatpak

A new module before `leafmd` in the recipe builds libspelling's latest 0.4.x release from its GNOME download tarball (pinned by sha256, as Flathub requires) with Meson, turning off its documentation, GObject introspection, Vala bindings and profiler support, none of which LeafMD needs. It installs into `/app`, so only the small shared library is added; the dictionaries keep coming from the runtime.

### One checker shared by every document, one adapter per document

LeafMD makes one `libspelling::Checker` for the whole app and, for each document, a `TextBufferAdapter` that joins that checker to the document's buffer. The adapter's menu model becomes the source view's extra menu (`set_extra_menu`), and the adapter is inserted as the view's `spelling` action group, which is how the menu's items work.

Sharing one checker is what makes Ignore and the language apply to every document at once, since both belong to the checker. Add to Dictionary writes to Enchant's personal word list, which lives inside LeafMD's sandbox (`~/.var/app/io.github.jeremehancock.LeafMD/config/enchant/`), so it lasts between runs and needs no code from LeafMD.

A document that moves to another window keeps its buffer, so it keeps its adapter.

### On and off: an app-wide action, like Line Numbers

A new stateful app action `spell-check` (set up in `src/main.rs` next to `line-numbers`) holds the choice, loads it from `state.ini` (`[editor] spell-check`, default `true`) and saves it when it changes. When it changes, every window sets `enabled` on each of its documents' adapters. The main menu gets "Check Spelling" pointing at `app.spell-check`, after Line Numbers, so it shows a tick.

libspelling's own right-click "Check Spelling" item only changes the one adapter it belongs to. Each adapter's `enabled` change is therefore watched: when it differs from the app action's state, LeafMD changes the app action to match, which brings every other document and the main menu's tick into step. A flag stops the change bouncing back and forth.

Alternative considered: leaving the right-click item per document. Rejected because the spec says one choice for everything, the same as Line Numbers, and two different meanings of "Check Spelling" would be confusing.

Distraction Free doesn't change spell checking, unlike line numbers: underlines are part of writing, not window clutter.

### Language: saved name, else the system's, else English (US)

At start-up LeafMD picks the checker's language in this order: the language saved in `state.ini` (`[editor] spell-check-language`), if a dictionary for it still exists; otherwise the first of GLib's language names for the system (`en_GB`, `en`, skipping `.UTF-8` forms and `C`) that has a dictionary; otherwise `en_US`. libspelling's own default isn't used, because with no match it falls back to the first dictionary it has (Australian English, alphabetically), not US English. Whether a dictionary exists is asked of the provider (`Provider::supports_language`).

The Languages submenu changes the shared checker's language. LeafMD watches the checker's `language` and saves the new value. Nothing is saved until the user chooses, so someone who never picks keeps following their system's language.

### No unsaved-changes side effects

Underlines are text tags, which don't change the text or the buffer's modified flag, so the unsaved dot and the file are unaffected. Choosing a suggestion is a normal edit and goes through the undo history in one step (libspelling wraps it in a user action). This is checked rather than assumed: see the tasks.

### LeafMD draws the wavy underline itself

Added after the owner tried it. libspelling's underline is a thin flat line in its own red, and GTK's built-in "error" (wavy) underline is a tiny wave pressed against the letters, with no setting for thickness or distance. Both were hard to see in light and dark style. So LeafMD's colour schemes switch the spell checker's underline off (`def:misspelled-word` with `underline="none"`), and the raw text editor is now `RawView` (`src/raw_view.rs`), GtkSourceView's editor with one addition: after drawing the text, it draws a thicker red wave about 1.5 pixels below every word that has the spell checker's tag, on screen only. The colour is GNOME's red 4 (`#c01c28`) in light style and red 1 (`#f66151`) in dark. Because the tag no longer changes how text looks, GTK doesn't redraw when it moves, so the view asks for a redraw whenever the tag is added or removed.

### Building outside the Flatpak

- `cargo build` on the owner's laptop uses the system's libspelling (Manjaro's `libspelling` package, 0.4.10, is installed). `docs/DEVELOPMENT.md` lists it among the packages to install.
- `build-aux/sdk-check.sh` builds libspelling once into `target/sdk-libspelling` inside the SDK (Meson and Ninja are in the SDK) and points `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` at it before running clippy and the tests. It reuses that build on later runs. The tarball's address and sha256 are read from the Flatpak recipe so the two can't differ. This keeps the GitHub build working without other changes.

### Testing

End-to-end scenarios on the hidden desktop can check what's not visual:
- `spell-check-remembered`: the `spell-check` action starts on, turning it off is saved as `[editor] spell-check=false`, and it's still off after restarting.
- `spell-check-suggestion`: type `The garden needs mroe water.`, put the cursor in `mroe`, open the right-click menu with the Menu key, press Home then Down twice to reach the first suggestion, choose it with Enter, save, and check the file says `more`; then Ctrl+Z, save, and check it says `mroe` again. The hidden desktop takes its language from the computer running the tests, so the fallback to English (US) was checked by hand instead (a saved language with no dictionary, on the hidden desktop, falls back as it should).
- Turning spell checking off and on leaves the title with no unsaved dot.

The underlines themselves, skipping code and links, and the Languages submenu are checked by hand on the owner's desktop and with a picture on the hidden desktop.

## Risks / Trade-offs

- **Bare web addresses are underlined.** `markdown.lang` only marks addresses in `<...>` and inside links. A bare `https://example.com/gardn` in running text may get its parts underlined. Accepted for now (out of scope in the proposal); if it's common, a follow-up could add LeafMD's own `no-spell-check` tagging for bare addresses.
- **Other markdown words.** HTML tags, heading anchors (`{#plan}`) and front matter aren't marked by `markdown.lang`, so may be underlined. Same follow-up.
- **Right-click position.** Suggestions are for the word at the cursor. GTK moves the cursor to a right-clicked spot unless it's inside a selection; with a selection, suggestions may be for the wrong word or missing. This is how GNOME Text Editor behaves too. To confirm while building.
- **A second build step.** libspelling adds a module to the Flatpak recipe and to `sdk-check.sh`, and its release needs bumping now and then. It's small and builds in seconds.
- **The last word in the file.** libspelling doesn't check a word with nothing at all after it (no space or line break) at the very end of the text. Most files end with a line break, and typing a space or Enter after the word checks it. Found while building; left as it is.
- **The word at the cursor.** libspelling doesn't underline the word the cursor is in, since that's usually the word being typed; it's checked when the cursor leaves. A file opens with the cursor at the start, so a misspelling in the first word waits until the cursor moves. Separately, a misspelling among the first few words of a file whose first line is ordinary text sometimes stays un-underlined (seen on the hidden desktop; a document starting with a heading isn't affected). Asking libspelling to recheck everything after loading didn't change it. Left as it is; worth reporting to libspelling.
- **Language list length.** The runtime has many dictionaries, so the Languages submenu is long. That's libspelling's menu and matches GNOME Text Editor.
- **Underline colour in dark style.** libspelling's underline is its own tag colour. To check it's clear in both light and dark style, and that it doesn't clash with the find highlight.

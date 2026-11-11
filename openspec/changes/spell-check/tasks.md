# Tasks

## 1. Getting libspelling into the build

- [x] 1.1 Add a `libspelling` module before `leafmd` in `build-aux/io.github.jeremehancock.LeafMD.json`: the latest 0.4.x release tarball from download.gnome.org, pinned by sha256, built with Meson with docs, introspection, Vala and sysprof turned off. Verify: `./build-aux/flatpak-install.sh` builds, and `/app/lib/libspelling-1.so*` is in the installed app (`flatpak run --command=ls io.github.jeremehancock.LeafMD /app/lib`).
- [x] 1.2 Add the `libspelling` crate (0.5) to `Cargo.toml` with a plain-English comment, and refresh `vendor/` so the offline build has it and `libspelling-sys`. Verify: `cargo build` on the laptop (system libspelling 0.4.10) and the Flatpak build both succeed.
- [x] 1.3 Make `build-aux/sdk-check.sh` build libspelling once into `target/sdk-libspelling` inside the SDK, using the address and sha256 read from the Flatpak recipe, and point `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` at it. Verify: `./build-aux/sdk-check.sh` passes from a clean `target/sdk-libspelling`, and passes faster on a second run.
- [x] 1.4 Run `./build-aux/flathub-lint.sh`. Verify: it passes with no new exceptions.

## 2. Spell checking the raw text

- [x] 2.1 Make one shared spell checker for the app, with its language picked as the design says (saved language if it has a dictionary, else the system's, else `en_US`). Add `load_spell_check`, `save_spell_check`, `load_spell_check_language` and `save_spell_check_language` to `src/state.rs` (`[editor] spell-check`, default true, and `[editor] spell-check-language`), with unit tests like the line-numbers ones. Verify: `cargo test`.
- [x] 2.2 In `Document::new` (`src/document.rs`), join the checker to the buffer with a `TextBufferAdapter`, set its menu model as the source view's extra menu and insert it as the `spelling` action group, enabled according to the setting. Verify on the hidden desktop: typing `mroe` in Raw view shows a red wavy underline (picture), and `garden` has none.
- [x] 2.3 Check what's skipped: inline code, a fenced block, an indented block (only 8+ spaces, owner accepted), `<https://...>`, and a link's address are not underlined, while a link's text is. Verify with a picture on the hidden desktop using the examples from the spec.
- [x] 2.4 Check the right-click menu: suggestions (below the usual items, owner accepted), Add to Dictionary, Ignore, Check Spelling and Languages, and the usual Cut, Copy and Paste still work. Check that the paste-a-link-over-text behaviour (`smart-typing` spec) still works with the new menu. Verify: picture of the menu, and the `paste-link` end-to-end scenario passes.

## 3. On, off and language for every window

- [x] 3.1 Add the stateful app action `spell-check` in `src/main.rs` next to `line-numbers`: loads from and saves to `state.ini`, and on change sets `enabled` on every document's adapter in every window. Verify: with two windows and two tabs each, turning it off removes underlines in all four (picture).
- [x] 3.2 Add "Check Spelling" (`app.spell-check`) to the main menu after Line Numbers in `src/window.rs`. Verify: the menu shows it with a tick while on (picture).
- [x] 3.3 Keep libspelling's right-click Check Spelling in step: when one adapter's `enabled` changes, update the app action, so every document and the main menu tick follow; guard against changes bouncing back. Verify: turning it off from the right-click menu clears the main menu's tick and the underlines in another tab.
- [x] 3.4 Save the language when the shared checker's language changes from the Languages submenu, and check every tab switches. Verify: after choosing Spanish (Spain), `jardín` isn't underlined and `garden` is in two tabs, and the choice is still there after restarting.
- [x] 3.5 Check nothing marks a document as changed: turning spell checking off and on, Add to Dictionary, Ignore and switching language leave no unsaved dot on a saved document. Choosing a suggestion is undone by one Ctrl+Z. Verify on the hidden desktop.
- [x] 3.6 Check the underline is clear in light and dark style and doesn't clash with the find highlight or the current-line highlight. Verify with pictures in both styles.

## 4. End-to-end scenarios

- [x] 4.1 Add `build-aux/end-to-end/spell-check-remembered.sh`: `spell-check` starts on, turning it off saves `[editor] spell-check=false`, and it's still off after restarting. Verify: `./build-aux/end-to-end-tests.sh spell-check-remembered` passes.
- [x] 4.2 Add `build-aux/end-to-end/spell-check-suggestion.sh`: type `The garden needs mroe water.`, put the cursor in `mroe`, open the right-click menu from the keyboard, choose the first suggestion, save and expect `more`; then Ctrl+Z, save and expect `mroe`. Also expect no unsaved dot after turning spell checking off and on with the file saved. Verify: the scenario passes.

## 5. Telling people about it

- [x] 5.1 Add a Spelling row to Help (`show_help` in `src/window.rs`): underlines in the raw text, code and addresses skipped, right-click for suggestions, Add to Dictionary and Ignore, Check Spelling in the main menu, Languages in the right-click menu. No Keyboard Shortcuts change (no new shortcut). Verify: Help shows the row (picture).
- [x] 5.2 Update README.md's features list, the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and the storefront in `site/index.html`. Verify: `./build-aux/sdk-check.sh` (which validates the metainfo) passes, and `build-aux/build-site.py` builds the site.
- [x] 5.3 Update `docs/DEVELOPMENT.md`: libspelling as a new library to install for `cargo build`, and the Flatpak module.
- [x] 5.4 Check the screenshots: if the sample in `data/screenshots/sample.md` now shows underlines in `raw.png`, `split.png` or `split-dark.png` (names like "LeafMD" or "GNOME"), decide with the owner whether to retake them with `build-aux/take-screenshots.sh` and follow `data/screenshots/README.md`.
- [x] 5.5 Move "Spell checking in the raw text" in ROADMAP.md out of "Ideas, not decided" into section 4 (Reading and writing comfort) as an unticked item, so `/ship` can tick it when archived. Verify: the roadmap reads correctly.

## 6. Ready for the owner

- [x] 6.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh` and `openspec validate --specs --strict`. Verify: all pass with no warnings.
- [x] 6.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh`. Verify: every scenario passes, and `flatpak run --command=true io.github.jeremehancock.LeafMD` still works afterwards.

## Workflow follow-up

- The owner tries it in the installed Flatpak (close every LeafMD window and reopen), especially the underline colour, the right-click menu and the Languages list.
- `/ship` commits, picks the version with the owner, archives the change (copying the specs into `openspec/specs/`) and ticks the roadmap item.

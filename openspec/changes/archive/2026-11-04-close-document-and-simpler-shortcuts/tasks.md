# Tasks

## 1. Which file to reopen

- [x] 1.1 In `src/state.rs`, tell the three cases apart (no saved value, a file, an empty value), add a way to save the empty value, and add a small GTK-free function that picks the file to reopen. Add `cargo test` cases for all three: no value uses the newest recent file, a file wins over the recent list, and an empty value opens nothing even with files on the list.
- [x] 1.2 Use that function in `open_last_file`, and verify on a throwaway settings folder that an empty value starts on the "No Document Open" page.
- [x] 1.3 When a window closes, record its file as the last file only if the file is on the saved recent list. Verify the "Following a link and closing both windows" scenario still holds: README.md reopens.

## 2. Close Document

- [x] 2.1 Add the `close-document` action and add it to `DOCUMENT_ACTIONS`. After the unsaved-changes check, it empties the document, closes the find bar, shows the "No Document Open" page, greys out the document actions, refreshes the recent menu and saves the empty last-file value. Verify each scenario under "Closing a document" in the documents spec, including Cancel on the "Save Changes?" dialog.
- [x] 2.2 Add "Close Document" to the main menu's first section, after Open…. Verify it's greyed out on the start page and that Ctrl+W still closes the window.
- [x] 2.3 Describe Close Document in Help (`show_help`) and in the README's feature list, and update the "Recent files" text in both, since LeafMD doesn't always reopen the last file now. Verify by reading both in the app and on the page.

## 3. Clear Recent Files

- [x] 3.1 Show Clear Recent Files whenever the saved list has anything in it, not only when files are showing. Verify with only the open file on the list ("No Recent Files" with Clear Recent Files under it) and with an empty list ("No Recent Files" only).
- [x] 3.2 Make clearing also save the empty last-file value. Verify the "Clearing while a file is open" and "Saving after clearing" scenarios by restarting on a throwaway settings folder.

## 4. Keyboard Shortcuts window

- [x] 4.1 Rebuild `show_shortcuts` as a plain dialog like `show_help`: a titled group per section, one row per shortcut, the keys as a dimmed label on the right. Remove the now unused `adw::ShortcutsDialog` code. Verify there's no search box, the list starts directly under the title bar, and it scrolls in a short window.
- [x] 4.2 List every shortcut with its keys as text, renaming "Close" to "Close window" with "Ctrl+W or Ctrl+Q". Compare the rows against `set_shortcuts` in `src/main.rs` and verify none are missing or wrong.
- [x] 4.3 Update the README's shortcuts table to say "Close window" for Ctrl+W or Ctrl+Q. Verify it matches the new window.

## 5. Final checks

- [x] 5.1 Check whether the app store description (`data/io.github.jeremehancock.LeafMD.metainfo.xml`) and storefront (`site/index.html`) need to mention this. They list main features only, so a change isn't expected; verify by reading them. ROADMAP.md has no item for this work, so there's nothing to tick.
- [x] 5.2 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate --specs --strict`, and verify they all pass.
- [x] 5.3 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, so the owner can try the change. Verify the install finishes.
- [x] 5.4 Owner's hand-check in the installed Flatpak: Close Document (with and without unsaved changes), restarting onto the start page, Clear Recent Files with only the open file on the list, and the look of the Keyboard Shortcuts window.

## Workflow follow-up

- Run `/ship` once the owner has tried it. It carries the already committed icon work too, so both go out in one version.
- Archive this change as part of `/ship`, after the owner's test.

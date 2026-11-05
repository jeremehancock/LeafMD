# Tasks

## 1. The icon

- [x] 1.1 Draw `data/resources/icons/scalable/actions/leafmd-new-symbolic.svg` (a page with a plus sign, filled shapes only), list it in `data/resources/resources.gresource.xml`, and verify it's clear at 16 pixels in light and dark styles
- [x] 1.2 Remove `leafmd-save-symbolic.svg` and its entry, and update the icon lists in the `resources.gresource.xml` comment and `docs/DEVELOPMENT.md`. Verify with a search that nothing else refers to the old icon

## 2. The buttons

- [x] 2.1 In `src/window.rs`, put the New icon button where the Save icon was, with the tooltip "New document (Ctrl+N)" and the `win.new` action. Verify that clicking it with unsaved changes asks "Save Changes?", and that it still works on the "No Document Open" page
- [x] 2.2 Add the Save word button just left of the view switcher, with the existing tooltip and `win.save` action. Verify that it saves, and is greyed out on the "No Document Open" page
- [x] 2.3 Check the header bar at the smallest window size with a long file name. Verify that every button shows, nothing is cut off, and the file name shortens with "…"
- [x] 2.4 Give Open and its recent files arrow a raised look with one shared background. Verify in light and dark styles that both parts are raised and match Save, and that the arrow still opens the recent files list
- [ ] 2.5 Add LeafMD's small icon theme with new maximize and restore icons, select it in `src/main.rs`, and install it from the Flatpak manifest. Verify that GTK picks LeafMD's two icons, that the theme is in the installed Flatpak, and that the owner sees both icons by maximizing and restoring a window
- [ ] 2.6 Draw LeafMD's own magnifying glass, main menu, up and down arrows, three dots, page, minimize and close in the same theme. Verify that GTK picks all of them from LeafMD's theme, that they read clearly in light and dark styles, and that the owner sees them in the installed Flatpak

## 3. Words for users

- [x] 3.1 Check Help (`show_help`), README, the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and `site/index.html` for descriptions of these buttons, and update any that are now wrong. Keyboard Shortcuts doesn't change. Verify with a search for "Save" and "floppy" in those files

## 4. Trying it for real

- [x] 4.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `build-aux/flathub-lint.sh` and `openspec validate --specs --strict`, and verify all pass with no warnings
- [x] 4.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, verify it finishes, and ask the owner to close every LeafMD window, reopen it and try the change

## Workflow follow-up

- The header bar shows in the screenshots, so the owner needs to retake them before a release (`data/screenshots/README.md` has the steps).
- This isn't on `ROADMAP.md`, so there's nothing to tick when it's archived.

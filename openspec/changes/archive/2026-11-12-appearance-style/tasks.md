# Tasks

## 1. Remembering and applying the style

- [x] 1.1 In `src/state.rs`, add `load_style` and `save_style` for `style=system|light|dark` in the preferences group (anything missing or unreadable is `system`), with unit tests beside the existing ones for reading a missing, valid and nonsense value; verify with `cargo test`.
- [x] 1.2 In `src/main.rs`, add the app-wide stateful `style` action (string state `system`, `light` or `dark`) beside `spell-check`. On start, load the saved style and set libadwaita's colour scheme before the first window is built (`system` is "prefer light", `light` is "force light", `dark` is "force dark"); on change, set the scheme and save it. Verify with `cargo build`, then by starting LeafMD on the hidden desktop with `style=dark` in its settings file and checking the window is dark.
- [x] 1.3 Check that the raw text colours, formatted view and spelling underline follow a forced change straight away in an open Split view (they already listen for libadwaita's light/dark announcement); verify on the hidden desktop by changing the action's state and comparing pictures of the window before and after.

## 2. The Style row in Preferences

- [x] 2.1 In `src/preferences.rs`, add an Appearance group first, with a Style row holding a toggle group of Follow System, Light and Dark, showing the `style` action's current state and changing it when clicked. Make the dialog tall enough for three groups, and update the file's top comment. Verify by opening Preferences on the hidden desktop and taking a picture in light and in dark.
- [x] 2.2 Update Help (`show_help` in `src/window.rs`) where it describes Preferences, so it mentions the Style choice; verify by reading the Help text in the running app.

## 3. End-to-end test

- [x] 3.1 Add a helper to `build-aux/test-desktop.sh` that sets a string-valued app action (the existing `activate_app_action` only toggles actions with no value), for example through `org.gtk.Actions.SetState`.
- [x] 3.2 Add `build-aux/end-to-end/style-remembered.sh`: the style starts as `system`; setting it to `dark` saves `style=dark` in the preferences group; after Ctrl+Q and opening notes.md again the `style` action is still `dark`. Verify it passes in `./build-aux/end-to-end-tests.sh`.

## 4. Where users read about it

- [x] 4.1 README.md: extend the Fonts and zoom feature line (or add one) to mention Light, Dark and Follow System in Preferences; verify by reading it.
- [x] 4.2 The app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and the storefront in `site/index.html`: mention choosing light or dark; verify `./build-aux/flathub-lint.sh` passes.
- [x] 4.3 ROADMAP.md: add "Light and dark style" under section 4, Reading and writing comfort (ticked when the change is archived by `/ship`).

## 5. Checks and installing

- [x] 5.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, all passing with no warnings.
- [x] 5.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh`, all scenarios passing, so the owner can try it.

## Workflow follow-up

- The owner tries it in the installed Flatpak, then `/ship` commits, archives the change (copying the new rules into `openspec/specs/views` and `openspec/specs/text-size`), ticks the ROADMAP item and opens the pull request.

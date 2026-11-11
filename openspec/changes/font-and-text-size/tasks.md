# Tasks

## 1. Saving the fonts

- [x] 1.1 In `src/state.rs`, add loading and saving of `[preferences] raw-font` and `formatted-font` (a font description string such as `Fira Code 12`; missing or unreadable means the starting font), and clearing one for Reset. Add unit tests that a file with no `[preferences]` section gives the starting fonts, a saved font comes back as saved, and clearing a key gives the starting font again. Verify with `cargo test`

## 2. Fonts in the two views

- [x] 2.1 Add the app-wide raw text stylesheet (design decision 2): one rule per zoom step from 50% to 300% for `.leafmd-zoom-<percent> .leafmd-raw`, built from the saved raw font or, when there is none, libadwaita's `monospace-font-name`, rebuilt when either changes. Give each raw editor the `leafmd-raw` style name and each window `leafmd-zoom-100`. Verify on the test desktop that the raw text looks as it did before, and that a `raw-font` written into `state.ini` before starting shows in that font
- [x] 2.2 Change `src/preview.html`'s `body` font to `1em/1.7 sans-serif`, and set each formatted view's WebKit sans-serif family and default font size from the formatted font times the window's zoom (design decision 4). Check early whether a page already showing redraws when these change, and call `render_now()` after changing them if it doesn't. Verify with a screenshot on the test desktop that the formatted view looks the same as before with no `[preferences]` section, and that code is still monospace and headings keep their sizes with `formatted-font=Adwaita Sans 16` set
- [x] 2.3 Make sure the raw-only column re-measures (`column_width()` via `apply_view`) whenever the raw font or zoom changes. Verify on the test desktop in a 1400 pixel window in Raw view that a bigger raw font gives a wider column holding about 80 characters
- [x] 2.4 Add one place that walks every open window and its documents and applies the current fonts (design decision 6), used by both Preferences and the GNOME monospace font watcher. Verify with two windows open on the test desktop that a font change shows in both

## 3. The Preferences window

- [x] 3.1 Add `src/preferences.rs` with an `AdwPreferencesDialog`: Raw Text and Formatted View groups, each with a "Font" row holding a font button and a flat Reset button shown only while that font is saved (design decision 9). Choosing a font saves it and applies it everywhere at once; Reset clears it. Only one Preferences window at a time. Verify on the test desktop: the rows show "Adwaita Mono 11" (or GNOME's monospace font) and "Adwaita Sans 12" at first, with no Reset buttons, and a chosen font shows at once and gets a Reset button
- [x] 3.2 Add the `win.preferences` action, the Preferences item at the start of the main menu's last section, and `<Control>comma` in `src/main.rs`. It works with no document open. Verify by pressing Ctrl+, on the "No Document Open" page
- [x] 3.3 Help (`show_help` in `src/window.rs`): add a fonts and zoom section covering Preferences (Ctrl+,), the two fonts, Reset, following GNOME's monospace font, and zoom (filled in by 4.5). Keyboard Shortcuts (`show_shortcuts`): add "Preferences" with "Ctrl+," under Help. Change the About text's "There are no plugins to manage or settings to tune. It is just a fast, dependable app that feels obvious from the moment you open it, so you can spend your time on your writing instead of your tools." to "It is ready to use the moment you open it, and the few options it has stay out of your way, so you can focus on reading and writing." Verify by opening Help, Keyboard Shortcuts and About in the installed build
- [x] 3.4 Add `src/preferences.rs` to the file table in `docs/DEVELOPMENT.md`, and mention the `[preferences]` section in `src/state.rs`'s row. Verify by reading the table

## 4. Zoom

- [x] 4.1 Add the `zoom` window action (design decision 7): state is the percentage as text, `in`, `out` and `reset` change it in steps of 10 between 50 and 300, swap the window's `leafmd-zoom-<percent>` style name, and update each document's formatted view font size. Add the keys in `src/main.rs`: Ctrl+plus, Ctrl+=, keypad plus, Ctrl+minus, keypad minus, Ctrl+0 and keypad 0. Add a unit test for the step rules (110 + in = 120, 300 + in stays 300, 50 + out stays 50, reset = 100). Verify with `cargo test` and on the test desktop with the action's state
- [x] 4.2 Documents joining a window (new tab, opened file, link, dragged tab) take that window's zoom. New windows start at 100, and zoom is never saved. Verify on the test desktop: zoom one window, open a second tab and check its formatted view size, press Ctrl+N and check the new window's state is "100"
- [x] 4.3 Ctrl with the mouse wheel zooms over the raw text and the formatted view, without stopping ordinary scrolling (design decision 7). Verify with the zoom scenario (task 4.6), which turns the wheel with and without Ctrl through a new `wheel` helper in `build-aux/test-desktop.sh`
- [x] 4.4 Add the zoom row as the first section of the main menu (design decision 8): Zoom Out, the percentage (clicking it resets), Zoom In, with the buttons greyed out at 50% and 300%. Verify with a screenshot of the open menu on the test desktop at 100% and at 300%
- [x] 4.5 Finish Help's fonts and zoom section: Ctrl+plus, Ctrl+minus, Ctrl+0, Ctrl with the mouse wheel, the menu row, that zoom covers every tab in the window, and that it's back to 100% each time LeafMD starts. Keyboard Shortcuts: add "Zoom in" (Ctrl+Plus or Ctrl+=), "Zoom out" (Ctrl+Minus) and "Normal size" (Ctrl+0) under Views. Verify in the installed build
- [x] 4.6 Add `build-aux/end-to-end/zoom.sh` (text-size spec, "Zoom belongs to a window": Ctrl+plus twice and Ctrl+minus once gives "110", Ctrl+N gives a window at "100", Ctrl+0 gives "100") and `build-aux/end-to-end/zoom-not-remembered.sh` ("Zoom isn't remembered": Ctrl+plus three times, Ctrl+Q, start again, "100", and no zoom key in `state.ini`), using `expect_action_state zoom`. Verify with `./build-aux/end-to-end-tests.sh zoom zoom-not-remembered` after installing

## 5. Everywhere else people read about it

- [x] 5.1 README.md: add Preferences and zoom to the features list, and Ctrl+, , Ctrl+plus, Ctrl+minus and Ctrl+0 to the shortcuts table. Verify by reading it in LeafMD
- [x] 5.2 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: change "There are no plugins to manage or settings to tune," to match the new About wording, and add a line about choosing fonts and zooming. (Release notes wait for `/ship`, since they need a version number.) Verify with `./build-aux/sdk-check.sh`, which validates the file
- [x] 5.3 `site/index.html`: change "No plugins to manage or settings to tune." to "Great out of the box. The few options stay out of the way, so you can focus on reading and writing.", and add fonts and zoom to the features if it fits the page. Verify by building the site with `build-aux/build-site.py` and reading it
- [x] 5.4 Screenshots: the windows themselves don't change with the starting fonts, so check with `./build-aux/take-screenshots.sh` that they come out the same. Retake only if they differ. Verify by comparing the new and old pictures
- [x] 5.5 Tick "Font and text size" and "Zoom" in ROADMAP.md. Verify by reading it

## 6. Checks and trying it

- [x] 6.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate --strict font-and-text-size`, with no failures or warnings
- [x] 6.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh` and check it ends with "41 of 41 passed"
- [ ] 6.3 Ask the owner to close every LeafMD window, reopen it and try what the tests can't: the Preferences window and font picker, Reset, the fonts showing in every window at once, following GNOME's monospace font, the zoom row in the menu, 

## Workflow follow-up

- `/ship` writes the release notes, commits, archives the change (which copies the spec changes into `openspec/specs/`) and opens the pull request.

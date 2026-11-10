# Tasks

The code for groups 1 to 3 was built in a scratch build before this proposal, and the owner tried it and approved it. Those tasks are ticked, with how they were checked. The same uncommitted working folder also holds the `distraction-free` change's code (mostly in `src/window.rs`), which is planned in its own proposal.

## 1. The formatted view

- [x] 1.1 Restyle `src/preview.html`: GNOME palette for light and dark, Adwaita Sans and Adwaita Mono with fallbacks, a 44em column, headings without rules, card tables and code blocks, header cells aligned with their column unless the table sets an alignment, softer quotes, dimmer list bullets, accent-tinted selection, and CSS-drawn checkboxes that stay disabled. Checked with screenshots of Split and Formatted view in light and dark on the test desktop, against the approved mockup
- [x] 1.2 Send the accent colour to the page in `Document::update_style` (`--accent` from the accent, `--link` from libadwaita's standalone accent for the current style) and call it when the accent changes (`connect_accent_color_notify` in `src/window.rs`). Change the dark background to `#1d1d20` on the page and the web view. Checked: ticked boxes and links show the accent in the screenshots (blue on the test desktop). Changing the accent while LeafMD is open could only be tried in the owner's own session

## 2. The raw text

- [x] 2.1 Add `data/resources/styles/leafmd.xml` and `leafmd-dark.xml` (parent schemes Adwaita and Adwaita-dark, overriding only `line-numbers`, `current-line-number` and `current-line`), list them in `data/resources/resources.gresource.xml`, add the resource folder to GtkSourceView's search path at startup in `src/main.rs`, and use them in `update_style` with GNOME's schemes as the fallback. Checked: dimmer numbers and a faint current line in the screenshots, light and dark
- [x] 2.2 Set the editor's margins to 16 (top and bottom) and 18 (sides) and 3 pixels below lines and inside wrapped lines. Checked in the screenshots
- [x] 2.3 Wrap the editor in an `adw::ClampScrollable` with the "view" style, sized in `apply_view` to about 80 characters plus margins (plus room for line numbers) in Raw view and unlimited otherwise. Checked: the Raw view screenshot shows a centred column with the scroll bar at the edge, and Split view fills its half

## 3. Line numbers

- [x] 3.1 Save and load `[editor] line-numbers` in `src/state.rs` (off when missing). Add the app-level `line-numbers` action in `src/main.rs`, new documents reading it in `src/document.rs`, and the Line Numbers item in the main menu next to Side Panel. Checked on the test desktop: off by default, on from state.ini, and shown in a new window. The owner tried the menu item
- [x] 3.2 Add a unit test in `src/state.rs` that a state file without `[editor]`, or with a value that isn't true or false, loads as off. Verify with `cargo test`

## 4. Help and documentation

- [x] 4.1 Help (`show_help` in `src/window.rs`): say under the views or editing section that line numbers are off unless turned on with Line Numbers in the main menu, and that the formatted view uses the desktop's accent colour. Verify by opening Help (F1) in the installed build
- [x] 4.2 README.md: mention Line Numbers in the menu and the accent colour in the features list. Verify by reading the rendered README in LeafMD itself
- [x] 4.3 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: add a line about the GNOME look and accent colour to the description. (The release notes need a version number, which the owner picks, so `/ship` writes them.) Verify with `./build-aux/sdk-check.sh` (it validates the file)
- [x] 4.4 `site/index.html`: update the Raw screenshot's alt text, which says "with line numbers". Verify by reading the page source
- [x] 4.5 docs/DEVELOPMENT.md: add `data/resources/styles/` to the file table. Verify by reading it
- [x] 4.6 Retake the screenshots with `./build-aux/take-screenshots.sh` and check them against `data/screenshots/README.md` (its list may mention line numbers; update it if so). Verify by looking at all four pictures

## 5. Checks and trying it

- [x] 5.1 Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh` and `openspec validate --specs --strict` (after archive) with no failures or warnings
- [x] 5.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, and ask the owner to close every LeafMD window, reopen it and check Help and the screenshots

## Workflow follow-up

- `/ship` decides whether this commits on its own or together with `distraction-free`, since both changes' code is in the same files.
- ROADMAP.md has no item for this change, so there's nothing to tick.

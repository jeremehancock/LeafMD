# Tasks

## 1. The app

- [x] 1.1 Draw `leafmd-heart-symbolic.svg` and `leafmd-coffee-symbolic.svg` in `data/resources/icons/scalable/actions/` with filled shapes only, list them in `data/resources/resources.gresource.xml`, and check `cargo build` succeeds (the resource bundle fails to build if a listed file is missing)
- [x] 1.2 Make `open_in_usual_app` in `src/document.rs` usable from `src/window.rs` (no change to what it does) and check `cargo build`
- [x] 1.3 Add the `win.support` action and "Support Development" just above About LeafMD in the main menu's last section; check by opening the menu on the test desktop that it sits there with no shortcut
- [x] 1.4 Build the support window (design decision 2): close button, heart, "Support Development", the paragraph from design decision 1, and a "Buy me a coffee" pill button that opens `https://www.buymeacoffee.com/jeremehancock` with `open_in_usual_app`, closes the window, and shows "Couldn't open the link: ..." as a toast if it fails; check it at a normal and a 360-pixel-wide window on the test desktop, in light and dark style
- [x] 1.5 Add the "Support Development" link to About next to Website and GitHub, and check it shows there
- [x] 1.6 Add a final "Support" section to Help (`show_help`) mentioning Support Development in the main menu, and check it shows at the end of Help
- [x] 1.7 Add `build-aux/end-to-end/support-window.sh`: the `support` action is enabled, activating it leaves LeafMD running, and Escape then leaves the document as it was; check it passes on its own with `./build-aux/end-to-end-tests.sh support-window`
- [x] 1.8 Run `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh` with no warnings

## 2. The website

- [x] 2.1 Add the `#support` section after "Early days" in `site/index.html` (design decision 7), with the inline heart and coffee-cup SVGs and the button opening `https://www.buymeacoffee.com/jeremehancock` in a new tab with `rel="noopener"`
- [x] 2.2 Add the `.support` styles to `site/style.css` using the existing colour tokens, and "Support development" to the footer links
- [x] 2.3 Preview the site locally for the owner: check it at 1280 and 360 pixels wide in light and dark, that nothing loads from another website, and that the footer link jumps to the section

## 3. The README, app store description and docs

- [x] 3.1 Add "## Support Development" to `README.md` after Contributing: the prose from design decision 1, a link to `https://leafmd.app/#support`, the menu item, and the yellow Buy Me A Coffee badge line used in Cinemetric's README; check the README's link to `#support` matches the section id
- [x] 3.2 Add `<url type="donation">https://www.buymeacoffee.com/jeremehancock</url>` to `data/io.github.jeremehancock.LeafMD.metainfo.xml` and check `build-aux/flathub-lint.sh` passes with no new exceptions
- [x] 3.3 Update `src/window.rs`'s row in the docs/DEVELOPMENT.md file table to mention the support window, and check `openspec validate --specs --strict` still passes
- [x] 3.4 Keyboard Shortcuts needs no change (no new shortcut), no screenshot shows the menu or the new window, and ROADMAP.md has no item for this: confirm and note it in the commit

## 4. Trying it

- [x] 4.1 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh` and check every scenario passes, so the owner can try the change

## Workflow follow-up

- `/ship` once the owner has tried it: commit, version, archive (which copies the spec changes into `openspec/specs/`), push and pull request.

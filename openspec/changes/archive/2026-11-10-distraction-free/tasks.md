# Tasks

The code for groups 1 to 4 was built in a scratch build before this proposal, on top of `visual-polish`, and the owner tried it and approved it. Those tasks are ticked, with how they were checked. `visual-polish` is archived before this change.

## 1. The window's bars in two layers

- [x] 1.1 Split the window's ToolbarView in `src/window.rs`: header bar, tab bar, formatting bar and the narrow bottom bar in the outer one (`controls`), the banner and find bar in an inner one around the document, with the style rule that removes the inner layer's 6 pixel gap. Checked: in normal mode the bars measure exactly the same height as before (pixel comparison of screenshots), and the owner was asked to try Ctrl+F and reported everything looked good

## 2. Distraction Free

- [x] 2.1 Add the per-window `distraction-free` action (Ctrl+Shift+D, a checkbox in the main menu), hiding and showing the outer bars and the side panel, extending the document to the edges, the bars' solid background in that mode, and keeping the side panel hidden when the window is resized. Checked on the test desktop in Split, Formatted and Raw view, light and dark
- [x] 2.2 Reveal the bars at the top or bottom edge and hide them once the pointer is back over the document, except while a menu is open. Checked on the test desktop (bars slide in over the document with a solid background); the menu case was on the owner's list to try, and they reported everything looked good
- [x] 2.3 F10 in Distraction Free: reveal, open the main menu once the bars have arrived, ignore pointer movement meanwhile, hide again when the menu closes. Checked on the test desktop with a full-screen picture: the menu opened, then Escape closed it and the bars slid away with Distraction Free still on

## 3. Line numbers and Escape

- [x] 3.1 Make Line Numbers a window action whose tick shows what the window shows, changing the shared setting through `change_action_state`; hide line numbers on entering Distraction Free, including new tabs and tabs dragged in. Checked on the test desktop: hidden on entering with the setting on, unticked in the F10 menu, and choosing it there showed them
- [x] 3.2 Escape leaves Distraction Free and Full Screen together, after the side panel rule and only when the find bar is closed. Checked on the test desktop: Escape brought the bars and line numbers back

## 4. Full Screen

- [x] 4.1 Add the `fullscreen` action (F11, a checkbox in the main menu) following the window's `fullscreened` state. F11 with Escape was on the owner's list to try, and they reported everything looked good

## 5. Help and documentation

- [x] 5.1 Help (`show_help` in `src/window.rs`): add a Distraction Free and Full Screen section matching the "Reading about Distraction Free" scenario. Verify by opening Help (F1) in the installed build
- [x] 5.2 Keyboard Shortcuts (`show_shortcuts`): add Ctrl+Shift+D (Distraction Free) and F11 (Full Screen) under Views, and mention Escape leaving them. Verify by opening it with Ctrl+?
- [x] 5.3 README.md: Distraction Free and Full Screen in the features list and the shortcuts table. Verify by reading the rendered README in LeafMD
- [x] 5.4 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: a line about Distraction Free in the description and the release notes. Verify with `./build-aux/sdk-check.sh`
- [x] 5.5 `site/index.html`: mention Distraction Free in the features. Verify by reading the page source
- [x] 5.6 ROADMAP.md: change the Distraction-free item to say it always starts off and that F11 is separate. (`/ship` ticks it when the change is archived, as CLAUDE.md says.) Verify by reading it

## 6. Checks and trying it

- [x] 6.1 Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh` and `openspec validate --specs --strict` (after archive) with no failures or warnings
- [x] 6.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, and ask the owner to close every LeafMD window, reopen it and check Help and Keyboard Shortcuts

## Workflow follow-up

- Archive `visual-polish` before this change, because this change's `help-and-about` delta is written on top of its Main menu text.

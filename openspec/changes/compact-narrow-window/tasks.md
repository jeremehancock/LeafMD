# Tasks

## 1. The compact layout for the header bar

- [x] 1.1 In `src/window.rs`, add a bottom bar to the window, hidden by default, and verify with `cargo build` and by checking at 1000 pixels wide that nothing looks different
- [x] 1.2 Add a breakpoint at `max-width: 639sp` (a window of exactly 640 stays wide) that shows the bottom bar and moves New, Save, the magnifying glass and the view switcher into it, then back in their original order, with a comment saying any new header button needs adding here too. Verify by opening the window at 500 pixels and at 800 pixels on the hidden desktop and looking at both
- [x] 1.3 Verify the "Nothing is lost while moved" scenario: cross 640 pixels both ways with the find bar open, Split view on and unsaved changes, and confirm typing carries on without clicking back into the editor
- [x] 1.4 Verify the greyed-out states with no document open, in both layouts, and that clicking New in the bottom bar turns Save back on

## 2. The find bar

- [x] 2.1 In `src/find.rs`, group the count, arrows, replace button and options button into their own box inside the find row, and verify with `take-screenshots.sh` into a temporary folder that wide windows are unchanged and with the find bar open at 1000 pixels
- [x] 2.2 Add the breakpoint's setters: the find row goes vertical, both text boxes get a small `width-chars`, the find box expands, and the size group turns off. Verify at 360 pixels that the find box fills its row, and that "No matches" and every button show in full
- [x] 2.3 Verify the replace row at 360 pixels (Ctrl+H in Raw view) with its box and both buttons in full, and that opening and closing it in the compact layout leaves the window's width alone

## 3. The size promise

- [x] 3.1 Measure the narrowest window on the hidden desktop in Raw, Split and Formatted views and on the "No Document Open" page, and verify each is 360 by 300
- [x] 3.2 Repeat at 360 pixels wide with minimize and maximize buttons added to the title bar, and verify every header button shows in full
- [x] 3.3 Check a window 640 to 660 pixels wide on the hidden desktop, with those extra buttons too, and verify nothing in the wide layout is clipped
- [x] 3.4 Add a Help entry in `show_help` (`src/window.rs`) about narrow windows moving some buttons to the bottom, add a sentence to "Using LeafMD" in `README.md`, and verify `grep` finds both. Keyboard Shortcuts needs no change, because no shortcut changes
- [x] 3.5 Check the metainfo description and `site/index.html` for anything about window size. Confirm `display_length` stays at 360, and verify `openspec validate --specs --strict` and `openspec validate compact-narrow-window --strict` pass (the specs themselves are updated when `/ship` archives the change). There's no `ROADMAP.md` item to tick

## 4. Final checks

- [x] 4.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, and verify they all pass
- [x] 4.2 Run `build-aux/take-screenshots.sh` into a temporary folder after installing this build, and verify the four pictures match `data/screenshots/`, which shows wide windows haven't changed
- [ ] 4.3 Rebuild and install with `./build-aux/flatpak-install.sh`, then ask the owner to close every LeafMD window, reopen it and try the narrow window scenarios (the owner confirms)

## Workflow follow-up

- `/ship` handles the commit, the release notes and version question, archiving the change into `openspec/specs/`, and the pull request, once the owner has tried it.

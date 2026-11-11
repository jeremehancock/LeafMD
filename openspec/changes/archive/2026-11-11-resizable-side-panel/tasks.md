# Tasks

## 1. Remembering the width

- [x] 1.1 In `src/state.rs`, add `load_panel_width` and `save_panel_width` for `[side-panel] width=` (design decision 2): 240 when missing, not a number, or outside 200 to 400. Add a unit test like the ones for `open` and `page` covering a saved 320, a missing width, `width=wide` and `width=900`. Verify with `cargo test`

## 2. The drag handle and the window's width

- [x] 2.1 Give each window its chosen width (from `load_panel_width`), and apply it through the split view's smallest and largest sidebar widths instead of the fixed 240 (design decision 2). Verify on the test desktop that a hand-edited `width=320` in the test home's `state.ini` opens the panel 320 wide, with a picture
- [x] 2.2 Add the handle (design decision 1): a transparent strip about 6 pixels wide over the document's left edge, with the `col-resize` pointer and a drag gesture that sets the width between 200 and the current limit, saving it with `save_panel_width` when the drag ends. Shown only while the panel sits beside the document (not collapsed, not in Distraction Free). Verify on the test desktop with pictures before and after a drag, at both limits, and that clicking just right of the strip still places the cursor in the raw text
- [x] 2.3 Follow the window's width (design decision 3): wrap the split view in a `WidthWatcher` that reports its width just before laying the split view out, and show the panel at the chosen width but no wider than the window minus 680 (converted from `sp`). (The owner's hand test found the first build didn't follow a window dragged narrower: `adw::Bin`'s layout helper stopped the watcher ever reporting a width. Rebuilt to lay out its child itself, and to set the width before layout rather than afterwards.) In a narrow window, set the width to 240 and hide the handle; back in a wide one, restore the chosen width. Verify on the test desktop: a 400 pixel panel in a 1200 window shows 320 at 1000 and 400 again at 1200, and at 700 the panel slides over at 240. Check pictures taken while the window narrows in small steps, like a hand
- [x] 2.4 Check how the panel looks at 200 and 400 pixels: the Files title, the page switcher at the bottom, and long names cut short in the middle. Verify with pictures in light and dark style. If something doesn't fit at 200, raise the smallest width and update the side-panel spec delta
- [x] 2.5 Add a `drag_window <x> <y> <dx>` helper to `build-aux/test-desktop.sh` (design decision 6), built like `click_window`, and `build-aux/end-to-end/side-panel-width.sh` (side-panel spec, "The side panel remembers its width" and "The document keeps its room": a 1200 pixel window, F9, drag the handle 80 to the right, `expect_setting side-panel width 320`, Ctrl+Q, start again, check the panel is 320 wide by dragging from its edge to 400, then narrow the window to 1000 in 8 pixel steps and check the panel shrank to 320 by dragging from there to 300). Verify with `./build-aux/end-to-end-tests.sh side-panel-width` after installing
- [x] 2.6 Help (`show_help` in `src/window.rs`): in the side panel section, say that dragging the panel's edge makes it wider or narrower, and that the width is remembered. Verify by opening Help in the installed build

## 3. Full names in Files

- [x] 3.1 In `src/folder_tree.rs`, set each row's tooltip to the file or folder's name, keeping "LeafMD can't open this type of file" for greyed-out files (design decision 4). Verify on the test desktop by hovering a long name, a folder and an image, with pictures

## 4. Web links without a portal

- [x] 4.1 In `src/document.rs`, add one helper used by both web links and links to files that open in their usual app (design decision 5): inside Flatpak, ask the portal for OpenURI's `version` with a short timeout first, and if there's no answer, show "Couldn't open the link: no app is available to open it" and log "could not open link" without calling GIO. Update the comment there. Verify on the test desktop that clicking a web link no longer closes LeafMD, and on the owner's desktop that web links still open the browser (hand check, task 6.3)
- [x] 4.2 Add `build-aux/end-to-end/web-link-no-portal.sh` (links spec, "Web links open in the browser": a document whose first line is `[Example](https://example.com)`, Formatted view, click the link, then check `leafmd_on_bus` and that LeafMD's log has "could not open link"). Verify with `./build-aux/end-to-end-tests.sh web-link-no-portal` after installing, and that it fails on the build from before task 4.1

## 5. Everywhere else people read about it

- [x] 5.1 README.md: mention resizing in the side panel feature. The shortcuts table doesn't change. Verify by reading it in LeafMD
- [x] 5.2 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: mention the resizable panel where the description covers the side panel. (Release notes wait for `/ship`.) Verify with `./build-aux/sdk-check.sh`, which validates the file
- [x] 5.3 `site/index.html`: add a few words about resizing to the "Tabs and folders" box only if it keeps that box about the same length as the others (the owner evened them out on purpose). Verify by building the site with `build-aux/build-site.py` and looking at it. Left unchanged: that box is already as long as its neighbours, and resizing doesn't fit without lengthening it
- [x] 5.4 ROADMAP.md: add "**Resizable side panel:** drag the panel's edge to make it wider or narrower, and the width is remembered" under "Reading and writing comfort", ticked. Verify by reading it

## 6. Checks and trying it

- [x] 6.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh` and `openspec validate --strict resizable-side-panel`, with no failures or warnings
- [x] 6.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh` and check it ends with "46 of 46 passed"
- [x] 6.3 Ask the owner to close every LeafMD window, reopen it and try what the tests can't: how dragging the edge feels and the pointer arrows, the panel at its narrowest and widest, resizing the window with a wide panel, clicking at the start of a line next to the handle, tooltips on long names in Files, and that a web link still opens the browser

## Workflow follow-up

- `/ship` asks about the version, writes the release notes, commits, archives the change (which copies the spec changes into `openspec/specs/`) and opens the pull request.

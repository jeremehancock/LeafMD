# Tasks

## 1. Check how drops arrive

- [x] 1.1 Checked by the owner with the installed Flatpak on 2026-11-12, without a debug print (the finished feature showed the answer directly): images in home arrive with their real paths and link correctly; an image from `/tmp` arrived as a plain path LeafMD can't see, which made a broken link. Recorded in design.md, and fixed by also checking LeafMD can see the image (see 3.3).

## 2. Making the link text

- [x] 2.1 Add `src/image_link.rs` with a pure function that takes the document's path, an image's path and the portal folder, and returns the link text or the reason it can't (unsaved, outside home). Cover with unit tests: same folder, subfolder, neighbouring folder (`../Pictures/photo.png`), several levels up, spaces (`beach%20day.jpg`), brackets, `%` and `<>` in names, `[` and `]` in the description, a name with no extension, a name with two dots (`photo.final.png` gives `photo.final`), non-English letters left as they are, and a document or image under the portal folder. Verify with `cargo test`.
- [x] 2.2 Add a function that says whether a file name is an image (GIO's content type guess is `image/*`), with tests for `.png`, `.JPG`, `.svg`, `.webp`, `.md` and a name with no extension. Verify with `cargo test`.
- [x] 2.3 Add `src/image_link.rs` to the file table in docs/DEVELOPMENT.md; verify by reading it.

## 3. The drop on the raw text

- [x] 3.1 In `src/document.rs`, add a drop target for file lists on the raw text view (capture phase if task 1.1 showed the text view grabs file drops). Images go through 2.1 and are inserted at the drop point between `begin_user_action` and `end_user_action`, one per line, with the cursor after the last and focus on the view. Other files are opened with the window's `open_file_here`. If any image can't be linked, show the matching message from the spec and insert none. Verify with `cargo build` and `cargo clippy --all-targets -- -D warnings`.
- [x] 3.3 After the owner's test: check the image is a file LeafMD can see before linking (an image from `/tmp` made a broken link), and when the drop point isn't right over a character, use the end of the on-screen line at that height instead of the end of the document (links sometimes landed at the bottom). Verify with the new `image_leafmd_cant_see` test, `cargo test`, clippy, and the owner dropping beside short lines and from `/tmp` again.
- [x] 3.2 Check on the hidden desktop that ordinary typing, pasting, the formatting bar and dropping a markdown file onto the window (outside the raw text) still work as before, by running the existing end-to-end tests in 5.2. Dragging itself can't be checked by the end-to-end tests, so there's no new scenario; the owner tries drops by hand.

## 4. Where users read about it

- [x] 4.1 Help (`show_help` in `src/window.rs`): in the editing or files and links section, explain dropping an image onto the raw text to insert a link, that the path is relative to the document, and that the document must be saved and the image be in the home folder. Verify by reading the Help text in the running app.
- [x] 4.2 README.md features list, the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml` and the storefront in `site/index.html`: mention dropping an image to insert its link. Verify `./build-aux/flathub-lint.sh` passes.
- [x] 4.3 Markdown Cheatsheet: if its Links and Images section has room for a one-line tip, add "Drop an image onto the raw text to insert its link"; otherwise leave it. Verify by opening the cheatsheet. (Left as is: every cheatsheet row is markdown beside what it looks like, so a drag tip doesn't fit. Help covers it.)

## 5. Checks and installing

- [x] 5.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, all passing with no warnings.
- [x] 5.2 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, then run `./build-aux/end-to-end-tests.sh`, all scenarios passing, so the owner can try it.

## Workflow follow-up

- The owner tries dropping images in the installed Flatpak (same folder, subfolder, a name with spaces, several at once, Ctrl+Z, an unsaved document, a markdown file), then `/ship` commits and archives the change (copying the new rules into `openspec/specs/smart-typing` and `openspec/specs/documents`).
- No ROADMAP item is added or ticked here: the owner plans to clear the roadmap down to Version 1.0 and Translations once this and the code colours change are done.

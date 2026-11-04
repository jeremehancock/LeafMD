# Tasks

## 1. Plain-text rules

- [x] 1.1 Make `parse_line` in `src/format.rs` visible to the rest of LeafMD (`pub(crate)`), and check `cargo test` still passes
- [x] 1.2 Add `src/typing.rs` with `on_enter` (continue, end list or normal) and unit tests covering every scenario in the spec: `- `, `* `, `+ ` bullets, indented items, splitting an item, `2.` to `3.`, `9)` to `10)`, ticked checklist to `- [ ] `, empty bullet, numbered and checklist items ending the list, headings and plain lines being normal, the cursor before or inside the marker being normal; verify with `cargo test`
- [x] 1.3 Add `in_code_block` with tests for ` ``` ` and `~~~` fences, an indented fence, a fence with a language name (` ```bash `), and an unclosed fence; verify with `cargo test`
- [x] 1.4 Add `is_web_address` with tests for `https://example.com/docs`, `http://x.org`, surrounding spaces or a trailing new line, plain words, `https://` on its own, and an address with a space inside; verify with `cargo test`

## 2. Enter in the raw text

- [x] 2.1 Add a capture-phase key controller on the raw text editor in `src/window.rs` that applies `on_enter` for Enter and keypad Enter (no Shift, Ctrl or Alt, no selection, not in a code block), inside one user action, and lets every other key through; verify with `cargo build` and the hand-check in 5.4
- [x] 2.2 Add a comment above the controller explaining what it does and why it uses a key controller rather than watching inserted text

## 3. Pasting a link over text

- [x] 3.1 Handle the editor's `paste-clipboard` signal: with a single-line selection, read the clipboard text, make `[selection](address)` in one user action if it's a web address and the selection hasn't changed, otherwise fall back to the normal paste; verify with `cargo build` and the hand-check in 5.4

## 4. Where users read about it

- [x] 4.1 Help (`show_help` in `src/window.rs`): describe lists continuing, ending on an empty item, Shift+Enter, and pasting an address over selected text; verify in the hand-check in 5.4
- [x] 4.2 Keyboard Shortcuts (`show_shortcuts`): add Shift+Enter, "New line without a list marker"; verify in the hand-check in 5.4
- [x] 4.3 README.md: add the new behaviour to the features list, Shift+Enter to the shortcuts table, and `src/typing.rs` to the file table
- [x] 4.4 `data/io.github.jeremehancock.LeafMD.metainfo.xml`: mention lists that continue themselves and pasting links; verify with `appstreamcli validate --no-net` (or `flatpak run org.freedesktop.appstream-glib validate` if that's what's available)
- [x] 4.5 Tick the three "Smarter typing" items in ROADMAP.md

## 5. Final checks

- [x] 5.1 `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh` all pass
- [x] 5.2 `openspec validate smarter-typing --strict` passes
- [x] 5.3 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, and check it finishes without errors
- [x] 5.4 The owner tries it in the installed app: every Enter and paste scenario in the spec (including pasting an address copied from a browser), one Ctrl+Z for each, Shift+Enter, Help and Keyboard Shortcuts; plus the two loose-end checks from ROADMAP.md section 1 (a link to an already-open file brings its window forward; a deleted file drops out of Recent files)

## Workflow follow-up

- Run `/ship`: it commits to `dev`, has the owner try the installed build (task 5.4), settles the version and release notes, archives the change (adding the `smart-typing` spec to `openspec/specs/`), pushes and opens the pull request.

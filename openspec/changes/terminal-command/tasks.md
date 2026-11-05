# Tasks

## 1. Giving the terminal back

- [x] 1.1 Add `libc` as a direct dependency in `Cargo.toml`, and verify `cargo build --offline` works and `Cargo.lock` adds no new package.
- [x] 1.2 In `src/main.rs`, add the check for when to detach (stdin, stdout and stderr are all terminals, and no argument starts with `-`) as a small function taking those facts as inputs, with unit tests covering: a normal terminal start, output redirected, `--foreground`, `--help`, and a file argument. Verify with `cargo test`.
- [x] 1.3 Start the detached copy before GTK starts (same arguments, connections pointed at `/dev/null`, `setsid` in the new process), and fall back to running in place if it fails. Verify with `cargo build` and code comments explaining what and why in plain English.
- [x] 1.4 Register `--foreground` as an application option, and verify `cargo run -- --help` lists it and `cargo run -- --foreground` keeps the terminal.
- [x] 1.5 Add `cargo run -- --foreground` to `docs/DEVELOPMENT.md`, and verify the line about `src/main.rs` in its file table still describes the file.

## 2. Help

- [x] 2.1 In `show_help` in `src/window.rs`, add the "From a terminal" row to "Files and Links", with a Setup link to `https://leafmd.app/#terminal`. Verify with `cargo build`, and in step 5 that the link opens and the row fits at 360 pixels wide.

## 3. Setup instructions

- [x] 3.1 Add a "Use LeafMD from a terminal" section with `id="terminal"` to `site/index.html`, after the install section: both commands in `.command` boxes with Copy buttons, the "works with every version, works best with the latest" wording, what to do if `leafmd` isn't found, and which file to delete to undo it. Verify by building the site with `build-aux/build-site.py` and checking that Copy gives the exact command, and that a 360-pixel-wide page doesn't scroll sideways.
- [x] 3.2 Add the same section to `README.md`, and replace the line "or with `leafmd file.md`" so it points to that section. Verify the commands match the site's exactly.
- [x] 3.3 Mention the terminal command in the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml`, and verify with `build-aux/flathub-lint.sh`.
- [x] 3.4 Test each command as written with a scratch folder in place of the real one, from bash, zsh and fish, and with `sh` as dash if a machine or VM has them. Verify the script's contents are exact, and that a second run refuses and leaves the file unchanged. Report any shell that couldn't be tested.

## 4. Checks

- [x] 4.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `./build-aux/sdk-check.sh`, and verify each passes.
- [x] 4.2 Run `openspec validate terminal-command --strict` and `openspec validate --specs --strict`, and verify both pass.

## 5. Owner's test

- [x] 5.1 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, and ask the owner to close every LeafMD window and reopen it.
- [x] 5.2 With the owner, check in the installed Flatpak: `leafmd` and the long `flatpak run` command both give the prompt back on a fresh start, with the formatted view working; LeafMD survives closing the terminal; a relative path, a name with spaces, a file in `/tmp` and a missing file all behave as the spec says; `leafmd --foreground` and `leafmd --help` stay in the terminal; and starting from the app grid and the file manager is unchanged. The owner's existing `~/.local/bin/leafmd` should be replaced with the published version for this.
- [x] 5.3 With the owner, try the published commands on Ubuntu and Fedora, if a machine or VM is available, and report what wasn't tried.

## Workflow follow-up

- `/ship` handles the version, release notes, commit, push and archive. This item isn't on `ROADMAP.md`, so there's nothing to tick.
- Screenshots don't change, because no window's layout changes apart from one more row inside Help.

# Proposal

## Why

Flatpak apps can't be started by a short name: LeafMD's command is `flatpak run io.github.jeremehancock.LeafMD`, and it keeps the terminal busy until its last window closes. The README's "`leafmd file.md`" doesn't work for anyone who installed the Flatpak. People who live in a terminal should be able to type `leafmd notes.md` and carry on working.

## What Changes

- LeafMD gives the terminal back. When started from a terminal, it starts a detached copy of itself (one no longer tied to that terminal) and exits, so the prompt returns right away and closing the terminal leaves LeafMD open. This works with the long `flatpak run` command as well as the short one.
- New command line option, `--foreground`, which keeps LeafMD attached to the terminal so its messages can be seen when something goes wrong.
- Starting LeafMD any other way (app grid, file manager, a link) behaves as it does today.
- The README and leafmd.app gain instructions for a one-time setup that adds a small `leafmd` script to the computer: `/usr/local/bin` as the main route (needs the admin password), `~/.local/bin` as the route without it. The setup refuses to overwrite an existing `leafmd`.
- The instructions make clear that the script works with every version of LeafMD, and works best with the latest, which doesn't hold the terminal. Older versions open files the same way but keep the terminal busy until LeafMD closes.
- The Help window gains a short description of the terminal command, with a link to the instructions on leafmd.app.
- The README's existing line about `leafmd file.md` is corrected, because today it only works from a source build.

New settings, shortcuts and menu items: none. The only new option is `--foreground`.

**Out of scope:**

- A folder tree, and `leafmd .` opening a folder. These are separate ideas that haven't been decided.
- Installing the script from inside LeafMD. A sandboxed app reaching outside its sandbox goes against what Flatpak is for, so the setup stays a command the person runs themselves.
- Native distro packages (`.deb`, `.rpm`, AUR) and a hosted install script.
- New Flatpak permissions. There are none.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `desktop-integration`: the command line requirement changes, because `leafmd` now needs a setup step and frees the terminal. Adds `--foreground`.
- `distribution`: the storefront gains a section with the terminal command setup.
- `help-and-about`: the Help window gains the terminal command and a link to the setup instructions.

## Impact

- `src/main.rs`: deciding whether to detach, starting the detached copy and accepting `--foreground`.
- `src/window.rs`: a new row in Help, which opens the web link.
- `Cargo.toml` and `Cargo.lock`: `libc` becomes a direct dependency. It's already in the project through other libraries, and already in `vendor/`, so no new code is downloaded.
- `site/index.html`: the new section.
- `README.md`, `docs/DEVELOPMENT.md` and the app store description in `data/io.github.jeremehancock.LeafMD.metainfo.xml`.
- Anyone relying on `flatpak run` waiting until LeafMD closes (for example `EDITOR="leafmd"`) needs `--foreground`, and that only waits if no other LeafMD is already open, the same as today.

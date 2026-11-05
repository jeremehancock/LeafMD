# Design

## Context

See proposal.md for why. What the plan relies on:

- `src/main.rs` builds a GTK application with the `HANDLES_OPEN` flag and calls `app.run()`, which keeps going until the last window closes. That's why the terminal stays busy. A second `leafmd` hands its files to the copy already running, and exits.
- Inside the sandbox, `flatpak run` keeps the working folder, and stdin, stdout and stderr (the three connections a program has to its terminal) still show up as a terminal. Redirecting stdout to a file shows up as not a terminal. Both were tested on the owner's laptop on 2026-11-05.
- A process detached with `setsid` (which cuts it loose from the terminal) keeps running after `flatpak run` returns, and could still reach the session bus and see the display socket. That test used a stand-in, not the real app.
- `flatpak run --file-forwarding APP @@ ... @@` passes option-like arguments through unchanged (`--foreground` arrived as `--foreground`), and passes nothing when no arguments are given. Both were tested.
- The owner tested the `setsid` version of the script on Manjaro: no arguments, a relative path, spaces, a file in `/tmp`, a missing file and closing the terminal all behaved correctly.

## Goals / Non-Goals

**Goals:**

- One rule for when to detach, which needs no setting.
- One copy-paste setup command that behaves the same across common shells and can't overwrite anything.

**Non-Goals:**

- Reporting errors in the terminal once detached. They appear in LeafMD's window, as they do when started from the app grid.
- Changing how a second `leafmd` hands off to a running copy.

## Decisions

**LeafMD detaches itself, rather than the script doing it.** This covers the long `flatpak run` command too, and works the same on every distro, because the code runs inside the Flatpak and needs nothing from the user's system. The alternative was D-Bus activation (having the desktop's message service start the app), which is more standard for GNOME but changes how LeafMD starts everywhere. It's far more than this needs.

**When to detach.** At the very start of `main`, before GTK starts, LeafMD detaches only if both of these are true:

1. stdin, stdout and stderr are all terminals. Anything redirected means someone wants that output, or something else is managing LeafMD. This is what leaves the app grid, the file manager and `build-aux/take-screenshots.sh` (which sends output to a log) unchanged.
2. No argument starts with `-`. `--help` must print before LeafMD exits, a mistyped option must show its error, and `--foreground` needs no special case: it's simply an option, so LeafMD never detaches.

`--foreground` is registered as a normal GTK application option, so it appears in `--help` and isn't rejected as unknown.

**How it detaches.** LeafMD starts itself again using `std::process::Command` with the same arguments, stdin, stdout and stderr pointed at `/dev/null`, and `setsid` called in the new process, then exits with success. The new copy's connections aren't terminals, so it runs normally and can't detach again, which avoids needing a marker to prevent a loop. It inherits the working folder, so `leafmd notes.md` still finds the file. If starting the copy fails, LeafMD carries on in the terminal as it does today, so a failure never means LeafMD doesn't open.

**`libc` for `setsid`.** It becomes a direct dependency, for the one call. It's already in `Cargo.lock` and `vendor/` through other libraries. `Command::process_group` needs no dependency, but it doesn't fully separate LeafMD from the terminal, and `setsid` is what was tested.

**The setup commands.** The script is:

```sh
#!/bin/sh
exec flatpak run --file-forwarding io.github.jeremehancock.LeafMD @@ "$@" @@
```

The commands that create it run inside `sh -c`, so the user's shell only has to pass along one quoted string:

```sh
sudo sh -c 'set -C; printf "%s\n" "#!/bin/sh" "exec flatpak run --file-forwarding io.github.jeremehancock.LeafMD @@ \"\$@\" @@" > /usr/local/bin/leafmd && chmod 755 /usr/local/bin/leafmd'
```

```sh
sh -c 'set -C; mkdir -p ~/.local/bin; printf "%s\n" "#!/bin/sh" "exec flatpak run --file-forwarding io.github.jeremehancock.LeafMD @@ \"\$@\" @@" > ~/.local/bin/leafmd && chmod 755 ~/.local/bin/leafmd'
```

`set -C` makes the shell refuse to overwrite an existing file, so an existing `leafmd` is left alone and the user sees an error. In a test with a scratch folder from bash and zsh, the first run created the correct script, and the second refused and left the file unchanged. `/usr/local/bin` is the main route because it's usually already on `PATH`, so it should work without logging out.

**No version requirement in the wording.** The script only calls `flatpak run` with file arguments, which every version accepts. Older versions open files the same way and just keep the terminal busy. So the docs say it works with every version and works best with the latest, and neither the commands nor the wording change when a new version ships.

**The Help link is a `gtk::LinkButton` labelled "Setup", at the end of the row.** It's clearly clickable, opens the browser through the usual desktop route, and needs no new icon.

**The storefront section is always visible**, not collapsed like the Ubuntu steps, so `#terminal` works from Help's link with or without scripts. It reuses the existing `.command` box and copy button.

## Risks / Trade-offs

- [The real app might behave differently from the stand-in once detached, for example WebKit's helper processes] → The owner's test includes the formatted view, and closing the terminal, in the installed Flatpak.
- [Errors no longer reach the terminal] → `--foreground`, and the error shown in LeafMD's window.
- [`EDITOR=leafmd` or scripts expecting LeafMD to wait now move on immediately] → `--foreground`, though as today it only waits when no other LeafMD is open.
- [`cargo run` also detaches, which is awkward during development] → `docs/DEVELOPMENT.md` says to use `cargo run -- --foreground`.
- [Fish and dash weren't available to test, and `sh` is bash on this laptop but dash on Ubuntu] → `set -C` is standard for every `sh`, but tasks include trying the commands on Ubuntu and Fedora, and in fish, before release.
- [`/usr/local/bin` can be read-only on some systems] → The error is visible, and `~/.local/bin` is the documented alternative.
- [`~/.local/bin` may not be on `PATH`] → The section explains checking with `command -v leafmd`, and that logging out and back in often fixes it.
- [A `leafmd` script stays behind after uninstalling LeafMD] → The section says which file to delete.

## Migration Plan

Nothing to migrate. Anyone with the earlier `setsid` version of the script (the owner, in `~/.local/bin`) is fine: its output goes to `/dev/null`, so LeafMD sees no terminal and doesn't detach a second time. Rolling back means reverting the code. The script keeps working either way.

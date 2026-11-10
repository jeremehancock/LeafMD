# Design

## Context

`build-aux/take-screenshots.sh` already does the hardest part. It starts a real GNOME Shell with no visible screen, on a private message bus (D-Bus, the channel desktop programs use to talk to each other) that can't start any services by itself, with a private home folder and runtime folder. It then runs the installed Flatpak inside it and drives it by running small bits of JavaScript inside that hidden Shell (`org.gnome.Shell.Eval`, allowed because the Shell is started with `--unsafe-mode`). Every one of its isolation rules is there because an earlier, looser approach broke the real document portal and stopped every Flatpak app from launching.

Lessons from earlier test sessions on that hidden desktop, which this design relies on:

- Key presses work through a virtual keyboard created inside the Shell (`notify_keyval`).
- The first pointer movement triggers the hot corner (the top left corner that opens the Activities overview), so the pointer must move somewhere harmless before focusing a window, or key presses go to the overview's search box.
- The first key press in a new window is sometimes lost, so each scenario starts with a harmless warm-up press.
- When LeafMD opens a file, keyboard focus starts on the side panel button, not in the text, so typed letters go nowhere and a typed space presses that button. Scenarios click into the raw text before typing, as a person would. (Found while checking task 1.2. Shortcuts like Ctrl+S work from anywhere.)
- Capitals and `!` type correctly through the virtual keyboard: GNOME Shell presses Shift by itself for them. LeafMD's actions can also be called over the private bus, through the Flatpak's bus filter (tried with `win.view` and `source`).
- Only the first formatted view (WebKit) paints. Later ones stay blank. That's the hidden desktop, not a bug.
- File dialogs may reach the real desktop portal. Never open one.
- Stop test copies by their exact process ID, never by a name pattern that could match the owner's real LeafMD.

## Goals / Non-Goals

**Goals:**
- One shared copy of the hidden-desktop setup, used by both the screenshots and the tests.
- Adding a scenario is a small, readable file that a future proposal can write without understanding the setup.
- A full run in under five minutes.

**Non-Goals:**
- A general testing framework. This is a few helpers for one app.
- Reading what's on screen (accessibility tools or picture comparison). See Decisions.

## Decisions

### Bash, sharing the screenshot script's setup

The tests are Bash scripts, like the rest of `build-aux/`. The parts of `take-screenshots.sh` that start and stop the hidden desktop, start LeafMD, run JavaScript in the Shell, move the pointer and press keys move into `build-aux/test-desktop.sh`, which both scripts load (`source`). The safety checks at the end (real settings unchanged, real portal not restarted, Flatpak apps still start) move there too.

*Alternatives:* Python with a test library would give nicer error messages, but adds a second way of doing the same thing, and the screenshot script would still be Bash. A Rust test that drives GTK directly (inside `cargo test`) would test the code, not the installed Flatpak, and would need its own display and message bus, which is exactly what broke the portal before.

### One hidden desktop per run, a fresh home per scenario

Starting GNOME Shell takes several seconds, so it starts once per run. Each scenario gets a brand new home folder with fresh copies of its sample files, so nothing carries between scenarios (see the spec's "Each scenario starts fresh"). Between scenarios, every LeafMD window is closed from inside the Shell and its process waited for.

### Scenarios as small files

Each scenario is `build-aux/end-to-end/<name>.sh`, defining a function with the same name. The runner finds them by listing that folder, so adding a scenario needs no registration anywhere. A scenario reads like the spec it follows, for example:

```bash
lists-continue() {
    sample notes.md "- milk"
    open_leafmd notes.md
    press ctrl+2          # Raw view, so the text is where typing goes
    press ctrl+End
    press Return
    type_text "eggs"
    press ctrl+s
    expect_file notes.md "- milk
- eggs"
}
```

Helpers available to scenarios: `sample` (write a file into the fresh home), `open_leafmd` (start LeafMD with files, wait for its window), `press` (a key with optional ctrl, shift, alt), `type_text`, `close_window`, `wait_for_title`, `expect_file`, `expect_file_contains`, `expect_title`, `expect_windows <count>`, `expect_setting <section> <key> <value>` (reads LeafMD's `state.ini` in the fresh home) and `fail "<message>"`.

Typed text goes in one character at a time through the virtual keyboard. Characters that need Shift (capitals, `!`, `#`) press it the way a person would.

### Checking facts, not pictures

The checks look at things that have one right answer: the saved file, the window title, the number of windows, and `state.ini`. LeafMD shows its view, tabs and find bar on screen, but reading those would mean turning on the accessibility bus (more services on the private bus, more risk) or comparing pictures (breaks with every visual change). So scenarios turn on-screen state into a fact: save the file to see the text, switch tabs to see the title change, close the window to see what LeafMD saved.

### Running the app's actions over the bus (fallback only)

LeafMD, like every GTK application, offers its actions (New Window, Save, the views) on its message bus connection. The tests use key presses first, because that's what a person does and it tests the shortcuts too. If a key press proves unreliable for something, a scenario may call the action over the private bus instead. The first task checks this works through the Flatpak's bus filter.

### Where failures go

On a failure, the scenario's picture of the whole hidden screen (menus and dialogs only show up in a whole-screen picture), LeafMD's messages and a copy of its home folder's `state.ini` go to `target/end-to-end/<scenario>/`. `target/` is already ignored by git and already holds Cargo's build output. The folder is emptied at the start of each run so old failures don't linger.

### Where it runs in /ship

In `/ship`'s local test step: after `./build-aux/flatpak-install.sh`, before the owner is asked to try the build. Not in the checks gate before commit, because the tests need the installed build and the checks gate runs before anything is installed.

## Risks / Trade-offs

- [Timing: the hidden desktop is slower than a real one, and a check run too early fails for no real reason] → Helpers wait for a result (up to a few seconds) instead of sleeping a fixed time, the same way the screenshot script waits for windows.
- [A lost key press makes a scenario fail now and then] → The warm-up press at the start of each scenario. If a scenario is still unreliable after that, it's fixed or removed, never retried until it passes, because a test that sometimes fails teaches you to ignore it.
- [Moving the setup out of `take-screenshots.sh` could change the screenshots] → After the move, take screenshots into a temporary folder and compare them with the committed ones before going further.
- [The owner's real LeafMD or the real portal gets affected] → The isolation rules are moved as they are, not rewritten, and the end-of-run safety checks run after every run, pass or fail.
- [The run adds a few minutes to every `/ship`] → The owner can run single scenarios while working, and the full run only once per `/ship`.
- [A scenario's sample file name happens to match a file the owner has open] → Impossible: the files live in the scenario's private home, and the test LeafMD is on a different message bus, so it can't reach the owner's LeafMD at all.

## Open Questions

- Whether a scenario for the "opens scrolled partway down" roadmap bug belongs here. The screenshot script already notes the editor opens scrolled down a little. It can be added when that bug is fixed, as its proof.

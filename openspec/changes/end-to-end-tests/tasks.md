# Tasks

## 1. Shared hidden desktop

- [x] 1.1 Move the hidden-desktop setup out of `build-aux/take-screenshots.sh` into `build-aux/test-desktop.sh` (starting and stopping the private bus and GNOME Shell, `in_shell`, `wait_for_answer`, the virtual pointer and keyboard, starting LeafMD, the end-of-run safety checks), keeping every isolation rule and its explaining comment unchanged. Verify by running `build-aux/take-screenshots.sh` into a temporary folder and checking the four pictures match the committed ones, and that its safety checks pass.
- [x] 1.2 Check the two things the design relies on but hasn't proven: that a LeafMD action (for example `win.view` with `source`) can be called over the private bus through the Flatpak's bus filter, and that typing a capital letter and `!` through the virtual keyboard comes out right. Verify with a throwaway script in the session scratchpad, and write the findings into design.md if either doesn't work as expected.

## 2. The runner and its helpers

- [x] 2.1 Write `build-aux/end-to-end-tests.sh`: checks the Flatpak is installed, starts the hidden desktop once, finds the scenarios in `build-aux/end-to-end/`, runs all of them or the named ones, gives each a fresh home folder, prints "passed" or "failed" for each and the "N of M passed" total, runs the safety checks, and fails if any scenario or check failed. Verify with a scenario that always passes and one that always fails, and with a misspelt scenario name.
- [x] 2.2 Add the scenario helpers to `test-desktop.sh` (`sample`, `open_leafmd`, `press`, `type_text`, `close_window`, `wait_for_title`, `expect_file`, `expect_file_contains`, `expect_title`, `expect_windows`, `expect_setting`, `fail`), with the warm-up key press and the hot corner workaround built into `open_leafmd`, and waiting instead of fixed sleeps in every `expect_` helper. Verify through the two trial scenarios from 2.1.
- [x] 2.3 On a failure, save the whole-screen picture, LeafMD's messages and `state.ini` to `target/end-to-end/<scenario>/`, emptied at the start of each run. Verify with the always-failing scenario, then delete both trial scenarios.
- [x] 2.4 Verify the run never touches the real desktop: with the owner's LeafMD running (ask first, and don't touch it), run the trial scenarios and check the real `~/.var/app/io.github.jeremehancock.LeafMD` didn't change, the real document portal's process ID stayed the same, and `flatpak run --command=true io.github.jeremehancock.LeafMD` still works.

## 3. The twelve scenarios

Each scenario follows its spec in `openspec/specs/`, and each task is verified by the scenario passing three runs in a row, and failing when its expected result is deliberately changed (then changed back).

- [x] 3.1 `save` and `unsaved-guard` (documents spec: Saving, Unsaved changes are never lost silently).
- [x] 3.2 `lists-continue` and `lists-end` (smart-typing spec).
- [x] 3.3 `bold` and `replace-all`, each including its single Ctrl+Z (formatting-bar and find-replace specs).
- [x] 3.4 `tabs` and `already-open` (tabs and windows specs).
- [x] 3.5 `reopen-last`, `remember-view` and `window-size` (recent-files, views and documents specs).
- [x] 3.6 `reload` (documents spec: Banner when another app changes the file, Reloading the file).
- [x] 3.7 Run the full set and check it finishes in under five minutes with "12 of 12 passed", and that the safety checks pass.

## 4. Fitting it into the workflow

- [x] 4.1 Update `.claude/commands/ship.md`: in the local test step, run `./build-aux/end-to-end-tests.sh` after installing, stop and report on any failure, and leave tested behaviour out of the owner's list (saying it was checked automatically). Verify by reading the step against the spec's "Run before every hand test" requirement.
- [x] 4.2 Add a rule to `openspec/config.yaml` under `tasks`: when a change adds behaviour the end-to-end tests can check, add a scenario for it. Verify with `openspec instructions tasks --change end-to-end-tests --json` showing the new rule.
- [x] 4.3 Update docs/DEVELOPMENT.md (a "The end-to-end tests" section: what they cover, how to run all or one, what they can't test, where failures go; and the new files in the file table) and CLAUDE.md ("Trying a change" mentions the tests; `end-to-end-tests` added to the list of spec areas). Verify the documented commands run as written.
- [x] 4.4 Add "Automated end-to-end tests" to ROADMAP.md (left unticked; `/ship` ticks it when the change is archived). Help, Keyboard Shortcuts, README, the app store description and leafmd.app don't change, because nothing in the app changes.
- [x] 4.5 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh`, `openspec validate --specs --strict` and `openspec validate end-to-end-tests --strict`, and verify they all pass.
- [x] 4.6 Rebuild and install the Flatpak with `./build-aux/flatpak-install.sh`, run `./build-aux/end-to-end-tests.sh` against it, and ask the owner to run it themselves and to check their own LeafMD was undisturbed.

## 5. Second batch of scenarios (the owner asked for these after the first 12 passed)

Verified the same way as group 3: three passing runs in a row, and each one failing when its expected result is deliberately changed.

- [x] 5.1 Typing: `numbered-lists`, `checklist`, `shift-enter` and `paste-link` (smart-typing spec).
- [x] 5.2 Formatting and find: `text-styles` (italic and strikethrough) and `whole-words` (find-replace spec, Search options).
- [x] 5.3 Tabs and views: `new-tab` (Ctrl+T and Ctrl+W) and `view-per-tab` (views spec, Each tab has its own view).
- [x] 5.4 Files: `save-over-change` (documents spec, Saving over a change made elsewhere) and `open-folder` (desktop-integration spec, Folders on the command line).
- [x] 5.5 Remembered switches: `side-panel-remembered`, `distraction-free-starts-off` and `line-numbers-remembered` (side-panel, distraction-free and views specs).
- [x] 5.6 Update the scenario list in the change's spec, and the run time in docs/DEVELOPMENT.md, then run the full set and check it passes with the safety checks.

## 6. Third batch of scenarios (the owner asked for these after reviewing what the first 25 left out)

Verified the same way as group 3.

- [x] 6.1 Work-losing cases: `close-unsaved-tabs`, `file-deleted`, `discard-and-reload` and `ignore-change` (documents spec).
- [x] 6.2 Find: `replace-one` and `match-case` (find-replace spec).
- [x] 6.3 Windows: `new-window`, `empty-window-reused`, `full-screen` and `distraction-free-escape` (windows and distraction-free specs).
- [x] 6.4 `clear-recent`, `line-styles`, `style-in-word` and `open-from-tree` (recent-files, formatting-bar and folder-tree specs). Help (F1) was left out: it opens inside the window, where the tests can't see it.
- [x] 6.5 Update the scenario list in the change's spec and the run time in the docs, then run the full set three times and check it passes with the safety checks.

## Workflow follow-up

- `/ship` commits, archives the change (which creates `openspec/specs/end-to-end-tests/`) and ticks the ROADMAP item. This change touches only `build-aux/`, docs and planning files, so it needs no new version of LeafMD.

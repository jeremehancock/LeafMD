# Proposal

## Why

Every new feature means another round of trying things by hand, and the list keeps growing: tabs, the folder panel, reloading changed files, Distraction Free. The owner wants a computer to re-check the things that already work, so a hand test after a feature only needs to cover what's new and how it looks. With seven roadmap items still to build, doing this now means every one of them gets the benefit.

LeafMD already has unit tests (`cargo test`), but those only check pieces of code on their own, such as the markdown conversion. Nothing checks the real app the way a person uses it: start it, type, press keys, save, close, reopen.

## What Changes

- A new script, `build-aux/end-to-end-tests.sh`, starts the installed Flatpak on a hidden GNOME desktop (the same kind `build-aux/take-screenshots.sh` already uses), acts it out the way a person would (opening files, typing, pressing shortcuts, closing windows) and checks the results.
- It starts with 39 scenarios taken from the existing specs, chosen for things that have broken before or would be tedious to re-check by hand: saving, the unsaved-changes guard, list typing (continuing, ending, numbering, checklists, Shift+Enter, pasting links), text styles, Replace All and Whole Words Only, tabs and each tab's view, already-open files, reopening the last files, the remembered view, window size, side panel and line numbers, Distraction Free starting off and leaving it with Escape, full screen, new and reused windows, Clear Recent Files, opening a folder and a file from its tree, closing a window with several unsaved tabs, and files another app changed or deleted. (The first 12 were planned; the owner asked for two more batches once those passed.)
- The checks look at facts, not pictures: what's in the file on disk after saving, the window's title, how many windows are open, and LeafMD's saved settings file. When a scenario fails, a screenshot of the hidden desktop and LeafMD's messages are kept so the failure can be seen.
- The hidden-desktop setup moves out of `take-screenshots.sh` into one shared file both scripts use, so the careful rules that keep tests away from the real desktop live in one place.
- `/ship` runs the tests after installing the new build and before asking the owner to try it. A failing test stops there. The owner's list of things to try then leaves out what the tests already covered.
- Future proposals add scenarios for their own feature where it can be checked this way (a rule added to `openspec/config.yaml`).

Nothing changes in the app itself. There is no new keyboard shortcut, menu item or setting.

## Out of scope

- **Running on GitHub.** The tests need a GNOME Shell and the installed Flatpak, which GitHub's build machines don't have set up. They run on the owner's laptop through `/ship`. Running them on GitHub can be a later change.
- **How things look.** The tests can't judge colours, spacing, or whether the formatted view looks right. The screenshots and the owner's eye still cover that.
- **File dialogs** (Open, Save As, Open Folder). On a past test these appeared to reach the real desktop, so the tests never open one.
- **Dragging tabs** between windows. It may be possible, but it's unreliable on the hidden desktop, so it's left for later.
- **What's inside the formatted view.** Only the first formatted view paints on the hidden desktop, and its contents can't be read from outside. The markdown conversion keeps its unit tests.
- **Replacing the owner's hand test.** New features still get tried by hand before release. The tests make that shorter, not unnecessary.

## Capabilities

### New Capabilities
- `end-to-end-tests`: what the automated test run checks, how it stays away from the real desktop and the owner's own LeafMD, what it reports, and when it runs.

### Modified Capabilities
None. No behaviour of LeafMD itself changes.

## Impact

- New: `build-aux/end-to-end-tests.sh`, a shared hidden-desktop file in `build-aux/`, and one small file per scenario in `build-aux/end-to-end/`.
- Changed: `build-aux/take-screenshots.sh` uses the shared file instead of its own copy. Its pictures must come out the same.
- Changed: `.claude/commands/ship.md` (run the tests in the local test step), `openspec/config.yaml` (new rule for proposals), CLAUDE.md and docs/DEVELOPMENT.md (how to run the tests, new files in the file table).
- Needs `gnome-shell`, `dbus-daemon`, `gdbus` and the installed Flatpak, the same as the screenshot script. No new libraries or downloads.
- A full run takes a few minutes, which adds to every `/ship`.

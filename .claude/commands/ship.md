---
name: "Ship"
description: Carry a LeafMD change from written to released (checks, local test, commit, version, archive, push, PR, resync) without skipping a step
allowed-tools: Bash(git:*), Bash(gh:*), Bash(openspec:*), Bash(cargo:*), Bash(./build-aux/*), Bash(build-aux/*), Read, Edit, Grep, Glob, AskUserQuestion
category: Workflow
tags: [workflow, release, git, openspec, flatpak]
---

Carry a change the rest of the way: check it, have the owner try it on this
laptop, commit it, set the version and release notes, archive its OpenSpec
change, push `dev`, open a pull request into `main`, and tidy up after the
owner merges it.

This exists so the steps after `/opsx:apply` (or after a small fix) don't have
to be remembered. Run `/ship` at any point: it works out where things stand and
does the next right thing. Run it again later and it picks up where it left off.

**Don't follow these as a fixed sequence.** Find out the current state first,
then act. A change can arrive half-done (tasks left, committed but not tested,
a PR already merged) and starting from the top every time would redo work or
skip what matters.

Write everything for the owner in plain English, with no em dashes (see the
user's CLAUDE.md). The `openspec` command needs Node 22 or newer; if `node
--version` says older, put `~/.nvm/versions/node/v22*/bin` first on `PATH` for
those commands.

## How LeafMD is released

- **`dev`** is where work happens. **`main`** only changes through a pull
  request from `dev` that the owner merges on GitHub. GitHub enforces this:
  `main` is protected (pushes straight to it are refused, for the owner too,
  and a pull request can't be merged until the `Check · Build Flatpak` job has
  passed), and "Create a merge commit" is the only way to merge, so `dev` can
  always catch up with `main` afterwards.
- **The owner tests locally before anything is pushed:** `./build-aux/flatpak-install.sh`
  builds the working folder and installs it over their LeafMD.
- **The version** is in `Cargo.toml`, and the newest `<release>` in
  `data/io.github.jeremehancock.LeafMD.metainfo.xml` must have the same number.
  That entry's notes become the GitHub Release notes and what GNOME Software
  shows. `build-aux/release-info.py` checks the two match.
- **A `<release>` marked `type="development"`** is the next version, still
  collecting changes. Merging it into `main` publishes nothing.
- **A release** happens when a merge into `main` carries a `<release>` without
  that mark, whose `v<version>` tag doesn't exist yet. GitHub then creates the
  tag and a GitHub Release with `LeafMD-<version>.flatpak` attached, signs
  the build with LeafMD's key, and publishes leafmd.app (the storefront and
  the Flatpak repository installed copies update from) on GitHub Pages
  (`.github/workflows/ci.yml`). Before the first release, the owner's one-time
  "Publishing setup" in docs/PUBLISHING.md must be done (signing key secret, Pages,
  the leafmd.app DNS); check `gh secret list` shows `LEAFMD_GPG_PRIVATE_KEY`
  and `gh api repos/jeremehancock/LeafMD/pages` works before opening a
  release PR.
- Every push, to any branch, runs `build-aux/sdk-check.sh` on GitHub, builds
  a test `.flatpak` (kept 7 days under the run's Artifacts), and runs
  `build-aux/flathub-lint.sh` (Flathub's checker) on the recipe, the app store
  description and the finished build. Locally, `/ship` runs it on the first
  two only, since checking a finished build means a second full build. The
  one exception is the resync after a merge: `main` has just built that same
  merge commit, so the run on `dev` shows as skipped, and that's expected.

## Step 1: Find out where things stand

Run these together and read the answers before doing anything:

```bash
git branch --show-current                         # expect: dev
git status --porcelain                            # uncommitted work
git fetch origin
git log --oneline origin/main..dev                # on dev, not yet in main
git log --oneline origin/dev..dev 2>/dev/null     # committed, not pushed (no origin/dev yet = nothing pushed)
git log --oneline dev..origin/main                # main has something dev doesn't (after a merge)
openspec list --json                              # active OpenSpec changes and their progress
gh pr list --state open --head dev --json number,title,url
build-aux/release-info.py                         # version, and whether it's marked as a finished release
gh release list --limit 1 --json tagName --jq '.[0].tagName'
```

## Step 2: Do the next thing

| State | Do this |
| --- | --- |
| On `main` | Stop. Don't commit here. If there's no `dev` branch, create it from `main` (`git switch -c dev`); otherwise `git switch dev`, carrying any uncommitted work across. Then start again. |
| An active OpenSpec change has unticked tasks, other than the owner's hand-check | Stop. Tell the owner to run `/opsx:apply` first. `/ship` doesn't write features. |
| Uncommitted changes | The checks gate, then the docs gate (both below), then commit to `dev`. |
| Committed on `dev`, the owner hasn't tried this build | The local test (below). Stop until the owner confirms it works. |
| Owner confirmed, version and notes not yet decided for this work | The version step (below). |
| Version settled, OpenSpec change still active | `openspec archive <change> --yes`, tick its items in `ROADMAP.md`, run the checks gate, commit ("Archive <change>"). |
| `dev` has commits not on GitHub | `git push -u origin dev`, then wait for GitHub's build (below). |
| Pushed, GitHub's build passed, no open PR, `dev` ahead of `main` | Create the PR (below). |
| PR open | Stop. Report its URL. Merging is the owner's decision. |
| PR merged (`main` has commits `dev` doesn't) | Check the release, then resync (both below). |
| Everything lined up, no active change | Done. Report the final state. |

## The checks gate

Before any commit:

```bash
cargo build && cargo test && cargo clippy --all-targets -- -D warnings
build-aux/sdk-check.sh            # the same checks GitHub runs, in the GNOME SDK
build-aux/flathub-lint.sh         # Flathub's checker on the recipe and app store description
openspec validate --specs --strict
openspec validate <change> --strict   # for each active change
```

Everything must pass. Report failures and stop; never commit around one.
`cargo` may update `Cargo.lock` after a version change. That's expected:
commit it too.

## The docs gate

Before committing, check whether the work leaves anything describing LeafMD
out of date, and fix it in the **same** commit. Look at everything that
changed, not just the last edit:

```bash
git diff --stat origin/main..HEAD; git diff --stat
```

| A change touching... | Re-read and fix... |
| --- | --- |
| Anything a user sees or does | Help (`show_help` in `src/window.rs`), README features, the metainfo `<description>`, the storefront's text (`site/index.html`: the feature cards, hero and install cards), the matching spec in `openspec/specs/` |
| How a window looks (the header bar, formatting bar, views, menus, dialogs or styles) | The screenshots in `data/screenshots/`. Say in the report that they're now out of date. The release check below asks about them. |
| A keyboard shortcut | Keyboard Shortcuts (`show_shortcuts`), Help, README shortcuts table |
| A new or renamed source file | The file table in docs/DEVELOPMENT.md |
| `.github/workflows/`, `build-aux/`, this file, or how work is released | CLAUDE.md ("How work gets released"), docs/DEVELOPMENT.md or docs/PUBLISHING.md, this file |
| A rule an agent must follow | CLAUDE.md, since it's loaded into every session |

- If something is out of date, fix it now with a small, accurate edit.
- If the docs were updated as part of the work, confirm they cover the final
  state and move on.
- If nothing a user or developer reads changed, say so plainly ("docs gate:
  nothing user-facing changed"). Don't invent edits.

Run this gate again before creating the PR, over the whole `origin/main..dev`
difference.

## The local test

```bash
./build-aux/flatpak-install.sh
```

Then ask the owner to close every LeafMD window and reopen it, and give them a
short, concrete list of what to try: the change's spec scenarios in plain words
(real text to type, real keys to press), plus anything the tasks list says to
hand-check. Never close their running LeafMD yourself; it may have unsaved
work.

Stop and wait. Only carry on once the owner says it works. If they find a
problem, it goes back to `/opsx:apply` (or a fix), and through the gates again.
When they confirm, tick the hand-check task in the change's `tasks.md`.

## The version step

Never pick a version or release on your own. Use **AskUserQuestion**, giving
the current version and what each answer would make it.

**If the newest `<release>` is marked `type="development"`** (the next version
is still collecting changes), ask:

- **Keep collecting:** add this work's notes to that entry; merging the PR
  publishes nothing.
- **Release now:** add the notes, remove `type="development"` and set `date`
  to today; merging the PR publishes `v<version>`.

**Before releasing, check the screenshots.** They're on leafmd.app and in
every app store, so they should show the version being released. List the
interface changes since they were last updated:

```bash
last=$(git log -1 --format=%H -- data/screenshots/*.png)
git log --oneline "$last"..HEAD -- src/ data/resources/ data/icon-theme/
```

Read those commits for anything a screenshot shows (`data/screenshots/README.md`
says which view and style each one is), not just whether there are any. If
something visible changed, ask the owner with **AskUserQuestion**: retake the
screenshots now, or release with the old ones. Never release with outdated
screenshots without asking. A tooltip or menu wording usually doesn't show.

To retake, rebuild with `./build-aux/flatpak-install.sh`, then run
`build-aux/take-screenshots.sh <a temporary folder>`. Check each new picture
against the "Checking them" list in `data/screenshots/README.md` (look at it)
before copying it over the old one under the same name. If the script doesn't
work, give the owner that README's steps for taking them by hand.

**If the work doesn't change the app at all** (only `site/`, `docs/`, README,
`openspec/`, `.claude/`, CLAUDE.md, the workflow, or `build-aux/` scripts the
app's build doesn't use), it needs no version: skip this step and say so.
Merging it publishes nothing on its own. If `site/` changed, offer to
republish leafmd.app after the merge (see "After a merge"). If you're unsure
whether something affects the app, ask.

**If the newest `<release>` is already released** (its tag exists), this work
needs a new version. Ask patch, minor or major (recommend one: a fix is a
patch, a new feature is a minor; before 1.0, nothing is a major), then ask
whether to release it with this PR or keep collecting. Set the new number in
`Cargo.toml`, add a new `<release>` at the top of the metainfo `<releases>`
(with `type="development"` unless releasing now), and run `cargo build` so
`Cargo.lock` picks up the number.

**Release notes** are for people using LeafMD, not developers: one short
paragraph or a `<ul>` of what they'll notice, in plain words. Use only `<p>`,
`<ul>`/`<ol>` and `<li>` (what GNOME Software shows). Show the owner the notes
before committing.

Then run the checks gate (it confirms the numbers match) and commit
("Version <version>" or "Release notes for <version>").

## Waiting for GitHub's build

```bash
gh run list --branch dev --limit 1 --json databaseId,status,conclusion,url
gh run watch <id> --exit-status
```

If it fails, read the failing step (`gh run view <id> --log-failed`), report
it in plain words, and stop. Don't open a PR on a red build.

## Creating the PR

Never open a PR while an OpenSpec change for this work is still active: the
archive belongs in the same PR, or `main` gets the code while its specs
describe the old behaviour.

```bash
gh pr create --base main --head dev --title "<subject>" --body-file <file>
```

Write the body to a file in the scratchpad. Cover: why (the problem, not the
patch), what changed, how it was tested (the checks, the owner's local test,
GitHub's build), and **what merging does**:

- the newest `<release>` is finished and has no tag yet: merging publishes
  `v<version>` with a `.flatpak` and these release notes, and puts the new
  version on leafmd.app (the storefront and the Flatpak repository)
- otherwise: merging publishes nothing; the work waits for a later release

End the body with the attribution lines from the session's system reminder.

## After a merge

**Republish leafmd.app** (only if the merge changed `site/` without making a
release, and the owner agrees):

```bash
gh workflow run ci.yml --ref main -f republish=true
gh run list --branch main --limit 1 --json databaseId,status,conclusion,url
curl -fsS https://leafmd.app/ | grep -o 'Version [0-9.]*' | head -1
```

Wait for the run, then check the site shows the change and still names the
newest release's version. A republish never changes what installed copies
receive.

**Check the release** (only if this merge should have published one):

```bash
gh run list --branch main --limit 1 --json databaseId,status,conclusion,url
gh release view v<version> --json tagName,url,assets
curl -fsS https://leafmd.app/ | grep -o 'Version [0-9.]*' | head -1
curl -fsS https://leafmd.app/LeafMD.flatpakref | head -3
```

The release job also publishes leafmd.app. The storefront should say the new
version, and `LeafMD.flatpakref` should download. Report the release URL and
that leafmd.app is updated, or what failed. If the release worked but
publishing leafmd.app failed, say so plainly: people who already installed
LeafMD won't get the update until it's published.

**Resync `dev`:**

```bash
git switch dev && git fetch origin && git merge --ff-only origin/main && git push
git fetch origin main:main    # refresh the local main without switching to it
```

Merge `origin/main`, not `main`: the local `main` is out of date because this
workflow never switches to it. If `--ff-only` refuses, `dev` has commits `main`
doesn't, such as work committed to `dev` after the pull request was merged
(GitHub only allows merge commits, so a squash merge isn't the cause). Don't
force anything; say what you found.

## Guardrails

- **Never merge a PR.** Open it, report the URL, stop.
- **Never push work the owner hasn't tried locally**, and never archive before
  they have. Archiving rewrites `openspec/specs/`.
- **Never commit with a failing check**, and never commit to `main`.
- **Never pick a version, or decide to release, without asking.**
- **Never commit a behaviour change with out-of-date docs.**
- **Never skip the archive** to get a PR out faster. Code and specs ship
  together.
- If the state is unclear, say what's unclear and ask.

## Finish

Report what was done, what's still outstanding, and the one thing the owner
needs to do next. If everything is lined up, show the final state: branches,
the version and whether it's released, spec validation, open PRs.

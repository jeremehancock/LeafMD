# Proposal

## Why

Ubuntu doesn't come with Flatpak, so when an Ubuntu user clicks Install on leafmd.app, the downloaded `LeafMD.flatpakref` opens in a text editor and nothing explains why. Ubuntu is one of the most common desktop Linux systems, so this is many people's first experience of LeafMD. Fixing the page also showed a second gap: leafmd.app only changes when a new version is released, so a wording fix like this one would have to wait for the next release.

## What Changes

- **Help for systems without Flatpak, Ubuntu first.** On leafmd.app:
  - Under the hero's Install button, everyone sees a short line: "On Ubuntu? Set up Flatpak first." It links to a new "Setting up Flatpak on Ubuntu" box in the install section.
  - The box has the two one-time setup commands, each with a Copy button, and says to restart afterwards. It starts closed, and opens with a click.
  - Firefox on Ubuntu tells websites it's on Ubuntu (other browsers don't), so for those visitors the box is already open, and the hero line says they're on Ubuntu.
  - Everyone also gets the hint for the symptom itself: "If the file opens in a text editor, Flatpak isn't set up yet", with a link to flatpak.org's setup page for every other system.
  - All of it works with scripts turned off. Scripts only open the box on Ubuntu and add the Copy buttons.
- **Republishing leafmd.app without a new version.** A manual "Republish leafmd.app" run of the GitHub build, started from GitHub's Actions page or with one `gh` command:
  - It publishes the site from `main`, so text and design fixes go live once they're merged.
  - It never publishes an unreleased app, and never makes installed copies download a "new" copy of the same version. The app, its version, icon and screenshots all come from the newest release, exactly as published.
  - To make that possible, each release also attaches its signed Flatpak repository to the GitHub Release as an archive, and republishing reuses that archive instead of building the app again. The existing 0.1.0 release gets its archive added once, copied from what leafmd.app serves today.
- **A sharper main screenshot.** The big screenshot near the top is 1050 pixels wide, but the page gives it 1036, so browsers shrink it slightly, which blurs thin text like the header's labels. It gets room to show at its exact size on wide screens. On narrower screens it still shrinks to fit, as it must.
- **`/ship` learns that site-only work doesn't need a new version:** it merges, then republishes. Changes to the app still go through versions and releases as before.
- **A README for people who use LeafMD.** README.md keeps only what someone installing and using LeafMD needs: the screenshot and a short introduction, how to install (including the Ubuntu setup), the features, keyboard shortcuts, permissions, safety, getting help, the roadmap, the license and the AI disclosure. Everything about working on LeafMD moves, unchanged in substance, to two new files:
  - `docs/DEVELOPMENT.md`: running and building it, the checks, how changes are planned, branches and releases, and what each file does
  - `docs/PUBLISHING.md`: what a release publishes, the one-time publishing setup, republishing, and previewing leafmd.app locally

  The README links to both under "Contributing".
- **A Snap package for Ubuntu** joins ROADMAP.md's "Ideas, not decided". It would let Ubuntu users install from Ubuntu's own App Center with no setup, at the cost of a second package format to maintain.

No new keyboard shortcuts, menu items or settings. Nothing inside the app changes.

## Out of scope

- Building a Snap, AppImage or distribution packages (`.deb`, `.rpm`). The Snap is only added to the roadmap as an idea.
- Setup help for systems other than Ubuntu. They get the flatpak.org link, which covers every system. Specific boxes can be added later if another system causes trouble.
- Detecting Ubuntu in browsers other than Firefox. They don't tell websites which Linux they run on.
- Republishing older versions. Republishing always uses the newest release.
- Changing the app, its version or the 0.1.0 release notes.

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `distribution`: the storefront helps people without Flatpak set up (Ubuntu above all), and the site can be republished between releases without changing what installed copies receive.

## Impact

- `site/index.html`, `site/style.css`, `site/copy.js`: the Ubuntu line, the setup box, the text-editor hint, more than one Copy button. Possibly a small new script for spotting Ubuntu, or added to an existing one.
- `.github/workflows/ci.yml`: releases attach the repository archive. A new manual "republish" option publishes the site with the newest release's archive, skipping the build.
- `build-aux/build-site.py`: can take the version, icon and screenshots from a release tag instead of the working folder.
- The v0.1.0 GitHub Release: gets its repository archive added once. This changes a published release, so the owner confirms it first.
- README.md, and the new `docs/DEVELOPMENT.md` and `docs/PUBLISHING.md`: the split above, plus how and when to republish.
- `.claude/commands/ship.md` and CLAUDE.md: republishing, site-only work not needing a version, and pointing at the new docs files where they pointed at README sections (the docs gate's "README's file table" and "development section", and the publishing setup check).
- `.github/workflows/ci.yml`: its "secret isn't set" error points at `docs/PUBLISHING.md` instead of the README.
- ROADMAP.md: the Snap idea.

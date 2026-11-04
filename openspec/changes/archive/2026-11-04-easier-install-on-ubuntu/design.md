# Design

## Context

leafmd.app is built by `build-aux/build-site.py` from `site/`, plus the version, icon and screenshots from the working folder. It's published only by the release job in `.github/workflows/ci.yml`, which builds and signs the Flatpak repository and deploys it to GitHub Pages with the site. The repository itself isn't kept anywhere except the live site: each release builds a fresh one.

Rebuilding the app just to republish isn't an option. Every build makes a new commit, even from the same source, because builds record when they ran. Installed copies would then see an "update" to the version they already have.

Browsers: Firefox's user agent (the description a browser gives websites about itself) on Ubuntu includes "Ubuntu", for example `Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:...)`. Chrome, Chromium, Brave and others say only "Linux".

## Goals / Non-Goals

**Goals:**
- An Ubuntu user can get from "it opened in a text editor" to an installed LeafMD by reading the page alone.
- Site fixes can go live within minutes of merging, without a version.
- Installed copies only ever receive real releases.

**Non-Goals:**
- Making Flatpak setup itself easier. That's Ubuntu's side.

## Decisions

### Ubuntu help that never depends on detection
- **Hero:** under the existing "Install downloads a small file" hint, a second line: `On Ubuntu? <a href="#ubuntu">Set up Flatpak first</a>.`
- **Install section:** a `<details id="ubuntu">` box under the three install cards, titled "Setting up Flatpak on Ubuntu". It holds:
  - one sentence explaining that Ubuntu doesn't include Flatpak, so it's set up once
  - the two commands, each in the same command block as the terminal card, with its own Copy button
  - "Then restart your computer, and the Install button works."
- **The note under the cards** starts with "If the file opens in a text editor, Flatpak isn't set up yet: on Ubuntu, see above, and for other systems see flatpak.org/setup." That replaces the current "If nothing opens the file" sentence.
- **`<details>`** is the browser's own open-and-close box. It works without scripts and is keyboard-accessible.
- *Instead of:* a box that's always open. It would add a long block to every visitor's page for one system's sake.

### A small script opens the box when it's useful
`site/install-help.js` does two things:
- **On Ubuntu:** if `navigator.userAgent` contains "Ubuntu", it opens the box and changes the hero line to "You're on Ubuntu: set up Flatpak first, then click Install."
- **For everyone:** clicking the hero link opens the box before jumping to it. Without scripts, the link still scrolls to the closed box, whose title says what it is.

It's a separate file because the page's security rule only allows scripts from leafmd.app's own files, not ones written into the page.

`copy.js` already handles every `button.copy` it finds, so the two new Copy buttons need no script changes.

### Repository archives on releases, so republishing never rebuilds
- **Each release** packs its signed repository (`repo/`, after signing and `build-update-repo`) into `leafmd-repository-<version>.tar.gz` and attaches it to the GitHub Release. The release notes' "Installing" text gains one line saying the archive is what leafmd.app serves, and isn't needed for installing.
- **A republish** downloads the newest release's archive and unpacks it as `repo/`. Its commits and signatures are byte-for-byte what installed copies already have, so `flatpak update` finds nothing new.
- *Instead of:*
  - **Copying the live repository from leafmd.app:** this depends on the site being healthy, which is exactly when you might be republishing to fix it.
  - **Keeping repositories as Actions artifacts:** GitHub deletes them after 90 days at most.

### The republish run
- **How it's started:** the workflow gets a `republish` input (true or false) on its manual trigger:
  ```
  gh workflow run ci.yml --ref main -f republish=true
  ```
  On the website it's Actions, Build, Run workflow, ticking "Republish leafmd.app".
- **What runs:** the `build` and `release` jobs skip when `republish` is true. A new `republish` job runs only when `republish` is true and the branch is `main`. It runs on plain `ubuntu-latest`, with no Flatpak tools, because nothing is built.
- **Its steps:**
  1. Check out `main` with tags.
  2. Find the newest release (`gh release view --json tagName`).
  3. Download its archive and unpack it to `repo/`.
  4. Run `build-aux/build-site.py _site --repo repo --key build-aux/leafmd-signing-key.gpg --release-tag <tag>`.
  5. Upload and deploy to Pages, as the release job does.
- **Permissions:** `contents: read`, `pages: write`, `id-token: write`, in the `github-pages` environment. That environment already only allows `main`, so a republish from `dev` is refused by GitHub as well as by the job's own check.
- **If the newest release has no archive,** the job fails with a clear message rather than publishing anything partial.

### `build-site.py --release-tag`
With `--release-tag v0.1.0`, the script reads the metainfo file, the app icon and `data/screenshots/*.png` from that tag (`git show v0.1.0:<path>`) instead of the working folder. The page's text and design (`site/`) always come from the working folder, which is `main` in a republish. The release job doesn't pass it, since there the working folder is the release.

### Adding the archive to v0.1.0 once
v0.1.0 was released before archives existed. Its repository is copied once from leafmd.app, which is serving exactly what v0.1.0 published:
1. `ostree init --mode=archive` makes an empty repository.
2. `ostree remote add` points it at `https://leafmd.app/repo/` with the public key, and `ostree pull --mirror` copies every commit with its signature.
3. The live `summary`, `summary.sig` and `config` are downloaded over the top, so the archive matches the live site exactly, including the signed summary.

It's checked before uploading, by installing from it in a throwaway Flatpak installation and comparing its commit with the live one (`abc19ac8...`). Then it's added with `gh release upload v0.1.0`. This changes a published release, so the owner confirms first.

### `/ship` and site-only work
`/ship`'s version step gets a case before "this work needs a new version": if nothing in the work changes the app (only `site/`, docs, `openspec/`, `.claude/`, `build-aux/` scripts the app doesn't use, or the workflow), it needs no version. Merging it publishes nothing, and if `site/` changed, `/ship` offers to republish after the merge. If unsure whether something affects the app, it asks.

### Sharp main screenshot
The large screenshot sits in `.wrap`, whose content is at most 1036 pixels wide (1100 minus 32 pixels each side), so the 1050-pixel image is shrunk slightly. The showcase gets its own width limit of 1050 pixels plus its side padding, so on wide windows it's shown at exactly 1050. Below that it scales to fit as before. The small screenshots are shrunk a lot, so they read as thumbnails, and the viewer shows them at full size, so they're unchanged.

### The README for users, the rest under docs/
The README is what GitHub shows first, and what people reach from "Source code" on leafmd.app, so it's written for someone deciding whether to use LeafMD and how to install it.

- **README.md**, in order:
  1. title, screenshot, the early-days note
  2. a short introduction in the storefront's "simple on purpose" voice
  3. Install, with the Ubuntu setup steps and updates
  4. Using LeafMD (today's features list)
  5. Keyboard shortcuts
  6. Permissions (the `desktop-integration` spec requires this table here)
  7. Safety in the formatted view, shortened to what a user wants to know
  8. Help and feedback, Roadmap, Contributing (links to the two docs files), License, AI disclosure
- **`docs/DEVELOPMENT.md`:** Running it while developing, Building the Flatpak yourself, the checks, How changes are planned, Branches and releases, How it's put together (the file table and the full safety details), App ID.
- **`docs/PUBLISHING.md`:** What a release publishes, Publishing setup, Republishing leafmd.app, Previewing leafmd.app locally.
- The moved sections keep their wording apart from what's needed to make sense in their new place. This is a move, not a rewrite.
- *Why `docs/` and not `CONTRIBUTING.md`:* there are two separate audiences (people changing the code, and the publisher), and GitHub links `docs/` files as easily.

References that follow the move:
- CLAUDE.md: "README.md has the feature list and a table of what each source file does", and "README.md (features, shortcuts table, file table)" in the list of what user-visible changes update
- `/ship`'s docs gate rows and its publishing-setup check
- `ci.yml`'s missing-secret error

"README features" and "README shortcuts table" stay as they are.

## Risks / Trade-offs

- **A republish while `main` has site text describing unreleased features** → the site could describe something the installable version doesn't have yet. *Mitigation:* the storefront's text is updated with the feature, and features merge with their release. `/ship` only offers to republish after merging site-only work.
- **Ubuntu changes its package names** (`gnome-software-plugin-flatpak`) → the copied command would fail. *Mitigation:* the box also links to flatpak.org's Ubuntu page, which Flathub keeps current.
- **Firefox stops saying "Ubuntu"** → the box simply starts closed, the same as for other browsers.
- **The archive grows the releases page** by one file (about 1 MB) per release. Accepted.
- **Screens with high resolution** (HiDPI) still enlarge the 1050-pixel screenshot to twice its size, which softens it slightly. Fixing that means taking screenshots at double resolution, which is a separate decision about file size.

## Migration Plan

1. Merge this change. It contains no new release, so the release job doesn't run.
2. Add the archive to v0.1.0 (above).
3. Run the first republish. leafmd.app gets the new Ubuntu help, still offering 0.1.0.

Undoing: revert the site change on `main` and republish, or wait for the next release, which publishes in full.

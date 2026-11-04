# Proposal

## Why

LeafMD can't go on Flathub for now: it's too new, and Flathub's rules on AI-written code make acceptance unlikely. Today the only way to get it is to download the `.flatpak` file from a GitHub release. That file has no link back to where it came from, so anyone who installs it never gets updates and has to download each new version by hand. There's also nowhere to send people that shows what LeafMD is.

LeafMD should install like any other Flatpak: one click to install, and updates arriving by themselves in GNOME Software. It should have a proper home on the web at `leafmd.app` (the domain the owner has bought). It should also keep meeting Flathub's requirements, so moving there later stays possible.

## What Changes

- **LeafMD gets its own Flatpak repository**, hosted on GitHub Pages at `https://leafmd.app/`. `www.leafmd.app` and the GitHub address (`jeremehancock.github.io/LeafMD`) both forward there. A Flatpak repository is a set of files on a web server that Flatpak knows how to install from and check for updates, the same kind of thing Flathub is, but holding only LeafMD.
- **The repository is published by the GitHub build** whenever a release goes out (a merge into `main` carrying a finished version, the same moment a GitHub Release is made today). Nothing is published from `dev`, and nothing is published for unreleased versions.
- **One-click install:** the site offers a `LeafMD.flatpakref` file (a small "install this app from here" file). Opening it shows LeafMD in GNOME Software with an Install button. The GNOME runtime LeafMD needs comes from Flathub, which Flatpak adds by itself if it isn't set up yet.
- **A polished storefront at `https://leafmd.app/`:** a modern, single-page site for LeafMD with:
  - its icon, name and a short pitch, and a prominent Install button
  - the screenshots
  - the main features
  - every way to install (GNOME Software, the terminal, the `.flatpak` file)
  - the current version and its release notes
  - links to GitHub, issues and the license, plus the same one-line AI disclosure the About window shows

  It follows the system's light or dark style, works on a phone, loads fast, and has no tracking, cookies or outside fonts. It's shared as a preview card (picture and description) when the link is posted in chat apps.
- **Screenshots** (the roadmap's "Screenshots of the three views"): the formatted, raw and split views, plus the split view in dark style, taken of the installed LeafMD with a good sample document and following Flathub's screenshot rules. They go on the storefront, in the app store listing, and at the top of the README. With them in place, Flathub's checker no longer needs its "missing screenshots" exception.
- **App store listing:** its homepage link becomes `https://leafmd.app/`, it gets the screenshots, and it gets a light and a dark brand colour (Flathub's recommendation, used as the banner colour behind the app in app stores), matching the storefront.
- **About LeafMD gets a "Website" link** to `https://leafmd.app/`, above the existing GitHub link.
- **Updates:** once installed, new versions arrive through GNOME Software's updates and `flatpak update`, like any other Flatpak.
- **Signed updates:** everything in the repository is signed with a LeafMD signing key (a GPG key: a digital signature that proves an update really came from the owner). Flatpak refuses anything that isn't signed with it. The owner creates the key; its private half is stored as a GitHub secret, and its public half is published.
- **The `.flatpak` file on each GitHub release is rebuilt to point at the repository**, so people who install from the file also get updates from then on.
- **Only the newest version is kept** in the repository, so it stays small.
- **Flathub's checker runs on every build:** `flatpak-builder-lint` (Flathub's own tool) checks the build recipe, the app store description and the finished build, and any problem fails the build, like a clippy warning does today. The only allowed exception is written down with its reason: screenshots not being copied into the repository itself, which only Flathub's own servers do.
- **README** gets a screenshot and an "Install" section near the top for people who just want the app, separate from building it yourself.
- **ROADMAP.md:** this change finishes "Screenshots of the three views", and the "downloadable `.flatpak`" item becomes "install from LeafMD's own repository, with updates".

- **Checklists in the formatted view** show only their checkbox, where the bullet would be, instead of a bullet and a checkbox side by side.

No new keyboard shortcuts, menu items or settings. The only changes inside the app are the Website link in About and the checklist fix.

## Out of scope

- Submitting to Flathub, or writing the separate recipe Flathub would need.
- Changing the AI disclosure. The owner is happy with it as it is, even though it's less detailed than Flathub asks for.
- Anything on the site beyond one page: no blog, documentation pages, news, donations, newsletter sign-up or analytics.
- Translating the site or the screenshots into other languages.
- Builds for ARM computers (aarch64). The repository serves x86_64 only, as the GitHub build does today.
- Keeping older versions in the repository to go back to.
- Moving people over if LeafMD later joins Flathub. Flathub doesn't accept an app whose author also runs a separate Flatpak repository for it, so this repository would be retired then, and that move would be its own change.

## Capabilities

### New Capabilities
- `distribution`: how people find, get and update LeafMD: the storefront at `leafmd.app`, the one-click install, the GitHub release file, signed updates, and the Flathub checks every build must pass.

### Modified Capabilities
- `desktop-integration`: the app store listing gains screenshots, brand colours and `https://leafmd.app/` as its homepage.
- `help-and-about`: the About window gains a Website link to `https://leafmd.app/`.
- `formatted-view`: checklist lines show only their checkbox, without a bullet beside it, as on GitHub. Found while taking the screenshots, and fixed first so they show it.

## Impact

- `.github/workflows/ci.yml`: adds the Flathub checks and, on release, signs the build, builds the site, publishes both to GitHub Pages, and builds the `.flatpak` file pointing at the repository.
- New `site/` folder: the storefront's HTML, CSS and images, and the templates for the `.flatpakref` and `.flatpakrepo` files.
- New `data/screenshots/`: the screenshots and the sample document they show.
- New files in `build-aux/`: the checker script and its exceptions list, the site-building script, the local preview script, and the public signing key.
- `data/io.github.jeremehancock.LeafMD.metainfo.xml`: homepage, screenshots, brand colours.
- `src/window.rs`: the Website link in About.
- The Flatpak recipe's build commands stay the same. Published builds use the branch name `stable`. The owner's own copy from `./build-aux/flatpak-install.sh` keeps using `master`, so the two don't get mixed up.
- One-time setup by the owner: create the signing key and add it as a GitHub secret, turn on GitHub Pages with "GitHub Actions" as its source, add `leafmd.app`'s DNS records at the domain provider, and set `leafmd.app` as the Pages custom domain.
- README.md, ROADMAP.md, CLAUDE.md (the storefront joins the list of places every user-visible change updates) and `.claude/commands/ship.md` (how a release is checked) describe the new steps.

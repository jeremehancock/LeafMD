# Design

## Context

The GitHub build (`.github/workflows/ci.yml`) already runs in Flathub's build image, builds an OSTree repository (the folder of files a Flatpak repository is made of) in `repo/`, and turns it into a `.flatpak` file with `flatpak build-bundle --runtime-repo=<Flathub>`. On a release, a second job attaches that file to a GitHub Release. Nothing is signed, the file has no `--repo-url`, and the branch name is flatpak-builder's default, `master`.

The repository on GitHub is public, and GitHub Pages isn't turned on yet.

A test run of `flatpak-builder-lint` on today's build found only `metainfo-missing-screenshots` and `appstream-screenshots-not-mirrored-in-ostree`. The recipe and the metainfo file pass.

## Goals / Non-Goals

**Goals:**
- Publishing is fully automatic once the owner has done the one-time setup.
- The owner can try the whole install-and-update flow on the laptop before anything is pushed, as with every other change.
- Flathub's checker runs the same way locally and on GitHub.

**Non-Goals:**
- A second recipe in Flathub's format. The one recipe keeps building from the working folder.

## Decisions

### Host on GitHub Pages, deployed by the build, with no history kept
The release job builds a fresh repository with only the new version and deploys it with GitHub's own Pages actions (`actions/upload-pages-artifact` and `actions/deploy-pages`), replacing the whole site.

- *Why:* Flatpak only needs the newest version to install or update. It downloads that version's files and never walks back through older ones. Starting fresh each time means nothing builds up, so the "keep it small" rule takes care of itself.
- *Instead of:* committing the repository to a `gh-pages` branch. That keeps every old version in git history forever, and Flatpak repositories are thousands of small files, which git handles badly. Hosting elsewhere (a VPS, a cloud bucket) costs money and needs another login.
- *Cost:* people can't use `flatpak update --commit=` to go back to an older version. Older `.flatpak` files stay on the GitHub releases for that.

### Served at `leafmd.app`
The site is served at `https://leafmd.app/`, set as the Pages custom domain in the repository's settings. With Actions-based deploys, that setting is the whole configuration, and no `CNAME` file is needed in the site.

- **DNS at the domain provider:** four A records for `leafmd.app` pointing at GitHub Pages (`185.199.108.153`, `185.199.109.153`, `185.199.110.153`, `185.199.111.153`), the matching four AAAA records for IPv6 (`2606:50c0:8000::153` to `2606:50c0:8003::153`), and a CNAME record for `www` pointing at `jeremehancock.github.io`. GitHub then forwards `www.leafmd.app` and `jeremehancock.github.io/LeafMD` to `leafmd.app`.
- **Domain verification:** the owner verifies `leafmd.app` in their GitHub account settings (Settings, Pages, Verified domains) with one TXT record, so no other GitHub account can point a site at it.
- **HTTPS only:** every `.app` domain must use HTTPS, and browsers refuse plain `http://` for it. So the site doesn't load at all until GitHub has issued its certificate, which takes from a few minutes up to a day after DNS is set. "Enforce HTTPS" is turned on once it's ready. This is why the domain is set up before the first release, not at it.
- *Why the bare domain and not `www`:* it's the shortest address to type and say. Every address baked into installed copies (`https://leafmd.app/repo/`) uses it.

### Branch `stable` for published builds
Published builds use `flatpak-builder --default-branch=stable`, Flathub's convention. `./build-aux/flatpak-install.sh` keeps `master`.

- *Why:* if the owner installs the published LeafMD as well as a local build, Flatpak keeps them apart instead of one quietly replacing the other. `flatpak list` shows which is which, and `flatpak make-current io.github.jeremehancock.LeafMD master` (or `stable`) picks the one the app grid opens.

### A dedicated signing key, stored as GitHub secrets
The owner creates a GPG key just for LeafMD (not a personal key), with no passphrase and no expiry date, on the laptop:

- `LEAFMD_GPG_PRIVATE_KEY` (secret): the private key, ASCII-armored.
- The public key is committed as `build-aux/leafmd-signing-key.gpg`. The build reads its key ID from there, so there's no second secret to keep in step.

The build imports the key into a throwaway GPG home folder, signs with `flatpak build-sign` and `flatpak build-update-repo --gpg-sign`, and writes the public key into the `.flatpakref`, the `.flatpakrepo` and the bundle (`--gpg-keys`).

- *Why no passphrase:* the build has to sign without anyone typing. A passphrase stored as a second secret next to the key protects nothing, since anyone who could read one could read both. The key's safety comes from GitHub's secret storage and the owner's offline backup.
- *Why no expiry:* an expired key stops updates for everyone. Rotating it means every user re-adding the repository, so it should be rare and deliberate.

### The site's source lives in `site/`, and one script builds it
A new top-level `site/` folder holds the storefront's source: `index.html`, `style.css`, and the templates `LeafMD.flatpakref.in` and `leafmd.flatpakrepo.in`. `build-aux/build-site.py <out folder> [--repo <repo>] [--base-url <url>] [--key <public key>]` turns it into the finished site:

- copies `site/`, the app icon (as the page's icon and favicon) and `data/screenshots/*.png` into `<out>`
- fills in `{{version}}` and `{{release_notes}}` from the newest `<release>` in the metainfo file. Its notes are already simple HTML (`<p>`, `<ul>`, `<li>`), so they're copied over as they are
- fills in the `.flatpakref` and `.flatpakrepo` templates with the base URL and the public key (base64)
- copies the repository to `<out>/repo` when given one, and adds `.nojekyll`, which stops GitHub from treating the site as a Jekyll project and skipping files

The `.flatpakref` holds `Name=io.github.jeremehancock.LeafMD`, `Branch=stable`, `Url=https://leafmd.app/repo/`, `SuggestRemoteName=leafmd`, `RuntimeRepo=https://dl.flathub.org/repo/flathub.flatpakrepo`, `GPGKey=`, `Title` and `Homepage`. The finished layout is `/index.html`, `/style.css`, `/icon.svg`, `/screenshots/*.png`, `/LeafMD.flatpakref`, `/leafmd.flatpakrepo` and `/repo/...`.

- *Why Python, without a site generator:* it's one page. The GitHub build image already has Python, and a generator (Hugo, Astro and so on) would be one more tool to install and keep updated for a single file.
- *Why `site/` and not `build-aux/`:* it's a thing people see, worked on like the app's own interface, not a build helper.

### The storefront's design
A single page in plain HTML and CSS, styled to feel like a modern GNOME app's home page:

- **Look:** libadwaita's style. Rounded cards, generous spacing, the system's own font (`"Adwaita Sans", Cantarell, Inter, system-ui, sans-serif`), and a green accent taken from the icon's greens (`#1f8f5a` in light style, `#57e389` in dark). Light and dark come from `prefers-color-scheme`, the browser's report of the visitor's system style.
- **Sections, top to bottom:**
  1. A hero: the icon, name, summary, Install button and "Other ways to install".
  2. A large split-view screenshot. Visitors in dark style see the dark one, using a `<picture>` element so no script is needed.
  3. The other screenshots with their captions.
  4. About six feature cards, each with a short heading, one line, and a small icon drawn inline as SVG.
  5. "Install" with three cards: GNOME Software, the terminal command, and the `.flatpak` file.
  6. "What's new": the current version and its notes.
  7. A footer with GitHub, issues, the license and the AI disclosure line.
- **Features are written by hand for the site,** grouped and shorter than the metainfo file's list, because a storefront reads better as headed cards than a bullet list. CLAUDE.md's "Every user-visible change also updates" list gains the storefront, so it stays in step.
- **Scripts:** none are needed. One small inline script adds a "Copy" button next to the terminal command. Without scripts the command is still plain, selectable text, so this counts as an extra, not a requirement.
- **Nothing from elsewhere:** no web fonts, analytics or CDNs. Everything is served from `leafmd.app`. This keeps it fast and means visitors aren't tracked.
- **Link previews:** Open Graph tags (the `<meta property="og:...">` lines chat apps read to draw a preview card), with the title, the summary and `https://leafmd.app/screenshots/split.png`.
- **Accessibility:** text and the accent pass WCAG AA contrast in both styles, every screenshot has alt text, there's a visible keyboard focus ring, and headings are in order.
- The owner approves the design in the local preview before anything is committed.

### Screenshots
Four screenshots go in `data/screenshots/`: `formatted.png`, `raw.png`, `split.png` and `split-dark.png`. They show `data/screenshots/sample.md`, a realistic sample document (a short project plan with headings, a task list, a table, a code block, a quote and a link, and no outside images), written for this change.

- **Flathub's rules, followed:** a native window with its title bar, shadow and rounded corners. The window is no larger than 1000×700 (or 2000×1400 on a HiDPI screen), not maximized, with no desktop or wallpaper showing, default fonts, English, and no editing. Light style first, with dark as an extra.
- **Captions,** one sentence each with no final period:
  - formatted: "Read your document cleanly formatted"
  - raw: "Edit the raw markdown with the formatting bar"
  - split: "Watch your changes appear side by side as you type"
  - split-dark: "Follows your system's dark style"
- **Who takes them:** Claude writes the sample document and the steps, and checks each image (size, shadow and corners present, nothing else on screen). The owner presses the keys, using GNOME's screenshot tool in Window mode (Print Screen, then Window), because GNOME on Wayland doesn't let other programs capture a window without the person choosing it.
- **Where they're used:**
  - the metainfo file's `<screenshots>`, pointing at `https://leafmd.app/screenshots/<name>.png`, formatted first and marked default
  - the storefront
  - the top of the README, using the image from the repository
- **Flathub check:** the `metainfo-missing-screenshots` exception is no longer needed and isn't added.
- **Retake guide:** `data/screenshots/README.md` records everything needed to take them again the same way: the sample document, the window size, which view and style each file shows, the captions, and the exact keys to press. A retake then takes a couple of minutes and matches the first set.

### Keeping the storefront and screenshots current through `/ship`
Three additions to `.claude/commands/ship.md`:

1. **Docs gate rows.** A change to anything a user sees or does also re-reads the storefront's text in `site/index.html`. A change to how a window looks (the header bar, formatting bar, views, menus, dialogs or styles) also marks the screenshots as out of date in `/ship`'s report.
2. **Release check.** At "Release now", before the version is finished, `/ship` lists the commits since `data/screenshots/` last changed that touch the interface (`git log <last screenshot commit>..HEAD -- src/ data/resources/`, read for visible changes, not just counted). If any changed what a screenshot shows, it asks the owner, with AskUserQuestion, whether to retake them now or release with the old ones. It never releases with out-of-date screenshots without asking.
3. **Retakes.** If the owner chooses to retake, `/ship` follows `data/screenshots/README.md`, gives the owner the steps, and checks each new image against the same rules as the first set before committing it.

- *Why a question and not a block:* some visible changes are too small to show in a screenshot (a tooltip, a menu item's wording). The owner decides.
- *Why from git history:* the screenshots' last commit is a reliable record of when they were taken, so there's no separate "screenshots version" to keep in step.

### Brand colours
The metainfo file gets `<branding>` with a light and a dark colour, chosen alongside the storefront's palette. Following Flathub's guidance, they're colourful but not the icon's own greens (so the icon stands out on them), and the dark one is a darker, more muted version of the light one. The owner approves them with the storefront.

### About window link
`show_about` in `src/window.rs` gets `about.add_link("Website", "https://leafmd.app/")` before the existing GitHub link. libadwaita's built-in `website` row isn't used, because it would show the address instead of the word "Website" and sit apart from the GitHub link.

### The Flathub checker as a script, with a user exceptions file
`build-aux/flathub-lint.sh` runs `flatpak-builder-lint` from the `org.flatpak.Builder` Flatpak (Flathub's supported way to get it) on the recipe and the metainfo file, and on a repository if given its path. It passes `--user-exceptions build-aux/flathub-lint-exceptions.json`, which allows only `appstream-screenshots-not-mirrored-in-ostree`, with the reason written in the script.

- On GitHub it runs after the build, against `repo/`, so all three are checked.
- Locally, `/ship` runs it with the recipe and metainfo file only. Checking a repository means a second full build, so the owner's local check stays fast and GitHub does the full one.
- `appstream-screenshots-not-mirrored-in-ostree` is allowed permanently: it's about Flathub's servers keeping their own copy of the screenshots, which a self-hosted repository doesn't do.
- **Before the first release** (no `v...` tag on GitHub), the script also allows `screenshot-image-not-found`, `url-not-reachable` and `appstream-missing-screenshots` (the build leaves screenshots it can't download out of the app catalogue), and checks the metainfo file with `appstreamcli validate --no-net`. The checker downloads every screenshot and visits the homepage, and `leafmd.app` only goes live when the first release is published. Without this, the first release's own checks would fail, and the site could never be published. Once a tag exists, the allowance switches itself off. The checker's `appstream` command ignores the exceptions file (it only passes on `appstreamcli`'s answer), which is why that check runs `appstreamcli` directly before the first release.
- A screenshot under a new file name fails the online check until it's released, so retakes keep the same four names.
- The checker's "pedantic" notes, such as the app ID containing capital letters, aren't failures and are left alone.
- *Instead of:* putting it inside `sdk-check.sh`. That script runs inside the GNOME SDK, where `org.flatpak.Builder` isn't available.

### Local preview script
`build-aux/flatpak-repo-preview.sh` builds a signed repository on the laptop with a throwaway key, runs `build-site.py` for `http://localhost:8642/`, and serves the whole site with Python's built-in web server until stopped. The owner can open `http://localhost:8642/` in a browser, click Install, then run the script again after a version bump to see the update arrive. It prints how to remove the preview copy and its remote afterwards.

- It doubles as the way to review the storefront: the page there is exactly what `leafmd.app` will show, apart from the address.
- *Why:* "nothing is pushed until the owner has tried it" would otherwise be impossible here, because the real repository only exists after a release is merged.

## Risks / Trade-offs

- **The signing key is lost** → no more updates can be signed, and everyone has to re-add the repository with a new key. *Mitigation:* the README's setup steps tell the owner to keep an offline backup of the private key, for example in a password manager.
- **The signing key leaks** → someone could sign a fake LeafMD, but they'd also need to put it in the repository, which only the GitHub build can do. *Mitigation:* a dedicated key with no other use, kept only in GitHub secrets and the owner's backup.
- **The first release goes out before Pages is set up** → the deploy step fails and the release job shows an error, though the GitHub Release itself is still made. *Mitigation:* the release job deploys after creating the GitHub Release, and `/ship` checks the Pages setup before a release PR.
- **People who installed an older `.flatpak` (made before this change) don't get updates** → that file has no source set. *Mitigation:* the release notes and README say to install once from the new page. No version has been released yet, so this shouldn't affect anyone.
- **The domain lapses** → installed copies stop getting updates, because `https://leafmd.app/repo/` is the address they check. Someone who bought it could serve a page but couldn't push a fake LeafMD, since updates must be signed with LeafMD's key. *Mitigation:* the README's setup steps tell the owner to turn on auto-renew for `leafmd.app`.
- **Moving to a different address later** → possible without breaking installs, by forwarding the old address and adding Flatpak's "this repository has moved" note (`flatpak build-update-repo --redirect-url`) for a few months. Not needed now.
- **Screenshots in GNOME Software don't load before the first release** → the metainfo file points at `https://leafmd.app/screenshots/`, which only exists once the site is published. Only the owner's local builds are affected, for that short gap.
- **GNOME's Window screenshot doesn't include the shadow** → Flathub asks for it. *Mitigation:* Claude checks the first image before the rest are taken. If the shadow is missing, the fallback is taking them with the Screenshot portal from a small helper, which also asks the owner to pick the window.
- **The storefront's features drift from the app** → *Mitigation:* the storefront is added to CLAUDE.md's list of places every user-visible change updates, and `/ship` checks it.
- **GitHub Pages limits** (1 GB site, 100 GB a month of downloads) → LeafMD is about 3 MB, so this is roughly 30,000 installs or updates a month before it matters.
- **Installing `org.flatpak.Builder` on every build** adds about a minute. Accepted.

## Migration Plan

1. The owner creates the key and adds the secret, following the README's new "Publishing setup" steps, turns on Pages (Settings, Pages, Source: GitHub Actions), adds the DNS records, verifies the domain, and sets `leafmd.app` as the custom domain. Once GitHub's certificate is issued, the owner turns on "Enforce HTTPS".
2. This change merges with no release, so nothing is published yet.
3. The first release (0.1.0) publishes the repository and the install page.

To undo: remove the deploy steps from the release job and unpublish the Pages site. Installed copies keep working; they just stop getting updates.

## Open Questions

- Whether to add a "republish" button (a manual run of the workflow) for re-deploying the newest release, for example after rotating the key. It isn't needed for normal releases and can be added later.

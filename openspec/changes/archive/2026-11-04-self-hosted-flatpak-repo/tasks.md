# Tasks

Inside the app, only About (a Website link) and how checklists look in the formatted view change. Help and Keyboard Shortcuts stay as they are, since no feature or shortcut changes.

## 1. Flathub checker

- [x] 1.1 Add `build-aux/flathub-lint-exceptions.json` allowing only `appstream-screenshots-not-mirrored-in-ostree` for `io.github.jeremehancock.LeafMD`. Verify it's valid JSON with `python3 -m json.tool`
- [x] 1.2 Add `build-aux/flathub-lint.sh`: installs `org.flatpak.Builder` from Flathub if missing, then runs `flatpak-builder-lint` with the exceptions file on the recipe and the metainfo file, and on a repository when given its path. Its comments explain the exception. Verify it passes on the recipe, and fails if `<summary>` is emptied (then put it back). Until screenshots land in group 3, the metainfo check is expected to fail with `metainfo-missing-screenshots`
- [x] 1.3 Call `build-aux/flathub-lint.sh repo` in `ci.yml` after "Build the Flatpak", with `--gha-format` so problems show on the right lines. Verify by reading the workflow, and on the first push to `dev` that the step runs and passes
- [x] 1.4 Add the checker to `/ship`'s checks (`.claude/commands/ship.md`) and to README's "Branches and releases" section next to `sdk-check.sh`. Verify the wording matches what the script does

## 2. Website link in About

- [x] 2.1 In `show_about` (`src/window.rs`), add a "Website" link to `https://leafmd.app/` before the GitHub link, with a comment saying why it's a custom link. Verify `cargo build` and `cargo clippy --all-targets -- -D warnings` pass, and that About shows Website above GitHub after the Flatpak is rebuilt in 6.2
- [x] 2.2 Update the About description in README if it lists About's links. Verify with `grep -n "About" README.md`

## 3. Screenshots and the app store listing

- [x] 3.0 Checklists in the formatted view: show only the checkbox, where the bullet would be (comrak's task list classes and a style rule in `src/preview.html`). Found in the first screenshot and fixed before the rest. Verify the new `marks_checklist_items_but_not_plain_ones` test passes, and the retaken `formatted.png` shows no bullets beside the checkboxes

- [x] 3.1 Write `data/screenshots/sample.md`: a short, realistic project plan with headings, a task list, a table, a code block, a quote and a link, and no outside images. Verify it looks good in all three views in the installed LeafMD
- [x] 3.2 Write `data/screenshots/README.md`, the retake guide from design.md (sample document, window size, which view and style each file shows, captions, the keys to press), and give its steps to the owner. Owner: take `formatted.png`, `raw.png`, `split.png` and `split-dark.png` into `data/screenshots/`. Check the first image before the rest are taken. Verify each is a window no larger than 1000×700 (2000×1400 on HiDPI) with its title bar, shadow and rounded corners, nothing else on screen, and default fonts
- [x] 3.3 In `build-aux/flathub-lint.sh`, until a `v...` tag exists, also allow `screenshot-image-not-found`, `url-not-reachable` and `appstream-missing-screenshots` and check the metainfo file offline (design.md). Add `<screenshots>` to the metainfo file pointing at `https://leafmd.app/screenshots/<name>.png`, formatted first and marked default, with the captions from design.md. Change `<url type="homepage">` to `https://leafmd.app/`. Verify `./build-aux/flathub-lint.sh` passes with no `metainfo-missing-screenshots` exception and prints the "No release yet" note, and `appstreamcli validate --no-net` passes
- [x] 3.4 Put `data/screenshots/split.png` at the top of README, under the title. Verify it shows on the README preview on GitHub after the push

## 4. The storefront

- [x] 4.1 Add `site/index.html` and `site/style.css` with the sections, look and accessibility rules in design.md, including the copy button as the only script, Open Graph tags, and the favicon. Verify:
  - it has no requests to other websites (check the browser's network panel)
  - it works with scripts turned off
  - it fits a 360-pixel-wide window with no sideways scrolling
  - it looks right in both light and dark style
- [x] 4.2 Choose the brand colours with the storefront's palette, add `<branding>` to the metainfo file, and have the owner approve both the colours and the page design in the local preview (task 4.4). Verify the colours pass `flathub-lint.sh` and the contrast checks in design.md
- [x] 4.3 Add `site/LeafMD.flatpakref.in`, `site/leafmd.flatpakrepo.in` and `build-aux/build-site.py` as described in design.md. Verify:
  - the finished site has `Branch=stable`, the right `Url`, and a `GPGKey` that decodes back to the key file
  - it shows the version and release notes from the metainfo file
  - it has `.nojekyll`
- [x] 4.4 Add `build-aux/flatpak-repo-preview.sh`: builds with `--default-branch=stable` into a repository signed with a throwaway key, runs `build-site.py` for `http://localhost:8642/`, and serves it. Verify:
  - the storefront at `http://localhost:8642/` installs LeafMD with its Install button
  - after a local version bump (not committed), rerunning the script makes `flatpak update` offer the new version
  - finally, uninstall the preview copy and remove its remote
- [x] 4.5 Add the storefront to CLAUDE.md's "Every user-visible change also updates" list. In `.claude/commands/ship.md`, add the docs gate rows, the screenshot check at "Release now" and the retake steps from design.md ("Keeping the storefront and screenshots current through `/ship`"). Verify:
  - CLAUDE.md and the docs gate list the same places
  - the screenshot check's `git log` command, run now, lists the commits since `data/screenshots/` last changed

## 5. Publishing from the GitHub build

- [x] 5.1 Owner: create the signing key and add the `LEAFMD_GPG_PRIVATE_KEY` secret, following the steps written for README's "Publishing setup" (task 5.6). Commit the public half as `build-aux/leafmd-signing-key.gpg`. Verify with `gh secret list` that the secret exists, and with `gpg --show-keys` that the committed file is a public key only
- [x] 5.2 Owner: turn on GitHub Pages with "GitHub Actions" as its source. Verify `gh api repos/jeremehancock/LeafMD/pages` shows `"build_type": "workflow"`
- [x] 5.3 Owner: set up `leafmd.app`:
  - at the domain provider, add the four A records, four AAAA records and the `www` CNAME from design.md
  - verify the domain in GitHub account settings (a TXT record)
  - set `leafmd.app` as the Pages custom domain
  - turn on "Enforce HTTPS" once the certificate is issued

  Verify with:
  - `dig +short leafmd.app`, which shows GitHub's four addresses
  - `dig +short www.leafmd.app`, which shows `jeremehancock.github.io`
  - `gh api repos/jeremehancock/LeafMD/pages`, which shows `"cname": "leafmd.app"` and `"https_enforced": true`
- [x] 5.4 In `ci.yml`, build with `--default-branch=stable`. In the release job only, after the GitHub Release is created:
  - sign the repository with the secret
  - run `build-site.py` for `https://leafmd.app/`
  - deploy the site with `actions/upload-pages-artifact` and `actions/deploy-pages`

  Give the job only the extra permissions this needs (`pages: write`, `id-token: write`). Verify by reading the workflow, and that a push to `dev` runs none of it
- [x] 5.5 Build the release's `.flatpak` from the signed repository with `--repo-url=https://leafmd.app/repo/` and `--gpg-keys`, and change the release notes' "Installing" text to point at `https://leafmd.app/` first. Verify the preview script's bundle, built the same way, installs with the repository as its source in `flatpak remotes`
- [x] 5.6 README:
  - an "Install" section near the top: the storefront, the command, and a note that older `.flatpak` files don't update
  - a "Publishing setup" section: the key steps (including keeping an offline backup), the Pages and `leafmd.app` DNS steps, and a reminder to keep the domain on auto-renew
  - rows in the file table for `site/`, `data/screenshots/` and the new `build-aux` files

  Verify every command in it runs as written
- [x] 5.7 Update CLAUDE.md ("How work gets released") and `.claude/commands/ship.md` ("How LeafMD is released", "Check the release") to say:
  - a release also publishes the repository and the storefront
  - `/ship` checks that the Pages deployment succeeded and that `https://leafmd.app/` shows the new version

  Verify the two files agree with the workflow
- [x] 5.8 ROADMAP.md: replace "A downloadable `.flatpak` file attached to the GitHub release" with "Install from LeafMD's own Flatpak repository with one click, and get updates automatically". Leave it and "Screenshots of the three views" unticked, since `/ship` ticks both on archive. Verify with `git diff ROADMAP.md`

## 6. Check it all together

- [x] 6.1 Run these, all passing:
  - `cargo build`
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `./build-aux/sdk-check.sh`
  - `./build-aux/flathub-lint.sh`
  - `openspec validate --specs --strict`
  - `openspec validate self-hosted-flatpak-repo --strict`
- [x] 6.2 Rebuild and install the owner's copy with `./build-aux/flatpak-install.sh`. Then ask the owner to:
  - close every LeafMD window and reopen it
  - check About's Website link
  - try `./build-aux/flatpak-repo-preview.sh` with the storefront in their browser

## Workflow follow-up

- `/ship` pushes `dev`, opens the PR, and after the owner merges it, archives the change and ticks both ROADMAP items.
- The repository and storefront first go live with the first release (0.1.0). After it, check that `https://leafmd.app/` loads, and that a fresh install from it works on the owner's laptop.

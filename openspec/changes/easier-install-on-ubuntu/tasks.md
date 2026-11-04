# Tasks

Nothing inside the app changes, so Help, Keyboard Shortcuts and the app store description stay as they are.

## 1. Help for systems without Flatpak

- [x] 1.1 In `site/index.html`, add the hero line "On Ubuntu? Set up Flatpak first." linking to `#ubuntu`, the `<details id="ubuntu">` box with the two commands (each with a Copy button) and the restart note, and change the note under the install cards to start with the text-editor hint and the flatpak.org/setup link. Style the box in `site/style.css` to match the cards. Verify:
  - with scripts off, the box is closed, opens when its title is clicked, and the hero link scrolls to it
  - both Copy buttons copy exactly their command
- [x] 1.2 Add `site/install-help.js`: opens the box and changes the hero line when the browser says it runs on Ubuntu, and opens the box when the hero link is clicked. Verify by driving headless Chrome:
  - with a Firefox-on-Ubuntu user agent, the box starts open and the hero line says "You're on Ubuntu"
  - with a Chrome-on-Linux user agent, it starts closed, and the hero link opens it
- [x] 1.3 Give the large screenshot room to show at 1050 pixels on wide windows. Verify in headless Chrome at 1280 pixels wide that it renders 1050 pixels wide, and at 390 pixels that it fits with no sideways scrolling

## 2. Republishing without a new version

- [x] 2.1 Add `--release-tag` to `build-aux/build-site.py`: reads the metainfo file, icon and screenshots from that tag with `git show`, and `site/` from the working folder. Verify `build-aux/build-site.py <out> --release-tag v0.1.0` shows "Version 0.1.0" and the tag's screenshots, and that the script fails clearly for a tag that doesn't exist
- [x] 2.2 In `ci.yml`, pack the signed `repo/` into `leafmd-repository-<version>.tar.gz` on a release, and attach it to the GitHub Release, with one line in the release notes saying what it's for. Verify by reading the workflow, and that a push to `dev` doesn't create it
- [x] 2.3 In `ci.yml`, add the `republish` input and the `republish` job as in design.md, with `build` and `release` skipping when it's set. Verify by reading the workflow, and with `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
- [x] 2.4 Owner confirms, then add the archive to v0.1.0: copy the live repository from leafmd.app as in design.md, install from the copy in a throwaway Flatpak installation, check its commit is `abc19ac8...`, and `gh release upload v0.1.0` it. Verify with `gh release view v0.1.0 --json assets`

## 3. Docs, roadmap and /ship

- [x] 3.1 `.claude/commands/ship.md`: site-only work needs no version, and after merging work that changed `site/`, offer to republish and check leafmd.app afterwards. CLAUDE.md ("How work gets released"): a republish publishes `site/` from `main`, with the newest release's app. Verify the two agree with the workflow
- [x] 3.2 Split the README as in design.md: README.md for users (with the Ubuntu setup steps under Install), `docs/DEVELOPMENT.md` and `docs/PUBLISHING.md` for the rest. Add republishing and the release archive to `docs/PUBLISHING.md`, and `site/install-help.js` and `docs/` to the file table. Verify:
  - every heading in the old README is in exactly one of the three files, apart from the intentionally shortened safety section (compare the old and new heading lists)
  - every link between them works (`grep -o '](docs/[^)]*)' README.md` and the reverse links)
  - every command shown is one the workflow and scripts accept
- [x] 3.3 Point the references at the new files: CLAUDE.md (its opening line and the list of what user-visible changes update), `/ship`'s docs gate and publishing-setup check, and `ci.yml`'s missing-secret error. Verify `grep -rn "README" CLAUDE.md .claude/commands/ship.md .github/workflows/ci.yml` leaves only references that are still right
- [x] 3.4 ROADMAP.md "Ideas, not decided": add "A Snap package, so Ubuntu users can install LeafMD from Ubuntu's own App Center without setting up Flatpak". Verify with `git diff ROADMAP.md`

## 4. Check it all together

- [x] 4.1 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./build-aux/sdk-check.sh`, `./build-aux/flathub-lint.sh`, `openspec validate --specs --strict` and `openspec validate easier-install-on-ubuntu --strict`, all passing
- [x] 4.2 Build the site for `http://localhost:8642/` and ask the owner to look over the Ubuntu line, the box, the large screenshot, the new README.md and the two docs files. Rebuild and install the owner's copy with `./build-aux/flatpak-install.sh` (nothing in the app changes, but the project's checks expect it)

## Workflow follow-up

- `/ship` commits, pushes and opens the pull request. After the owner merges it, the first republish puts the Ubuntu help on leafmd.app. Check that leafmd.app still says "Version 0.1.0", and that `flatpak update` offers nothing new.

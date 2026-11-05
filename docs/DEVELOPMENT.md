# Developing LeafMD

How to run, build and check LeafMD, how changes are planned, and what each file does. For using LeafMD, see the [README](../README.md). For publishing releases and leafmd.app, see [PUBLISHING.md](PUBLISHING.md).

Built with Rust, GTK4 and libadwaita. The app itself uses no npm or Node.js (only the optional OpenSpec planning tool below does).

## Running it while developing

System libraries needed (Manjaro/Arch package names):

```
sudo pacman -S rust gtk4 libadwaita gtksourceview5 webkitgtk-6.0
```

Then, from the top folder:

```
cargo run -- path/to/file.md
```

`cargo test` runs the tests for the markdown conversion, the safety filter and the formatting buttons.

## Building the Flatpak yourself

Needs `flatpak-builder` and the GNOME 51 runtime, SDK and Rust extension:

```
sudo pacman -S flatpak-builder
flatpak install flathub org.gnome.Platform//51 org.gnome.Sdk//51 org.freedesktop.Sdk.Extension.rust-stable//26.08
```

Then build and install for your user:

```
./build-aux/flatpak-install.sh
```

The script copies the Rust libraries into `./vendor` (Flatpak builds can't download anything), builds the app, and installs it. LeafMD then appears in your app grid.

## The checks

Every push to any branch is checked and built as a Flatpak on GitHub (`.github/workflows/ci.yml`), apart from a pull request's merge commit coming back to `dev`, because `main` has just built that same commit. To run the same checks GitHub runs (clippy, the tests, and the app store and `.desktop` files), inside the GNOME SDK:

```
./build-aux/sdk-check.sh
```

On GitHub, these checks reuse the Rust libraries compiled by an earlier run, so only LeafMD's own code is compiled again. The Flatpak build never uses those files, and a release skips them altogether, so its checks and its build both start from scratch.

LeafMD isn't on Flathub, but it keeps meeting Flathub's rules. To run Flathub's own checker (`flatpak-builder-lint`) on the Flatpak recipe and the app store description:

```
./build-aux/flathub-lint.sh
```

It installs the checker (the `org.flatpak.Builder` Flatpak) the first time. GitHub also runs it on every finished build (`./build-aux/flathub-lint.sh repo`). Any problem fails, except the two in `build-aux/flathub-lint-exceptions.json`. Both are about Flathub keeping its own copies of every app's screenshots (inside its repository, and on its media server at `dl.flathub.org/media`), which nothing outside Flathub can do.

## How changes are planned

LeafMD uses [OpenSpec](https://github.com/Fission-AI/OpenSpec) to keep a written description of how the app behaves, in plain English. `openspec/specs/` has one file per area (views, documents, recent files, find and replace and so on), each listing the rules the app follows with a short example for each. New features start as a proposal in `openspec/changes/` that says what will change and why. Once a feature is built and tested, its rules are added to the specs.

[ROADMAP.md](../ROADMAP.md) is the short list of what's coming. The specs are the detailed record of what's already there.

To check the specs are well formed (needs Node 22 or newer):

```
npm install -g @fission-ai/openspec
openspec validate --specs --strict
```

## Branches and releases

Work happens on the `dev` branch. `main` only changes through a pull request from `dev`, and pushes are checked and built as a Flatpak on GitHub ([The checks](#the-checks) has the one exception). A test `.flatpak` from each build is kept for a week under that run's Artifacts.

The version number is in `Cargo.toml`, and the newest `<release>` in `data/*.metainfo.xml` has the same number and the release notes. While that entry is marked `type="development"`, it's the next version still collecting changes. When a merge into `main` carries a finished entry whose version hasn't been released yet, GitHub tags it `v<version>` and publishes a [release](https://github.com/jeremehancock/LeafMD/releases) with the `.flatpak` attached, and publishes leafmd.app. [PUBLISHING.md](PUBLISHING.md) has the details.

With Claude Code, `/ship` walks a change through all of this: the checks, trying it locally, the version and release notes, the pull request, and tidying up after the merge.

## How it's put together

| File | What it does |
|---|---|
| `src/main.rs` | Starts the app, sets up keyboard shortcuts, opens files passed on the command line |
| `src/window.rs` | The window: toolbar, formatting bar, raw editor, formatted view, find and replace, Help, cheatsheet and About, opening and saving |
| `src/file_change.rs` | Noticing when another app changes the open file: what counts as a change, what the banner says, and the banner's widgets, with tests |
| `src/find.rs` | The find and replace bar's buttons and boxes |
| `src/format.rs` | What each formatting button does to the text (bold, lists, headings, tables and so on), with tests |
| `src/typing.rs` | Help while typing: what Enter does in a list, spotting code blocks, and spotting a pasted web address, with tests |
| `src/render.rs` | Turns markdown into HTML (comrak), then removes anything unsafe (ammonia) |
| `src/preview.html` | The page the formatted view is drawn into, including its light and dark styles |
| `src/preview.js` | Double-click to jump, synced scrolling and find in the formatted view, run in a private script area the document can't reach |
| `src/state.rs` | Remembers the last view and window size in `~/.config/leafmd/state.ini` |
| `data/*.desktop` | The app menu entry: name, icon, and the file types LeafMD opens |
| `data/*.metainfo.xml` | The description, license and release notes shown in software centers |
| `build-aux/` | The Flatpak recipe, the script that builds and installs it, `sdk-check.sh` (the checks GitHub runs), `flathub-lint.sh` and its exceptions file (Flathub's checker), `release-info.py` (reads the version and release notes, and checks they match), `build-site.py` (builds leafmd.app), `flatpak-repo-preview.sh` (a private copy of leafmd.app on this computer), `screenshot-mode.sh` (GNOME's default look for taking screenshots) and `leafmd-signing-key.gpg` (the public half of LeafMD's signing key) |
| `site/` | The storefront at leafmd.app: the page, its styles, its three small scripts (the Copy buttons, the viewer that shows screenshots full size, and the Ubuntu setup help in `install-help.js`), and the templates for the install files |
| `data/screenshots/` | The screenshots used on leafmd.app, in app stores and in the README, the sample document they show, and how to take them again |
| `.github/workflows/ci.yml` | GitHub's build: checks and builds each push, and when a new version reaches `main`, signs it, publishes a GitHub Release and publishes leafmd.app. Also republishes leafmd.app on request |
| `data/resources/` | Files built into the program: the view icons, formatting bar icons, replace icon and new document icon. LeafMD always uses its own theme in `data/icon-theme/`, so icons look the same on every distro (see `src/main.rs`). LeafMD's own icons are drawn with filled shapes only, because GTK doesn't paint outlined lines in these icons |
| `openspec/specs/` | How LeafMD behaves, written as rules with examples, one folder per area |
| `openspec/changes/` | Features being planned or built, and finished ones under `archive/` |
| `openspec/config.yaml` | Guidance for writing proposals and specs in this project |
| `docs/` | This file, and [PUBLISHING.md](PUBLISHING.md) |
| `.claude/` | OpenSpec's commands for Claude Code (`/opsx:propose`, `/opsx:apply` and so on), generated by `openspec init`, and `/ship` (`.claude/commands/ship.md`), which takes a change from written to released |
| `CLAUDE.md` | How to work on LeafMD with Claude Code |
| `build.rs` | Packs `data/resources` into the program when it's compiled, and notes the Rust version for the About window |
| `data/icons/` | The app icon, plus a one-colour version GNOME uses in some places |
| `data/icon-theme/` | LeafMD's own small icon theme, for icons known by a standard name, like the window's buttons, the main menu and Find. Anything drawn here replaces Adwaita's icon of the same name, and everything else (mostly small icons GTK adds by itself) still comes from Adwaita. Icons bundled in `data/resources/` can't do this. New icons here are installed automatically, using the same filled-shapes rule |

### Safety in the formatted view

Markdown files can contain raw HTML, including scripts. LeafMD protects against that in three ways:

1. Everything unsafe (scripts, event handlers, iframes) is stripped from the HTML before it's shown.
2. Scripts inside the page are switched off in the web view as well.
3. The formatted view can never leave LeafMD's own page. Clicking a link opens it in your browser instead, and links to headings in the same document just scroll.

## App ID

`io.github.jeremehancock.LeafMD`

# LeafMD

> **Early days:** LeafMD is a very new app and is in active, early development. It works, but expect rough edges, missing features and changes from one version to the next. Bug reports and ideas are welcome in [Issues](https://github.com/jeremehancock/LeafMD/issues), and [ROADMAP.md](ROADMAP.md) shows what's coming.

A markdown viewer and editor for Linux with three views, including a formatted-only view for pure reading. Switch views with the icons in the toolbar or Ctrl+1, 2 and 3.

- **Formatted:** the rendered document, full window. Just for reading.
- **Raw:** the markdown text, full window. Type here to edit.
- **Split:** raw text on the left, formatted on the right. The formatted side updates as you type.

The raw text can always be edited in Raw and Split view, and changes are only written to the file when you save.

- **Formatting bar:** in Raw and Split view, a row of buttons above the text adds the markdown for you: bold, italic, strikethrough, inline code, links, headings, bulleted, numbered and check lists, quotes, code blocks and tables. Select some text and press a button, or press it with nothing selected to format the word or line the cursor is on. Press it again to take the formatting off.
- **Markdown cheatsheet** (Shift+F1, or the Cheatsheet button) shows the most common markdown next to what it looks like.

- In split view, **scrolling either side** scrolls the other to the same spot.
- In split view, **double-click** a word on the formatted side to select that same word in the raw text. Its line lights up briefly so you can spot it.
- **Links** to websites open in your browser. Links to other markdown or text files (like `LICENSE`) open them in a new LeafMD window, scrolled to the heading if the link names one.
- **Find and replace** (Ctrl+F, Ctrl+H, the magnifying glass button, or the main menu): highlights every match with a count like "3 of 12". It searches the raw text in Raw and Split view, and the formatted text in Formatted view. Replace one match or all of them, and undo Replace All with one Ctrl+Z. Options: match case, and whole words only.
- **Recent files:** the arrow next to **Open** lists the files you opened recently, and LeafMD reopens your last file when you start it.
- **Help** (F1) explains all of this inside the app, and **Keyboard Shortcuts** (Ctrl+?) lists every shortcut.

Built with Rust, GTK4 and libadwaita. The app itself uses no npm or Node.js (only the optional OpenSpec planning tool below does).

## Keyboard shortcuts

| Keys | Action |
|---|---|
| Ctrl+1 / Ctrl+2 / Ctrl+3 | Formatted / Raw / Split view |
| Ctrl+N | New document |
| Ctrl+O | Open |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |
| Ctrl+W or Ctrl+Q | Close (asks first if there are unsaved changes) |
| Ctrl+B / Ctrl+I | Bold / italic |
| Ctrl+Shift+X | Strikethrough |
| Ctrl+E | Inline code |
| Ctrl+K | Link |
| Ctrl+F | Find |
| Ctrl+H | Find and replace |
| Enter / Shift+Enter, or F3 / Shift+F3 | Next / previous match |
| F1 | Help |
| Shift+F1 | Markdown cheatsheet |
| Ctrl+? | Keyboard shortcuts |

You can also open a file by dropping it on the window, or with `leafmd file.md`.

## Running it while developing

System libraries needed (Manjaro/Arch package names):

```
sudo pacman -S rust gtk4 libadwaita gtksourceview5 webkitgtk-6.0
```

Then, from this folder:

```
cargo run -- path/to/file.md
```

`cargo test` runs the tests for the markdown conversion, the safety filter and the formatting buttons.

## How changes are planned

LeafMD uses [OpenSpec](https://github.com/Fission-AI/OpenSpec) to keep a written description of how the app behaves, in plain English. `openspec/specs/` has one file per area (views, documents, recent files, find and replace and so on), each listing the rules the app follows with a short example for each. New features start as a proposal in `openspec/changes/` that says what will change and why. Once a feature is built and tested, its rules are added to the specs.

[ROADMAP.md](ROADMAP.md) is the short list of what's coming. The specs are the detailed record of what's already there.

To check the specs are well formed (needs Node 22 or newer):

```
npm install -g @fission-ai/openspec
openspec validate --specs --strict
```

## Installing as a Flatpak

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

### Permissions

| Permission | Why |
|---|---|
| Your home folder | So images and links next to a markdown file show up in the formatted view |
| Network | So images hosted online (like README badges) load |
| Graphics (dri) | The formatted view draws faster with the graphics card |

## Branches and releases

Work happens on the `dev` branch. `main` only changes through a pull request from `dev`, and every push to any branch is checked and built as a Flatpak on GitHub (`.github/workflows/ci.yml`). A test `.flatpak` from each build is kept for a week under that run's Artifacts.

To run the same checks GitHub runs (clippy, the tests, and the app store and `.desktop` files), inside the GNOME SDK:

```
./build-aux/sdk-check.sh
```

The version number is in `Cargo.toml`, and the newest `<release>` in `data/*.metainfo.xml` has the same number and the release notes. While that entry is marked `type="development"`, it's the next version still collecting changes. When a merge into `main` carries a finished entry whose version hasn't been released yet, GitHub tags it `v<version>` and publishes a [release](https://github.com/jeremehancock/LeafMD/releases) with the `.flatpak` attached.

With Claude Code, `/ship` walks a change through all of this: the checks, trying it locally, the version and release notes, the pull request, and tidying up after the merge.

## How it's put together

| File | What it does |
|---|---|
| `src/main.rs` | Starts the app, sets up keyboard shortcuts, opens files passed on the command line |
| `src/window.rs` | The window: toolbar, formatting bar, raw editor, formatted view, find and replace, Help, cheatsheet and About, opening and saving |
| `src/find.rs` | The find and replace bar's buttons and boxes |
| `src/format.rs` | What each formatting button does to the text (bold, lists, headings, tables and so on), with tests |
| `src/render.rs` | Turns markdown into HTML (comrak), then removes anything unsafe (ammonia) |
| `src/preview.html` | The page the formatted view is drawn into, including its light and dark styles |
| `src/preview.js` | Double-click to jump, synced scrolling and find in the formatted view, run in a private script area the document can't reach |
| `src/state.rs` | Remembers the last view and window size in `~/.config/leafmd/state.ini` |
| `data/*.desktop` | The app menu entry: name, icon, and the file types LeafMD opens |
| `data/*.metainfo.xml` | The description, license and release notes shown in software centers |
| `build-aux/` | The Flatpak recipe, the script that builds and installs it, `sdk-check.sh` (the checks GitHub runs) and `release-info.py` (reads the version and release notes, and checks they match) |
| `.github/workflows/ci.yml` | GitHub's build: checks and builds every push, and publishes a release when a new version reaches `main` |
| `data/resources/` | Files built into the program: the view icons and formatting bar icons (so they look the same with any icon theme). They're drawn with filled shapes only, because GTK doesn't paint outlined lines in these icons |
| `openspec/specs/` | How LeafMD behaves, written as rules with examples, one folder per area |
| `openspec/changes/` | Features being planned or built, and finished ones under `archive/` |
| `openspec/config.yaml` | Guidance for writing proposals and specs in this project |
| `.claude/` | OpenSpec's commands for Claude Code (`/opsx:propose`, `/opsx:apply` and so on), generated by `openspec init`, and `/ship` (`.claude/commands/ship.md`), which takes a change from written to released |
| `CLAUDE.md` | How to work on LeafMD with Claude Code |
| `build.rs` | Packs `data/resources` into the program when it's compiled, and notes the Rust version for the About window |
| `data/icons/` | The app icon, plus a one-colour version GNOME uses in some places |

### Safety in the formatted view

Markdown files can contain raw HTML, including scripts. LeafMD protects against that in three ways:

1. Everything unsafe (scripts, event handlers, iframes) is stripped from the HTML before it's shown.
2. Scripts inside the page are switched off in the web view as well.
3. The formatted view can never leave LeafMD's own page. Clicking a link opens it in your browser instead, and links to headings in the same document just scroll.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for what's planned next.

## App ID

`io.github.jeremehancock.LeafMD`

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

## AI Disclosure

This project was created with the help of AI.

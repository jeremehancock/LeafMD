# Developing LeafMD

How to run, build and check LeafMD, how changes are planned, and what each file does. For using LeafMD, see the [README](../README.md). For publishing releases and leafmd.app, see [PUBLISHING.md](PUBLISHING.md).

Built with Rust, GTK4 and libadwaita. The app itself uses no npm or Node.js (only the optional OpenSpec planning tool below does).

## Running it while developing

System libraries needed (Manjaro/Arch package names):

```
sudo pacman -S rust gtk4 libadwaita gtksourceview5 webkitgtk-6.0 libspelling
```

`libspelling` is the spell checker for the raw text (the same one GNOME Text Editor uses). The GNOME runtime doesn't include it, so the Flatpak recipe builds it first, from the release named in `build-aux/io.github.jeremehancock.LeafMD.json`, and the dictionaries come from the runtime. Only English dictionaries are always there; Flatpak adds other languages' dictionaries when your system is set up for those languages.

Then, from the top folder:

```
cargo run -- --foreground path/to/file.md
```

`--foreground` keeps LeafMD attached to the terminal, so its messages stay visible. Without it, LeafMD started from a terminal hands over to a detached copy of itself, and exits straight away.

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

Pushes are checked and built as a Flatpak on GitHub (`.github/workflows/ci.yml`) when the build could give a new answer. On `dev` (or any branch but `main`), a push is skipped if the files that go into the app and its checks are exactly the same as in a build that already passed on that branch: for example a push that only changes `docs/`, `openspec/`, `site/`, README.md or ROADMAP.md, or the push that brings `dev` up to `main` after a merge. On `main`, only a release is built, because a merge can't happen until `dev`'s build of the same app has passed. A skipped build counts as passing, and "Run workflow" on the Actions tab always builds. To run the same checks GitHub runs (clippy, the tests, and the app store and `.desktop` files), inside the GNOME SDK:

```
./build-aux/sdk-check.sh
```

Before the checks, it builds libspelling inside the SDK (the same release the Flatpak recipe uses, downloaded and checked against the recipe's checksum) into `target/sdk-libspelling`, and reuses that build until the release changes. On GitHub, these checks reuse the Rust libraries compiled by an earlier run, so only LeafMD's own code is compiled again. The Flatpak build never uses those files, and a release skips them altogether, so its checks and its build both start from scratch.

LeafMD isn't on Flathub, but it keeps meeting Flathub's rules. To run Flathub's own checker (`flatpak-builder-lint`) on the Flatpak recipe and the app store description:

```
./build-aux/flathub-lint.sh
```

It installs the checker (the `org.flatpak.Builder` Flatpak) the first time. GitHub also runs it on every finished build (`./build-aux/flathub-lint.sh repo`). Any problem fails, except the two in `build-aux/flathub-lint-exceptions.json`. Both are about Flathub keeping its own copies of every app's screenshots (inside its repository, and on its media server at `dl.flathub.org/media`), which nothing outside Flathub can do.

## The end-to-end tests

`cargo test` checks pieces of the code on their own. The end-to-end tests check the whole app the way a person uses it: they start the installed Flatpak, type, press shortcuts, save, close and reopen, then check the results. Rebuild first, because they test what's installed:

```
./build-aux/flatpak-install.sh
./build-aux/end-to-end-tests.sh              # every scenario, about five minutes
./build-aux/end-to-end-tests.sh save tabs    # only these
```

Nothing shows on screen, and any LeafMD you have open can stay open. Everything happens on a hidden GNOME desktop with its own home folder, set up by `build-aux/test-desktop.sh` (which explains how it keeps away from the real desktop). Afterwards the run checks that your real LeafMD settings didn't change, the real document portal didn't restart and Flatpak apps still start. If you use LeafMD during a run, it will warn that your saved data changed; that's expected.

Each scenario is a short file in `build-aux/end-to-end/`, named after what it checks and following one of the specs in `openspec/specs/`. To add one, copy a similar file: the helpers it can use (`sample`, `open_leafmd`, `press`, `type_text`, `click_text`, `expect_file`, `expect_title` and others) are described in `test-desktop.sh`. They check facts with one right answer: what's in a saved file, the window's title, how many windows are open, and what LeafMD saved for next time.

When a scenario fails, the output says what it found instead, and `target/end-to-end/<scenario>/` holds a picture of the hidden screen, LeafMD's messages and its saved settings.

What they can't test: how things look (colours, spacing, the formatted page), file dialogs (one could reach the real desktop, so they're never opened), and dragging tabs. Those stay hand checks. They also don't run on GitHub, only on a computer with GNOME and the Flatpak installed. `/ship` runs them before asking for a hand test.

## How changes are planned

LeafMD uses [OpenSpec](https://github.com/Fission-AI/OpenSpec) to keep a written description of how the app behaves, in plain English. `openspec/specs/` has one file per area (views, documents, recent files, find and replace and so on), each listing the rules the app follows with a short example for each. New features start as a proposal in `openspec/changes/` that says what will change and why. Once a feature is built and tested, its rules are added to the specs.

[ROADMAP.md](../ROADMAP.md) is the short list of what's coming. The specs are the detailed record of what's already there.

To check the specs are well formed (needs Node 22 or newer):

```
npm install -g @fission-ai/openspec
openspec validate --specs --strict
```

## Branches and releases

Work happens on the `dev` branch. `main` only changes through a pull request from `dev`, and pushes are checked and built as a Flatpak on GitHub when that could tell us something new ([The checks](#the-checks) says when). A test `.flatpak` from each build is kept for a week under that run's Artifacts.

The version number is in `Cargo.toml`, and the newest `<release>` in `data/*.metainfo.xml` has the same number and the release notes. While that entry is marked `type="development"`, it's the next version still collecting changes. When a merge into `main` carries a finished entry whose version hasn't been released yet, GitHub tags it `v<version>` and publishes a [release](https://github.com/jeremehancock/LeafMD/releases) with the `.flatpak` attached, and publishes leafmd.app. [PUBLISHING.md](PUBLISHING.md) has the details.

With Claude Code, `/ship` walks a change through all of this: the checks, trying it locally, the version and release notes, the pull request, and tidying up after the merge.

## How it's put together

| File | What it does |
|---|---|
| `src/main.rs` | Starts the app, gives the terminal back when started from one, sets up keyboard shortcuts and the app-wide settings (Line Numbers, Check Spelling, Style), opens files and folders passed on the command line |
| `src/window.rs` | The window: toolbar, formatting bar, find and replace, recent files, showing and remembering the side panel, each window's chosen folder, Help, cheatsheet and About |
| `src/side_panel.rs` | The side panel itself: its header, its pages and the switcher along its bottom for choosing one |
| `src/folder_tree.rs` | The Files page of the side panel: the folder tree, which files are greyed out, highlighting the open document, and which folders LeafMD can see, with tests |
| `src/headings.rs` | Finding a document's headings, and the Headings page of the side panel that lists them, jumps to them and highlights the section you're reading, with tests |
| `src/highlight.rs` | Colours code blocks that name their language (syntect), in six colour groups the stylesheet colours, keeping each block's line tags; added after cleaning, and kept so typing only recolours the block that changed, with tests |
| `src/image_link.rs` | Turns an image dropped onto the raw text into a markdown image link (a path relative to the document, written so spaces and brackets don't break it), says why when it can't, and decides which files are images, with tests |
| `src/word_count.rs` | Counting the words a reader would see, and the wording of the label in the document's corner ("1,240 words · 7 min read"), with tests |
| `src/raw_view.rs` | The raw text editor: GtkSourceView's editor plus LeafMD's own wavy red underline under misspelt words, drawn a little below the letters because GTK's built-in one is too small to see well |
| `src/spelling.rs` | Spell checking in the raw text: the one spell checker every document shares, picking its language (the one the user chose, else the system's, else US English), joining it to each document's text and right-click menu, and keeping Check Spelling the same in every window |
| `src/document.rs` | One open document: the raw editor, the formatted view, links, saving and reloading, and noticing changes to its file |
| `src/text_size.rs` | How big the text is and in which font: the two fonts from Preferences, each window's zoom on top of them, and the stylesheet that sets the raw text's font, with tests |
| `src/preferences.rs` | The Preferences window: the Style choice (Follow System, Light or Dark), the raw text and formatted view fonts, and their Reset buttons |
| `src/dialog.rs` | The questions about saving: a heading, some text and a row of equal buttons, laid out like gedit's |
| `src/opening.rs` | The rules for which window a file or folder opens in and where a reopened tab goes, with tests |
| `src/file_change.rs` | Noticing when another app changes the open file: what counts as a change, what the banner says, and the banner's widgets, with tests |
| `src/find.rs` | The find and replace bar's buttons and boxes |
| `src/format.rs` | What each formatting button does to the text (bold, lists, headings, tables and so on), with tests |
| `src/language_suggestions.rs` | The list of language names that pops up after typing ``` and a letter in the raw text (GtkSourceView's suggestion pop-up) |
| `src/typing.rs` | Help while typing: what Enter does in a list, spotting code blocks, spotting a pasted web address, and which code block languages to suggest, with tests |
| `src/render.rs` | Turns markdown into HTML (comrak), then removes anything unsafe (ammonia) and puts the coloured code blocks in (highlight.rs), for the formatted view and for printing and exporting. Also holds the markdown features LeafMD understands, shared with the headings and word count |
| `src/preview.html` | The empty page the formatted view is drawn into. Its styles are placed into it from `preview.css` |
| `src/preview.css` | The formatted view's styles, light and dark, plus the rules for paper. Shared with printed pages, PDFs and exported web pages, so they all look the same |
| `src/export.rs` | Print, Export as PDF and Export as HTML: the document as a standalone page with no scripts (pictures packed in for the web page), printed or turned into a PDF from a hidden copy of the formatted view, with tests |
| `src/preview.js` | Double-click to jump, synced scrolling, find, and reporting the selected text for the word count in the formatted view, run in a private script area the document can't reach |
| `src/state.rs` | Remembers the last view, window size, recent files, tabs, whether the side panel is open and on which page, and the style and fonts chosen in Preferences in `~/.config/leafmd/state.ini`, and decides which tabs to save and reopen, with tests |
| `data/*.desktop` | The app menu entry: name, icon, and the file types LeafMD opens |
| `data/*.metainfo.xml` | The description, license and release notes shown in software centers |
| `build-aux/` | The Flatpak recipe, the script that builds and installs it, `sdk-check.sh` (the checks GitHub runs), `flathub-lint.sh` and its exceptions file (Flathub's checker), `release-info.py` (reads the version and release notes, and checks they match), `build-site.py` (builds leafmd.app), `flatpak-repo-preview.sh` (a private copy of leafmd.app on this computer), `take-screenshots.sh` and `screenshot-settings-portal.py` (take the screenshots on a hidden desktop), `test-desktop.sh` (that hidden desktop, shared with the end-to-end tests), `end-to-end-tests.sh` and the scenarios in `end-to-end/` (the end-to-end tests), `screenshot-mode.sh` (GNOME's default look for taking screenshots by hand) and `leafmd-signing-key.gpg` (the public half of LeafMD's signing key) |
| `site/` | The storefront at leafmd.app: the page, its styles, its four small scripts (the Copy buttons, the viewer that shows screenshots full size, the Ubuntu setup help in `install-help.js`, and `nav.js`, which highlights the top bar's link for the section on screen), and the templates for the install files |
| `data/screenshots/` | The screenshots used on leafmd.app, in app stores and in the README, the sample document they show, and how to take them again |
| `.github/workflows/ci.yml` | GitHub's build: checks and builds pushes that change the app (skipping ones that only touch docs, specs or the site), and when a new version reaches `main`, signs it, publishes a GitHub Release and publishes leafmd.app. Also republishes leafmd.app on request |
| `data/resources/` | Files built into the program: the view icons, formatting bar icons, replace icon, new document icon and the side panel's icons, and in `styles/` the raw text's colours (`leafmd.xml` and `leafmd-dark.xml`: GNOME's Adwaita editor colours with quieter line numbers and current line, found through `src/main.rs`). LeafMD always uses its own theme in `data/icon-theme/`, so icons look the same on every distro (see `src/main.rs`). LeafMD's own icons are drawn with filled shapes only, because GTK doesn't paint outlined lines in these icons |
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

Coloured code blocks are the one thing added after cleaning, for speed (cleaning thousands of coloured labels was slow). That's safe because LeafMD writes that HTML itself in `src/highlight.rs`, from code it has escaped, and the only tags it writes are `<span class="hl-...">`. Until then each block is a placeholder made of U+FDD0, a character Unicode reserves for programs' own use, and any U+FDD0 in a document is replaced before rendering, so a document can't fake one. `src/render.rs` has a test for this.

Printing and exporting use the same cleaned HTML. Exported web pages carry no scripts at all, not even LeafMD's own, and the hidden page used for printing has scripts switched off and can't go anywhere either.

## App ID

`io.github.jeremehancock.LeafMD`

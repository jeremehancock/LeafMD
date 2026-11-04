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
- **Recent files:** the arrow next to **Open** lists the files you opened recently, and LeafMD reopens your last file when you start it.
- **Help** (F1) explains all of this inside the app, and **Keyboard Shortcuts** (Ctrl+?) lists every shortcut.

Built with Rust, GTK4 and libadwaita. No npm.

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

## How it's put together

| File | What it does |
|---|---|
| `src/main.rs` | Starts the app, sets up keyboard shortcuts, opens files passed on the command line |
| `src/window.rs` | The window: toolbar, formatting bar, raw editor, formatted view, Help, cheatsheet and About, opening and saving |
| `src/format.rs` | What each formatting button does to the text (bold, lists, headings, tables and so on), with tests |
| `src/render.rs` | Turns markdown into HTML (comrak), then removes anything unsafe (ammonia) |
| `src/preview.html` | The page the formatted view is drawn into, including its light and dark styles |
| `src/preview.js` | Double-click to jump and synced scrolling, run in a private script area the document can't reach |
| `src/state.rs` | Remembers the last view and window size in `~/.config/leafmd/state.ini` |
| `data/*.desktop` | The app menu entry: name, icon, and the file types LeafMD opens |
| `data/*.metainfo.xml` | The description, license and release notes shown in software centers |
| `build-aux/` | The Flatpak recipe and the script that builds and installs it |
| `data/resources/` | Files built into the program: the view icons and formatting bar icons (so they look the same with any icon theme). They're drawn with filled shapes only, because GTK doesn't paint outlined lines in these icons |
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

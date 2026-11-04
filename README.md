# LeafMD

A markdown viewer for Linux that switches between three views, with editing when you want it.

- **Formatted:** the rendered document, full window
- **Raw:** the markdown text, full window
- **Split:** raw text on the left, formatted on the right

Files open read-only. Turn on **Edit** to type in the raw text. While editing is on, the Edit button turns into a filled **Editing** button and a bar appears across the top of the raw text, both in your system's accent colour. If you're in the formatted view when you turn editing on, LeafMD switches to split view so you can see both.

- **Double-click** anything in the formatted view to turn on editing with the cursor on that line in the raw text.
- In split view, **scrolling either side** scrolls the other to the same spot.
- Typing while editing is off shows a reminder with a button to turn it on.

Built with Rust, GTK4 and libadwaita. No npm.

## Keyboard shortcuts

| Keys | Action |
|---|---|
| Ctrl+1 / Ctrl+2 / Ctrl+3 | Formatted / Raw / Split view |
| Ctrl+E | Turn editing on or off |
| Ctrl+N | New document |
| Ctrl+O | Open |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |
| Ctrl+W or Ctrl+Q | Close (asks first if there are unsaved changes) |

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

`cargo test` runs the tests for the markdown conversion and the safety filter.

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
| `src/window.rs` | The window: toolbar, raw editor, formatted view, edit mode, opening and saving |
| `src/render.rs` | Turns markdown into HTML (comrak), then removes anything unsafe (ammonia) |
| `src/preview.html` | The page the formatted view is drawn into, including its light and dark styles |
| `src/preview.js` | Double-click to jump and synced scrolling, run in a private script area the document can't reach |
| `src/state.rs` | Remembers the last view and window size in `~/.config/leafmd/state.ini` |
| `data/*.desktop` | The app menu entry: name, icon, and the file types LeafMD opens |
| `data/*.metainfo.xml` | The description, license and release notes shown in software centers |
| `build-aux/` | The Flatpak recipe and the script that builds and installs it |
| `data/icons/` | The app icon, plus a one-colour version GNOME uses in some places |

### Safety in the formatted view

Markdown files can contain raw HTML, including scripts. LeafMD protects against that in three ways:

1. Everything unsafe (scripts, event handlers, iframes) is stripped from the HTML before it's shown.
2. Scripts inside the page are switched off in the web view as well.
3. The formatted view can never leave LeafMD's own page. Clicking a link opens it in your browser instead, and links to headings in the same document just scroll.

## App ID

`io.github.jeremehancock.LeafMD`

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

## AI Disclosure

This project was created with the help of AI.

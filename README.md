# LeafMD

![LeafMD in split view: the raw markdown of a community garden plan on the left, and the same plan formatted on the right](data/screenshots/split.png)

> **Early days:** LeafMD is a very new app and is in active, early development. It works, but expect rough edges, missing features and changes from one version to the next. Bug reports and ideas are welcome in [Issues](https://github.com/jeremehancock/LeafMD/issues), and [ROADMAP.md](ROADMAP.md) shows what's coming.

A markdown editor for Linux that's simple on purpose. Every feature is easy to find and obvious to use, so you can focus on your writing instead of learning the app.

## Install

Go to **[leafmd.app](https://leafmd.app/)** and click **Install**, then open the small file it downloads. GNOME Software (or another app store that supports Flatpak) opens with LeafMD ready to install. Or, in a terminal:

```
flatpak install https://leafmd.app/LeafMD.flatpakref
```

Either way, updates arrive through your app store (or `flatpak update`) like any other app. Everything is signed with LeafMD's own key, so updates can only come from its developer.

LeafMD needs Linux with [Flatpak](https://flatpak.org/setup/) on an x86_64 computer. The GNOME parts it uses come from Flathub, which is added for you if it's missing. If the downloaded file opens in a text editor, Flatpak isn't set up yet.

### On Ubuntu

Ubuntu doesn't include Flatpak, so it needs setting up once:

```
sudo apt install flatpak gnome-software-plugin-flatpak
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
```

Then restart your computer, and the Install button works. The first command also adds the Software app, which is what opens LeafMD's install file. For other systems, see [flatpak.org/setup](https://flatpak.org/setup/).

### As a file

Each [release](https://github.com/jeremehancock/LeafMD/releases) also has a `LeafMD-<version>.flatpak` file. Open it with your app store, and it updates from leafmd.app afterwards.

## Using LeafMD

LeafMD has three views. Switch between them with the icons in the header bar, or Ctrl+1, 2 and 3.

- **Formatted:** the rendered document, full window. Just for reading.
- **Raw:** the markdown text, full window. Type here to edit.
- **Split:** raw text on the left, formatted on the right. The formatted side updates as you type.

The raw text can always be edited in Raw and Split view, and changes are only written to the file when you save.

- **Formatting bar:** in Raw and Split view, a row of buttons above the text adds the markdown for you: bold, italic, strikethrough, inline code, links, headings, bulleted, numbered and check lists, quotes, code blocks and tables. Select some text and press a button, or press it with nothing selected to format the word or line the cursor is on. Press it again to take the formatting off.
- **Markdown cheatsheet** (Shift+F1, or the Cheatsheet button) shows the most common markdown next to what it looks like.
- **Lists continue as you type:** press Enter in a list and the next bullet, number or unticked checkbox is added for you. Enter on an empty item ends the list, and Shift+Enter starts a plain line instead.
- **Paste a link over text:** select some text and paste a web address to turn it into `[text](address)`.

- In split view, **scrolling either side** scrolls the other to the same spot.
- In split view, **double-click** a word on the formatted side to select that same word in the raw text. Its line lights up briefly so you can spot it.
- **Links** to websites open in your browser. Links to other markdown or text files (like `LICENSE`) open them in a new LeafMD window, scrolled to the heading if the link names one.
- **Find and replace** (Ctrl+F, Ctrl+H, the magnifying glass button, or the main menu): highlights every match with a count like "3 of 12". It searches the raw text in Raw and Split view, and the formatted text in Formatted view. Replace one match or all of them, and undo Replace All with one Ctrl+Z. Options: match case, and whole words only.
- **Recent files:** the arrow next to **Open** lists the files you opened recently, and LeafMD reopens your last file when you start it. **Clear Recent Files** empties the list and forgets the last file too.
- **Close Document** (in the main menu) closes the file but keeps the window open, and LeafMD starts with no document next time.
- **Help** (F1) explains all of this inside the app, and **Keyboard Shortcuts** (Ctrl+?) lists every shortcut.

## Keyboard shortcuts

| Keys | Action |
|---|---|
| Ctrl+1 / Ctrl+2 / Ctrl+3 | Formatted / Raw / Split view |
| Ctrl+N | New document |
| Ctrl+O | Open |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |
| Ctrl+W or Ctrl+Q | Close window (asks first if there are unsaved changes) |
| Ctrl+B / Ctrl+I | Bold / italic |
| Ctrl+Shift+X | Strikethrough |
| Ctrl+E | Inline code |
| Ctrl+K | Link |
| Shift+Enter (in the raw text) | New line without a list marker |
| Ctrl+F | Find |
| Ctrl+H | Find and replace |
| Enter / Shift+Enter, or F3 / Shift+F3 | Next / previous match |
| F1 | Help |
| Shift+F1 | Markdown cheatsheet |
| Ctrl+? | Keyboard shortcuts |

You can also open a file by dropping it on the window, or with `leafmd file.md`.

## Permissions

| Permission | Why |
|---|---|
| Your home folder | So images and links next to a markdown file show up in the formatted view |
| Network | So images hosted online (like README badges) load |
| Graphics (dri) | The formatted view draws faster with the graphics card |

## Safety

Markdown files can contain web page code, including scripts. LeafMD never runs them: anything unsafe is removed before a document is shown, scripts are switched off in the formatted view as well, and links open in your browser instead of inside LeafMD. The details are in [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md#safety-in-the-formatted-view).

## Help and feedback

Found a problem, or have an idea? Open an [issue](https://github.com/jeremehancock/LeafMD/issues). [ROADMAP.md](ROADMAP.md) shows what's planned next.

## Contributing

- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md): running and building LeafMD, the checks, how changes are planned, and what each file does
- [docs/PUBLISHING.md](docs/PUBLISHING.md): how releases and leafmd.app are published

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

## AI Disclosure

This project was created with the help of AI.

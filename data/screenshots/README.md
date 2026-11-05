# LeafMD's screenshots

These four pictures appear in the app store listing (GNOME Software and other
app stores, through the metainfo file), on [leafmd.app](https://leafmd.app/)
and at the top of the README.

| File | View | Style | Caption |
|---|---|---|---|
| `formatted.png` | Formatted (Ctrl+1) | Light | Read your document cleanly formatted |
| `raw.png` | Raw (Ctrl+2) | Light | Edit markdown with the formatting bar |
| `split.png` | Split (Ctrl+3) | Light | Watch your changes appear side by side as you type |
| `split-dark.png` | Split (Ctrl+3) | Dark | Follows your system's dark style |

Every one shows `sample.md`, the community garden plan in this folder.

## When to take them again

Whenever a release changes something they show: the header bar, the
formatting bar, a view, or the styles. `/ship` checks for this before each
release and asks.

Keep the same four file names. The app store description points at them on
leafmd.app, and Flathub's checker fails on a screenshot it can't download, so
a new name would fail until it's released.

## The rules they follow

These are Flathub's screenshot rules, so the pictures look right in app
stores:

- A real window with its title bar, shadow and rounded corners, taken on
  Linux. No cropping, editing, added text or arrows.
- The window is 1000 by 700 (no larger), and not maximized. Nothing else on
  screen shows: no desktop, wallpaper or other windows.
- GNOME's default settings: accent colour, fonts and title bar buttons.
  `build-aux/screenshot-mode.sh` sets these for you and puts yours back
  afterwards.
- Light style first. Dark is an extra, never the only one.

## Taking them

1. Close every LeafMD window.
2. Rebuild so the pictures show the current code:
   `./build-aux/flatpak-install.sh`
3. Switch to screenshot mode:
   `build-aux/screenshot-mode.sh light`
4. Open the sample:
   `flatpak run io.github.jeremehancock.LeafMD data/screenshots/sample.md`
5. For each light picture:
   1. Press the view's keys: Ctrl+1, Ctrl+2 or Ctrl+3 (the numbers, not the
      letter I, which is Italic). Leave both sides scrolled to the top.
      Then click once in the document, so no button is left with a blue
      focus outline from the key press: in Raw and Split, just before the "O"
      in "Our plan" on line 3 (the text cursor sits there, with nothing
      selected, and line 3 lights up instead of line 1); in Formatted, on a
      blank part of the page.
   2. Move the mouse onto the header bar's empty space, beside the file name,
      and leave it there, because scrollbars stay visible while the mouse
      moves over the document. Wait a slow count of three, until the right
      edge is clear of any scrollbar. If one shows up during the capture,
      wait for it to fade.
   3. Press **Print Screen**, choose **Window** at the bottom of the
      screenshot bar, click the LeafMD window, then the round capture button.
      GNOME saves it in `~/Pictures/Screenshots/`.
6. Close LeafMD (if it asks, don't save: the sample should stay as it is),
   run `build-aux/screenshot-mode.sh dark`, open the sample
   again, and take the split view the same way.
7. Close LeafMD and put your own settings back:
   `build-aux/screenshot-mode.sh off`
8. Rename the four pictures as in the table above and move them into this
   folder (or tell Claude, which picks the newest four from
   `~/Pictures/Screenshots/`).

## Checking them

Before they're committed, each picture is checked for:

- the window at 1000 by 700 (the picture itself is a little bigger, because
  of the shadow), or double that on a HiDPI screen
- the shadow and rounded corners present, and nothing else in the picture
- the right view, style and document
- no blue focus outline on a button, and no scrollbar showing

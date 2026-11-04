# LeafMD Roadmap

What's planned for LeafMD, in the order it will probably be done. LeafMD aims to stay a simple, dependable markdown viewer and editor, so every feature here should feel obvious without needing a setting.

Items are ticked off as they're finished. The order can change.

## 1. Loose ends

- [ ] Check that clicking a link to a file that's already open brings its window to the front
- [ ] Check that files that no longer exist are left out of the Recent files list

## 2. Find and replace

- [ ] **Find** (Ctrl+F): a search bar that highlights every match in the raw text, with next and previous (Enter and Shift+Enter, or F3 and Shift+F3), and a count like "3 of 12"
- [ ] **Find in the formatted view:** the same search bar works when only the formatted view is showing
- [ ] **Replace** (Ctrl+H): replace one match or all of them, undone in one step with Ctrl+Z
- [ ] Options kept to a minimum: match case, and whole words only

## 3. Smarter typing

- [ ] **Lists continue on Enter:** pressing Enter at the end of a list item starts the next one (`- `, `2. `, `- [ ] `), keeping the same indent
- [ ] **Lists end on an empty item:** pressing Enter on an empty item removes its marker and ends the list
- [ ] **Paste a link over text:** pasting a web address while text is selected turns the text into `[text](address)`

## 4. Reading comfort

- [ ] **Reload when the file changes:** if another app changes the open file, LeafMD shows the new version. If there are unsaved changes too, it asks which to keep instead of overwriting anything
- [ ] **Zoom:** Ctrl+plus, Ctrl+minus and Ctrl+0 make the text bigger, smaller or back to normal in both views, and the size is remembered
- [ ] **Word count:** the number of words (and reading time) shown somewhere unobtrusive

## 5. Sharing

- [ ] **Print** (Ctrl+P): prints the formatted document
- [ ] **Export to PDF:** saves the formatted document as a PDF
- [ ] **Export to HTML:** saves a single, self-contained web page with LeafMD's styles, which opens in any browser

## 6. First public release

- [ ] Screenshots of the three views for the README and the app store listing
- [ ] Release notes for version 0.1.0 listing what's included
- [ ] Publish the code on GitHub
- [ ] A downloadable `.flatpak` file attached to the GitHub release, so people can install LeafMD without building it

## Later

- [ ] Submit LeafMD to Flathub, so it can be installed from GNOME Software and other app stores
- [ ] Translations into other languages

## Ideas, not decided

These might fit LeafMD, or might add more than they're worth. Each one needs a decision before it moves up.

- Spell checking in the raw text
- A headings menu for jumping to a section of a long document
- A distraction-free full screen mode (F11)
- Coloured syntax highlighting inside code blocks in the formatted view
- Remembering where you were scrolled to in each file
- Dropping an image onto the raw text inserts a markdown image link

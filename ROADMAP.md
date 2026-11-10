# LeafMD Roadmap

What's planned for LeafMD, in the order it will probably be done. LeafMD aims to stay a simple, dependable markdown viewer and editor, so every feature here should feel obvious without needing a setting.

Items are ticked off as they're finished. The order can change.

Each feature is planned in detail with [OpenSpec](https://github.com/Fission-AI/OpenSpec) before it's built: the plan goes in `openspec/changes/`, and once it's finished, how the feature behaves is recorded in `openspec/specs/`.

## 1. Loose ends

- [x] Check that clicking a link to a file that's already open brings its window to the front
- [x] Check that files that no longer exist are left out of the Recent files list
- [x] Fix documents sometimes opening scrolled partway down, instead of at the top

## 2. Find and replace

- [x] **Find** (Ctrl+F): a search bar that highlights every match in the raw text, with next and previous (Enter and Shift+Enter, or F3 and Shift+F3), and a count like "3 of 12"
- [x] **Find in the formatted view:** the same search bar works when only the formatted view is showing
- [x] **Replace** (Ctrl+H): replace one match or all of them, undone in one step with Ctrl+Z
- [x] Options kept to a minimum: match case, and whole words only

## 3. Smarter typing

- [x] **Lists continue on Enter:** pressing Enter at the end of a list item starts the next one (`- `, `2. `, `- [ ] `), keeping the same indent
- [x] **Lists end on an empty item:** pressing Enter on an empty item removes its marker and ends the list
- [x] **Paste a link over text:** pasting a web address while text is selected turns the text into `[text](address)`

## 4. Reading and writing comfort

- [x] **Distraction-free mode:** one switch (Ctrl+Shift+D, or the main menu) that hides the header bar, tabs, formatting bar, side panel and line numbers, leaving just your document. The bars slide back in when the pointer reaches the window's edge. Each window starts with it off. Full screen (F11) is separate, and the two can be used together
- [ ] **Font and text size:** choose the font and size for the raw text and for the formatted view, in a small Preferences window. LeafMD's description promises "no settings to tune", so that wording changes with this
- [x] **Reload when the file changes:** if another app changes the open file, LeafMD shows the new version. If there are unsaved changes too, it asks which to keep instead of overwriting anything
- [ ] **Zoom:** Ctrl+plus, Ctrl+minus and Ctrl+0 make the text bigger, smaller or back to normal in both views, for a quick change without opening Preferences
- [ ] **Word count:** the number of words (and reading time) shown somewhere unobtrusive
- [x] **Tabs:** several documents in one window, with links and opened files getting a tab of their own, tabs that can be dragged between windows, and all of them reopening next time
- [x] **Side panel with a folder tree** (F9): a panel on the left showing the folder you're working in, for moving between documents. `leafmd .` opens a folder in it from a terminal, and Open Folder… chooses one. It's closed until you open it, then stays open until you close it, and it's built so later features can add pages to it

## 5. Sharing

- [ ] **Print** (Ctrl+P): prints the formatted document
- [ ] **Export to PDF:** saves the formatted document as a PDF
- [ ] **Export to HTML:** saves a single, self-contained web page with LeafMD's styles, which opens in any browser

## 6. First public release

- [x] Screenshots of the three views for the README and the app store listing
- [x] Release notes for version 0.1.0 listing what's included
- [x] Publish the code on GitHub
- [x] Install from LeafMD's own Flatpak repository at [leafmd.app](https://leafmd.app/) with one click, and get updates automatically

## Behind the scenes

- [x] **Automated end-to-end tests:** a script that tries the installed app the way a person would (typing, shortcuts, saving, tabs, reopening) and checks the results, so every release is checked the same way and hand testing can focus on what's new

## Later

- [ ] Submit LeafMD to Flathub, so it can be installed from GNOME Software and other app stores
- [ ] Translations into other languages

## Ideas, not decided

These might fit LeafMD, or might add more than they're worth. Each one needs a decision before it moves up.

- Spell checking in the raw text
- A headings menu for jumping to a section of a long document
- Coloured syntax highlighting inside code blocks in the formatted view
- Remembering where you were scrolled to in each file
- Dropping an image onto the raw text inserts a markdown image link
- A Snap package, so Ubuntu users can install LeafMD from Ubuntu's own App Center without setting up Flatpak. It would be a second package format to build and test for every release

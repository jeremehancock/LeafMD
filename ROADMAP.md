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
- [x] **Font and text size:** choose the font and size for the raw text and for the formatted view, in a small Preferences window. LeafMD's description used to promise "no settings to tune", and now says it's great out of the box with its few options kept out of the way
- [x] **Reload when the file changes:** if another app changes the open file, LeafMD shows the new version. If there are unsaved changes too, it asks which to keep instead of overwriting anything
- [x] **Zoom:** Ctrl+plus, Ctrl+minus and Ctrl+0 make the text bigger, smaller or back to normal in both views, for a quick change without opening Preferences. Planned for the same release as Font and text size, since both change the same text size
- [x] **Headings in the side panel:** a Headings page in the side panel listing the document's headings, indented to match their level. Clicking one jumps to that section. It's the panel's second page, so it also adds a way to switch between Files and Headings
- [x] **Word count:** the number of words (and reading time) shown somewhere unobtrusive. Planned for the same release as Headings, since both keep track of what's in the document as you type
- [x] **Tabs:** several documents in one window, with links and opened files getting a tab of their own, tabs that can be dragged between windows, and all of them reopening next time
- [x] **Side panel with a folder tree** (F9): a panel on the left showing the folder you're working in, for moving between documents. `leafmd .` opens a folder in it from a terminal, and Open Folder… chooses one. It's closed until you open it, then stays open until you close it, and it's built so later features can add pages to it
- [x] **Resizable side panel:** drag the panel's edge to make it wider or narrower, and the width is remembered. Hovering a name in Files shows it in full
- [x] **Spell checking:** misspelt words in the raw text are underlined as you type, skipping code and web addresses. Right-click for suggestions, Add to Dictionary or Ignore. Check Spelling in the main menu turns it off everywhere, and the language follows your system unless you pick another

## 5. Sharing

These three are planned as one release, since they all start from the formatted document and Print and Export to PDF share most of their work.

- [x] **Print** (Ctrl+P): prints the formatted document
- [x] **Export to PDF:** saves the formatted document as a PDF
- [x] **Export to HTML:** saves a single, self-contained web page with LeafMD's styles, which opens in any browser

## 6. First public release

- [x] Screenshots of the three views for the README and the app store listing
- [x] Release notes for version 0.1.0 listing what's included
- [x] Publish the code on GitHub
- [x] Install from LeafMD's own Flatpak repository at [leafmd.app](https://leafmd.app/) with one click, and get updates automatically

## Behind the scenes

- [x] **Automated end-to-end tests:** a script that tries the installed app the way a person would (typing, shortcuts, saving, tabs, reopening) and checks the results, so every release is checked the same way and hand testing can focus on what's new

## Small fixes

Small things to tidy up, none of which change how LeafMD works.

- [x] **Credits:** list libspelling, the library behind spell checking, in About's "Built With" section alongside the other libraries
- [ ] **Spell checking gaps:** look into three cases libspelling misses: the word the cursor is on isn't underlined, the very last word of a file isn't checked, and a misspelling in the first few words of some files can stay un-underlined
- [ ] **Screenshot script:** move the script's virtual mouse away from GNOME's top-left hot corner before it starts, so it can't open the Activities overview by accident
- [x] **Files panel with a new document:** when a new, unsaved document is selected, the Files panel keeps showing the folder of the file that was selected before, until the new document is saved
- [x] **Files panel follows the file:** the panel always shows the folder of the document you're looking at (it used to stay on a wider folder once it had moved up), with an up arrow to see the folder above. Open Folder… with a document open now opens the folder in a new window
- [x] **Greyed-out file after saving:** after saving a new document for the first time, its name in the Files panel is sometimes greyed out (as if LeafMD couldn't open it) until it's saved again
- [ ] **GitHub's build machines:** GitHub's `ubuntu-latest` is moving to Ubuntu 26. Nothing has broken, but decide whether to pin the version LeafMD builds on

## Later

- [ ] **Version 1.0:** its own milestone once 0.12.0 and 0.13.0 have settled, which also retires the "Early days" note on leafmd.app
- [ ] Translations into other languages

## Ideas, not decided

These might fit LeafMD, or might add more than they're worth. Each one needs a decision before it moves up.

- Coloured syntax highlighting inside code blocks in the formatted view
- Dropping an image onto the raw text inserts a markdown image link
- A Snap package, so Ubuntu users can install LeafMD from Ubuntu's own App Center without setting up Flatpak. It would be a second package format to build and test for every release

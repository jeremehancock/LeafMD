# Design

## Context

See `proposal.md` for why, and the specs for what the user sees.

Today `src/window.rs` has one `Window` struct holding two kinds of thing together:

- **Things a window has one of:** the header bar, the bottom bar, the toast messages (the small pop-up notices), the recent files menu, the formatting bar, and the actions behind the menus and shortcuts.
- **Things a document has one of:** the text buffer, the raw text view, the formatted view (a WebKit web view), the file, the file watcher, the "last known version" used to spot changes, render timers, scroll syncing, and the search context.

`src/main.rs` keeps a list of windows and decides where a file opens. A file is identified by its `file://` address, and that address is only set once the file has finished loading.

Limits that shape the design:

- The wide header bar has to fit in 640 pixels, and it already needs about 614. So nothing new can go in it (a comment in `Window::new` records this).
- Moving the formatting bar, find bar and bottom bar buttons at the 640 pixel line relies on a window-level "breakpoint" (a libadwaita rule that changes widgets when the window crosses a width).
- GUI tests on a private message bus have broken the real document portal on this laptop before, so most of this has to be checked by hand. Rules that can be pulled out into plain functions get unit tests.

## Goals / Non-Goals

**Goals:**
- Split per-document state from per-window state, so each tab behaves exactly as a window's document does today.
- Do that split first, with no visible change, so it can be tried on its own before tabs are added on top.
- Keep single-document use looking and behaving as it does now, apart from the shortcuts the proposal lists.

**Non-Goals:**
- Reworking find, the banner logic, rendering or saving beyond moving them to where they now belong.
- Splitting up the rest of `window.rs` (Help, the cheatsheet, About), which isn't needed for this.
- Copying gedit's pixel-exact look. See the first decision.

## Decisions

### Use libadwaita's TabView and TabBar, not a copy of gedit's tabs

`adw::TabView` manages the set of open pages, and `adw::TabBar` draws the row of tabs. Together they give us, for free: hiding the bar with one tab (`autohide`, which is the same behaviour as gedit's automatic mode), shrinking and scrolling tabs in narrow windows, dragging tabs, moving them between windows, and the right-click menu.

*What this means for the look:* same layout and behaviour as gedit (a row of tabs under the header bar, filling the width, with close buttons), drawn in the Adwaita style, like GNOME's other apps. It won't be a pixel match for gedit, because gedit 50 draws its tabs with an older GTK3 widget (`GtkNotebook`).

*Alternative:* `gtk::Notebook`, which is closer to gedit's look. Rejected because LeafMD would have to write the dragging, moving between windows and narrow window handling itself, and it wouldn't match the rest of the app. `adw::TabOverview` (the full-screen tab grid from GNOME Web) was also rejected: it isn't how gedit works.

### A new `Document` struct in `src/document.rs`, one per tab

`Document` owns everything in the "one per document" list above, and each tab's page is the `Document`'s widgets. The code that works on a document (rendering, jumping, scroll sync, the file watcher, saving, reloading, formatting, typing rules) moves there largely unchanged.

`Window` keeps the header, bars and actions, and each action runs on the selected document. So `win.save` becomes "find the selected document and save it".

A `Document` needs its window for three things: toasts, dialogs and updating the title. It holds a replaceable weak link to its current window, updated whenever the tab moves to another window (`TabView`'s `page-attached` signal). It never keeps a window alive by itself, and never assumes the window stays the same.

Things that listen globally, like the system's dark style change, connect once per window and pass the news to its documents. If each `Document` connected for itself, each closed tab would leave its connection behind on the global style manager.

*Alternative:* keep one web view and raw editor per window and swap contents on tab switch. It would save memory, but switching would lose undo history and scroll positions unless all of that were saved and restored by hand, and moving tabs between windows would be harder.

### One find bar, banner and formatting bar per window, with state kept per tab

These three stay as single widgets in the window's top bars, and the `Document` stores what they should show: whether find is open, the two text boxes, whether the replace row shows, and the banner's version. Selecting a tab loads that tab's state into the bar. A guard flag stops the load from being treated as the user typing, which would start a new search or move the cursor.

This matches what the user sees (each tab has its own) while avoiding a real problem: the find bar's narrow layout is a set of rules attached to the window's breakpoint, and libadwaita offers no way to remove those rules from a breakpoint when a tab moves to another window.

Each `Document` keeps its own `sourceview5::SearchContext` (the object that highlights and counts matches), so highlights belong to that tab's text. `FindBar::new` stops taking a buffer. Match Case and Whole Words Only stay window-wide, and a moved tab picks up its new window's options.

*Alternative:* a full find bar in each tab. Rejected because of the breakpoint problem above.

### Banners keep working in tabs that aren't selected

The file watcher, version tracking and file checks stay with each `Document` and keep running whether or not its tab is selected. So there are two separate steps:

1. **Working out the banner's state:** any document can do this at any time, and stores the result on itself. This includes its check timer and its protection against slow reads (`file_generation`), so one tab's answer can't land on another tab.
2. **Drawing the banner:** only the selected document in a window may touch the one banner widget. Selecting a tab or moving it to a window redraws the banner from that document's stored state, or hides it.

`update_file_banner` splits along that line. The banner's buttons look up the selected document when clicked, so the wiring doesn't belong to any one tab. The rules for comparing versions in `file_change.rs` don't change, and their unit tests still cover them.

### Page layout

```
header bar
tab bar                    (hidden with one tab)
banner, formatting bar, find bar
toast overlay
  stack: "empty" (No Document Open) | "loading" (blank) | "tabs" (the TabView)
bottom bar                 (narrow windows only)
```

The window's stack keeps the existing start page. When the last page goes away it shows "empty" again. Inside each tab, the document's own "loading" state stays blank until its formatted view has been drawn, or for one second at most, so it reuses today's reveal logic.

### A file is read before its tab exists, and its address is reserved first

Opening reads the file, then adds the tab. So a file that can't be opened leaves nothing behind: just the existing message, no tab that appears and vanishes. The selected tab stays selected until the new one is ready.

Because reading takes a moment, the file first goes on a per-window list of files waiting to open, and stays there until it has its tab. Without that, a double double-click, or `leafmd a.md a.md`, would read the file twice and open two tabs, because the check for "already open" only looks at finished tabs. A window with files waiting doesn't count as empty, so `leafmd a.md b.md` puts both in the same window. The window reads its list one file at a time, so tabs land in the order asked for, and the last one asked for ends up selected, even if an earlier file is slower to read.

### One route for every way of opening a file

All routes (Open, Recent, drop, link, file manager, terminal, restoring at startup) go through one function:

1. Already open or being opened, in any window? Select that tab, present its window, scroll to the heading.
2. Asked from inside a window (Open, Recent, drop, link)? Use that window.
3. From outside? Use an empty window, otherwise the most recently used one, otherwise make a new one.
4. In the chosen window: if the selected tab is empty and untouched, load the file into it. Otherwise add a tab.

The decision in steps 1 to 3 is a plain function over simple values in `src/opening.rs`, so it can be unit tested. The most recently used order is kept by moving a window to the end of the `Windows` list when it becomes active. `Windows` is passed to each `Window`, because Move to New Window and dragging a tab out both need to create windows. The old `open-in-new-window` app action goes away.

### Closing

`TabView` asks permission through its `close-page` signal, and that's where the existing unsaved-changes check goes, so the close button, Ctrl+W, Close Document and the tab menu all share one path.

- `win.close-tab` (Ctrl+W): two or more tabs, close the selected one. One or none, call `window.close()`, which is the path Ctrl+Q uses.
- `win.close-document` (menu): close the selected tab, and if it was the last, show the start page and call `state::forget_last_file()`, as it does today.
- `window.close` (Ctrl+Q): the close request counts unsaved tabs. None: close. One: today's dialog. Several: the new tick box dialog, with a scrolling list inside it, modelled on gedit's. All the save questions use a small dialog of LeafMD's own in `src/dialog.rs` (an `adw::Dialog`), which gives gedit's row of three equal buttons. `adw::AlertDialog` was tried first, but it stacks three buttons this long even with its wide layout turned on. Only the button that loses work has a colour. Save isn't marked as recommended, because GNOME draws that in the desktop's accent colour, which is green on some systems and clashes with the red. Its Save button saves the ticked documents in order, selecting each one so any "Replace the Other Version?" or save dialog makes sense, and stops at the first failure or cancel. Unticked documents are only discarded once every save has worked, because until then the window is still open.

### Remembering tabs

When a window closes, it records its saved files that are still on the recent list (today's rule, so Clear Recent Files keeps working), in order, and which is selected. If it has none, nothing is written, also today's rule.

In `state.ini`, the new `tabs` key holds the addresses, one per line, like the existing `files` key. The existing `last` key holds the selected tab, or the nearest saved tab when the selected one was never saved. `file_to_reopen` becomes a function returning the list and the selected file, with unit tests. If `tabs` is missing (an older state file), `last` alone gives one tab, and `forget_last_file` clears both. An older version of LeafMD reading a newer state file just reopens `last`.

Files are read in order at startup, and the window stays blank until the selected one is ready, so "No Document Open" never flashes. Files in tabs that aren't selected don't draw their formatted view until first selected. That keeps startup fast with many tabs.

### Keyboard shortcuts for tabs

`TabView` has built-in shortcuts, but they include Ctrl+Home and Ctrl+End, which would take over the raw text's "go to start/end of document". LeafMD turns on only the ones it wants through `set_shortcuts`: Ctrl+Tab, Ctrl+Shift+Tab and Alt+1 to 9. Ctrl+Page Up and Down, and gedit's Ctrl+Alt+Page Up and Down, are handled by a key watcher on the window that sees keys before anything inside it, because the raw text has its own use for Ctrl+Page Down and would otherwise get it first. `TabView`'s own shortcuts work anywhere in the window, not just with the focus inside the tab area. `TabView`'s built-in Ctrl+Page Up and Down aren't used, because they stop at the ends and the spec wants them to wrap round.

### The launcher's New Window

The `.desktop` file gains `Actions=new-window;` and a `[Desktop Action new-window]` section that runs `leafmd --new-window`. Flatpak rewrites the command when it exports the file, so this works the same as the main entry.

`--new-window` has to work whether or not LeafMD is running, so it's handled in `handle-local-options`, which runs before GTK decides which copy is in charge. LeafMD registers with the desktop:

- If another copy is already running, it asks that copy to open a window and exits.
- If not, it remembers to start with an empty window instead of reopening tabs.

`should_detach` gets a small exception so `--new-window` also gives the terminal back. It has a unit test already, which is extended.

## Risks / Trade-offs

- **[Moving a web view to another window could misbehave]** Moving a tab removes the WebKit view from one window and adds it to another. It may lose its drawn page or flicker. → This is checked first, right after the split, in its own task. If the page comes back wrong, LeafMD redraws it after the move. That's the fallback, since the markdown text is in the buffer and nothing can be lost.
- **[Memory]** Each tab has its own web view. → Not drawing unselected restored tabs keeps it down at startup. WebKit shares its web process between views where it can. Measure with 10 tabs during the check in the tasks, and report the number.
- **[A big move of code in one file]** Moving more than half of `window.rs` could quietly break something. → The split comes first, as its own group, with no behaviour change, and the owner tries it before any tab code goes in.
- **[Ctrl+N changes meaning]** People used to Ctrl+N for a new document get a new window. → The owner chose this to match gedit. The release notes call it out, and a window on the start page is easy to close.
- **[Ctrl+Tab taken from the raw text]** Ctrl+Tab moves focus in GTK text views by default, and tabs take it over. → This matches other tabbed GNOME apps, and Tab itself still works.
- **[Hidden file changes]** A background tab's file can change with no sign until that tab is selected. → Its banner shows on selection, and the check before saving still applies, so nothing is overwritten silently. A mark on the tab itself is out of scope, and easy to add later.
- **[Automated testing is limited]** The real app can't safely run under a private message bus here. → Plain functions get unit tests, and the rest is covered by a check-list in the tasks for the installed Flatpak. If a headless run helps, it uses the owner-approved approach in `build-aux/take-screenshots.sh`, and only after asking.

## Migration Plan

Nothing to convert: older state files keep working as described above. To roll back, revert the change. An older version ignores the `tabs` key.

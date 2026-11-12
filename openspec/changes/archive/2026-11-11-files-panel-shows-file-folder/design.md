# Design

## Context

See proposal.md for why. Today the Files page's folder comes from `folder_tree::folder_to_show` (src/folder_tree.rs), a small function with unit tests, called by `update_files_page` in src/window.rs. It's given the window's chosen folder, the selected document's file, and `showing`, the folder the panel shows now. `update_files_page` already runs at every moment that matters: switching tabs, a document getting a file (opening, or saving somewhere new), the panel opening, and choosing a folder.

What the panel can list is decided separately, by `can_see` in the same file. It already counts every folder inside a folder shared through the document portal (Flatpak's way of giving an app a folder outside home) as visible, and treats the bare folder of a document shared by itself as not visible. That split stays as it is.

Open Folder… (`ask_open_folder`) currently always calls `choose_folder` on its own window. `leafmd .` goes through `open_folder` (src/window.rs), which picks the window with `opening::choose_window_for_folder`: a window with that folder already, else an empty one, else a new one.

## Goals / Non-Goals

**Goals:**
- Keep "which folder" in one tested function.
- Reuse the window choice `leafmd .` already has, rather than a second set of rules.

**Non-Goals:**
- Changing how folders outside home are shared, or what counts as visible.

## Decisions

### `folder_to_show` becomes "the file's folder, else the chosen folder"

New inputs: the chosen folder and the selected document's file (the `showing` and `can_show` inputs go). Result:
1. A file: its own folder. The second value (today "is this the chosen folder") becomes "was this folder shared whole", true when the folder is the chosen folder or inside it. That's what `can_see` and the expanded-folders memory need, so they keep working for /tmp/notes/drafts after `leafmd .` in /tmp/notes.
2. No file (no document, or Untitled): the chosen folder, or nothing.

A document shared by itself now gets its own folder like any other, and the existing "only one file" message appears, because `can_see` says that folder can't be listed. No special case is needed.

*Alternative:* keep `showing` as a hint. Rejected: the owner's rule doesn't depend on what's showing, and dropping it removes the "never comes back down" problem at its source.

### Open Folder… reuses `leafmd .`'s window choice

`ask_open_folder` keeps its folder picker. After a folder is picked: if this window `is_empty()` (no tabs and nothing loading), it calls `choose_folder` on itself, as today. Otherwise it calls the same `open_folder` function `leafmd .` uses, which brings forward a window that already has the folder, or uses an empty window, or opens a new one. The only difference from `leafmd .` is that the window you're in counts as empty even if it already has a different chosen folder, since you asked from it.

*Alternative:* always open a new window when this one has a document, ignoring other windows. Rejected: two windows for the same folder would be confusing, and `leafmd .` already avoids that.

### The up button

The button is a plain `gtk::Button` with Adwaita's `go-up-symbolic` icon, packed at the start of the side panel's own header bar (`SidePanel` in src/side_panel.rs gets a `set_start_button` for it). The window shows it only while the Files page is showing, from the same place that updates the panel's title.

Going up is kept as a window field, `went_up: RefCell<Option<(String, u32)>>`: the selected file's address and how many folders up. `update_files_page` takes the file's folder from `folder_to_show` and, if `went_up` is for the same file, steps up that many parents. `selected_tab_changed` clears it, and a document saved somewhere new has a different address, so it no longer matches. Keying it to the file, rather than clearing it every time `update_files_page` runs, matters because that function also runs on small things like the unsaved dot appearing.

*Alternative:* clear it in `update_files_page` unless a flag says the up button caused this update. Rejected: easy to break when some later change calls `update_files_page` from somewhere new.

The button is greyed out when there's no folder showing, the folder has no parent, or the parent can't be listed: it isn't shared whole (the chosen folder or inside it) and `can_show_unchosen` says no. Inside the Flatpak that stops it above the home folder and above a folder shared from outside home; outside the Flatpak it stops at `/`. Its tooltip is "Show the folder above (<parent's name>)", updated with the folder.

When the panel shows a folder above the file's, the existing highlight code already expands the folders leading to the highlighted file, so "docs expanded, setup.md highlighted" needs nothing new.

## Risks / Trade-offs

- [The tree rebuilds more often, because opening a file in a subfolder now moves it] → it's the same rebuild as switching between tabs in different folders today, and it re-expands the folders leading to the highlighted file.
- [People used to the panel staying on a chosen folder] → the up button, and the Help, README and site wording explain the new rule. The change is in the release notes through `/ship`.

## Migration Plan

None. Nothing is saved differently.

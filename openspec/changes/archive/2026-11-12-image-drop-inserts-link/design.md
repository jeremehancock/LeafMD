# Design

## Context

Today the whole window has one drop target (in `src/window.rs`): whatever file is dropped, anywhere, is handed to `open_file_here`, which tries to open it as a document. An image fails with "This file isn't plain text".

The raw text is a GtkSourceView text view. GTK gives a drop to the innermost widget that accepts it, so a drop target on the text view itself gets first say over drops onto the raw text, and drops everywhere else still reach the window's target unchanged.

LeafMD runs in a Flatpak with access to the home folder. Files dragged in from the file manager arrive through the document portal (the Flatpak service that hands an app files it otherwise couldn't see). For a file the app can already reach, the portal is expected to pass the real path, like `/home/jereme/notes/beach.png`. For a file outside home it passes a stand-in path under `/run/user/1000/doc/<id>/`, which only works inside LeafMD's sandbox, and only while that share exists. A link written with that path would be broken for everyone else, and for LeafMD too once the share is gone. `folder_tree::portal_folder()` already gives that stand-in folder.

## Goals / Non-Goals

**Goals:**
- Turning a dropped image into link text is a plain function with no GTK in it, so its cases (relative paths, encoding, escaping) are covered by unit tests, like `typing.rs`.
- The drop handler decides per file: image, other, or can't link.

**Non-Goals:**
- Trying to recover the real path behind a stand-in path (the portal has a call for that). Even with the real path, LeafMD's formatted view couldn't show an image outside home, so the link would look broken in LeafMD itself.
- Showing a moving insertion cursor while dragging. If GTK's text view already does that for file drags, good; it isn't built here.

## Decisions

**1. A drop target on the raw text view, accepting a file list.**
It accepts `gdk::FileList` (several files at once) with the Copy action. On drop it sorts the files:
- images: link them (below)
- anything else: hand to the window's `open_file_here`, exactly as the window's own target would

Alternative considered: one window-level handler that checks whether the pointer was over the raw text. Rejected because it needs coordinate maths across widgets, while GTK already routes drops to the widget under the pointer.

**2. "Is it an image" is judged by name, with GIO's content type guess, and counts any `image/*` type.**
This is the same way the Files panel decides what to grey out (`folder_tree.rs`), so PNG, JPEG, GIF, WebP, SVG, AVIF, BMP and so on all count, and nothing is read from disk. Alternative: a fixed list of extensions. Rejected because GIO's list is already maintained and matches the Files panel.

**3. The link text comes from a pure function in a new small module, `src/image_link.rs`.**
Given the document's folder and the image's path, it returns `Ok("![photo](images/photo.png)")` or an error saying why it can't (the reasons map to the two messages in the spec). It:
- works out the relative path by dropping the shared start of both paths and adding one `../` per remaining folder of the document's path
- percent-encodes, per path part, a space, `(`, `)`, `<`, `>`, `%`, `\`, `#`, `?` and any control character (`#` and `?` were added after the owner dropped `#00080, tour ….jpg`, which a bare `#` turned into a jump to a spot on the page). Everything else, including non-English letters, is written as is, since comrak and WebKit handle them and they stay readable
- takes the description from the file name without its extension and puts a backslash before `[`, `]` and `\`

Alternative for spaces: angle brackets, `![x](<my photo.png>)`. Valid CommonMark, but fewer other markdown tools understand it, while `%20` works everywhere. LeafMD's export already handles `%20` (see the `my%20photo.png` test in `export.rs`).

**4. Can't-link checks happen before anything is inserted.**
In order: the document has no file yet (unsaved), then the image isn't a file LeafMD can see at that path, or the document's or the image's path is under `portal_folder()`. The "can see" check was added after the owner's test: an image dragged from the real `/tmp` arrived as plain `/tmp/...`, which inside the Flatpak is LeafMD's own empty folder. If any image in the drop fails, show that message and insert no images (other files in the drop still open). Linking half of a drop would be more confusing than linking none.

**5. Inserting: at the drop point, as one undo step.**
The drop coordinates are turned into a text position with the text view's window-to-buffer conversion and `iter_at_location`. That only answers when the point is over a character, so otherwise the line at that height is found (`line_at_y`) and the link goes at the end of the on-screen line the drop was beside, or at the end of the text when below it. The first version fell back to the end of the document, which the owner saw as links sometimes landing at the bottom. All links are inserted between one `begin_user_action` and `end_user_action`, joined with new lines, then the cursor goes after the last one and the view takes keyboard focus, so typing continues there.

## Risks / Trade-offs

- [The portal might pass stand-in paths even for files in home, on some systems] → Then every drop would show the "home folder" message instead of making a broken link, which is safe. Checked on the owner's laptop on 2026-11-12 (GNOME Files, xdg-desktop-portal 1.22): images in home arrive with their real paths and link correctly. An image from `/tmp` arrived as a plain `/tmp/...` path LeafMD couldn't see, not a stand-in path, which led to the "can see" check in Decision 4.
- [GtkTextView's own built-in drop handling might grab file drags first, for example inserting the file's address as text] → Checked in the same first task. If it does, the new target is added in the capture phase so it runs first.
- [Dragging can't be checked by the end-to-end tests] → The link-making function gets thorough unit tests, and the owner tries real drags by hand.
- [A link to `../../..` far away from the document is fragile] → Accepted. It's still correct, and it's what the user dragged.

## Open Questions

None.

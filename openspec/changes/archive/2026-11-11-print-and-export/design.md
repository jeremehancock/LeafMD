# Design

## Context

What the formatted view is today, as far as this change cares:

- `src/render.rs` turns markdown into HTML and cleans it with ammonia (a library that strips anything unsafe, such as scripts). The HTML carries `data-sourcepos` tags (which lines each block came from) for click-to-jump and synced scrolling.
- `src/preview.html` is the page the formatted view shows. All of its styles live in one `<style>` block, with light colours on `:root` and dark ones on `:root[data-theme="dark"]`. `src/document.rs` loads it with the document's folder as its base address (so `img/cat.png` finds `~/notes/img/cat.png`), then pours each new render into it with a little JavaScript.
- The font, size and zoom reach the page through WebKit's settings (`apply_fonts` in `document.rs`), and the light or dark style and accent colour through JavaScript (`update_style`). So the on-screen page is zoomed, may be dark, and may have find highlights in it.
- A tab that hasn't been looked at yet may not have drawn its formatted view at all ("preview waiting").
- The Flatpak has no printer access (`--socket=cups` isn't in `finish-args`, and shouldn't be, to keep to Flathub's rules). Flatpak apps print through the desktop's print portal: a desktop service that shows the Print dialog and sends the job to the printer for the app.
- The tab bar is libadwaita's `AdwTabBar`. It scrolls to the selected tab when the selection changes, but not when the window gets narrower, and it has no "scroll to this tab" function.

## Goals / Non-Goals

**Goals:**
- One way of building "the document as a standalone page", used by Print, Export as PDF and Export as HTML, so all three look the same and only differ where the specs say (packed pictures, light only or following the reader).
- Never disturb the on-screen view: printing doesn't flash the window light, drop the zoom or lose the scroll position.
- No new dependencies and no new sandbox permissions.

**Non-Goals:**
- Making the formatted view itself printable with WebKit's default behaviour. The on-screen page stays as it is.
- Any setting for paper size, margins or what to include.

## Decisions

### 1. The styles move out of `preview.html` into `src/preview.css`

The `<style>` block becomes its own file, included into the app at build time (`include_str!`, as `preview.html` already is) and placed into both the on-screen page and the export page. This is the only way to keep the exported page looking exactly like the formatted view without keeping two copies of the styles in step by hand.

The stylesheet gains a `@media print` section (rules that only apply on paper): code blocks wrap (`white-space: pre-wrap`), tables can be as wide as the page instead of scrolling, `break-inside: avoid` on code blocks, table rows and pictures, `break-after: avoid` on headings, and the find highlights are switched off. The on-screen view never prints, so these rules don't change how it looks.

*Alternative considered:* a separate print stylesheet. Rejected: the colours, cards and checkboxes would have to be copied, and would drift.

### 2. A new `src/export.rs` builds the standalone page

`export::page(markdown, options)` returns a complete HTML document as text:

- The body is the same cleaned HTML as the formatted view, made by a second ammonia cleaner in `render.rs` that also drops `data-sourcepos` (it means nothing outside LeafMD).
- The stylesheet from decision 1, plus a small block setting what WebKit's settings set on screen: the Preferences font as an explicit `font-family` (followed by Adwaita Sans, Cantarell, system-ui and sans-serif, for computers that don't have it), its size in pixels at 100% zoom, its weight and slant, and the accent colour LeafMD is using.
- Colours: for printing and PDF, light only. For the HTML file, light by default plus a `@media (prefers-color-scheme: dark)` rule (which browsers apply when the reader's desktop or browser is dark) carrying the same dark colours as `:root[data-theme="dark"]`. The dark colours stay written once in `preview.css`, marked so `export.rs` can reuse them.
- A `<title>`: the first level-one heading's text, else the file name without its ending (decision in the spec). Headings are already found by `src/headings.rs`, which is reused.
- No `<script>` at all, not even LeafMD's `preview.js`.

For the HTML file only, pictures are packed in: each `<img src>` that is a relative path or a `file://` address is read from disk (relative to the document's folder), and replaced with a `data:` address (the picture's bytes written as text inside the page, using `glib::base64_encode`, with its type from `gio::content_type_guess`). A picture that can't be read is left as written, so the browser shows its description. Online pictures are left alone. This is done as the cleaner's attribute filter (ammonia lets a function rewrite each attribute as it cleans), so the HTML is never parsed by hand. The cleaner for the export allows `data:` addresses on `img` only.

*Alternative considered:* rewriting `src` with a regular expression after cleaning. Rejected: fragile with raw HTML in documents, and the cleaner already visits every attribute.

### 3. Print and PDF use a hidden WebView made for the occasion

Print and Export as PDF create a fresh `webkit6::WebView` that isn't shown in any window, load the light standalone page into it with the document's folder as the base address (so pictures load from disk, with no need to pack them), wait for it to finish loading (pictures included), print, then let it go.

This gives the light colours, the Preferences size without zoom, and no find highlights for free, works the same in every view (including a tab whose formatted view never drew), and leaves the on-screen view untouched. Scripts stay off in it, as in the formatted view, and it is never allowed to navigate (the same navigation guard as the formatted view).

*Alternative considered:* printing the on-screen WebView with print-only CSS. Rejected: it would print the zoomed size, can't print a formatted view that hasn't drawn, and the dark style is set by LeafMD rather than the system, so CSS alone can't undo it reliably.

### 4. Export as PDF: WebKit prints to a file, then LeafMD copies it into place

`webkit6::PrintOperation` can print without any dialog to GTK's "Print to File" printer, given an output address and `pdf` as the format. LeafMD:

1. Asks where to save with `gtk::FileDialog` (the same kind of dialog as Save As…, which goes through the file chooser portal in the Flatpak), suggesting `<name>.pdf` in the document's folder, with a PDF filter.
2. Prints to a temporary file in LeafMD's cache folder, with the usual paper size for the computer's language and region (`gtk::PaperSize::default()`) and 2 cm margins.
3. Copies the temporary file over the chosen file with `gio` (which works for files chosen through the portal), deletes the temporary one, and shows "Saved garden.pdf".

Printing straight to the chosen place was rejected: a file chosen through the portal can have an address the print system can't write to.

### 5. Print: WebKit's own dialog if it reaches the portal, otherwise LeafMD asks the portal itself

Two ways exist, and which one works inside the Flatpak can only be found by trying (task 1.1):

- **A. WebKit's dialog.** `PrintOperation::run_dialog` with the window as parent. Recent WebKitGTK versions hand this to the print portal when sandboxed; if so, nothing more is needed.
- **B. The portal by hand.** If A only offers "Print to File" in the Flatpak, LeafMD talks to the print portal (`org.freedesktop.portal.Print`) over D-Bus with `gio`: `PreparePrint` shows the system's Print dialog and returns the chosen paper and settings; LeafMD makes a PDF with that paper the same way as decision 4; then `Print` hands the PDF to the portal, which sends it to the printer. No new library is needed (`gio::DBusProxy` can pass the file handle).

Either way the specs are met; B is more code but is how every other Flatpak app that prints a ready-made PDF does it. If the portal isn't there at all, Print shows "Couldn't print:" with the reason instead of failing silently, the same lesson as the web link crash fixed in 0.11.0 (ask the portal first).

### 6. Export as HTML writes the page with `gio`

`gtk::FileDialog` suggesting `<name>.html`, with an HTML filter, then `gio::File::replace_contents_future` with the page from decision 2, then "Saved garden.html". Errors go to "Couldn't export:" with the reason `gio` gives.

### 7. The menu and actions

Three new window actions, `win.print`, `win.export-pdf` and `win.export-html`, added to `DOCUMENT_ACTIONS` so they're greyed out with no document open, in a new menu section between the Save section and the Find section. `<Control>p` is set for `win.print` in `src/main.rs` next to the other shortcuts. The actions are window actions, so they work in Distraction Free like the others. They act on the selected tab and read its text with `Document::text()`, which includes unsaved changes, and never touch the buffer or its saved state.

The suggested name comes from the tab's file name with its ending replaced (`garden.md` to `garden.pdf`, `README` to `README.pdf`), or "Untitled"; the folder is the document's folder when it has one.

### 8. Keeping the selected tab in view when the window narrows

LeafMD finds the tab bar's own scrolled area by walking the tab bar's child widgets for the `gtk::ScrolledWindow` inside it, and watches its sideways scroll position object (its "adjustment") for a change in how much is visible (`page-size`). When that shrinks, LeafMD works out where the selected tab sits and scrolls just enough to show it. Tabs in the scrolling state are all the same width, so the selected tab's left edge is its position times (total width ÷ number of tabs). This is done after GTK has finished laying out the bar (on the next idle moment), so the numbers are current.

*Alternatives considered:* selecting another tab and back again (the tab bar scrolls to a newly selected tab). Rejected: it would briefly switch documents, redraw the side panel's Headings and run everything a tab switch runs. Asking libadwaita for a "scroll to tab" function would be the real fix; LeafMD can drop this workaround if one arrives.

## Risks / Trade-offs

- [WebKit's print dialog may only offer "Print to File" inside the Flatpak] → Task 1.1 tries it first; decision 5B is the planned fallback, with the same result for the user.
- [Printing a WebView that isn't in a window might not lay out or load pictures] → Also tried in task 1.1 for the PDF. If it fails, the hidden WebView goes into a hidden, never-shown window, which is enough for WebKit to lay it out.
- [Big pictures make big HTML files] → Accepted; packing them in is what "stands on its own" means. A document with a 10 MB photo makes an HTML file of about 13 MB.
- [The Preferences font may not be on the reader's computer] → The font list falls back to common ones, ending at the browser's own sans-serif.
- [The tab bar's insides are libadwaita's, not LeafMD's, and could change in a future GNOME runtime] → If the scrolled area isn't found, the fix quietly does nothing and the tab bar behaves as it does today. The end-to-end tests can't check this, so it's in the owner's hand-test list.
- [`data:` pictures are allowed in the exported page] → Only on `img`, only in the export cleaner, and the page has no scripts, so a picture can't do anything but show.

## What task 1.1 found

- **Print uses way A, WebKit's own dialog.** The WebKitGTK in the GNOME 51 runtime has the print portal built in (its library contains `webkitPrintOperationPreparePrint` and talks to `org.freedesktop.portal.Print`), so `run_dialog` shows the desktop's Print dialog when sandboxed. LeafMD still checks that the portal answers first, so a desktop without one gets "Couldn't print:" (checked on the test desktop, which has no portal). Seeing real printers in the dialog is part of the owner's hand test, since the test desktop has no portal.
- **The hidden WebView needs no window.** Printing it to a PDF file with no dialog worked on the test desktop, pictures included.
- **The tab bar's selected tab can be measured directly.** Instead of working out its place from equal tab widths (decision 8), LeafMD finds the tab widget in GTK's "selected" state and measures where it is. Checked with the fix switched off too: the eighth of 8 tabs was scrolled out of sight at 360 pixels without it, and fully in view with it.
- **One more print rule was needed:** the last paragraph's space underneath could spill onto an empty extra page when the text only just fitted, so it's dropped on paper.

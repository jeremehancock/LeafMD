# Design

## Context

The side panel sits in libadwaita's split view (`adw::OverlaySplitView` in `src/window.rs`), which puts the panel and the document side by side, or slides the panel over the document in narrow windows. Its width is fixed today by setting its smallest and largest width both to 240. The split view has no way for the user to resize it, so LeafMD has to add its own drag handle.

Two width rules already exist and stay as they are:
- Below 920 pixels (written as `919sp`, so it grows with GNOME's Large Text setting) the panel slides over the document. 920 is 240 for the panel plus 680 for the document, which the header bar needs.
- Below 680 the compact layout moves header buttons to a bottom bar.

What LeafMD remembers between runs lives in `state.ini`, read and written by `src/state.rs`. The panel already has a `[side-panel]` section with `open` and `page`.

Web links and "open in its usual app" links both go through `gio::AppInfo::launch_default_for_uri_future` in `src/document.rs`. Inside Flatpak, GIO hands the link to the desktop's portal (a desktop service that opens things on a sandboxed app's behalf). A comment there records that GTK's own `UriLauncher` was tried first and failed inside Flatpak, so switching to it isn't an option.

## Goals / Non-Goals

**Goals:**
- A drag handle that feels like GNOME's other resizable panes, and doesn't get in the way of the panel's lists or the document.
- The panel follows the window's width smoothly: it gives up room before the document goes below 680.
- No crash when there's no portal, for both kinds of link that use it.

**Non-Goals:**
- Replacing the split view with a different widget (such as `gtk::Paned`). The split view's narrow-window sliding, gestures and animations are what the side-panel spec describes, and they'd all have to be rebuilt.

## Decisions

### 1. The handle is a thin strip on the document's side of the line

A transparent strip about 6 pixels wide, laid over the left edge of the document area (an overlay on the content side of the split view). It shows the `col-resize` pointer, and a drag gesture on it changes the panel's width. It only exists while the panel sits beside the document. The line itself stays the split view's own separator, so nothing changes visually.

*Alternatives:* inside the panel's right edge. Rejected because the Files and Headings lists put their scrollbars there, and the handle would steal drags meant for them. Straddling the line would need two strips in two widgets sharing one drag; more parts for little gain. The document's left edge is safe: the header bar has padding there, and the raw text and formatted view both have margins.

### 2. The width is set through the split view's smallest and largest widths

As today, the smallest and largest sidebar widths are set to the same number, so the split view draws exactly that width. During a drag, the new width is the width at the start of the drag plus how far the pointer moved, kept between 200 and the current limit (decision 3). The width is saved to `state.ini` (`[side-panel] width=320`) once, when the drag ends, not on every movement. Each window keeps its own chosen width, so dragging in one doesn't change another. New windows read it from `state.ini`. A saved width outside 200 to 400, or one that isn't a number, reads as 240, with unit tests in `src/state.rs` like the ones for `open` and `page`.

In a narrow window, where the panel slides over the document, the width is set back to 240 and the handle is hidden. Coming back to a wide window restores the chosen width.

### 3. Keeping the document at 680 pixels

The panel's shown width is the chosen width, but no more than the window's width minus 680 (converted from `sp` to pixels with libadwaita's `LengthUnit`, so it agrees with the breakpoint under Large Text). At the 920 breakpoint that limit is exactly 240, so the panel narrows smoothly down to 240 and then slides over the document, as it does today.

To follow the window's width, the split view goes inside a small widget of LeafMD's own (`WidthWatcher` in `src/side_panel.rs`, built on `adw::Bin`). It lays out its one child itself and, just before doing so, tells the window the width it was given. The window sets the panel's width right then, so the panel and document are laid out together at widths that add up.

*Alternatives:* `gtk::CustomLayout`, the first plan, isn't available in GTK's Rust bindings. The first build of `WidthWatcher` didn't work, which the owner found in their hand test: dragging the window narrower left the panel at its full width. `adw::Bin` hands its layout to a separate helper (a "layout manager"), and while a widget has one, GTK never calls the widget's own layout code, so the watcher never reported a width. It now switches that helper off and lays out its child itself. That first build also set the width afterwards, from an idle callback (a "when you're free" job). It now sets it before the split view is laid out, which avoids a moment where the panel and document don't add up. Watching the window's `default-width`, `maximized` and `fullscreened` properties was rejected because `default-width` doesn't follow every change (maximizing, tiling). A per-frame tick callback would keep the app waking up 60 times a second.

### 4. Tooltips in Files

Every row's tooltip is set when the row is filled in: the file or folder's name for files LeafMD can open and for folders, and the existing "LeafMD can't open this type of file" for greyed-out files. Always showing it, rather than only when the name is cut short, matches what Headings does and avoids measuring text.

### 5. Asking the portal before handing it a link

Inside Flatpak (when `/.flatpak-info` exists), before calling GIO, LeafMD asks the portal for its OpenURI `version` property over D-Bus (the desktop's message bus), with a short timeout. Reading the property also starts the portal if it isn't running yet. If there's no answer, LeafMD shows "Couldn't open the link: no app is available to open it" and doesn't call GIO. Otherwise it carries on as today. One shared helper is used by both web links and links to files that open in their usual app. Outside Flatpak (`cargo run`), GIO doesn't use the portal, so the check is skipped.

*Alternatives:* GTK's `UriLauncher`, already tried and found not to work inside Flatpak. Checking whether the portal's name has an owner on the bus. Rejected because the portal is started on demand, so having no owner yet is normal and doesn't mean it's missing.

### 6. Testing

- A `drag_window <x> <y> <dx>` helper in `build-aux/test-desktop.sh`: moves the virtual pointer to a point, presses the button, moves in a few steps, then releases. It is built like `click_window`. The `side-panel-width` scenario drags the handle (the document's left edge, just right of 240) by 80 and checks `expect_setting side-panel width 320`. Where the panel's edge is afterwards is checked by dragging again from just right of where it should be: if the edge isn't there, the press lands elsewhere and nothing new is saved. It does this after restarting, and after narrowing the window in small steps.
- The `web-link-no-portal` scenario opens a document whose first line is a link, in Formatted view, clicks it, and checks LeafMD is still on the bus and its log has "could not open link".
- How the handle looks and feels, the pointer arrows, and the tooltip are hand checks.

## Risks / Trade-offs

- [Changing the panel's width in the middle of GTK's layout could upset it] → The width is set before the split view is laid out, not during, and test runs show no GTK warnings. The side-panel-width scenario narrows the window in small steps, like a hand, and checks where the panel's edge ends up.
- [The handle overlaps a few pixels of the document's left edge, so a click there resizes instead of placing the cursor] → 6 pixels inside the raw text's left margin and the formatted view's padding. Check that clicking at the start of a line still places the cursor.
- [The Files page title or page switcher may not fit at 200 pixels] → Check at 200 with a picture. If they don't fit, raise the smallest width to what fits and update the spec before archiving.
- [The portal check adds a short wait before a link opens] → One D-Bus round trip, a few milliseconds when the portal is there. The timeout (about two seconds) only matters when it's missing.
- [The portal check could pass while the portal later fails in the way that crashes GIO] → Then the crash isn't fixed for that case. The end-to-end scenario only covers the missing portal, which is the case actually seen.

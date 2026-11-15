# Design

## Context

See proposal.md for why. The pieces this builds on:

- **The main menu** is built in `src/window.rs` as sections of menu items, each naming a window "action" (a named command, like `win.about`). The last section holds Preferences, Help, Markdown Cheatsheet, Keyboard Shortcuts and About LeafMD.
- **Small windows over the document** (Help, Keyboard Shortcuts, the save questions) are libadwaita `Dialog`s: a window that floats over LeafMD on a desktop and slides up from the bottom on a narrow one, with Esc and a close button built in.
- **Opening a web link from LeafMD** goes through `open_in_usual_app` in `src/document.rs`. Inside Flatpak it first checks the desktop's portal answers (the service that opens links for sandboxed apps), because GIO otherwise stops LeafMD altogether. A failure is shown as a toast (a short message at the bottom of the window): "Couldn't open the link: ...".
- **About** is libadwaita's `AboutDialog`, with two custom links today, Website and GitHub, added with `add_link`.
- **The website** loads nothing from other websites (its Content-Security-Policy, the rule the browser enforces, allows only its own files), so Buy Me a Coffee's own button picture can't be used there.

The other projects' versions, used as the model:

- cinemetric.dev and getmarquee.now: a centred card with a heart on a soft tinted tile, an `<h2>` "Support development", one muted paragraph, and an accent button with a coffee cup icon reading "Buy me a coffee".
- Cinemetric's and Marquee's README: a "Support Development" section of prose plus the yellow Buy Me A Coffee badge.
- Marquee's app: a "Support Development" menu entry that opens the same ask in a dialog, one button, which closes the dialog when clicked.

## Goals / Non-Goals

**Goals:**

- The same ask, in the same words, in all three places (site, README, app), and recognisably the same as the owner's other projects.
- Inside the app, look like any other GNOME dialog: nothing that feels like an advert.

**Non-Goals:**

- Remembering whether someone has donated, or anything that changes LeafMD's behaviour afterwards.
- A reusable "dialog with a call to action" helper. One window needs it.

## Decisions

### 1. The words

Adapted from Cinemetric's and Marquee's paragraph. Their joke about a hard drive for the Plex server is left out (the owner's call: LeafMD has nothing to do with media servers); the ending is about coffee instead:

> LeafMD is free and always will be. If it has earned a spot on your desktop, you can throw a few dollars at ongoing development. Or, let's be honest, at the coffee that keeps it going.

Used word for word on the site and in the app. The README uses Cinemetric's plainer README wording instead ("LeafMD is free and open source, and it stays that way. If it has been useful to you..."), because the README is where people read about the project, not where they get asked.

The heading follows each place's own capitalisation rule: "Support development" on the website (sentence case, as on cinemetric.dev), "Support Development" in the app's menu and window and in the README (GNOME menu items and LeafMD's README headings use Title Case, as Marquee's do). The button reads "Buy me a coffee" everywhere, matching Marquee's later decision that the button should name where it goes.

### 2. The app window: a small custom `adw::Dialog`, not `adw::AlertDialog`

**Chosen:** an `adw::Dialog` about 360 pixels wide, with an empty header bar (for the close button), then centred: a heart in the desktop's accent colour on a soft round patch of the same colour, the heading in GNOME's "title-2" style, the paragraph wrapped, and one wide pill-shaped "Buy me a coffee" button with a coffee cup icon.

*Rejected:* `adw::AlertDialog` (GNOME's ready-made question box). It has no place for the heart above the heading, and it's built for questions with two or more answers ("Cancel" / "Buy me a coffee"). A "Cancel" button for something that was never a question reads oddly, and the close button and Esc already cover it.

The button uses GNOME's `suggested-action` style, so it's coloured with the desktop's accent colour, like the rest of LeafMD. It is the one thing to do in the window, which is exactly what that style is for. It has the keyboard focus when the window opens, so Enter presses it.

### 3. Opening the link

The button calls `open_in_usual_app` (made visible to `window.rs`), then closes the dialog. If opening fails, the window shows the same "Couldn't open the link: ..." toast a document link would. Reusing it keeps the Flatpak portal check, which a bare GTK link would skip.

*Rejected:* `gtk::LinkButton`, which Help's Setup row uses. It doesn't let LeafMD close the dialog after the click or show its toast on failure.

### 4. The heart icon

A new filled heart, `leafmd-heart-symbolic`, in `data/resources/icons`, drawn with filled shapes only (the project's icon rule). The coffee cup on the button reuses a filled cup drawn the same way (`leafmd-coffee-symbolic`), so the button matches the website's.

### 5. Where the menu item sits

Just above About LeafMD, as Marquee puts it last-but-one. GNOME apps keep About last, and Support Development belongs with the "about the app" items rather than the document items.

### 6. About's link

`add_link("Support Development", ...)` beside Website and GitHub. libadwaita opens these itself, as it already does for Website, so nothing new is needed there.

### 7. The website section

A new `<section id="support">` after "Early days", modelled on cinemetric.dev's `.support` card: centred, max 640 pixels wide, a rounded border, a faint green glow at the top, the heart on a tinted tile, heading, muted paragraph and a `button primary` link with an inline coffee-cup SVG. All drawn with the page's existing colour tokens, so dark style comes for free. "Early days" and the support card both being centred bands one after the other could look repetitive, so the support section sits on the plain background (not `band`) and as a card, which separates it visually. The footer's link list gets "Support development" pointing at `#support`. The top bar gets "Support" after Terminal (added at the owner's request after trying it). Below 600 pixels wide it is hidden, because the bar has no room for a fourth section link there; the footer link still reaches the section.

### 8. The app store donation link

`<url type="donation">https://www.buymeacoffee.com/jeremehancock</url>` in the metainfo, next to the existing `homepage` and `bugtracker` URLs. This is a standard AppStream link type that Flathub's checks accept, so `build-aux/flathub-lint.sh` should stay clean with no new exceptions.

## Risks / Trade-offs

- [The ask feels pushy in a calm app] → It's only ever behind a menu item and an About link; nothing appears unprompted, and the spec says so.
- [The README badge is a picture from buymeacoffee.com] → Only on GitHub, where every badge is remote; the website, which promises nothing from elsewhere, draws its own button.
- [The opening portal isn't running] → Same handling as document links: a toast, and LeafMD keeps running.

## Migration Plan

Nothing to migrate: no settings or stored data. Undoing it is reverting the commit.

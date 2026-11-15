# Proposal

## Why

LeafMD is free, and there's no way for someone who likes it to say thanks with a few dollars. The owner's other projects (Marquee, Glimpse, Cinemetric) all have a "Support development" ask with a Buy me a coffee button, and LeafMD should match them on the website, in the README and inside the app.

## What Changes

- **Website (leafmd.app):** a new "Support development" section near the bottom of the page, after "Early days": a heart, a short friendly ask, and a "Buy me a coffee" button that opens `https://www.buymeacoffee.com/jeremehancock` in a new tab. It reads like the same section on cinemetric.dev and getmarquee.now, drawn in LeafMD's own colours. The top bar gets a "Support" link after Terminal (on screens at least 600 pixels wide, where it fits), and the footer a "Support development" link, both jumping to it. The button is drawn by the page itself, not Buy Me a Coffee's picture, because the site loads nothing from other websites.
- **README:** a new "Support Development" section (after Contributing), worded like Cinemetric's: LeafMD is free and stays that way, a link to `leafmd.app/#support`, a mention of the menu item in the app, and the standard yellow Buy Me A Coffee badge (the same line Glimpse, Marquee and Cinemetric use).
- **In the app:**
  - **New main menu item: "Support Development"**, in the last section, just above About LeafMD. No keyboard shortcut.
  - It opens a small window over the document with a heart, the heading "Support Development", the same ask as the website, and one "Buy me a coffee" button. The button opens the Buy Me a Coffee page in the web browser and closes the window. Esc or the close button closes it without doing anything.
  - **About LeafMD** gets a third link, "Support Development", next to Website and GitHub, opening the same page.
  - **Help** gets a short "Support" section saying where to find it.
- **App store description** (metainfo): a donation link (`<url type="donation">`), which GNOME Software shows as a "Donate" link on LeafMD's page.

Nothing is ever shown unasked: no pop-up on start, no reminder after a number of uses, no nag of any kind. The ask is only seen by someone who goes looking for it.

### Out of scope

- Any other way to pay (GitHub Sponsors, Ko-fi, Liberapay). One button, one destination, as in the other projects.
- Paying inside LeafMD. The button only opens the web page.
- Changing the "Early days" section or anything else on the site.
- New screenshots: no screenshot shows the main menu or the new window, so none change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `help-and-about`: the main menu gains Support Development; a new requirement for the support window it opens; About gains a Support Development link; Help gains a Support section.
- `distribution`: the storefront gains a "Support development" section and footer link; the app store description gains a donation link.

## Impact

- `src/window.rs`: the menu item and its action, the support window, the About link, the Help section.
- `data/resources/icons` and `resources.gresource.xml`: a filled heart icon (`leafmd-heart-symbolic`) for the support window.
- `site/index.html` and `site/style.css`: the new section, button and footer link.
- `README.md`, `data/io.github.jeremehancock.LeafMD.metainfo.xml`, and the two specs.
- `build-aux/end-to-end/`: one new scenario that the menu item's action is there and opening the window doesn't upset LeafMD.
- No new dependencies, settings or keyboard shortcuts.

// Help for visitors whose computer doesn't have Flatpak set up yet, which
// mostly means Ubuntu: it doesn't include Flatpak, so LeafMD's install file
// opens in a text editor there.
//
// Firefox on Ubuntu says so in the description it gives websites (its user
// agent), so for those visitors the setup box starts open and the line under
// the Install button speaks to them directly. Other browsers don't say which
// Linux they run on, so everyone else sees the plain line and a closed box.
//
// Without scripts, the line still links to the box, and clicking the box's
// title opens it.
const box = document.getElementById("ubuntu");
const hint = document.getElementById("ubuntu-hint");

if (box && hint) {
  if (navigator.userAgent.includes("Ubuntu")) {
    box.open = true;
    hint.innerHTML = 'You\'re on Ubuntu: <a href="#ubuntu">set up Flatpak first</a>, then click Install.';
  }

  // Following the link opens the box too, rather than landing on its title
  hint.addEventListener("click", (event) => {
    if (event.target.closest('a[href="#ubuntu"]')) box.open = true;
  });
}

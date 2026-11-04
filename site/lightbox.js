// The screenshot viewer. On a screen at least 700 pixels wide, clicking one
// of the small screenshots shows it full size over the page, with its
// caption and previous and next buttons. Esc, the close button or a click
// outside the picture closes it.
//
// It uses the browser's own <dialog>, which already handles Esc and keeps
// keyboard focus inside the viewer while it's open. On a phone, or without
// scripts, each screenshot is a plain link that opens the picture by itself.
const dialog = document.querySelector("dialog.lightbox");
const links = [...document.querySelectorAll(".gallery a.zoom")];
const wide = window.matchMedia("(min-width: 700px)");

if (dialog && typeof dialog.showModal === "function" && links.length) {
  const picture = dialog.querySelector("img");
  const caption = dialog.querySelector("figcaption");
  let current = 0;

  const show = (index) => {
    current = (index + links.length) % links.length;
    const small = links[current].querySelector("img");
    // currentSrc is the picture actually showing, which for the light or
    // dark screenshot depends on the visitor's style
    picture.src = small.currentSrc || links[current].href;
    picture.alt = small.alt;
    caption.textContent = links[current].closest("figure").querySelector("figcaption").textContent;
  };

  links.forEach((link, index) => {
    link.addEventListener("click", (event) => {
      if (!wide.matches) return;
      event.preventDefault();
      show(index);
      dialog.showModal();
    });
  });

  dialog.querySelector(".lightbox-close").addEventListener("click", () => dialog.close());
  dialog.querySelector(".lightbox-prev").addEventListener("click", () => show(current - 1));
  dialog.querySelector(".lightbox-next").addEventListener("click", () => show(current + 1));

  // A click on the dark area around the picture, not on it or a button
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog || event.target.classList.contains("lightbox-inner")) {
      dialog.close();
    }
  });

  dialog.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft") show(current - 1);
    if (event.key === "ArrowRight") show(current + 1);
  });
}

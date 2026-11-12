// Highlights the top bar's link for the section on screen, whether the
// visitor got there by clicking the link or by scrolling.
//
// A section counts as "on screen" once its top has passed a line a third of
// the way down the window, and until the next section's top passes it. Above
// the first linked section (the top of the page and the screenshots), no
// link is highlighted. At the very bottom of the page the last section wins,
// even if it's too short to reach the line.
//
// Without scripts, the links still work; nothing is highlighted.
const navLinks = [...document.querySelectorAll('.nav-links a[href^="#"]')];
const sections = navLinks
  .map((link) => document.getElementById(link.getAttribute("href").slice(1)))
  .filter(Boolean);

if (sections.length) {
  let scheduled = false;

  const update = () => {
    scheduled = false;
    const line = window.innerHeight / 3;
    let current = null;
    for (const section of sections) {
      if (section.getBoundingClientRect().top <= line) current = section;
    }
    const atBottom = window.innerHeight + window.scrollY >= document.documentElement.scrollHeight - 2;
    if (atBottom) current = sections[sections.length - 1];

    navLinks.forEach((link) => {
      const on = current && link.getAttribute("href") === `#${current.id}`;
      link.classList.toggle("active", Boolean(on));
      if (on) link.setAttribute("aria-current", "location");
      else link.removeAttribute("aria-current");
    });
  };

  // Scrolling fires many events a second; work it out at most once a frame
  const schedule = () => {
    if (!scheduled) {
      scheduled = true;
      requestAnimationFrame(update);
    }
  };

  window.addEventListener("scroll", schedule, { passive: true });
  window.addEventListener("resize", schedule);
  update();
}

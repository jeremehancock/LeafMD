// Helpers for the formatted view. LeafMD runs this in its own private script
// "world", separate from the document, so nothing in a markdown file can reach
// these functions or send messages to the app. (Scripts inside documents are
// switched off anyway.)
(() => {
  if (window.leafmd) return;

  const post = (message) => window.webkit.messageHandlers.leafmd.postMessage(message);

  // Block-level elements that mark a position in the raw text
  const BLOCKS = "p,h1,h2,h3,h4,h5,h6,li,pre,blockquote,table,tr,hr,dl,dt,dd";

  function startLine(element) {
    return parseInt(element.dataset.sourcepos, 10);
  }

  // Every block with its first source line and its position on the page, in
  // order. Entries that would make line or position go backwards are skipped,
  // so the matching below always moves in one direction.
  function anchors() {
    const list = [];
    for (const element of document.querySelectorAll("#content [data-sourcepos]")) {
      if (!element.matches(BLOCKS)) continue;
      const line = startLine(element);
      if (!Number.isFinite(line)) continue;
      const top = element.getBoundingClientRect().top + window.scrollY;
      const last = list[list.length - 1];
      if (!last || (line > last.line && top >= last.top)) list.push({ line, top });
    }
    return list;
  }

  // Scroll so the given source line (can be fractional, like 12.5) is at the top
  function scrollToLine(line, atBottom) {
    if (atBottom) {
      window.scrollTo(0, document.documentElement.scrollHeight);
      return;
    }
    const list = anchors();
    if (!list.length) return;
    let before = list[0];
    let after = null;
    for (const anchor of list) {
      if (anchor.line <= line) before = anchor;
      else { after = anchor; break; }
    }
    let y = before.top;
    if (line < before.line) y = list[0].top;
    else if (after) y += (after.top - before.top) * (line - before.line) / (after.line - before.line);
    window.scrollTo(0, Math.max(0, y - list[0].top));
  }

  // The source line (fractional) that matches the top of the visible page
  function lineAtTop() {
    const list = anchors();
    if (!list.length) return 1;
    const y = window.scrollY + list[0].top;
    if (y <= list[0].top) return 1;
    let before = list[0];
    let after = null;
    for (const anchor of list) {
      if (anchor.top <= y) before = anchor;
      else { after = anchor; break; }
    }
    if (!after || after.top === before.top) return before.line;
    return before.line + (after.line - before.line) * (y - before.top) / (after.top - before.top);
  }

  // Tell LeafMD when the formatted view is scrolled (at most once per frame)
  let scrollQueued = false;
  window.addEventListener("scroll", () => {
    if (scrollQueued) return;
    scrollQueued = true;
    requestAnimationFrame(() => {
      scrollQueued = false;
      const root = document.documentElement;
      const atBottom = window.scrollY > 0 && window.innerHeight + window.scrollY >= root.scrollHeight - 2;
      post({ kind: "scroll", line: lineAtTop(), bottom: atBottom });
    });
  });

  // True in split view, where a double-click jumps to the raw text
  let jumpOnDoubleClick = false;

  // Double-click a word to find it in the raw text (split view only). Sends the
  // source lines of the clicked element, the word, and which copy of the word it
  // was in that element, so the editor can select the exact same spot.
  // In the formatted-only view, double-click just selects the word as usual.
  document.addEventListener("dblclick", (event) => {
    if (!jumpOnDoubleClick) return;
    const target = event.target instanceof Element ? event.target : event.target.parentElement;
    const element = target && target.closest("[data-sourcepos]");
    if (!element) return;
    const lines = /^(\d+):\d+-(\d+):\d+/.exec(element.dataset.sourcepos);
    if (!lines) return;

    const selection = window.getSelection();
    const word = selection ? selection.toString().trim() : "";
    let occurrence = 0;
    if (word && selection.rangeCount) {
      const before = document.createRange();
      before.setStart(element, 0);
      const picked = selection.getRangeAt(0);
      before.setEnd(picked.startContainer, picked.startOffset);
      occurrence = before.toString().split(word).length - 1;
    }
    // Only one selection can be active on Linux (it's what middle-click pastes),
    // so clear this one to let the raw text keep its selection.
    if (selection) selection.removeAllRanges();

    post({ kind: "jump", start: Number(lines[1]), end: Number(lines[2]), word, occurrence });
  });

  window.leafmd = {
    scrollToLine,
    setSplitView(on) { jumpOnDoubleClick = on; },
  };
})();

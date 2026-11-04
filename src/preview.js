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

  // ---- Find, when only the formatted view is showing ----
  //
  // Matches are coloured with the browser's highlight feature (the
  // ::highlight() styles in preview.html), which paints over the text without
  // changing the page. Each match is a Range: a start and end point in the text.

  let findQuery = null; // { text, matchCase, wholeWords } of the last search
  let findRanges = [];
  let findCurrent = -1; // which match is selected, or -1

  // Elements whose text shouldn't run into the next one's when searching.
  // "end of one paragraph" + "start of the next" isn't a match.
  const FIND_BLOCKS = "p,h1,h2,h3,h4,h5,h6,li,pre,blockquote,td,th,dt,dd,div,figcaption,summary";

  // All the visible text in one string, with a note of where each piece came from
  function pageText() {
    const content = document.getElementById("content");
    const walker = document.createTreeWalker(content, NodeFilter.SHOW_TEXT);
    let text = "";
    const pieces = [];
    let lastBlock;
    for (let node = walker.nextNode(); node; node = walker.nextNode()) {
      const block = node.parentElement.closest(FIND_BLOCKS);
      if (pieces.length && block !== lastBlock) text += "\n";
      lastBlock = block;
      pieces.push({ node, start: text.length });
      text += node.data;
    }
    return { text, pieces };
  }

  // Turns a position in pageText()'s string back into a spot in the page
  function spotAt(pieces, offset, isEnd) {
    let low = 0;
    let high = pieces.length - 1;
    while (low < high) {
      const mid = (low + high + 1) >> 1;
      const start = pieces[mid].start;
      // The end of a match belongs to the piece it finishes in, not the next one
      if (start < offset || (!isEnd && start === offset)) low = mid;
      else high = mid - 1;
    }
    const piece = pieces[low];
    return { node: piece.node, offset: offset - piece.start };
  }

  function findAll(query) {
    const { text, pieces } = pageText();
    const escaped = query.text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    // "Whole words": no letter, digit or underscore right before or after
    const pattern = query.wholeWords
      ? `(?<![\\p{L}\\p{N}_])${escaped}(?![\\p{L}\\p{N}_])`
      : escaped;
    const regex = new RegExp(pattern, query.matchCase ? "gu" : "giu");
    const ranges = [];
    for (const match of text.matchAll(regex)) {
      const start = spotAt(pieces, match.index, false);
      const end = spotAt(pieces, match.index + match[0].length, true);
      const range = document.createRange();
      range.setStart(start.node, start.offset);
      range.setEnd(end.node, end.offset);
      ranges.push(range);
    }
    return ranges;
  }

  function paintFind() {
    if (!window.CSS || !CSS.highlights) return;
    CSS.highlights.set("leafmd-find", new Highlight(...findRanges));
    if (findCurrent >= 0) {
      const current = new Highlight(findRanges[findCurrent]);
      current.priority = 1; // drawn on top of the other matches
      CSS.highlights.set("leafmd-find-current", current);
    } else {
      CSS.highlights.delete("leafmd-find-current");
    }
  }

  // Scroll the selected match into view if it's off screen
  function revealFind() {
    if (findCurrent < 0) return;
    const range = findRanges[findCurrent];
    // Code blocks and tables can scroll sideways, so bring those round first
    const holder = range.startContainer.parentElement?.closest("pre, table");
    if (holder) {
      const inner = range.getBoundingClientRect();
      const outer = holder.getBoundingClientRect();
      if (inner.left < outer.left || inner.right > outer.right) {
        holder.scrollLeft += inner.left - outer.left - outer.width / 3;
      }
    }
    const box = range.getBoundingClientRect();
    if (box.top < 0 || box.bottom > window.innerHeight) {
      window.scrollBy(0, box.top - window.innerHeight / 3);
    }
  }

  function reportFind() {
    post({ kind: "find", position: findCurrent + 1, count: findRanges.length });
  }

  // step 0: a new search, starting at the first match on screen.
  // step 1 or -1: the next or previous match, wrapping round at the ends.
  function find(text, matchCase, wholeWords, step) {
    const query = { text, matchCase, wholeWords };
    const same = findQuery && findQuery.text === text &&
      findQuery.matchCase === matchCase && findQuery.wholeWords === wholeWords;
    const count = findRanges.length;
    if (same && step !== 0 && count) {
      findCurrent = (findCurrent + step + count) % count;
    } else {
      findQuery = query;
      findRanges = findAll(query);
      findCurrent = findRanges.findIndex((range) => range.getBoundingClientRect().bottom > 0);
      if (findCurrent < 0 && findRanges.length) findCurrent = 0;
    }
    paintFind();
    revealFind();
    reportFind();
  }

  // The page was redrawn: find the same text again, keeping the same match if it's still there
  function findAgain() {
    if (!findQuery) return;
    findRanges = findAll(findQuery);
    findCurrent = Math.min(findCurrent, findRanges.length - 1);
    if (findCurrent < 0 && findRanges.length) findCurrent = 0;
    paintFind();
    reportFind();
  }

  function clearFind() {
    findQuery = null;
    findRanges = [];
    findCurrent = -1;
    if (window.CSS && CSS.highlights) {
      CSS.highlights.delete("leafmd-find");
      CSS.highlights.delete("leafmd-find-current");
    }
  }

  window.leafmd = {
    scrollToLine,
    setSplitView(on) { jumpOnDoubleClick = on; },
    find,
    findAgain,
    clearFind,
  };
})();

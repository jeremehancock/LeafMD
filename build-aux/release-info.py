#!/usr/bin/env python3
"""Reads LeafMD's version and release notes, and checks they agree.

The version number lives in two places:
  - Cargo.toml, which the About window shows
  - the newest <release> in the metainfo file, whose notes GNOME Software
    (and later Flathub) show to people

This script fails if those two disagree, so a release can't go out with
mismatched numbers. It's run by /ship before committing, and by the GitHub
build on every push.

A <release> marked type="development" is work in progress: merging it into
main publishes nothing. Removing that mark (and setting the date) is what
turns it into a real release.

Usage:
  build-aux/release-info.py          prints version=..., stable=true|false
  build-aux/release-info.py --notes  prints the newest release's notes as markdown
"""

import sys
import tomllib
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
METAINFO = ROOT / "data" / "io.github.jeremehancock.LeafMD.metainfo.xml"


def fail(message: str) -> None:
    print(f"release-info: {message}", file=sys.stderr)
    sys.exit(1)


def cargo_version() -> str:
    with open(ROOT / "Cargo.toml", "rb") as f:
        return tomllib.load(f)["package"]["version"]


def newest_release() -> ET.Element:
    releases = ET.parse(METAINFO).getroot().find("releases")
    if releases is None or len(releases) == 0:
        fail(f"{METAINFO.name} has no <release> entries")
    # The first entry is the newest one, as the AppStream format requires
    return releases[0]


def notes_as_markdown(release: ET.Element) -> str:
    """Turns the <description> of a release (paragraphs and bullet lists)
    into markdown for the GitHub Release page."""
    description = release.find("description")
    if description is None:
        return ""
    blocks = []
    for element in description:
        if element.tag == "p":
            blocks.append(" ".join((element.text or "").split()))
        elif element.tag in ("ul", "ol"):
            bullet = "-" if element.tag == "ul" else "1."
            items = [f"{bullet} {' '.join((li.text or '').split())}" for li in element]
            blocks.append("\n".join(items))
    return "\n\n".join(blocks) + "\n"


def main() -> None:
    version = cargo_version()
    release = newest_release()
    listed = release.get("version")
    if listed != version:
        fail(
            f"Cargo.toml says {version} but the newest release in "
            f"{METAINFO.name} is {listed}. Make them match."
        )
    if "--notes" in sys.argv:
        print(notes_as_markdown(release), end="")
        return
    stable = release.get("type", "stable") == "stable"
    print(f"version={version}")
    print(f"stable={'true' if stable else 'false'}")


if __name__ == "__main__":
    main()

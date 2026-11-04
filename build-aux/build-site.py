#!/usr/bin/env python3
"""Builds leafmd.app: the storefront, the install files and the repository.

Takes the storefront's source in site/ and turns it into the finished site:

  - copies site/ (leaving out the .in templates), the app icon as icon.svg,
    and the screenshots in data/screenshots/
  - fills in the newest release's version from the metainfo file (linked to
    its release notes on GitHub), so the site always names the version the
    repository offers
  - fills in LeafMD.flatpakref and leafmd.flatpakrepo (the "install from
    here" files) with the site's address and LeafMD's public signing key
  - copies the Flatpak repository into repo/, when given one
  - adds .nojekyll, so GitHub Pages serves every file as it is

The GitHub build runs it for https://leafmd.app/ on each release and when
the site is republished, and build-aux/flatpak-repo-preview.sh runs it for a
copy on this computer.

Usage:
  build-aux/build-site.py OUT [--base-url URL] [--repo REPO] [--key KEY]
                              [--release-tag TAG]

  OUT         folder to build the site in (emptied first)
  --base-url  the address the site is served at (default https://leafmd.app/)
  --repo      a signed Flatpak repository to publish under repo/
  --key       LeafMD's public signing key (needed for the install files)
  --release-tag
              take the version, app icon and screenshots from this git tag
              (like v0.1.0) instead of the working folder. A republish uses
              it, so the site's text and design are the latest while
              everything about the app matches the release being offered.
"""

import argparse
import base64
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
APP_ID = "io.github.jeremehancock.LeafMD"
# Paths from the top of the project, for reading them from a tag as well
METAINFO = f"data/{APP_ID}.metainfo.xml"
ICON = f"data/icons/hicolor/scalable/apps/{APP_ID}.svg"
SCREENSHOTS = "data/screenshots"
SITE = ROOT / "site"


def fail(message: str) -> None:
    print(f"build-site: {message}", file=sys.stderr)
    sys.exit(1)


class AppFiles:
    """The files that describe the app: from the working folder, or as they
    were at a release tag."""

    def __init__(self, tag: str | None) -> None:
        self.tag = tag
        if tag and subprocess.run(
            ["git", "-C", str(ROOT), "rev-parse", "--verify", "--quiet", f"{tag}^{{commit}}"],
            capture_output=True,
        ).returncode != 0:
            fail(f"there's no tag {tag} in this repository (git fetch --tags fetches them)")

    def read(self, path: str) -> bytes:
        if not self.tag:
            return (ROOT / path).read_bytes()
        return subprocess.run(
            ["git", "-C", str(ROOT), "show", f"{self.tag}:{path}"], capture_output=True, check=True
        ).stdout

    def screenshots(self) -> list[str]:
        if not self.tag:
            return sorted(f"{SCREENSHOTS}/{p.name}" for p in (ROOT / SCREENSHOTS).glob("*.png"))
        names = subprocess.run(
            ["git", "-C", str(ROOT), "ls-tree", "--name-only", self.tag, f"{SCREENSHOTS}/"],
            capture_output=True, text=True, check=True,
        ).stdout.split()
        return [name for name in names if name.endswith(".png")]

    def newest_version(self) -> str:
        """The version of the newest <release> in the metainfo file."""
        release = ET.fromstring(self.read(METAINFO)).find("releases/release")
        if release is None or not release.get("version"):
            fail(f"no <release> with a version in {METAINFO}")
        return release.get("version")


def fill(text: str, values: dict[str, str]) -> str:
    for name, value in values.items():
        text = text.replace("{{" + name + "}}", value)
    if "{{" in text:
        fail(f"a placeholder was left unfilled: {text[text.index('{{'):][:40]}")
    return text


def main() -> None:
    parser = argparse.ArgumentParser(description="Builds leafmd.app.")
    parser.add_argument("out", type=Path)
    parser.add_argument("--base-url", default="https://leafmd.app/")
    parser.add_argument("--repo", type=Path)
    parser.add_argument("--key", type=Path)
    parser.add_argument("--release-tag")
    args = parser.parse_args()

    app = AppFiles(args.release_tag)
    base_url = args.base_url if args.base_url.endswith("/") else args.base_url + "/"
    values = {"base_url": base_url, "version": app.newest_version()}
    if args.key:
        # Flatpak wants the key in its binary form, written as base64
        values["gpg_key"] = base64.b64encode(args.key.read_bytes()).decode()
    elif args.repo:
        fail("--key is needed with --repo, so the install files can check signatures")

    out = args.out
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    for source in SITE.iterdir():
        if source.suffix == ".in":
            # The install files only make sense with a repository to install from
            if args.repo:
                (out / source.stem).write_text(fill(source.read_text(), values))
        elif source.suffix in (".html", ".css", ".js"):
            text = source.read_text()
            if source.suffix == ".html":
                text = fill(text, values)
            (out / source.name).write_text(text)
        else:
            shutil.copy(source, out / source.name)

    (out / "icon.svg").write_bytes(app.read(ICON))
    (out / "screenshots").mkdir()
    for picture in app.screenshots():
        (out / "screenshots" / Path(picture).name).write_bytes(app.read(picture))

    if args.repo:
        shutil.copytree(args.repo, out / "repo", symlinks=True)

    (out / ".nojekyll").touch()
    source = f" (app files from {args.release_tag})" if args.release_tag else ""
    print(f"build-site: built version {values['version']}{source} for {base_url} in {out}")


if __name__ == "__main__":
    main()

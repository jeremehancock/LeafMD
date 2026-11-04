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

The GitHub build runs it for https://leafmd.app/ on each release, and
build-aux/flatpak-repo-preview.sh runs it for a copy on this computer.

Usage:
  build-aux/build-site.py OUT [--base-url URL] [--repo REPO] [--key KEY]

  OUT         folder to build the site in (emptied first)
  --base-url  the address the site is served at (default https://leafmd.app/)
  --repo      a signed Flatpak repository to publish under repo/
  --key       LeafMD's public signing key (needed for the install files)
"""

import argparse
import base64
import shutil
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
APP_ID = "io.github.jeremehancock.LeafMD"
METAINFO = ROOT / "data" / f"{APP_ID}.metainfo.xml"
ICON = ROOT / "data" / "icons" / "hicolor" / "scalable" / "apps" / f"{APP_ID}.svg"
SCREENSHOTS = ROOT / "data" / "screenshots"
SITE = ROOT / "site"


def fail(message: str) -> None:
    print(f"build-site: {message}", file=sys.stderr)
    sys.exit(1)


def newest_version() -> str:
    """The version of the newest <release> in the metainfo file."""
    release = ET.parse(METAINFO).getroot().find("releases/release")
    if release is None or not release.get("version"):
        fail(f"no <release> with a version in {METAINFO.name}")
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
    args = parser.parse_args()

    base_url = args.base_url if args.base_url.endswith("/") else args.base_url + "/"
    values = {"base_url": base_url, "version": newest_version()}
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

    shutil.copy(ICON, out / "icon.svg")
    (out / "screenshots").mkdir()
    for picture in SCREENSHOTS.glob("*.png"):
        shutil.copy(picture, out / "screenshots" / picture.name)

    if args.repo:
        shutil.copytree(args.repo, out / "repo", symlinks=True)

    (out / ".nojekyll").touch()
    print(f"build-site: built version {values['version']} for {base_url} in {out}")


if __name__ == "__main__":
    main()

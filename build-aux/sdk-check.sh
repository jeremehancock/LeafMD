#!/usr/bin/env bash
# Runs LeafMD's checks inside the GNOME SDK: the same Rust, GTK and
# libadwaita the Flatpak is built with. The GitHub build runs this on every
# push, and /ship runs it before committing, so both see the same results.
#
# Checks, in order:
#   - the version in Cargo.toml matches the newest release in the metainfo file
#   - (before those) libspelling is built, since LeafMD needs it and the SDK lacks it
#   - clippy (Rust's checker for mistakes and untidy code), with any warning
#     counted as a failure
#   - the unit tests
#   - the app store description and the .desktop file are valid
#
# Needs the GNOME SDK and its Rust extension, which the Flatpak build installs.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> Checking the version number"
build-aux/release-info.py

# The SDK version comes from the Flatpak recipe, so the two can't drift apart
runtime=$(python3 -c 'import json; print(json.load(open("build-aux/io.github.jeremehancock.LeafMD.json"))["runtime-version"])')

# The SDK can be installed for this user, for the whole system, or both (as on
# GitHub's build image), and flatpak stops to ask which one if it isn't told.
# Prefer the user copy: that's where the Flatpak build installs missing parts,
# like the Rust extension, when run with --user.
if flatpak info --user "org.gnome.Sdk//$runtime" > /dev/null 2>&1; then
    installation=--user
else
    installation=--system
fi

# libspelling (the spell checker for the raw text) isn't part of the GNOME SDK,
# so it's built here first, the same release the Flatpak recipe builds. Its
# address and checksum come from the recipe, so the two can't drift apart. The
# download happens out here, where it can be checked; the build happens inside
# the SDK below, once, and is reused until the release changes.
read -r spelling_url spelling_sha256 < <(python3 -c '
import json
recipe = json.load(open("build-aux/io.github.jeremehancock.LeafMD.json"))
source = next(m for m in recipe["modules"] if m["name"] == "libspelling")["sources"][0]
print(source["url"], source["sha256"])')
spelling_dir="target/sdk-libspelling/$(basename "$spelling_url" .tar.xz)"
if [[ ! -f "$spelling_dir/install/lib/pkgconfig/libspelling-1.pc" ]]; then
    echo "==> Downloading $(basename "$spelling_url")"
    rm -rf "$spelling_dir"
    mkdir -p "$spelling_dir"
    curl -sSfL "$spelling_url" -o "$spelling_dir/source.tar.xz"
    if ! echo "$spelling_sha256  $spelling_dir/source.tar.xz" | sha256sum --check --quiet; then
        echo "The libspelling download doesn't match the checksum in the Flatpak recipe." >&2
        exit 1
    fi
    tar -xf "$spelling_dir/source.tar.xz" -C "$spelling_dir" --strip-components=1
fi

# Everything Cargo writes goes under ./target, which git and the Flatpak build ignore
flatpak run "$installation" \
    --share=network \
    --filesystem="$PWD" \
    --env=CARGO_HOME="$PWD/target/sdk-cargo-home" \
    --env=CARGO_TARGET_DIR="$PWD/target/sdk" \
    --env=PATH=/usr/lib/sdk/rust-stable/bin:/usr/bin \
    --env=SPELLING_DIR="$PWD/$spelling_dir" \
    --command=bash \
    "org.gnome.Sdk//$runtime" -c '
set -euo pipefail
# Builds libspelling inside the SDK, unless an earlier run already did
if [[ ! -f "$SPELLING_DIR/install/lib/pkgconfig/libspelling-1.pc" ]]; then
    echo "==> Building libspelling"
    meson setup "$SPELLING_DIR/build" "$SPELLING_DIR" --prefix="$SPELLING_DIR/install" \
        --libdir=lib -Ddocs=false -Dintrospection=disabled -Dvapi=false -Dsysprof=false > /dev/null
    ninja -C "$SPELLING_DIR/build" install > /dev/null
fi
# So the Rust build finds it, and the tests can load it
export PKG_CONFIG_PATH="$SPELLING_DIR/install/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
export LD_LIBRARY_PATH="$SPELLING_DIR/install/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

# Use the copy of the Rust libraries in ./vendor, like the Flatpak build does
echo "==> Copying Rust libraries into ./vendor"
cargo vendor --locked vendor > /dev/null
offline=(--offline --locked
    --config "source.crates-io.replace-with=\"vendored-sources\""
    --config "source.vendored-sources.directory=\"vendor\"")

echo "==> Clippy"
cargo clippy "${offline[@]}" --all-targets -- -D warnings

echo "==> Tests"
cargo test "${offline[@]}"

echo "==> App store description and .desktop file"
appstreamcli validate --no-net data/io.github.jeremehancock.LeafMD.metainfo.xml
desktop-file-validate data/io.github.jeremehancock.LeafMD.desktop
'

echo "==> All checks passed"

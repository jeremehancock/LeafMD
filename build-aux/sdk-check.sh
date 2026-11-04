#!/usr/bin/env bash
# Runs LeafMD's checks inside the GNOME SDK: the same Rust, GTK and
# libadwaita the Flatpak is built with. The GitHub build runs this on every
# push, and /ship runs it before committing, so both see the same results.
#
# Checks, in order:
#   - the version in Cargo.toml matches the newest release in the metainfo file
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

# Everything Cargo writes goes under ./target, which git and the Flatpak build ignore
flatpak run \
    --share=network \
    --filesystem="$PWD" \
    --env=CARGO_HOME="$PWD/target/sdk-cargo-home" \
    --env=CARGO_TARGET_DIR="$PWD/target/sdk" \
    --env=PATH=/usr/lib/sdk/rust-stable/bin:/usr/bin \
    --command=bash \
    "org.gnome.Sdk//$runtime" -c '
set -euo pipefail
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

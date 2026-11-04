#!/usr/bin/env bash
# Builds LeafMD as a Flatpak and installs it for the current user.
#
# Flatpak builds can't download anything, so this first copies every Rust
# library LeafMD uses into ./vendor (cargo vendor), then builds from that copy.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> Copying Rust libraries into ./vendor"
cargo vendor --locked vendor > /dev/null

echo "==> Building and installing the Flatpak (the first build takes a while)"
flatpak-builder --user --install --force-clean \
    --state-dir=.flatpak-builder flatpak-build \
    build-aux/io.github.jeremehancock.LeafMD.json

echo "==> Done. Start LeafMD from your app grid, or run:"
echo "    flatpak run io.github.jeremehancock.LeafMD"

#!/usr/bin/env bash
# Builds a private copy of leafmd.app on this computer and serves it at
# http://localhost:8642/, so the storefront, the one-click install and
# updates can all be tried before anything is published.
#
# It does what the GitHub build does on a release, with two differences:
#   - it signs with a throwaway key made here (kept in
#     .flatpak-builder/preview/gnupg and reused, so updates check out),
#     never the real LeafMD key
#   - the site's address is http://localhost:8642/ instead of leafmd.app
#
# Run it again after changing something (or bumping the version) to publish
# a new build, and the installed preview copy is offered the update.
#
# The preview copy is installed as LeafMD's "stable" branch, next to your own
# build from flatpak-install.sh ("master"). The script prints how to remove
# it again.
set -euo pipefail
cd "$(dirname "$0")/.."

app_id=io.github.jeremehancock.LeafMD
port=8642
base_url="http://localhost:$port/"
# Everything goes under .flatpak-builder, which git and the Flatpak build
# ignore. The build folder must be on the same disk as flatpak-builder's
# state folder.
preview=.flatpak-builder/preview
export GNUPGHOME="$PWD/$preview/gnupg"

mkdir -p "$preview"

if [[ ! -f "$preview/key.gpg" ]]; then
    echo "==> Making a throwaway signing key for the preview"
    mkdir -p -m 700 "$GNUPGHOME"
    gpg --batch --passphrase '' --quick-gen-key \
        "LeafMD preview (not the real key) <preview@localhost>" rsa4096 sign never
    gpg --export > "$preview/key.gpg"
fi
key_id=$(gpg --list-keys --with-colons | awk -F: '/^pub/ { print $5; exit }')

echo "==> Copying Rust libraries into ./vendor"
cargo vendor --locked vendor > /dev/null

echo "==> Building and signing LeafMD (branch stable)"
flatpak-builder --user --force-clean --default-branch=stable \
    --state-dir=.flatpak-builder --repo="$preview/repo" \
    --gpg-sign="$key_id" --gpg-homedir="$GNUPGHOME" \
    "$preview/build" "build-aux/$app_id.json"
flatpak build-update-repo --title=LeafMD --generate-static-deltas --prune \
    --gpg-sign="$key_id" --gpg-homedir="$GNUPGHOME" "$preview/repo"

echo "==> Building the site"
build-aux/build-site.py "$preview/site" --base-url "$base_url" \
    --repo "$preview/repo" --key "$preview/key.gpg"

echo "==> Making the .flatpak file, set to update from the preview"
version=$(build-aux/release-info.py | sed -n 's/^version=//p')
flatpak build-bundle "$preview/repo" "$preview/site/LeafMD-$version.flatpak" "$app_id" stable \
    --repo-url="${base_url}repo/" --gpg-keys="$preview/key.gpg" \
    --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo

cat << EOF

The preview of leafmd.app is at $base_url (Ctrl+C stops it).

  Install from it:  open $base_url and click Install, or run
                    flatpak install --user ${base_url}LeafMD.flatpakref
  Try the file:     $preview/site/LeafMD-$version.flatpak
  Try an update:    stop this, change something, run this script again,
                    then flatpak update (or GNOME Software's Updates)

  Remove the preview copy afterwards:
                    flatpak uninstall --user $app_id//stable
                    flatpak remote-delete --user leafmd
  Your own build from flatpak-install.sh (branch master) is left alone. If
  the app grid opens the wrong one:
                    flatpak make-current --user $app_id master

EOF
exec python3 -m http.server "$port" --bind 127.0.0.1 --directory "$preview/site"

#!/usr/bin/env bash
# Runs Flathub's own checker, flatpak-builder-lint, so LeafMD keeps meeting
# Flathub's rules even though it's published from its own repository.
#
# Checks, in order:
#   - the Flatpak recipe (build-aux/io.github.jeremehancock.LeafMD.json)
#   - the app store description (the metainfo file)
#   - a finished build, if given the path of the repository it was exported
#     to (the GitHub build does this; locally it would need a second build)
#
# Any problem fails, except those in build-aux/flathub-lint-exceptions.json.
# It allows only one:
#   appstream-screenshots-not-mirrored-in-ostree
#     Flathub's servers keep their own copy of every app's screenshots inside
#     the repository. A self-hosted repository serves them from leafmd.app
#     instead, so this can never pass outside Flathub.
#
# Until LeafMD's first release (while GitHub has no v... tag), two more are
# allowed (and the app store description is checked without going online),
# because the checker downloads the screenshots and visits the
# homepage, and leafmd.app only goes live when the first release is published:
#   screenshot-image-not-found, url-not-reachable, and
#   appstream-missing-screenshots (the build leaves screenshots it couldn't
#   download out of the app catalogue, so the catalogue has none)
# Without this the first release's own checks would fail and stop it from
# ever publishing the site. Once a release exists, they're checked in full.
# (A screenshot added under a new file name still fails until it's released,
# so retakes keep the same four names.)
#
# The checker comes as the org.flatpak.Builder Flatpak (Flathub's supported
# way to get it), installed here if it's missing.
#
# Usage:
#   build-aux/flathub-lint.sh                  recipe and metainfo file
#   build-aux/flathub-lint.sh repo             also the build in ./repo
#   build-aux/flathub-lint.sh --gha-format repo   the same, with problems
#                                              shown as GitHub annotations
set -euo pipefail
cd "$(dirname "$0")/.."

app_id=io.github.jeremehancock.LeafMD
format=()
if [[ "${1:-}" == --gha-format ]]; then
    format=(--gha-format)
    shift
fi
repo="${1:-}"

# Install the checker for this user if it isn't installed anywhere. Flathub
# can be set up for this user, for the whole system or both, so try each.
if ! flatpak info org.flatpak.Builder > /dev/null 2>&1; then
    echo "==> Installing Flathub's checker (org.flatpak.Builder)"
    flatpak install --user -y --noninteractive flathub org.flatpak.Builder 2> /dev/null ||
        flatpak install --system -y --noninteractive flathub org.flatpak.Builder
fi

# The checker runs inside its own sandbox, so give it this folder (and the
# repository, which may be elsewhere) to read. This folder can't be read-only:
# checking the recipe writes a small cache to ./.flatpak-builder, which git
# and the Flatpak build already ignore.
access=(--filesystem="$PWD")
if [[ -n "$repo" ]]; then
    repo=$(realpath "$repo")
    access+=(--filesystem="$repo:ro")
fi

exceptions="$PWD/build-aux/flathub-lint-exceptions.json"

# Has LeafMD been released yet? Asks GitHub for v... tags. If GitHub can't be
# reached, the checker couldn't reach leafmd.app either, so stop and say so.
if ! release_tags=$(git ls-remote --tags origin 'refs/tags/v*'); then
    echo "Couldn't ask GitHub whether LeafMD has been released. Is this computer online?" >&2
    exit 1
fi
if [[ -z "$release_tags" ]]; then
    echo "==> No release yet, so leafmd.app isn't live: allowing its screenshots and homepage to be unreachable"
    mkdir -p target
    exceptions="$PWD/target/flathub-lint-exceptions.json"
    python3 - "$PWD/build-aux/flathub-lint-exceptions.json" "$exceptions" "$app_id" << 'EOF'
import json, sys
source, destination, app_id = sys.argv[1:]
with open(source) as f:
    exceptions = json.load(f)
exceptions[app_id] += [
    "screenshot-image-not-found",
    "url-not-reachable",
    "appstream-missing-screenshots",
]
with open(destination, "w") as f:
    json.dump(exceptions, f, indent=4)
EOF
fi

lint() {
    flatpak run "${access[@]}" --command=flatpak-builder-lint org.flatpak.Builder \
        "${format[@]}" --exceptions --user-exceptions "$exceptions" "$@"
}

echo "==> Flathub checks: the recipe"
lint manifest "build-aux/$app_id.json"

echo "==> Flathub checks: the app store description"
if [[ -z "$release_tags" ]]; then
    # The checker's appstream check is just "appstreamcli validate --explain"
    # and ignores the exceptions file, so before the first release run that
    # same command with --no-net, which skips visiting the screenshots and
    # homepage but checks everything else
    flatpak run "${access[@]}" --command=appstreamcli org.flatpak.Builder \
        validate --explain --no-net "data/$app_id.metainfo.xml"
else
    lint appstream "data/$app_id.metainfo.xml"
fi

if [[ -n "$repo" ]]; then
    echo "==> Flathub checks: the finished build"
    lint repo "$repo"
fi

echo "==> Flathub checks passed"

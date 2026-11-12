#!/usr/bin/env bash
# Tries the installed LeafMD the way a person would, and checks it still does
# what the specs in openspec/specs/ say. Each scenario is a small file in
# build-aux/end-to-end/ (its name is the file name without .sh).
#
#   build-aux/end-to-end-tests.sh                 runs every scenario
#   build-aux/end-to-end-tests.sh save tabs       runs only those
#
# It tests the installed Flatpak, so rebuild first: ./build-aux/flatpak-install.sh
# Nothing shows on screen, and any LeafMD you have open can stay open: it all
# happens on a hidden desktop with its own home folder (see test-desktop.sh).
#
# When a scenario fails, a picture of the hidden screen, LeafMD's messages and
# its saved settings are kept in target/end-to-end/<scenario>/.
set -uo pipefail
cd "$(dirname "$0")/.."

source build-aux/test-desktop.sh

mapfile -t known < <(cd build-aux/end-to-end && ls -- *.sh 2> /dev/null | sed 's/\.sh$//')
if (($# > 0)); then
    for name in "$@"; do
        if [[ ! -f "build-aux/end-to-end/$name.sh" ]]; then
            echo "There's no scenario called '$name'. These are the ones there are:" >&2
            printf '  %s\n' "${known[@]}" >&2
            exit 1
        fi
    done
    chosen=("$@")
else
    chosen=("${known[@]}")
fi
if ((${#chosen[@]} == 0)); then
    echo "There are no scenarios in build-aux/end-to-end/." >&2
    exit 1
fi

desktop_check_tools
evidence="$PWD/target/end-to-end"
rm -rf "$evidence"
desktop_start
start_settings_portal light "$work/logs/settings-portal"

passed=0
for name in "${chosen[@]}"; do
    home="$work/scenarios/$name"
    docs="$home/Documents"
    log="$work/logs/$name.log"
    new_home "$home"
    mkdir -p "$docs"
    rm -f "$work/failure"

    # In a subshell (a separate copy of this script), so a failed check can end
    # the scenario straight away without ending the whole run
    (
        source "build-aux/end-to-end/$name.sh"
        "$name"
    )
    result=$?

    if ((result == 0)); then
        echo "$name: passed"
        passed=$((passed + 1))
    else
        reason=$(cat "$work/failure" 2> /dev/null || echo "it stopped unexpectedly (exit code $result)")
        echo "$name: failed: $reason"
        mkdir -p "$evidence/$name"
        screen_picture "$evidence/$name/screen.png"
        cp "$log" "$evidence/$name/leafmd.log" 2> /dev/null || touch "$evidence/$name/leafmd.log"
        cp "$home/.var/app/$app/config/leafmd/state.ini" "$evidence/$name/" 2> /dev/null || true
    fi
    close_everything
done

desktop_stop
problems=0
desktop_check_real_untouched || problems=1

echo "$passed of ${#chosen[@]} passed"
if ((passed < ${#chosen[@]})); then
    echo "Pictures and messages from the failures are in $evidence"
    exit 1
fi
exit "$problems"

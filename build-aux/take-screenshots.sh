#!/usr/bin/env bash
# Takes LeafMD's four screenshots from the installed Flatpak, without showing
# anything on screen or changing anything on this computer.
#
#   build-aux/take-screenshots.sh           replaces the ones in data/screenshots/
#   build-aux/take-screenshots.sh <folder>  saves them somewhere else
#
# Rebuild first so they show the current code: ./build-aux/flatpak-install.sh
# Any LeafMD you have open can stay open. It takes about a minute. The pictures
# only replace the old ones once all four have worked, and data/screenshots/
# README.md has what to check before committing them.
#
# How it works: it starts a real GNOME Shell with no visible screen (see
# test-desktop.sh, which also explains how that stays away from the real
# desktop), because the shadow and rounded corners only appear on a real
# desktop. It opens LeafMD in it once per picture, clicks into the document the
# way a person would, and copies the window out the same way Print Screen does.
set -euo pipefail
cd "$(dirname "$0")/.."

source build-aux/test-desktop.sh

out=$(realpath -m "${1:-data/screenshots}")
desktop_check_tools
desktop_start
home="$work/home"
mkdir -p "$work/pictures"

fail() { desktop_fail "$1"; }

# The sample in the same place inside the temporary home, so the title bar still
# says ~/github/LeafMD/data/screenshots
sample="$home/github/LeafMD/data/screenshots/sample.md"
mkdir -p "$(dirname "$sample")"
cp "$repo/data/screenshots/sample.md" "$sample"

take() { # <picture name> <LeafMD's name for the view> <light or dark> <click x> <click y>
    local name=$1 view=$2 style=$3 click_x=$4 click_y=$5
    echo "==> $name"

    # LeafMD opens in whichever view and size its settings file says
    local data="$home/.var/app/$app"
    rm -rf "$data"
    mkdir -p "$data/config/leafmd"
    printf '[window]\nview=%s\nwidth=1000\nheight=700\nmaximized=false\n' "$view" \
        > "$data/config/leafmd/state.ini"

    start_settings_portal "$style" "$work/logs/portal-$name"
    local portal=$settings_portal
    start_leafmd "$home" "$work/logs/app-$name" "$sample"
    local running=$leafmd

    wait_for_answer 'global.get_window_actors().length' 1 30 ||
        fail "LeafMD's window didn't appear for $name. $(tail -3 "$work/logs/app-$name")"

    # GNOME Shell shows its overview whenever there are no windows
    in_shell "Main.overview.hide();
        $window.meta_window.activate(global.get_current_time()); 0" > /dev/null
    sleep 4

    # What Flathub allows, and what the checks in the README expect. As one
    # number, because a line of text comes back with extra quotes around it.
    wait_for_answer "let f = $window.meta_window.get_frame_rect();
        f.width * 10000 + f.height" 10000700 3 ||
        fail "The window for $name isn't 1000 by 700."

    # LeafMD's editor opens scrolled down a little, so scroll to the top. Then
    # click so the text cursor lands in the document and no button keeps a blue
    # focus outline. Then rest on empty header bar, because scrollbars stay
    # visible while the mouse moves over the document, and wait for them to fade.
    move_pointer "$click_x" "$click_y"
    sleep 0.3
    in_shell 'const C = imports.gi.Clutter;
        for (let i = 0; i < 8; i++)
            globalThis.pointer.notify_discrete_scroll(0, C.ScrollDirection.UP, C.ScrollSource.WHEEL);
        0' > /dev/null
    sleep 1
    in_shell 'const C = imports.gi.Clutter;
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.PRESSED);
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.RELEASED); 0' > /dev/null
    sleep 0.5
    move_pointer 250 24
    sleep 6

    # The window's own picture, with its shadow and transparent corners. This is
    # what Print Screen's Window button saves too.
    local picture="$work/pictures/$name.png"
    in_shell "const {Gio, Shell} = imports.gi;
        let texture = $window.paint_to_content(null).get_texture();
        let file = Gio.File.new_for_path('$picture').replace(null, false, Gio.FileCreateFlags.NONE, null);
        globalThis.captureError = '';
        Shell.Screenshot.composite_to_stream(texture, 0, 0, -1, -1, 1, null, 0, 0, 1, file)
            .then(() => file.close(null))
            .catch(error => { globalThis.captureError = String(error); });
        0" > /dev/null
    sleep 1
    local problem
    problem=$(in_shell 'globalThis.captureError')
    [[ "$problem" == *'""'* && -s "$picture" ]] || fail "Couldn't capture $name: $problem"

    # Close it the normal way. The document is unchanged, so nothing asks to save.
    in_shell "$window.meta_window.delete(global.get_current_time()); 0" > /dev/null
    wait_for_answer 'global.get_window_actors().length' 0 10 ||
        fail "LeafMD's window for $name didn't close."
    wait "$running" || true
    kill "$portal" 2> /dev/null || true
    wait "$portal" 2> /dev/null || true
}

# The click positions land on: a blank part of the page in Formatted, and just
# before the "O" in "Our plan" on line 3 in the others (Raw view's text sits in
# a centred column, so its click is further right). If the header bar or
# formatting bar changes height, or the raw text's margins, line spacing or
# column width change, they need changing too.
take formatted preview light 25 650
take raw source light 61 153
take split split light 19 153
take split-dark split dark 19 153

# Close the hidden desktop before checking, then look for any effect on the real one.
desktop_stop
problems=0
desktop_check_real_untouched || problems=1

mkdir -p "$out"
cp "$work/pictures/"*.png "$out/"
echo "Saved four pictures in $out"
echo "Check them against the list in data/screenshots/README.md before committing."
exit "$problems"

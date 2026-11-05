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
# How it works: it starts a real GNOME Shell with no visible screen, because the
# shadow and rounded corners only appear on a real desktop. It opens LeafMD in
# it once per picture, clicks into the document the way a person would, and
# copies the window out the same way Print Screen does.
#
# How it keeps away from the real desktop. All of this matters: a test on a
# private message bus once crashed the real document portal (the Flatpak service
# that gives apps access to files), and every Flatpak app failed to start.
#   - env -i starts everything with an empty environment, so nothing inherits
#     the real desktop's message bus, display or runtime folder.
#   - The private message bus (D-Bus) has no list of services it can start, so
#     no second document portal can appear on it.
#   - XDG_RUNTIME_DIR is private, so the real /run/user/<id>/doc is out of reach.
#   - HOME is private, so LeafMD's saved settings and recent files (kept in
#     ~/.var/app) aren't touched. Pointing XDG_CONFIG_HOME somewhere else isn't
#     enough, because LeafMD then still uses the real ones.
#   - GSETTINGS_BACKEND=memory means GNOME Shell can't save any settings.
# The checks at the end confirm it, and fail loudly if they find otherwise.
set -euo pipefail
cd "$(dirname "$0")/.."

app=io.github.jeremehancock.LeafMD
repo=$PWD
out=$(realpath -m "${1:-data/screenshots}")
real_data="$HOME/.var/app/$app"

for tool in dbus-daemon gdbus gnome-shell flatpak python3; do
    if ! command -v "$tool" > /dev/null; then
        echo "This needs $tool, which isn't installed." >&2
        exit 1
    fi
done
if ! flatpak info "$app" > /dev/null 2>&1; then
    echo "LeafMD isn't installed as a Flatpak. Run ./build-aux/flatpak-install.sh first." >&2
    exit 1
fi

# Socket paths have a length limit of about 100 characters, so keep this short
work=$(mktemp -d /tmp/leafmd-shots.XXXXXX)
chmod 700 "$work"
home="$work/home"
mkdir -p "$work"/{run,pictures,logs} "$home"/{.config,.cache,.local/share,.local/state}

processes=()
stop_everything() {
    # Only the processes started here, by their exact IDs
    for process in "${processes[@]}"; do
        kill "$process" 2> /dev/null || true
    done
    sleep 1
    for process in "${processes[@]}"; do
        kill -9 "$process" 2> /dev/null || true
    done
    rm -rf "$work"
}
trap stop_everything EXIT

# For the checks at the end
touch "$work/started"
real_portal=$(systemctl --user show -p MainPID --value xdg-document-portal.service 2> /dev/null || true)

private=(env -i HOME="$home" PATH="$PATH" USER="$USER" LANG="${LANG:-C.UTF-8}"
    XDG_RUNTIME_DIR="$work/run"
    XDG_CONFIG_HOME="$home/.config" XDG_CACHE_HOME="$home/.cache"
    XDG_DATA_HOME="$home/.local/share" XDG_STATE_HOME="$home/.local/state"
    DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus"
    GSETTINGS_BACKEND=memory GIO_USE_VFS=local GVFS_DISABLE_FUSE=1 NO_AT_BRIDGE=1 GTK_A11Y=none)

# No <servicedir> and no <standard_session_servicedirs/>: this is what stops the
# bus starting anything by itself. The policy allows everything, but only the
# programs started below can reach it.
cat > "$work/bus.conf" << EOF
<!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-Bus Bus Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
<busconfig>
  <type>session</type>
  <listen>unix:path=$work/run/bus</listen>
  <policy context="default">
    <allow send_destination="*" eavesdrop="true"/>
    <allow eavesdrop="true"/>
    <allow own="*"/>
  </policy>
</busconfig>
EOF

"${private[@]}" dbus-daemon --nofork --config-file="$work/bus.conf" > "$work/logs/bus" 2>&1 &
processes+=($!)
sleep 1

# Plenty big enough for the window and its shadow, at normal (not HiDPI) scale
"${private[@]}" gnome-shell --headless --no-x11 --unsafe-mode \
    --wayland-display leafmd --virtual-monitor 1400x1000 > "$work/logs/shell" 2>&1 &
processes+=($!)

# Runs a line of JavaScript inside the hidden GNOME Shell and prints the answer.
# --unsafe-mode is what allows this, and this Shell only exists on the private bus.
in_shell() {
    timeout 15 "${private[@]}" gdbus call --session --dest org.gnome.Shell \
        --object-path /org/gnome/Shell --method org.gnome.Shell.Eval "$1" 2>&1 || true
}

wait_for_answer() { # <javascript> <answer> <seconds to wait>
    for ((i = 0; i < $3; i++)); do
        [[ "$(in_shell "$1")" == *"'$2'"* ]] && return 0
        sleep 1
    done
    return 1
}

fail() {
    echo "$1" >&2
    echo "--- GNOME Shell's last messages:" >&2
    tail -5 "$work/logs/shell" >&2
    exit 1
}

wait_for_answer '1+1' 2 30 || fail "The hidden GNOME Shell didn't start."

# A mouse, and a keyboard that never gets pressed. With no keyboard at all, the
# editor doesn't count as having typing focus, and only then does it draw the
# highlight on the current line.
in_shell 'const C = imports.gi.Clutter;
    const seat = C.get_default_backend().get_default_seat();
    globalThis.pointer = seat.create_virtual_device(C.InputDeviceType.POINTER_DEVICE);
    globalThis.keyboard = seat.create_virtual_device(C.InputDeviceType.KEYBOARD_DEVICE); 0' > /dev/null

window='global.get_window_actors()[0]'

# Positions are measured from the window's top left corner, not counting its shadow
move_pointer() { # <x> <y>
    in_shell "let f = $window.meta_window.get_frame_rect();
        globalThis.pointer.notify_absolute_motion(0, f.x + $1, f.y + $2); 0" > /dev/null
}

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

    "${private[@]}" python3 -I "$repo/build-aux/screenshot-settings-portal.py" "$style" \
        > "$work/logs/portal-$name" 2>&1 &
    local portal=$!
    processes+=("$portal")
    sleep 0.5

    # FLATPAK_USER_DIR: HOME is temporary, so say where the installed apps are.
    # WebKit's own inner sandbox relies on a service this bus doesn't have. It's
    # safe to turn off here, because the only document opened is our own sample.
    "${private[@]}" FLATPAK_USER_DIR="$HOME/.local/share/flatpak" WAYLAND_DISPLAY=leafmd \
        flatpak run --env=WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS=1 "$app" "$sample" \
        > "$work/logs/app-$name" 2>&1 &
    local running=$!
    processes+=("$running")

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
# before the "O" in "Our plan" on line 3 in the others. If the header bar or
# formatting bar changes height, they need changing too.
take formatted preview light 25 650
take raw source light 38 143
take split split light 38 143
take split-dark split dark 38 143

# Close the hidden desktop before checking, then look for any effect on the real one.
stop_processes() {
    for process in "${processes[@]}"; do
        kill "$process" 2> /dev/null || true
    done
}
stop_processes
sleep 1

problems=0
if [[ -d "$real_data" ]]; then
    changed=$(find "$real_data" -newer "$work/started" -type f | head -5)
    if [[ -n "$changed" ]]; then
        echo "WARNING: LeafMD's real saved data changed during this run:" >&2
        echo "$changed" >&2
        echo "If you were using LeafMD at the time that's expected. Otherwise, something got out." >&2
        problems=1
    fi
fi
now_portal=$(systemctl --user show -p MainPID --value xdg-document-portal.service 2> /dev/null || true)
if [[ "$now_portal" != "$real_portal" ]]; then
    echo "WARNING: the real document portal restarted during this run ($real_portal, now $now_portal)." >&2
    problems=1
fi
if ! flatpak run --command=true "$app" 2> /dev/null; then
    echo "WARNING: Flatpak apps aren't starting. To fix it:" >&2
    echo "  systemctl --user restart xdg-document-portal.service" >&2
    problems=1
fi

mkdir -p "$out"
cp "$work/pictures/"*.png "$out/"
echo "Saved four pictures in $out"
echo "Check them against the list in data/screenshots/README.md before committing."
exit "$problems"

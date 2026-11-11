# A hidden GNOME desktop for running the installed LeafMD Flatpak without
# showing anything on screen or changing anything on this computer. It isn't a
# script to run by itself: take-screenshots.sh and end-to-end-tests.sh load it
# with `source`, then call desktop_start.
#
# It starts a real GNOME Shell with no visible screen, so LeafMD runs the way it
# does on a real desktop (with window shadows and rounded corners too). Things
# happen inside it by running small bits of JavaScript in that Shell.
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
#   - Processes are stopped by their exact process IDs, never by name, so the
#     owner's own LeafMD can't be closed by mistake.
#   - Never open a file dialog (Open, Save As, Open Folder) in it. On a past
#     test, one appeared to go to the real desktop's portal instead.
# desktop_check_real_untouched confirms it afterwards, and says loudly if not.

app=io.github.jeremehancock.LeafMD
repo=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
real_data="$HOME/.var/app/$app"
# FLATPAK_USER_DIR: HOME is private, so say where the installed apps really are
real_flatpaks="$HOME/.local/share/flatpak"

# Stops with a message if something this needs is missing
desktop_check_tools() {
    local tool
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
}

# Runs a command in the empty, private environment, with <home> as its home
# folder. Each end-to-end scenario gets its own home; GNOME Shell and the
# message bus use $work/home.
run_private() { # <home> <command...>
    local home=$1
    shift
    env -i HOME="$home" PATH="$PATH" USER="$USER" LANG="${LANG:-C.UTF-8}" \
        XDG_RUNTIME_DIR="$work/run" \
        XDG_CONFIG_HOME="$home/.config" XDG_CACHE_HOME="$home/.cache" \
        XDG_DATA_HOME="$home/.local/share" XDG_STATE_HOME="$home/.local/state" \
        DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus" \
        GSETTINGS_BACKEND=memory GIO_USE_VFS=local GVFS_DISABLE_FUSE=1 NO_AT_BRIDGE=1 GTK_A11Y=none \
        "$@"
}

new_home() { # <folder>
    mkdir -p "$1"/{.config,.cache,.local/share,.local/state}
}

# Every process started here, by its exact ID. Kept in a file rather than a
# variable, so processes started inside an end-to-end scenario (which runs in a
# subshell, a separate copy of the script) are still stopped at the end.
track() { # <process ID>
    echo "$1" >> "$work/processes"
}

# Only the processes started here, by their exact IDs
desktop_stop() {
    local process
    [[ -f "$work/processes" ]] || return 0
    while read -r process; do
        kill "$process" 2> /dev/null || true
    done < "$work/processes"
    sleep 1
    while read -r process; do
        kill -9 "$process" 2> /dev/null || true
    done < "$work/processes"
    rm -f "$work/processes"
}

desktop_cleanup() {
    desktop_stop
    [[ -n "${work:-}" ]] && rm -rf "$work"
}

desktop_fail() {
    echo "$1" >&2
    echo "--- GNOME Shell's last messages:" >&2
    tail -5 "$work/logs/shell" >&2
    exit 1
}

# Starts the private message bus and the hidden GNOME Shell, with a virtual
# mouse and keyboard. Everything is removed again when the script exits.
desktop_start() {
    # Socket paths have a length limit of about 100 characters, so keep this short
    work=$(mktemp -d /tmp/leafmd-test.XXXXXX)
    chmod 700 "$work"
    mkdir -p "$work"/{run,logs}
    new_home "$work/home"
    trap desktop_cleanup EXIT

    # For desktop_check_real_untouched
    touch "$work/started"
    real_portal=$(systemctl --user show -p MainPID --value xdg-document-portal.service 2> /dev/null || true)

    # No <servicedir> and no <standard_session_servicedirs/>: this is what stops the
    # bus starting anything by itself. The policy allows everything, but only the
    # programs started here can reach it.
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

    run_private "$work/home" dbus-daemon --nofork --config-file="$work/bus.conf" \
        > "$work/logs/bus" 2>&1 &
    track $!
    sleep 1

    # Plenty big enough for the window and its shadow, at normal (not HiDPI) scale
    run_private "$work/home" gnome-shell --headless --no-x11 --unsafe-mode \
        --wayland-display leafmd --virtual-monitor 1400x1000 > "$work/logs/shell" 2>&1 &
    track $!

    wait_for_answer '1+1' 2 30 || desktop_fail "The hidden GNOME Shell didn't start."

    # A mouse, and a keyboard. Even when nothing types, the keyboard matters: with
    # no keyboard at all, the editor doesn't count as having typing focus, and
    # only then does it draw the highlight on the current line.
    in_shell 'const C = imports.gi.Clutter;
        const seat = C.get_default_backend().get_default_seat();
        globalThis.pointer = seat.create_virtual_device(C.InputDeviceType.POINTER_DEVICE);
        globalThis.keyboard = seat.create_virtual_device(C.InputDeviceType.KEYBOARD_DEVICE); 0' > /dev/null
}

# Runs a line of JavaScript inside the hidden GNOME Shell and prints the answer.
# --unsafe-mode is what allows this, and this Shell only exists on the private bus.
in_shell() {
    timeout 15 env -i PATH="$PATH" DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus" \
        gdbus call --session --dest org.gnome.Shell \
        --object-path /org/gnome/Shell --method org.gnome.Shell.Eval "$1" 2>&1 || true
}

wait_for_answer() { # <javascript> <answer> <seconds to wait>
    local i
    for ((i = 0; i < $3; i++)); do
        [[ "$(in_shell "$1")" == *"'$2'"* ]] && return 0
        sleep 1
    done
    return 1
}

# The first LeafMD window. Positions are measured from its top left corner, not
# counting its shadow.
window='global.get_window_actors()[0]'

move_pointer() { # <x> <y>
    in_shell "let f = $window.meta_window.get_frame_rect();
        globalThis.pointer.notify_absolute_motion(0, f.x + $1, f.y + $2); 0" > /dev/null
}

# Stands in for the desktop's settings portal, so LeafMD gets GNOME's default
# fonts, title bar buttons and the given style. Sets $settings_portal to its
# process ID.
start_settings_portal() { # <light or dark> <log file>
    run_private "$work/home" python3 -I "$repo/build-aux/screenshot-settings-portal.py" "$1" \
        > "$2" 2>&1 &
    settings_portal=$!
    track "$settings_portal"
    sleep 0.5
}

# Starts the installed LeafMD on the hidden desktop, without waiting for its
# window. Sets $leafmd to the process ID. WebKit's own inner sandbox relies on a
# service the private bus doesn't have. It's safe to turn off here, because
# only our own sample files are ever opened.
start_leafmd() { # <home> <log file> <arguments for LeafMD...>
    local home=$1 log=$2
    shift 2
    run_private "$home" FLATPAK_USER_DIR="$real_flatpaks" WAYLAND_DISPLAY=leafmd \
        flatpak run --env=WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS=1 "$app" "$@" \
        < /dev/null >> "$log" 2>&1 &
    leafmd=$!
    track "$leafmd"
}

# After desktop_stop: checks nothing reached the real desktop. Prints a warning
# for each problem and returns how many there were.
desktop_check_real_untouched() {
    local problems=0 changed now_portal
    if [[ -d "$real_data" ]]; then
        changed=$(find "$real_data" -newer "$work/started" -type f | head -5)
        if [[ -n "$changed" ]]; then
            echo "WARNING: LeafMD's real saved data changed during this run:" >&2
            echo "$changed" >&2
            echo "If you were using LeafMD at the time that's expected. Otherwise, something got out." >&2
            problems=$((problems + 1))
        fi
    fi
    now_portal=$(systemctl --user show -p MainPID --value xdg-document-portal.service 2> /dev/null || true)
    if [[ "$now_portal" != "$real_portal" ]]; then
        echo "WARNING: the real document portal restarted during this run ($real_portal, now $now_portal)." >&2
        echo "If Flatpak apps stop starting, run: systemctl --user restart xdg-document-portal.service" >&2
        problems=$((problems + 1))
    fi
    if ! flatpak run --command=true "$app" 2> /dev/null; then
        echo "WARNING: Flatpak apps aren't starting. To fix it:" >&2
        echo "  systemctl --user restart xdg-document-portal.service" >&2
        problems=$((problems + 1))
    fi
    return "$problems"
}

# ---------------------------------------------------------------------------
# Helpers for end-to-end scenarios (build-aux/end-to-end/*.sh). Each scenario
# runs with these set by end-to-end-tests.sh:
#   $home   its own fresh home folder
#   $docs   the folder its sample files go in ($home/Documents)
#   $log    where LeafMD's messages go
# A failed check calls fail, which ends the scenario there.

fail() { # <what went wrong>
    echo "$1" > "$work/failure"
    exit 1
}

# Keeps checking a condition (a line of Bash, run afresh each time) for up to
# <seconds>, so checks don't fail just because the hidden desktop is a little
# slow. Returns whether it ever came true.
wait_until() { # <seconds> <condition>
    local tries=$(($1 * 4))
    while ((tries-- > 0)); do
        eval "$2" && return 0
        sleep 0.25
    done
    eval "$2"
}

# The answer to a line of JavaScript that gives text, without gdbus's quoting
shell_text() {
    in_shell "$1" | python3 -I -c '
import json, sys
# The answer is "(true, " then the result as JSON inside single quotes
answer = sys.stdin.read().strip()
start = "(true, \x27"
if answer.startswith(start) and answer.endswith("\x27)"):
    inner = answer[len(start):-2].replace("\\\x27", "\x27")
    try:
        print(json.loads(inner))
    except ValueError:
        pass'
}

# Every window's title, separated by " | "
titles() {
    shell_text 'global.get_window_actors().map(a => a.meta_window.get_title()).join(" | ")'
}

window_count() {
    local count
    count=$(shell_text 'String(global.get_window_actors().length)')
    echo "${count:-0}"
}

# The window keys go to: the focused one, or else the newest
focused='(global.display.focus_window || global.get_window_actors().at(-1).meta_window)'

leafmd_on_bus() {
    timeout 5 env -i DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus" \
        gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus \
        --method org.freedesktop.DBus.NameHasOwner "$app" 2> /dev/null | grep -q true
}

# Writes a sample file into the scenario's Documents folder, exactly as given.
# With no line break at the end, Ctrl+End lands at the end of the last line.
sample() { # <file name> <text>
    mkdir -p "$docs"
    printf '%s' "$2" > "$docs/$1"
}

# Starts LeafMD with files from the Documents folder (or options such as
# --new-window, or nothing), the way a terminal or file manager would, then
# waits for it to settle and gives its window the keyboard.
open_leafmd() { # [file names or options...]
    local args=() arg before
    for arg in "$@"; do
        if [[ "$arg" == -* || "$arg" == /* ]]; then args+=("$arg"); else args+=("$docs/$arg"); fi
    done
    before=$(window_count)
    start_leafmd "$home" "$log" "${args[@]}"
    if [[ "$before" == 0 ]]; then
        wait_until 30 '(( $(window_count) >= 1 ))' || fail "LeafMD's window didn't appear."
    else
        # A second start hands its files to the LeafMD already running, then exits
        wait_until 15 "! kill -0 $leafmd 2> /dev/null" ||
            fail "The second start of LeafMD didn't hand over and exit."
    fi
    sleep 2
    in_shell "Main.overview.hide(); $focused.activate(global.get_current_time()); 0" > /dev/null
    sleep 0.5
    # The first key press in a new window is sometimes lost, so spend it on Shift
    press Shift_L
}

# Presses a key, with any of ctrl, shift and alt held down. Keys are written the
# way GTK names them: "ctrl+s", "ctrl+shift+Home", "Return", "Page_Down", "F3".
press() { # <keys>
    local keys=$1 mods=() key
    while [[ "$keys" == *+?* ]]; do
        case "${keys%%+*}" in
            ctrl) mods+=(Control_L) ;;
            shift) mods+=(Shift_L) ;;
            alt) mods+=(Alt_L) ;;
            *) fail "press doesn't know the modifier '${keys%%+*}'." ;;
        esac
        keys=${keys#*+}
    done
    key=$keys
    local js='const C = imports.gi.Clutter; const k = globalThis.keyboard;
        const code = name => name.length == 1 ? C.unicode_to_keysym(name.codePointAt(0)) : C["KEY_" + name];'
    for m in "${mods[@]}"; do js+=" k.notify_keyval(0, C.KEY_$m, C.KeyState.PRESSED);"; done
    js+=" k.notify_keyval(0, code('$key'), C.KeyState.PRESSED); k.notify_keyval(0, code('$key'), C.KeyState.RELEASED);"
    for m in "${mods[@]}"; do js+=" k.notify_keyval(0, C.KEY_$m, C.KeyState.RELEASED);"; done
    in_shell "$js 0" > /dev/null
    sleep 0.3
}

# Types text one character at a time. A new line presses Enter. Each key goes
# in its own call: sent all at once, GNOME Shell dropped the letters.
type_text() { # <text>
    local text=$1 i ch key
    for ((i = 0; i < ${#text}; i++)); do
        ch=${text:i:1}
        case "$ch" in
            $'\n') key='C.KEY_Return' ;;
            "'") key='C.KEY_apostrophe' ;;
            '\\') key='C.KEY_backslash' ;;
            *) key="C.unicode_to_keysym('$ch'.codePointAt(0))" ;;
        esac
        in_shell "const C = imports.gi.Clutter; const k = globalThis.keyboard;
            k.notify_keyval(0, $key, C.KeyState.PRESSED);
            k.notify_keyval(0, $key, C.KeyState.RELEASED); 0" > /dev/null
    done
    sleep 0.3
}

# Clicks into the middle of the focused window's raw text, so typing goes there.
# Typing doesn't, otherwise: a freshly opened file has the keyboard on the side
# panel button. Use Raw view (ctrl+2) first, so the middle is the raw text.
click_text() {
    click_window "f.width * 0.6" "f.height * 0.5"
}

# Clicks a point in the focused window, measured from its top left corner (not
# counting its shadow). Each position can be a sum using the window's frame f,
# such as "f.width - 130" for 130 pixels in from the right edge.
click_window() { # <x> <y>
    in_shell "let f = $focused.get_frame_rect();
        globalThis.pointer.notify_absolute_motion(0, f.x + ($1), f.y + ($2)); 0" > /dev/null
    sleep 0.3
    in_shell 'const C = imports.gi.Clutter;
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.PRESSED);
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.RELEASED); 0' > /dev/null
    sleep 0.3
}

# Presses the button at a point in the focused window (measured like
# click_window), moves the pointer sideways by <dx> pixels in five steps, the
# way a hand would, and lets go. Negative <dx> drags to the left.
drag_window() { # <x> <y> <dx>
    in_shell "let f = $focused.get_frame_rect();
        globalThis.pointer.notify_absolute_motion(0, f.x + ($1), f.y + ($2)); 0" > /dev/null
    sleep 0.3
    in_shell 'const C = imports.gi.Clutter;
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.PRESSED); 0' > /dev/null
    sleep 0.2
    local step
    for step in 1 2 3 4 5; do
        in_shell "let f = $focused.get_frame_rect();
            globalThis.pointer.notify_absolute_motion(0, f.x + ($1) + ($3) * $step / 5, f.y + ($2)); 0" > /dev/null
        sleep 0.1
    done
    sleep 0.2
    in_shell 'const C = imports.gi.Clutter;
        globalThis.pointer.notify_button(0, C.BUTTON_PRIMARY, C.ButtonState.RELEASED); 0' > /dev/null
    sleep 0.3
}

# Closes the focused window the way its close button does
# Turns the mouse wheel one notch over wherever the pointer is (see
# move_pointer), holding Ctrl if asked. Ctrl goes down a moment before the
# wheel turns: pressed at the same instant, GTK didn't see it held.
wheel() { # <up or down> [ctrl]
    local direction=${1^^}
    if [[ "${2:-}" == ctrl ]]; then
        in_shell 'const C = imports.gi.Clutter;
            globalThis.keyboard.notify_keyval(0, C.KEY_Control_L, C.KeyState.PRESSED); 0' > /dev/null
        sleep 0.3
    fi
    in_shell "const C = imports.gi.Clutter;
        globalThis.pointer.notify_discrete_scroll(0, C.ScrollDirection.$direction, C.ScrollSource.WHEEL); 0" > /dev/null
    sleep 0.3
    if [[ "${2:-}" == ctrl ]]; then
        in_shell 'const C = imports.gi.Clutter;
            globalThis.keyboard.notify_keyval(0, C.KEY_Control_L, C.KeyState.RELEASED); 0' > /dev/null
    fi
    sleep 0.3
}

close_window() {
    local before
    before=$(window_count)
    in_shell "$focused.delete(global.get_current_time()); 0" > /dev/null
    wait_until 10 '(( $(window_count) < before ))' || fail "The window didn't close."
}

title() {
    shell_text "$focused.get_title()"
}

file_text() { # <file name>
    cat "$docs/$1" 2> /dev/null
}

expect_title() { # <text the focused window's title should contain>
    local want=$1
    wait_until 5 '[[ "$(title)" == *"$want"* ]]' ||
        fail "the title is '$(title)', not one containing '$want'"
}

expect_title_lacks() { # <text the focused window's title shouldn't contain>
    local unwanted=$1
    wait_until 3 '[[ "$(title)" != *"$unwanted"* ]]' ||
        fail "the title is '$(title)', which contains '$unwanted'"
}

expect_windows() { # <number>
    local want=$1
    wait_until 5 '(( $(window_count) == want ))' ||
        fail "there are $(window_count) windows, not $want"
}

expect_file() { # <file name> <exactly the text it should hold>
    local name=$1 want=$2
    wait_until 5 '[[ "$(file_text "$name")" == "$want" ]]' ||
        fail "$name holds '$(file_text "$name")', not '$want'"
}

expect_file_contains() { # <file name> <text>
    local name=$1 want=$2
    wait_until 5 '[[ "$(file_text "$name")" == *"$want"* ]]' ||
        fail "$name doesn't contain '$want' (it holds '$(file_text "$name")')"
}

expect_file_lacks() { # <file name> <text>
    local name=$1 unwanted=$2
    wait_until 2 '[[ "$(file_text "$name")" != *"$unwanted"* ]]' ||
        fail "$name contains '$unwanted' (it holds '$(file_text "$name")')"
}

# What LeafMD saved for next time, from the state.ini in the scenario's home
setting() { # <section> <key>
    python3 -I - "$home/.var/app/$app/config/leafmd/state.ini" "$1" "$2" << 'PY'
import configparser, sys
ini = configparser.ConfigParser(interpolation=None)
ini.read(sys.argv[1])
print(ini.get(sys.argv[2], sys.argv[3], fallback=""))
PY
}

expect_setting() { # <section> <key> <value>
    local section=$1 key=$2 want=$3
    wait_until 5 '[[ "$(setting "$section" "$key")" == "$want" ]]' ||
        fail "state.ini has $section $key = '$(setting "$section" "$key")', not '$want'"
}

# A picture of the whole hidden screen, menus and dialogs included
screen_picture() { # <file>
    in_shell "const {Gio, Shell} = imports.gi;
        let file = Gio.File.new_for_path('$1').replace(null, false, Gio.FileCreateFlags.NONE, null);
        new Shell.Screenshot().screenshot(false, file).then(() => file.close(null)); 0" > /dev/null
    sleep 1.5
}

# Closes every LeafMD window and waits for LeafMD to leave the message bus, so
# the next scenario starts a fresh copy with its own home. A window that won't
# close (an unsaved changes question) is ended by its exact process ID, which
# the hidden desktop knows because the window is on it.
close_everything() {
    in_shell 'global.get_window_actors().forEach(a => a.meta_window.delete(global.get_current_time())); 0' > /dev/null
    if ! wait_until 5 '(( $(window_count) == 0 ))'; then
        local pid
        for pid in $(shell_text 'global.get_window_actors().map(a => a.meta_window.get_pid()).join(" ")'); do
            [[ "$pid" =~ ^[0-9]+$ && "$pid" -gt 1 ]] && kill -9 "$pid" 2> /dev/null
        done
    fi
    wait_until 15 '! leafmd_on_bus' ||
        echo "warning: LeafMD was still running after its windows closed" >&2
}

# The current state of one of a LeafMD window's actions, as GTK shares it on the
# message bus: for example "split" for the "view" action, or "true" for
# "side-panel" while the panel shows. Windows are numbered from 1 in the order
# the running LeafMD opened them.
action_state() { # <action> [window number]
    timeout 5 env -i DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus" \
        gdbus call --session --dest "$app" --object-path "/io/github/jeremehancock/LeafMD/window/${2:-1}" \
        --method org.gtk.Actions.Describe "$1" 2> /dev/null |
        sed -n -e "s/.*\[<'\(.*\)'>\].*/\1/p" -e "s/.*\[<\(true\|false\)>\].*/\1/p"
}

# Runs one of a LeafMD window's actions the way its menu item or button does,
# for the few things that have no key (menus and small buttons are hard to aim
# at on the hidden desktop). Keys are better where there is one, because they
# test the shortcut too.
# Some actions take a word saying which: "format" takes "task-list" for the
# Checklist button, for example. Always the first window.
activate_action() { # <action> [word]
    local parameter='[]'
    [[ -n "${2:-}" ]] && parameter="[<'$2'>]"
    timeout 5 env -i DBUS_SESSION_BUS_ADDRESS="unix:path=$work/run/bus" \
        gdbus call --session --dest "$app" --object-path /io/github/jeremehancock/LeafMD/window/1 \
        --method org.gtk.Actions.Activate "$1" "$parameter" '{}' > /dev/null 2>&1 ||
        fail "couldn't run LeafMD's $1 action"
    sleep 0.5
}

# Puts text on the hidden desktop's clipboard, ready for Ctrl+V
copy_text() { # <text>
    local text
    text=$(printf '%s' "$1" | python3 -I -c 'import json, sys; print(json.dumps(sys.stdin.read()))')
    in_shell "const St = imports.gi.St;
        St.Clipboard.get_default().set_text(St.ClipboardType.CLIPBOARD, $text); 0" > /dev/null
    sleep 0.3
}

expect_action_state() { # <action> <state> [window number]
    local action=$1 want=$2 number=${3:-1}
    wait_until 5 '[[ "$(action_state "$action" "$number")" == "$want" ]]' ||
        fail "the window's $action is '$(action_state "$action" "$number")', not '$want'"
}

# Waits until LeafMD has closed completely, after its last window
expect_leafmd_closed() {
    wait_until 10 '(( $(window_count) == 0 ))' || fail "LeafMD's windows didn't close."
    wait_until 15 '! leafmd_on_bus' || fail "LeafMD kept running after its last window closed."
}

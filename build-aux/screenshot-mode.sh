#!/usr/bin/env bash
# Gets this computer ready for taking LeafMD's screenshots, and back again.
#
# Flathub asks for screenshots taken with the desktop's default settings, so
# they look the same as on most people's computers. Some Linux distributions
# (Manjaro, for one) change GNOME's defaults: a different accent colour,
# different fonts, extra title bar buttons. This switches those few settings
# to plain GNOME for a while, and sizes LeafMD's window for the screenshots.
#
#   build-aux/screenshot-mode.sh light   GNOME defaults, light style
#   build-aux/screenshot-mode.sh dark    the same, dark style
#   build-aux/screenshot-mode.sh off     puts your own settings back
#
# Your settings, and LeafMD's saved window size, are copied to
# ~/.cache/leafmd-screenshot-mode/ the first time, and "off" restores them
# from there. LeafMD must be closed when you run it, because LeafMD saves its
# window size when it closes and would overwrite the one set here.
#
# The steps for taking the screenshots are in data/screenshots/README.md.
set -euo pipefail

backup_dir="$HOME/.cache/leafmd-screenshot-mode"
leafmd_state="$HOME/.var/app/io.github.jeremehancock.LeafMD/config/leafmd/state.ini"

# GNOME's own defaults for the settings that show in a screenshot.
# "schema key value", one per line.
defaults="
org.gnome.desktop.interface accent-color 'blue'
org.gnome.desktop.interface font-name 'Adwaita Sans 11'
org.gnome.desktop.interface document-font-name 'Adwaita Sans 11'
org.gnome.desktop.interface monospace-font-name 'Adwaita Mono 11'
org.gnome.desktop.wm.preferences button-layout 'appmenu:close'
"

# The window size Flathub allows: no larger than 1000 by 700
width=1000
height=700

if flatpak ps --columns=application 2> /dev/null | grep -qx io.github.jeremehancock.LeafMD; then
    echo "Please close every LeafMD window first, then run this again." >&2
    exit 1
fi

save_settings() {
    # Only the first time, so running "light" then "dark" keeps your originals
    [[ -d "$backup_dir" ]] && return
    mkdir -p "$backup_dir"
    while read -r schema key _; do
        [[ -z "$schema" ]] && continue
        echo "$schema $key $(gsettings get "$schema" "$key")" >> "$backup_dir/settings"
    done <<< "$defaults
org.gnome.desktop.interface color-scheme"
    if [[ -f "$leafmd_state" ]]; then
        cp "$leafmd_state" "$backup_dir/state.ini"
    fi
}

apply() {
    local scheme=$1
    save_settings
    while read -r schema key value; do
        [[ -z "$schema" ]] && continue
        gsettings set "$schema" "$key" "$value"
    done <<< "$defaults"
    gsettings set org.gnome.desktop.interface color-scheme "$scheme"

    # LeafMD's window: the right size, not maximized. The [recent] part of
    # the file (recently opened files) is left alone.
    mkdir -p "$(dirname "$leafmd_state")"
    touch "$leafmd_state"
    python3 - "$leafmd_state" "$width" "$height" << 'EOF'
import configparser, sys
path, width, height = sys.argv[1:]
state = configparser.ConfigParser(interpolation=None)
state.optionxform = str
state.read(path)
if not state.has_section("window"):
    state.add_section("window")
state["window"]["width"] = width
state["window"]["height"] = height
state["window"]["maximized"] = "false"
with open(path, "w") as f:
    state.write(f, space_around_delimiters=False)
EOF
    echo "Screenshot mode is on ($2 style). When you're done: build-aux/screenshot-mode.sh off"
}

case "${1:-}" in
light) apply default light ;;
dark) apply prefer-dark dark ;;
off)
    if [[ ! -d "$backup_dir" ]]; then
        echo "Screenshot mode isn't on, so there's nothing to put back."
        exit 0
    fi
    while read -r schema key value; do
        gsettings set "$schema" "$key" "$value"
    done < "$backup_dir/settings"
    if [[ -f "$backup_dir/state.ini" ]]; then
        cp "$backup_dir/state.ini" "$leafmd_state"
    fi
    rm -r "$backup_dir"
    echo "Your own settings and LeafMD's window size are back."
    ;;
*)
    echo "Usage: build-aux/screenshot-mode.sh light|dark|off" >&2
    exit 2
    ;;
esac

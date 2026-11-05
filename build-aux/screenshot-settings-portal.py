#!/usr/bin/env python3
"""Tells LeafMD the desktop's appearance settings, for take-screenshots.sh.

A Flatpak app asks a desktop service called the settings portal for its fonts,
title bar buttons and light or dark style. Inside the hidden desktop that
take-screenshots.sh starts there is no such service, and without one GTK falls
back to different fonts and extra title bar buttons. So this stands in for it,
answering with GNOME's defaults, which is what Flathub wants screenshots to
show. It can only read out the settings below, and it's only ever reachable on
that hidden desktop's private message bus.

Usage: screenshot-settings-portal.py light|dark
"""
import sys

import gi

gi.require_version("Gio", "2.0")
from gi.repository import Gio, GLib

DARK = sys.argv[1] == "dark"


def text(value):
    return GLib.Variant("s", value)


def flag(value):
    return GLib.Variant("b", value)


def number(value):
    return GLib.Variant("i", value)


SETTINGS = {
    "org.freedesktop.appearance": {
        # 0 means no preference, which GNOME shows as light. 1 means dark.
        "color-scheme": GLib.Variant("u", 1 if DARK else 0),
        # GNOME's blue accent colour, as red, green and blue from 0 to 1
        "accent-color": GLib.Variant("(ddd)", (0.2078, 0.5176, 0.8941)),
        "contrast": GLib.Variant("u", 0),
    },
    "org.gnome.desktop.interface": {
        "font-name": text("Adwaita Sans 11"),
        "document-font-name": text("Adwaita Sans 11"),
        "monospace-font-name": text("Adwaita Mono 11"),
        # Left out, this counts as zero and the formatted view collapses into
        # a tiny narrow column
        "text-scaling-factor": GLib.Variant("d", 1.0),
        "font-antialiasing": text("grayscale"),
        "font-hinting": text("slight"),
        "font-rgba-order": text("rgb"),
        "font-rendering": text("automatic"),
        "gtk-theme": text("Adwaita"),
        "icon-theme": text("Adwaita"),
        "cursor-theme": text("Adwaita"),
        "cursor-size": number(24),
        "enable-animations": flag(True),
        "overlay-scrolling": flag(True),
        # Not GNOME's default: a blinking text cursor would make each run's
        # pictures differ for no reason
        "cursor-blink": flag(False),
    },
    "org.gnome.desktop.wm.preferences": {
        "button-layout": text("appmenu:close"),
    },
}

INTERFACE = """
<node>
  <interface name='org.freedesktop.portal.Settings'>
    <method name='ReadAll'>
      <arg type='as' direction='in'/>
      <arg type='a{sa{sv}}' direction='out'/>
    </method>
    <method name='Read'>
      <arg type='s' direction='in'/>
      <arg type='s' direction='in'/>
      <arg type='v' direction='out'/>
    </method>
    <method name='ReadOne'>
      <arg type='s' direction='in'/>
      <arg type='s' direction='in'/>
      <arg type='v' direction='out'/>
    </method>
    <signal name='SettingChanged'>
      <arg type='s'/>
      <arg type='s'/>
      <arg type='v'/>
    </signal>
    <property name='version' type='u' access='read'/>
  </interface>
</node>
"""


def on_call(_bus, _sender, _path, _interface, method, args, call):
    if method == "ReadAll":
        call.return_value(GLib.Variant("(a{sa{sv}})", (SETTINGS,)))
        return
    value = SETTINGS.get(args[0], {}).get(args[1])
    if value is None:
        call.return_dbus_error("org.freedesktop.portal.Error.NotFound", "Not set")
    elif method == "Read":
        # The older Read wraps the answer one level deeper than ReadOne does
        call.return_value(GLib.Variant("(v)", (GLib.Variant("v", value),)))
    else:
        call.return_value(GLib.Variant("(v)", (value,)))


def on_bus(bus, _name):
    interface = Gio.DBusNodeInfo.new_for_xml(INTERFACE).interfaces[0]
    bus.register_object(
        "/org/freedesktop/portal/desktop",
        interface,
        on_call,
        lambda *_: GLib.Variant("u", 2),  # the one property: this interface's version
        None,
    )


Gio.bus_own_name(
    Gio.BusType.SESSION,
    "org.freedesktop.portal.Desktop",
    Gio.BusNameOwnerFlags.NONE,
    on_bus,
    None,
    None,
)
GLib.MainLoop().run()

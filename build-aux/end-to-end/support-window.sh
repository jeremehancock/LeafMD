# help-and-about spec, "Support window": Support Development in the main menu
# opens a small window, Esc closes it without opening anything, and its Buy me
# a coffee button opens the web page. The hidden desktop has no portal (the
# desktop service that hands a Flatpak app's links to the browser), so there
# the button can only fail: LeafMD must stay open and say so, as it does for
# a link in a document. The document stays untouched throughout.
support-window() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_action_enabled support true
    activate_action support
    sleep 1
    press Escape
    sleep 1
    leafmd_on_bus || fail "LeafMD closed after the support window was closed"
    grep -q "could not open link" "$log" && fail "Escape opened the link instead of just closing the window"
    # The button has the focus when the window opens, so Enter presses it
    activate_action support
    sleep 1
    press Return
    sleep 3
    leafmd_on_bus || fail "LeafMD closed after Buy me a coffee was pressed"
    expect_windows 1
    grep -q "could not open link" "$log" || fail "LeafMD's messages don't say it couldn't open the Buy me a coffee page"
    expect_title notes.md
    expect_title_lacks "•"
    expect_file notes.md "Notes"
}

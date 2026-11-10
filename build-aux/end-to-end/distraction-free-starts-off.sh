# distraction-free spec, "Turning Distraction Free on and off": Ctrl+Shift+D
# turns it on, and every window starts with it off, even after it was on.
distraction-free-starts-off() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_action_state distraction-free false
    press ctrl+shift+d
    expect_action_state distraction-free true
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    expect_action_state distraction-free false
}

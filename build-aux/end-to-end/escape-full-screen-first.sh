# distraction-free spec, "Leaving with Escape", "Both on": with Distraction
# Free and full screen both on, the first Escape leaves full screen only, and
# the second leaves Distraction Free.
escape-full-screen-first() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+shift+d
    press F11
    expect_action_state distraction-free true
    expect_action_state fullscreen true
    press Escape
    expect_action_state fullscreen false
    expect_action_state distraction-free true
    press Escape
    expect_action_state distraction-free false
}

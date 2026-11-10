# windows spec, "Full screen": F11 fills the screen, and Escape leaves it.
full-screen() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press F11
    expect_action_state fullscreen true
    expect_action_state distraction-free false
    press Escape
    expect_action_state fullscreen false
}

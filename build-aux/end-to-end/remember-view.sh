# views spec, "The view is remembered": the next file opens in the view used last.
remember-view() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+3
    expect_action_state view split
    press ctrl+q
    expect_leafmd_closed
    expect_setting window view split
    open_leafmd notes.md
    expect_action_state view split
}

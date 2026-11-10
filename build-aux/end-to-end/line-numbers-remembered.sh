# views spec, "Line numbers are a choice": off at first, and once turned on
# (Line Numbers in the main menu) they stay on after starting LeafMD again.
line-numbers-remembered() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+2
    expect_action_state line-numbers false
    activate_action line-numbers # the main menu's Line Numbers item
    expect_action_state line-numbers true
    press ctrl+q
    expect_leafmd_closed
    expect_setting editor line-numbers true
    open_leafmd notes.md
    expect_action_state line-numbers true
}

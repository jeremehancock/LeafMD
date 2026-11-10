# side-panel spec, "Opening and closing the side panel" and "The side panel
# stays open until closed": closed at first, F9 opens it, and it's still open
# after starting LeafMD again.
side-panel-remembered() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_action_state side-panel false
    press F9
    expect_action_state side-panel true
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    expect_action_state side-panel true
}

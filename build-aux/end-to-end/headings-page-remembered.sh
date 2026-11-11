# side-panel spec, "The side panel remembers its page": the panel opens on
# Files at first, and on Headings after choosing it and starting again.
headings-page-remembered() {
    sample notes.md "# Notes"
    open_leafmd notes.md
    press F9
    expect_action_state side-panel-page files
    activate_action side-panel-page headings
    expect_action_state side-panel-page headings
    press ctrl+q
    expect_leafmd_closed
    expect_setting side-panel page headings
    open_leafmd notes.md
    expect_action_state side-panel true
    expect_action_state side-panel-page headings
}

# desktop-integration spec, "Folders on the command line": `leafmd <folder>`
# opens the side panel on that folder, and remembers the panel as open.
open-folder() {
    sample notes.md "Notes"
    open_leafmd "$docs"
    expect_windows 1
    expect_action_state side-panel true
    press ctrl+q
    expect_leafmd_closed
    expect_setting side-panel open true
}

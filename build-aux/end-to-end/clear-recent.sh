# recent-files spec, "Clearing the list": after Clear Recent Files, starting
# LeafMD again opens on "No Document Open" instead of the last file.
clear-recent() {
    sample notes.md "Notes"
    open_leafmd notes.md
    activate_action clear-recent # the item at the bottom of the recent files list
    press ctrl+q
    expect_leafmd_closed
    expect_setting recent files ""
    open_leafmd
    expect_windows 1
    expect_title_lacks notes.md
}

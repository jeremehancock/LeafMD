# views spec, "The margins are remembered": the margins start as Normal, and
# once Wide is chosen (Margins in Preferences) LeafMD starts with Wide next time.
margins-remembered() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_app_action_state margins normal
    set_app_action margins wide # choosing Wide next to Margins in Preferences
    expect_app_action_state margins wide
    expect_setting preferences margins wide
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    expect_app_action_state margins wide
}

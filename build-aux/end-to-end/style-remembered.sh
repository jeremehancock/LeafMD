# views spec, "The style is remembered": the style starts as Follow System,
# and once Dark is chosen (Style in Preferences) LeafMD starts dark next time.
style-remembered() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_app_action_state style system
    set_app_action style dark # choosing Dark next to Style in Preferences
    expect_app_action_state style dark
    expect_setting preferences style dark
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    expect_app_action_state style dark
}

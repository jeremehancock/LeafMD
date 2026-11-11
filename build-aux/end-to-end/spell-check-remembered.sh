# spell-check spec, "Turning spell checking on and off": on the first time
# LeafMD runs, Check Spelling turns it off, and it stays off after starting
# LeafMD again. Turning it off and on doesn't mark a saved document as changed.
spell-check-remembered() {
    sample notes.md "The garden needs mroe water."
    open_leafmd notes.md
    press ctrl+2
    expect_app_action_state spell-check true
    activate_app_action spell-check # the main menu's Check Spelling item
    expect_app_action_state spell-check false
    expect_setting editor spell-check false
    activate_app_action spell-check
    expect_app_action_state spell-check true
    expect_title_lacks "•"
    activate_app_action spell-check
    press ctrl+q
    expect_leafmd_closed
    expect_setting editor spell-check false
    open_leafmd notes.md
    expect_app_action_state spell-check false
}

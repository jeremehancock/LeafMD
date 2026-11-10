# distraction-free spec, "Leaving with Escape"
distraction-free-escape() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+shift+d
    expect_action_state distraction-free true
    press Escape
    expect_action_state distraction-free false
}

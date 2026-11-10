# views spec, "Each tab has its own view": changing one tab's view leaves the
# others as they were.
view-per-tab() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    press ctrl+2
    expect_action_state view source
    open_leafmd b.md # opens in the view last chosen in the window: Raw
    expect_title b.md
    expect_action_state view source
    press ctrl+3
    expect_action_state view split
    press ctrl+Page_Up
    expect_title a.md
    expect_action_state view source
}

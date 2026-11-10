# formatting-bar spec, "Text styles" and "One undo per button press"
bold() {
    sample notes.md "Our plan works"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+Right # to the end of "Our"
    press ctrl+Right # to the end of "plan"
    press ctrl+shift+Left # selects "plan"
    press ctrl+b
    press ctrl+s
    expect_file notes.md "Our **plan** works"
    press ctrl+z
    press ctrl+s
    expect_file notes.md "Our plan works"
}

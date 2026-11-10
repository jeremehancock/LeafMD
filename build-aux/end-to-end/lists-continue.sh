# smart-typing spec, "Lists continue on Enter"
lists-continue() {
    sample notes.md "- milk"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    press Return
    type_text "eggs"
    press ctrl+s
    expect_file notes.md "- milk
- eggs"
}

# smart-typing spec, "Lists end on an empty item": Enter on an item with nothing
# after its marker removes the marker and leaves a blank line.
lists-end() {
    sample notes.md "- milk"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    press Return
    press Return
    type_text "Done"
    press ctrl+s
    expect_file notes.md "- milk
Done"
}

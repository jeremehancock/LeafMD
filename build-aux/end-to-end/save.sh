# documents spec, "Saving": Ctrl+S writes the document to its file.
save() {
    sample notes.md "Shopping"
    open_leafmd notes.md
    press ctrl+2 # Raw view, so the middle of the window is the text
    click_text
    press ctrl+End
    type_text " list: milk, eggs!"
    press ctrl+s
    expect_file notes.md "Shopping list: milk, eggs!"
}

# formatting-bar spec, "Text styles with nothing selected": Ctrl+I with the
# cursor inside a word makes that word italic.
style-in-word() {
    sample notes.md "Some text"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    press Left
    press Left # between "te" and "xt"
    press ctrl+i
    press ctrl+s
    expect_file notes.md "Some *text*"
}

# smart-typing spec, "Pasting a web address over selected text makes a link"
paste-link() {
    sample notes.md "Read the docs first"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    press ctrl+Left # to the start of "first"
    press Left # back over the space
    press ctrl+shift+Left # selects "docs"
    press ctrl+shift+Left # and "the "
    copy_text "https://example.com/docs"
    press ctrl+v
    press ctrl+s
    expect_file notes.md "Read [the docs](https://example.com/docs) first"
}

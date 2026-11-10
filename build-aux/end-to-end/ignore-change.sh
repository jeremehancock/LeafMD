# documents spec, "Ignoring a change": Ignore keeps LeafMD's text, and saving
# afterwards replaces the other version without asking.
ignore-change() {
    sample notes.md "Hello"
    open_leafmd notes.md
    press ctrl+2
    click_text
    printf '%s' "Changed elsewhere" > "$docs/notes.md"
    sleep 2
    click_window "f.width - 53" 108 # Ignore, at the banner's right-hand end
    sleep 1
    click_text
    press ctrl+End
    type_text "!"
    press ctrl+s
    expect_file notes.md "Hello!"
}

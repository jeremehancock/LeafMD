# documents spec, "Reloading with unsaved changes": with unsaved text, the
# banner's button reads Discard and Reload, and it brings in the other version.
discard-and-reload() {
    sample notes.md "Hello"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    type_text " there"
    printf '%s' "Changed elsewhere" > "$docs/notes.md"
    sleep 2
    # The banner's wider red button, left of Ignore at the right-hand end
    click_window "f.width - 187" 108
    sleep 1
    click_text
    press ctrl+End
    type_text "!"
    press ctrl+s
    expect_file notes.md "Changed elsewhere!"
}

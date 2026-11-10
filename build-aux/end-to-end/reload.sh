# documents spec, "Banner when another app changes the file" and "Reloading the
# file": when another app changes an open file, LeafMD shows a banner and
# leaves the text alone, and the banner's Reload button brings in the new version.
reload() {
    sample notes.md "Hello"
    open_leafmd notes.md
    press ctrl+2
    click_text
    printf '%s' "Changed elsewhere" > "$docs/notes.md"
    sleep 2
    # The banner sits under the formatting bar, its Reload button just left of
    # Ignore at the right-hand end
    click_window "f.width - 130" 108
    sleep 1
    # Typing at the end and saving shows what LeafMD has now. Had it not
    # reloaded, saving would stop to ask about replacing the other version.
    click_text
    press ctrl+End
    type_text "!"
    press ctrl+s
    expect_file notes.md "Changed elsewhere!"
}

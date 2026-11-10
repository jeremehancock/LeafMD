# documents spec, "Saving over a change made elsewhere": LeafMD asks before
# replacing a file another app changed, and Cancel leaves it alone.
save-over-change() {
    sample notes.md "Hello"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    type_text "!"
    printf '%s' "Changed elsewhere" > "$docs/notes.md"
    sleep 2
    press ctrl+s # asks "Replace the Other Version?"
    sleep 1
    expect_file notes.md "Changed elsewhere"
    press Escape # Cancel
    sleep 1
    expect_file notes.md "Changed elsewhere"
    expect_windows 1
}

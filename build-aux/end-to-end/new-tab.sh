# documents spec, "New documents", and tabs spec, "Closing a tab": Ctrl+T opens
# an Untitled tab, and Ctrl+W closes it again.
new-tab() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+t
    expect_title Untitled
    press ctrl+w
    expect_title notes.md
    expect_windows 1
}

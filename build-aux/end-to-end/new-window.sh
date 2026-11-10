# windows spec, "New windows": Ctrl+N opens a second, empty window and the first
# keeps its document.
new-window() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+n
    expect_windows 2
    [[ "$(titles)" == *notes.md* ]] || fail "notes.md isn't open any more: $(titles)"
    expect_title_lacks notes.md
}

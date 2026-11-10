# tabs and windows specs: opening a second file from outside LeafMD gives it a
# tab in the same window, and Ctrl+Page Up goes back to the first.
tabs() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    expect_title a.md
    open_leafmd b.md
    expect_windows 1
    expect_title b.md
    press ctrl+Page_Up
    expect_title a.md
}

# recent-files spec, "Reopening the last file": starting LeafMD without a file
# brings back the last window's tabs, with the same one selected.
reopen-last() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    open_leafmd b.md
    press ctrl+Page_Up
    expect_title a.md
    press ctrl+q
    expect_leafmd_closed
    open_leafmd
    expect_windows 1
    expect_title a.md
    press ctrl+Page_Down
    expect_title b.md
}

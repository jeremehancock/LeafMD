# windows spec, "Empty windows are reused": a file opened from outside LeafMD
# goes into the window showing "No Document Open".
empty-window-reused() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    press ctrl+n
    expect_windows 2
    open_leafmd b.md
    expect_windows 2
    local all
    all=$(titles)
    [[ "$all" == *a.md* && "$all" == *b.md* ]] || fail "the windows are: $all"
    # b.md has its own window, so a.md's window still has just the one tab
    [[ "$(grep -o 'b\.md' <<< "$all" | wc -l)" == 1 && "$(grep -o 'a\.md' <<< "$all" | wc -l)" == 1 ]] ||
        fail "the windows are: $all"
}

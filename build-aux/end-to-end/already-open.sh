# windows spec, "Already open files are brought forward": opening a file that's
# already open selects its tab instead of opening it twice.
already-open() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    open_leafmd b.md
    expect_title b.md
    open_leafmd a.md
    expect_windows 1
    expect_title a.md
    # The tabs LeafMD saves when it closes show there are still only two
    press ctrl+q
    expect_leafmd_closed
    expect_setting recent tabs "file://$docs/a.md\\nfile://$docs/b.md"
}

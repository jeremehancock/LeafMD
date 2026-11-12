# folder-tree spec, "Which folder the tree shows" and "Going up a folder": the
# panel shows the folder of the file you're looking at, even one opened from a
# subfolder in the tree, and the up button shows the folder above until another
# tab is selected. The tests can't read the panel, so each step clicks a row
# that only holds the expected file if the panel shows the expected folder.
# Rows are 38 pixels apart, the first at 69; the up button is at 23, 22.
files-panel-file-folder() {
    mkdir -p "$docs/sub"
    printf 'A' > "$docs/sub/a.md"
    printf 'B' > "$docs/sub/b.md"
    sample top.md "Top"
    open_leafmd "$docs"
    # Documents: sub, top.md. Open sub, then click a.md under it.
    click_window 88 69
    click_window 88 108
    sleep 2
    expect_title a.md
    # The panel moved into sub (a.md, b.md), so its second row is b.md. Had it
    # stayed on Documents, the second row would be a.md again.
    click_window 88 108
    sleep 2
    expect_title b.md
    # Up: Documents with sub open (sub, a.md, b.md, top.md), so row four is top.md
    click_window 23 22
    sleep 1.5
    click_window 88 183
    sleep 2
    expect_title top.md
    # Back to b.md's tab: the panel is on sub again, so its first row is a.md
    press ctrl+Page_Up
    sleep 1.5
    expect_title b.md
    click_window 88 69
    sleep 1.5
    expect_title a.md
}

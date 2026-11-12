# folder-tree spec, "Files LeafMD can't open are greyed out": files are judged
# by their names only, and that includes empty ones. GIO calls any empty file
# "empty file" rather than markdown, which used to grey out a new document
# saved into the folder the panel was showing (it's empty for a moment while
# it's written). Clicking a greyed-out row does nothing, so the title only
# changes if the empty plan.md shows normally.
empty-file-opens() {
    sample plan.md ""
    open_leafmd "$docs"
    expect_action_state side-panel true
    click_window 88 69 # the tree's first row: plan.md
    expect_title plan.md
}

# folder-tree spec, "Opening files from the tree": clicking a file in the side
# panel's tree opens it in a new tab.
open-from-tree() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd "$docs"
    expect_action_state side-panel true
    click_window 88 69 # the tree's first row: a.md
    expect_title a.md
}

# folder-tree spec, "Open Folder… with a document open": a folder never joins a
# window that has a document open, so the panel doesn't show a folder unrelated
# to the document beside it. Open Folder… uses a folder picker, which the tests
# can't drive, so this checks the same window choice through `leafmd <folder>`,
# which Open Folder… shares when its own window has a document open.
open-folder-new-window() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_title notes.md
    open_leafmd "$docs"
    expect_windows 2
    [[ "$(titles)" == *notes.md* ]] || fail "notes.md's window changed: $(titles)"
}

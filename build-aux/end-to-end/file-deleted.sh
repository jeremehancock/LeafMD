# documents spec, "The file is deleted or moved": the text stays, and Ctrl+S
# writes the file back without asking.
file-deleted() {
    sample notes.md "Keep me"
    open_leafmd notes.md
    rm "$docs/notes.md"
    sleep 2
    press ctrl+s
    expect_file notes.md "Keep me"
}

# find-replace spec, "Replacing one match": Enter in the replace box replaces the
# selected match and moves to the next.
replace-one() {
    sample notes.md "cat dog cat dog cat"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+shift+Right # selects the first "cat", which fills the find box
    press ctrl+h
    type_text "bird"
    press Return
    press Return
    press ctrl+s
    expect_file notes.md "bird dog bird dog cat"
}

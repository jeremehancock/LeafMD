# find-replace spec, "Replacing all matches": every match is replaced, and one
# Ctrl+Z puts them all back.
replace-all() {
    sample notes.md "cat and cat
the cat sat with a cat"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+shift+Right # selects the first "cat", which fills the find box
    press ctrl+h # with something to find, the keyboard goes to "Replace with"
    type_text "dog"
    press Tab # to Replace
    press Tab # to Replace All
    press space
    press ctrl+s
    expect_file notes.md "dog and dog
the dog sat with a dog"
    press Escape # closes the find bar, back to the text
    press ctrl+z
    press ctrl+s
    expect_file notes.md "cat and cat
the cat sat with a cat"
}

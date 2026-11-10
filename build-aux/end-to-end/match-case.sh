# find-replace spec, "Search options": with Match Case ticked, "Cat" doesn't
# match "cat" or "CAT".
match-case() {
    sample notes.md "Cat cat CAT"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+shift+Right # selects "Cat", which fills the find box
    press ctrl+h
    activate_action find-match-case # the tick box in the find bar's options menu
    expect_action_state find-match-case true
    type_text "Dog"
    press Tab # to Replace
    press Tab # to Replace All
    press space
    press ctrl+s
    expect_file notes.md "Dog cat CAT"
}

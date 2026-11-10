# find-replace spec, "Search options": with Whole Words Only ticked, "cat"
# doesn't match inside "concat" or "cat_food".
whole-words() {
    sample notes.md "cat concat cat_food cat"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+shift+Right # selects the first "cat", which fills the find box
    press ctrl+h
    activate_action find-whole-words # the tick box in the find bar's options menu
    expect_action_state find-whole-words true
    type_text "dog"
    press Tab # to Replace
    press Tab # to Replace All
    press space
    press ctrl+s
    expect_file notes.md "dog concat cat_food dog"
}

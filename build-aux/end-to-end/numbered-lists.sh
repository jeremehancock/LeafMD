# smart-typing spec, "Numbered items count up"
numbered-lists() {
    sample recipe.md "2. Preheat the oven"
    open_leafmd recipe.md
    press ctrl+2
    click_text
    press ctrl+End
    press Return
    type_text "Bake"
    press ctrl+s
    expect_file recipe.md "2. Preheat the oven
3. Bake"
}

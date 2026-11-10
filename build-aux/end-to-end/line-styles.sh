# formatting-bar spec, "Line styles": the Checklist button applies to every
# selected line.
line-styles() {
    sample notes.md "Milk
Eggs"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+a
    activate_action format task-list # the formatting bar's Checklist button
    press ctrl+s
    expect_file notes.md "- [ ] Milk
- [ ] Eggs"
}

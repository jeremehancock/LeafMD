# smart-typing spec, "Checklist items continue unticked"
checklist() {
    sample trip.md "- [x] Book flights"
    open_leafmd trip.md
    press ctrl+2
    click_text
    press ctrl+End
    press Return
    type_text "Pack"
    press ctrl+s
    expect_file trip.md "- [x] Book flights
- [ ] Pack"
}

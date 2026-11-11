# headings spec, "Jumping to a heading": clicking a heading on the Headings
# page puts the cursor at the start of its line in the raw text, so typing
# goes there.
jump-to-heading() {
    sample notes.md $'# Garden Plan\n\nWe start in May.\n\n## Seeds\n\nTomatoes and beans.\n\n## Beds\n\nFour of them.\n'
    open_leafmd notes.md
    press ctrl+2 # Raw view
    press F9
    activate_action side-panel-page headings
    click_window 88 141 # the Headings page's third row: Beds
    type_text "Raised "
    press ctrl+s
    expect_file_contains notes.md "Raised ## Beds"
}

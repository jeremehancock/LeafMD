# documents spec, "Closing a window with several unsaved tabs": one question for
# both. Cancel (Escape) keeps everything; Save (the default, Enter) saves both.
close-unsaved-tabs() {
    sample a.md "A"
    sample b.md "B"
    open_leafmd a.md
    press ctrl+2
    click_text
    press ctrl+End
    type_text "1"
    open_leafmd b.md
    click_text
    press ctrl+End
    type_text "2"
    press ctrl+q
    sleep 1
    press Escape # Cancel
    sleep 1
    expect_windows 1
    expect_file a.md "A"
    expect_file b.md "B"
    press ctrl+q
    sleep 1
    press Return # Save
    expect_leafmd_closed
    expect_file a.md "A1"
    expect_file b.md "B2"
}

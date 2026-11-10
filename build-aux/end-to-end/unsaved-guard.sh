# documents spec, "Unsaved changes are never lost silently": closing a window
# with unsaved changes asks first, so the window stays and the file is unchanged.
unsaved-guard() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    type_text " draft"
    press ctrl+q
    sleep 1
    expect_windows 1
    expect_file notes.md "Notes"
}

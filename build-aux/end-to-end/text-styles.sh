# formatting-bar spec, "Text styles": italic and strikethrough wrap the selection
text-styles() {
    sample notes.md "one two three"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+Home
    press ctrl+shift+Right # selects "one"
    press ctrl+i
    press ctrl+End
    press ctrl+shift+Left # selects "three"
    press ctrl+shift+x
    press ctrl+s
    expect_file notes.md "*one* two ~~three~~"
}

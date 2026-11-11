# spell-check spec, "Suggestions on right-click": choosing a suggestion for a
# misspelt word replaces it, and one Ctrl+Z puts it back. The menu is opened
# with the Menu key. Home moves to the menu's first item that can be chosen,
# whatever was highlighted when it opened. Paste can only be chosen when
# there's something on the clipboard, and an earlier scenario may have left
# some, so this one copies text first: then Home always lands on Paste, and
# three Downs (Select All, Insert Emoji) reach the first suggestion. "mroe"
# is misspelt in every English dictionary, so this works whichever English
# the hidden desktop's language picks.
spell-check-suggestion() {
    sample notes.md "The garden needs mroe water."
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    local i
    for i in 1 2 3 4 5 6 7 8; do press Left; done # into "mroe"
    copy_text "anything"
    sleep 1
    press Menu
    sleep 1
    press Home # Paste
    press Down # Select All
    press Down # Insert Emoji
    press Down # the first suggestion, "more"
    press Return
    press ctrl+s
    expect_file notes.md "The garden needs more water."
    press ctrl+z
    press ctrl+s
    expect_file notes.md "The garden needs mroe water."
}

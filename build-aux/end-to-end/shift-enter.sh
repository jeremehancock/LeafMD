# smart-typing spec, "Shift+Enter skips the marker"
shift-enter() {
    sample notes.md "- Milk"
    open_leafmd notes.md
    press ctrl+2
    click_text
    press ctrl+End
    press shift+Return
    type_text "from the farm"
    press ctrl+s
    expect_file notes.md "- Milk
from the farm"
}

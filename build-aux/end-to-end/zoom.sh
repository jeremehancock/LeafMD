# text-size spec, "Zooming" and "Zoom belongs to a window": Ctrl+plus and
# Ctrl+minus go in steps of 10%, a new tab keeps the window's zoom, a new
# window starts at 100%, Ctrl+0 goes back to 100%, and Ctrl with the mouse
# wheel zooms over either view while the wheel alone doesn't.
zoom() {
    sample notes.md "Notes"
    open_leafmd notes.md
    expect_action_state zoom 100
    press ctrl+plus
    press ctrl+plus
    expect_action_state zoom 120
    press ctrl+minus
    expect_action_state zoom 110
    press ctrl+equal # Ctrl+plus without Shift
    press ctrl+minus
    expect_action_state zoom 110
    press ctrl+t
    expect_action_state zoom 110
    press ctrl+n
    expect_windows 2
    expect_action_state zoom 100 2
    expect_action_state zoom 110 1
    close_window # the new, empty window
    expect_windows 1
    press ctrl+0
    expect_action_state zoom 100
    press ctrl+3
    move_pointer 200 300 # over the raw text
    wheel up ctrl
    wheel up ctrl
    expect_action_state zoom 120
    move_pointer 800 300 # over the formatted view
    wheel down ctrl
    expect_action_state zoom 110
    wheel down
    expect_action_state zoom 110
}

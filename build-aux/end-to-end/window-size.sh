# documents spec, "The window size is remembered"
window-size() {
    sample notes.md "Notes"
    open_leafmd notes.md
    in_shell "$focused.move_resize_frame(true, 100, 100, 900, 600); 0" > /dev/null
    sleep 1
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    local size
    size=$(shell_text "let f = $focused.get_frame_rect(); f.width + 'x' + f.height")
    [[ "$size" == "900x600" ]] || fail "the window reopened at $size, not 900x600"
}

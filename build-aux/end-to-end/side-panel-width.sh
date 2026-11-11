# side-panel spec, "Resizing the side panel", "The document keeps its room"
# and "The side panel remembers its width": dragging the panel's edge 80
# pixels to the right makes it 320 wide, that's saved, and it opens 320 wide
# after starting LeafMD again. Then a 400 pixel panel shrinks to 320 when the
# window is dragged narrower, bit by bit, to 1000.
side-panel-width() {
    sample notes.md "Notes"
    open_leafmd notes.md
    in_shell "$focused.move_resize_frame(true, 100, 100, 1200, 800); 0" > /dev/null
    sleep 1
    press F9
    expect_action_state side-panel true
    drag_window 243 300 80 # the strip just right of the panel's edge, at 240
    expect_setting side-panel width 320
    press ctrl+q
    expect_leafmd_closed
    open_leafmd notes.md
    # If the panel opened 320 wide, its edge is at 320, so dragging from just
    # right of there moves it. Had it opened at 240, this press would land in
    # the document and nothing would be saved.
    drag_window 323 300 80
    expect_setting side-panel width 400
    # Narrow the window 8 pixels at a time, about once a frame, the way a
    # hand drags it, to 1000. The document keeps 680, so the panel shows 320.
    in_shell "let w = 1200; let win = $focused;
        imports.gi.GLib.timeout_add(0, 16, () => {
            w -= 8; win.move_resize_frame(true, 100, 100, w, 800); return w > 1000; }); 0" > /dev/null
    sleep 1.5
    # So its edge is at 320 now: dragging from there moves it, and the width
    # chosen becomes 300. Had the panel stayed 400 wide, this press would land
    # inside it and nothing would be saved.
    drag_window 323 300 -20
    expect_setting side-panel width 300
}

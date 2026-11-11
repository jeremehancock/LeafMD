# links spec, "Web links open in the browser": if the link can't be opened, a
# message says so. The hidden desktop has no portal (the desktop service that
# hands a Flatpak app's links to the browser), so clicking a web link there
# can only fail. LeafMD must stay open and say it couldn't open the link,
# rather than crash as it once did.
web-link-no-portal() {
    sample notes.md "[Example](https://example.com)"
    open_leafmd notes.md
    in_shell "$focused.move_resize_frame(true, 0, 40, 1000, 600); 0" > /dev/null
    sleep 1
    press ctrl+1
    sleep 1
    click_window 180 100 # the link, the first thing on the page
    sleep 3
    leafmd_on_bus || fail "LeafMD closed after the link was clicked"
    expect_windows 1
    grep -q "could not open link" "$log" || fail "LeafMD's messages don't say it couldn't open the link"
}

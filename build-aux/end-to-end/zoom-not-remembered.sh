# text-size spec, "Zoom isn't remembered": a window zoomed to 130% opens at
# 100% after starting LeafMD again, and nothing about zoom is saved.
zoom-not-remembered() {
    sample notes.md "Notes"
    open_leafmd notes.md
    press ctrl+plus
    press ctrl+plus
    press ctrl+plus
    expect_action_state zoom 130
    press ctrl+q
    expect_leafmd_closed
    # A setting whose name mentions zoom (file addresses can, so only names count)
    if grep -qiE '^[^=]*zoom[^=]*=' "$home/.var/app/$app/config/leafmd/state.ini"; then
        fail "state.ini saved a zoom setting: $(grep -iE '^[^=]*zoom[^=]*=' "$home/.var/app/$app/config/leafmd/state.ini")"
    fi
    open_leafmd notes.md
    expect_action_state zoom 100
}

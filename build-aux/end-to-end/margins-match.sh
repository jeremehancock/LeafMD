# formatted-view spec, "Both halves of Split view match": the raw text and the
# formatted view share one column width for each Margins choice. Each half of
# Split view uses the same column as its view on its own, so this measures
# Raw view and Formatted view in a full-width window, where every choice's
# column fits (Split's halves on the test desktop's screen are narrower than
# most of them). The widths must agree to within 15 pixels, under two
# characters; before they shared one width, Normal was about 40 apart and
# Narrow about 70. They must also get narrower from None to Narrow, Normal
# and Wide, so a column that simply filled the window can't pass.
margins-match() {
    # One very long word per paragraph: both views break it right at the
    # column's edge, so ragged line endings can't blur the measurement
    local text="" i
    for i in 1 2 3; do
        text+=$(printf 'abcdefghij%.0s' {1..50})$'\n\n'
    done
    sample notes.md "$text"
    open_leafmd notes.md
    in_shell "$focused.maximize(3); 0" > /dev/null
    sleep 1
    local margins raw formatted previous=100000
    for margins in none narrow normal wide; do
        set_app_action margins "$margins"
        press ctrl+2
        sleep 1
        screen_picture "$home/raw-$margins.png"
        raw=$(text_width_in_picture "$home/raw-$margins.png" 150 900)
        press ctrl+1
        sleep 1
        screen_picture "$home/formatted-$margins.png"
        formatted=$(text_width_in_picture "$home/formatted-$margins.png" 150 900)
        ((raw > 0 && formatted > 0)) || fail "no text found in the $margins pictures"
        # None has no column: each view keeps its own small gap at the sides
        [[ $margins == none ]] || ((raw - formatted <= 15 && formatted - raw <= 15)) ||
            fail "with $margins margins the raw text is $raw pixels wide and the formatted text $formatted"
        ((raw < previous)) || fail "$margins margins aren't narrower than the choice before ($raw, then $previous)"
        previous=$raw
    done
}

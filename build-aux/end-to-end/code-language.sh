# smart-typing spec, "Language suggestions for code blocks" and "Picking or
# ignoring a suggestion": ```py then Enter picks python; Escape keeps what was
# typed; ``` then Enter is an ordinary new line.
code-language() {
    sample notes.md ""
    open_leafmd notes.md
    press ctrl+2
    click_text
    type_text '```py'
    sleep 1 # the list shows
    press Return
    sleep 0.5
    press Return
    type_text 'print(1)'
    press Return
    type_text '```'
    press Return
    press Return
    type_text '```py'
    sleep 1
    press Escape
    press Return
    type_text 'x'
    press Return
    type_text '```'
    press Return
    press Return
    type_text '```'
    press Return
    type_text 'plain'
    press ctrl+s
    expect_file notes.md $'```python\nprint(1)\n```\n\n```py\nx\n```\n\n```\nplain'
}

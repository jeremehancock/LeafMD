# print-and-export spec, "Only with a document open": Print, Export as PDF
# and Export as HTML are greyed out while the window shows "No Document
# Open", and can be used again once a document is open. (Printing and
# exporting themselves go through dialogs, which these tests never open.)
print-needs-document() {
    sample notes.md "Notes"
    open_leafmd notes.md
    for action in print export-pdf export-html; do expect_action_enabled "$action" true; done
    activate_action close-document
    expect_title_lacks notes.md
    for action in print export-pdf export-html; do expect_action_enabled "$action" false; done
    # Ctrl+P with nothing open does nothing, and LeafMD keeps running
    press ctrl+p
    leafmd_on_bus || fail "LeafMD stopped after Ctrl+P with no document open."
    activate_action open-recent "file://$docs/notes.md"
    expect_title notes.md
    for action in print export-pdf export-html; do expect_action_enabled "$action" true; done
}

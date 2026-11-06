# Spec Delta

## MODIFIED Requirements

### Requirement: Links to text files open in LeafMD
LeafMD SHALL open a clicked link to a text file on this computer (markdown, `.txt`, `LICENSE` and so on) in a new selected tab in the same window, so the current document stays open. Relative links are resolved from the open document's folder. If the link names a heading (`other.md#install`), the new tab SHALL scroll to that heading once it opens.

#### Scenario: Link to another markdown file
- **WHEN** the user clicks `[Guide](docs/guide.md#setup)`
- **THEN** `docs/guide.md` next to the current file opens in a new tab, selected and scrolled to the "setup" heading

#### Scenario: The current document stays as it was
- **WHEN** the user has unsaved changes in README.md and clicks a link to LICENSE
- **THEN** nothing is asked, LICENSE opens in a new tab, and README.md keeps its changes and its scroll position in its own tab

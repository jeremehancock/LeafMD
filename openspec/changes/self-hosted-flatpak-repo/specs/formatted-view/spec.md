# Spec Delta

## MODIFIED Requirements

### Requirement: GitHub-style markdown
LeafMD SHALL render CommonMark plus the GitHub-style extras: tables, task lists (shown as ticked or unticked checkboxes that can't be clicked, in place of the bullet), strikethrough, footnotes, and plain web addresses turned into links automatically.

#### Scenario: Task list
- **WHEN** the document contains `- [x] done` and `- [ ] todo`
- **THEN** the formatted view shows a ticked and an unticked checkbox, with no bullet beside them

#### Scenario: Checklist and plain items in one list
- **WHEN** the document contains `- [ ] todo` followed by `- plain`
- **THEN** "todo" has only a checkbox and "plain" has its bullet

#### Scenario: Bare web address
- **WHEN** the document contains `https://example.com` on its own
- **THEN** it is shown as a clickable link

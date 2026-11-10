# tabs Specification

## Purpose
How a LeafMD window holds several documents in tabs: the tab bar, moving between tabs, closing and rearranging them, and how they fit narrow windows.

## Requirements

### Requirement: The tab bar
LeafMD SHALL show a tab bar directly under the header bar whenever a window has two or more tabs, and SHALL hide it when the window has one tab or none. Clicking a tab SHALL select it. The selected tab's document fills the window.

#### Scenario: One document
- **WHEN** a window has only notes.md open
- **THEN** there is no tab bar, and the window looks as it did before tabs existed

#### Scenario: A second document
- **WHEN** notes.md is open and the user presses Ctrl+O and chooses todo.md
- **THEN** the tab bar appears with "notes.md" and "todo.md", and todo.md is selected

#### Scenario: Back to one
- **WHEN** a window has notes.md and todo.md open and the user closes todo.md
- **THEN** the tab bar disappears and notes.md fills the window

### Requirement: What a tab shows
Each tab SHALL show the file name, or "Untitled" for a new document, with a dot (•) before it while it has unsaved changes, and a close button. Hovering over a tab SHALL show the file's full location.

#### Scenario: Unsaved changes in a tab
- **WHEN** the user types in notes.md without saving
- **THEN** its tab reads "• notes.md", and pressing Ctrl+S removes the dot

#### Scenario: Two files with the same name
- **WHEN** `~/a/README.md` and `~/b/README.md` are both open and the user hovers over the second tab
- **THEN** a tooltip shows `~/b/README.md`

### Requirement: Each tab is its own document
Each tab SHALL keep its own text, cursor, scroll position, undo history and unsaved changes. Typing, saving, undoing and the formatting bar SHALL only affect the selected tab.

#### Scenario: Switching and returning
- **WHEN** the user scrolls to line 200 of notes.md, switches to todo.md, and switches back
- **THEN** notes.md is still at line 200 with the cursor where it was

#### Scenario: Undo stays in its tab
- **WHEN** the user types in notes.md, switches to todo.md and presses Ctrl+Z
- **THEN** todo.md's own last change is undone, and notes.md is unchanged

#### Scenario: Save only touches the selected tab
- **WHEN** notes.md and todo.md both have unsaved changes, todo.md is selected, and the user presses Ctrl+S
- **THEN** todo.md is saved and notes.md still shows its dot

### Requirement: Moving between tabs by keyboard
LeafMD SHALL go to the next tab with Ctrl+Tab, Ctrl+Page Down or Ctrl+Alt+Page Down, to the previous tab with Ctrl+Shift+Tab, Ctrl+Page Up or Ctrl+Alt+Page Up, wrapping round at the ends, and to a tab by its position with Alt+1 to Alt+9. These keys SHALL NOT take over any key the raw text uses for moving the cursor.

#### Scenario: Next tab
- **WHEN** the first of three tabs is selected and the user presses Ctrl+Tab
- **THEN** the second tab is selected

#### Scenario: Wrapping round
- **WHEN** the last of three tabs is selected and the user presses Ctrl+Page Down
- **THEN** the first tab is selected

#### Scenario: By position
- **WHEN** a window has three tabs and the user presses Alt+3
- **THEN** the third tab is selected

#### Scenario: Moving the cursor still works
- **WHEN** the user is typing in the raw text and presses Ctrl+Home
- **THEN** the cursor goes to the start of the document and the selected tab doesn't change

### Requirement: Closing a tab
Ctrl+W or a tab's close button SHALL close that tab, after the unsaved-changes check for that tab. Choosing Cancel SHALL keep it open and unchanged. Closing the selected tab SHALL select the tab next to it. Closing the last tab SHALL close the window.

#### Scenario: Closing one of two
- **WHEN** notes.md and todo.md are open, todo.md is selected, and the user presses Ctrl+W
- **THEN** todo.md's tab closes, notes.md is selected, and the window stays open

#### Scenario: Closing a tab with unsaved changes
- **WHEN** the user clicks the close button on a tab reading "• notes.md"
- **THEN** that tab is selected, LeafMD asks "Save changes to document “notes.md” before closing?", and Cancel leaves the tab open with its changes

#### Scenario: Closing the last tab
- **WHEN** only notes.md is open, with no unsaved changes, and the user presses Ctrl+W
- **THEN** the window closes

### Requirement: Tab menu
Right-clicking a tab SHALL show a menu with Move Left, Move Right, Move to New Window and Close, acting on the tab that was right-clicked, whether or not it's the selected one. Move Left SHALL be greyed out for the first tab and Move Right for the last.

#### Scenario: Moving a tab to its own window
- **WHEN** a window has notes.md and todo.md open and the user right-clicks todo.md and chooses Move to New Window
- **THEN** todo.md opens alone in a new window, with its unsaved changes, undo history and view intact, and notes.md stays in the first window

#### Scenario: The first tab
- **WHEN** the user right-clicks the first tab
- **THEN** Move Left is greyed out and the other items can be chosen

### Requirement: Dragging tabs
LeafMD SHALL let the user drag a tab along the tab bar to change the order, onto another LeafMD window's tab bar to move it there, anywhere on a window showing "No Document Open" to move it there, or away from any tab bar to move it to a new window. A moved tab SHALL keep its unsaved changes, undo history, view and find bar. A window that gives away its only tab SHALL close.

#### Scenario: Reordering
- **WHEN** the tabs are notes.md, todo.md and ideas.md, and the user drags ideas.md to the left end
- **THEN** the order becomes ideas.md, notes.md and todo.md

#### Scenario: Moving to another window
- **WHEN** the user drags todo.md, with unsaved changes, onto the tab bar of another LeafMD window
- **THEN** todo.md is a selected tab in that window, still showing its dot, and Ctrl+Z still undoes its changes

#### Scenario: Pulling a tab out
- **WHEN** the user drags todo.md out of the tab bar and drops it on the empty desktop
- **THEN** todo.md opens in a new window of its own

#### Scenario: Moving to an empty window
- **WHEN** one window has notes.md and todo.md in tabs, a second window shows "No Document Open", and the user drags todo.md onto the middle of the second window
- **THEN** todo.md opens in the second window, notes.md stays in the first, and no third window appears

### Requirement: Tabs in a narrow window
In a window narrower than 680 pixels, the tab bar SHALL stay on its own row under the header bar. Tabs SHALL get narrower as more are opened and then scroll sideways, and the tab bar SHALL NOT stop the window from reaching 360 pixels wide.

#### Scenario: Several tabs at the smallest size
- **WHEN** the window is 360 pixels wide and 8 documents are open
- **THEN** the header bar still shows the side panel button, Open, the title, the main menu and the window's buttons, the tab bar scrolls sideways, and nothing is cut off

#### Scenario: The selected tab stays in view
- **WHEN** the window is 360 pixels wide with 8 tabs open and the user presses Alt+8
- **THEN** the tab bar scrolls so the eighth tab is fully visible

#### Scenario: Bottom bar works with tabs
- **WHEN** the window is 500 pixels wide with two tabs and the user clicks New in the bottom bar
- **THEN** a third tab opens and is selected

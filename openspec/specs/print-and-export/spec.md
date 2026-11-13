# print-and-export Specification

## Purpose
Lets people get the formatted document out of LeafMD: printed on paper, saved as a PDF, or saved as a single web page that opens in any browser.

## Requirements

### Requirement: Print
Print (Ctrl+P, or Print… in the main menu) SHALL open the system's Print dialog for the selected tab, and printing from it SHALL print that tab's formatted document. This SHALL be the same whichever view is showing: Formatted, Raw or Split.

#### Scenario: Printing from the Raw view
- **WHEN** notes.md, containing `# Shopping` and `- **milk**`, is showing in the Raw view and the user presses Ctrl+P and clicks Print
- **THEN** the printed page shows "Shopping" as a large heading and "milk" in bold as a bulleted item, not the `#` and `**` characters

#### Scenario: Cancelling
- **WHEN** the user presses Ctrl+P and clicks Cancel in the Print dialog
- **THEN** nothing is printed and the document is unchanged

#### Scenario: Printing while in Distraction Free
- **WHEN** the window is in Distraction Free and the user presses Ctrl+P
- **THEN** the Print dialog opens, the same as outside it

### Requirement: Export as PDF
Export as PDF… in the main menu SHALL ask where to save, suggesting the document's name with `.pdf` in place of its ending, in the document's folder. Choosing a place SHALL save the formatted document there as a PDF that looks the way printing it would, apart from its page margins (see "Page margins in PDFs"), and show a message saying it was saved. Cancelling SHALL save nothing.

#### Scenario: Exporting a saved file
- **WHEN** `~/notes/garden.md` is selected and the user chooses Export as PDF…
- **THEN** the save dialog opens in `~/notes` suggesting "garden.pdf", and clicking Save writes `~/notes/garden.pdf` and shows "Saved garden.pdf"

#### Scenario: Replacing an earlier export
- **WHEN** `~/notes/garden.pdf` already exists and the user exports garden.md to it again
- **THEN** the dialog asks whether to replace it, and replacing it writes the new PDF

### Requirement: Export as HTML
Export as HTML… in the main menu SHALL ask where to save, suggesting the document's name with `.html` in place of its ending, in the document's folder. Choosing a place SHALL save the formatted document there as a single web page that opens in any browser, and show a message saying it was saved. Cancelling SHALL save nothing.

#### Scenario: Exporting and opening in a browser
- **WHEN** the user exports `~/notes/garden.md`, which has a heading, a table and a checklist, as HTML and opens `~/notes/garden.html` in Firefox
- **THEN** it shows the heading, the table as a rounded card and the checklist with GNOME-style checkboxes, as in LeafMD's formatted view, and the message "Saved garden.html" was shown in LeafMD

### Requirement: The exported web page stands on its own
The exported web page SHALL keep working after being moved or emailed: pictures stored on the computer SHALL be packed inside the file, and its styles SHALL be inside it too. Online pictures SHALL stay as links to where they are online. Links SHALL be kept as written in the document.

#### Scenario: A picture next to the document
- **WHEN** `~/notes/garden.md` contains `![](img/beds.png)`, it is exported to `~/notes/garden.html`, and the HTML file alone is emailed to someone else
- **THEN** they see the picture of the beds when they open it

#### Scenario: A picture that's missing
- **WHEN** the document contains `![Plan](img/missing.png)` and that file doesn't exist
- **THEN** the export still succeeds, and the page shows what a browser shows for a missing picture, with "Plan" as its description

#### Scenario: An online picture
- **WHEN** the document contains `![](https://example.org/logo.png)`
- **THEN** the exported page shows it from that address when the reader is online

### Requirement: The exported web page is safe
The exported web page SHALL contain no scripts, including none of LeafMD's own, and SHALL have the same unsafe parts removed as the formatted view (scripts, event handlers like `onerror`, and iframes).

#### Scenario: A document with a script in it
- **WHEN** the document contains `<script>alert(1)</script>` and `<img src="a.png" onerror="alert(2)">` and is exported as HTML
- **THEN** the exported file contains neither `<script` nor `onerror`

### Requirement: The exported web page follows the reader's style
The exported web page SHALL show light colours when the reader's browser or desktop is in light style and dark colours when it is in dark style, using the same colours as LeafMD's formatted view, and the accent colour LeafMD was using when it was exported.

#### Scenario: Opened on a dark desktop
- **WHEN** a page exported from LeafMD in light style is opened in a browser on a desktop set to dark style
- **THEN** it shows light text on a dark background

### Requirement: The exported web page's title
The exported web page's title (shown on the browser's tab) SHALL be the document's first top-level heading, or the file's name without its ending when the document has none.

#### Scenario: A document with a heading
- **WHEN** garden.md starts with `# Community Garden Plan` and is exported as HTML
- **THEN** the browser tab reads "Community Garden Plan"

#### Scenario: A document without a heading
- **WHEN** todo.md has no `#` heading and is exported as HTML
- **THEN** the browser tab reads "todo"

### Requirement: What's printed and exported
Print and both exports SHALL use the text in the selected tab as it is now, including unsaved changes, and SHALL NOT save the document or change it. An untitled document SHALL be offered as "Untitled.pdf" or "Untitled.html".

#### Scenario: Unsaved changes
- **WHEN** garden.md was saved with "Order seeds" and the user has since typed "Order soil" without saving, then exports as PDF
- **THEN** the PDF shows "Order soil", and the tab still shows "• garden.md"

#### Scenario: A new document
- **WHEN** the user presses Ctrl+T, types `# Ideas`, and chooses Export as HTML…
- **THEN** the dialog suggests "Untitled.html"

### Requirement: How printed pages look
Printed pages and exported PDFs SHALL always be light (dark text on white), even when LeafMD is in dark style, and SHALL use the font and text size from Preferences, not the window's zoom. Long lines in code blocks SHALL wrap instead of being cut off, and nothing LeafMD shows on screen only, such as find highlights, SHALL be printed.

#### Scenario: Printing in dark style
- **WHEN** GNOME is in dark style and the user prints a document
- **THEN** the pages have dark text on white

#### Scenario: Zoomed in
- **WHEN** the window is zoomed to 150% and the user exports as PDF
- **THEN** the text in the PDF is the size set in Preferences, the same as at 100%

#### Scenario: A long line of code
- **WHEN** a code block contains a 200 character command on one line and the document is printed
- **THEN** the whole command is on the page, wrapped onto more lines

#### Scenario: Find was open
- **WHEN** the find bar is highlighting "soil" in the formatted view and the user exports as PDF
- **THEN** the PDF has no highlighting

### Requirement: Page breaks
Printed pages and exported PDFs SHALL avoid splitting a code block, a table row or a picture across two pages where it fits on one, and SHALL avoid leaving a heading alone at the bottom of a page with its text on the next.

#### Scenario: A heading near the bottom of a page
- **WHEN** a `## Budget` heading would fall on the last line of a page
- **THEN** it moves to the top of the next page, with the paragraph that follows it

### Requirement: Only with a document open
Print…, Export as PDF… and Export as HTML… SHALL be greyed out in the main menu when the window shows "No Document Open", and Ctrl+P SHALL do nothing then.

#### Scenario: No document
- **WHEN** a window shows "No Document Open" and the user opens the main menu
- **THEN** Print…, Export as PDF… and Export as HTML… are greyed out

### Requirement: When something goes wrong
If printing or exporting fails, LeafMD SHALL show a message starting "Couldn't print:" or "Couldn't export:" followed by the reason, and SHALL leave the document as it was.

#### Scenario: A folder that can't be written to
- **WHEN** the user exports as PDF into a folder they don't have permission to write to
- **THEN** a message starting "Couldn't export:" appears, and nothing else changes

#### Scenario: Printing isn't available
- **WHEN** the desktop has no way to show the Print dialog and the user presses Ctrl+P
- **THEN** a message starting "Couldn't print:" appears instead of LeafMD closing

### Requirement: Code colours on paper and in exports
Printed pages, PDFs and exported web pages SHALL colour code blocks the same way the formatted view does. Printed pages and PDFs SHALL use the light colours. An exported web page SHALL use the light colours, or the dark ones when the reader's desktop or browser is dark, like the rest of the page, and SHALL still contain no scripts.

#### Scenario: Printing in dark style
- **WHEN** GNOME is in dark style and the user exports a document with a ` ```python ` block as PDF
- **THEN** the PDF's code is coloured with the light colours on white

#### Scenario: An exported web page opened in a dark browser
- **WHEN** the user exports a document with a ` ```json ` block as HTML and opens it in a browser set to dark
- **THEN** the page is dark and the JSON is coloured with the dark colours, and the file contains no `<script>`

### Requirement: Page margins in PDFs
Exported PDFs SHALL have the same margin on all four sides of the page, set by the Margins setting in Preferences: 5mm for None, 10mm for Narrow, 15mm for Normal and 20mm for Wide. The text SHALL fill the space between the margins. Printing SHALL keep using the margins the system's Print dialog gives, whatever the Margins setting.

#### Scenario: Exporting with Normal margins
- **WHEN** Margins is Normal and the user exports garden.md as PDF on A4 paper
- **THEN** every page of garden.pdf has 15mm of white space at the top, bottom, left and right, and the text runs from the left margin to the right one

#### Scenario: Exporting with Wide margins
- **WHEN** Margins is Wide and the user exports garden.md as PDF
- **THEN** the pages have 20mm margins on every side, the same as before the Margins setting existed

#### Scenario: Printing
- **WHEN** Margins is Narrow and the user presses Ctrl+P, picks A4 in the Print dialog and clicks Preview
- **THEN** the preview shows the margins the Print dialog chose for A4, the same as with any other Margins setting

### Requirement: The exported web page keeps the line width
The exported web page SHALL keep its text in a centred column as wide as the formatted view's column for the Margins setting at the time of export (about 140 characters for Narrow, 100 for Normal and 85 for Wide, or the whole width for None). In a browser window narrower than that, the text SHALL fill the window.

#### Scenario: Exported with Wide margins
- **WHEN** Margins is Wide, garden.md is exported as HTML and garden.html is opened full screen in Firefox on a 1920 pixel wide screen
- **THEN** the text sits in a centred column with lines of about 85 characters

#### Scenario: Exported with no margins
- **WHEN** Margins is None and garden.html is exported and opened full screen in Firefox
- **THEN** the text runs across the whole browser window

#### Scenario: Read on a phone
- **WHEN** the same garden.html is opened on a phone
- **THEN** the text fills the screen's width, with no sideways scrolling

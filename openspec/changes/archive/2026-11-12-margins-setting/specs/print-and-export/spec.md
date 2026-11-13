## MODIFIED Requirements

### Requirement: Export as PDF
Export as PDF… in the main menu SHALL ask where to save, suggesting the document's name with `.pdf` in place of its ending, in the document's folder. Choosing a place SHALL save the formatted document there as a PDF that looks the way printing it would, apart from its page margins (see "Page margins in PDFs"), and show a message saying it was saved. Cancelling SHALL save nothing.

#### Scenario: Exporting a saved file
- **WHEN** `~/notes/garden.md` is selected and the user chooses Export as PDF…
- **THEN** the save dialog opens in `~/notes` suggesting "garden.pdf", and clicking Save writes `~/notes/garden.pdf` and shows "Saved garden.pdf"

#### Scenario: Replacing an earlier export
- **WHEN** `~/notes/garden.pdf` already exists and the user exports garden.md to it again
- **THEN** the dialog asks whether to replace it, and replacing it writes the new PDF

## ADDED Requirements

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

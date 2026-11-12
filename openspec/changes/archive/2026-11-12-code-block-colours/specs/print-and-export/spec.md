## ADDED Requirements

### Requirement: Code colours on paper and in exports
Printed pages, PDFs and exported web pages SHALL colour code blocks the same way the formatted view does. Printed pages and PDFs SHALL use the light colours. An exported web page SHALL use the light colours, or the dark ones when the reader's desktop or browser is dark, like the rest of the page, and SHALL still contain no scripts.

#### Scenario: Printing in dark style
- **WHEN** GNOME is in dark style and the user exports a document with a ` ```python ` block as PDF
- **THEN** the PDF's code is coloured with the light colours on white

#### Scenario: An exported web page opened in a dark browser
- **WHEN** the user exports a document with a ` ```json ` block as HTML and opens it in a browser set to dark
- **THEN** the page is dark and the JSON is coloured with the dark colours, and the file contains no `<script>`

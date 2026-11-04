# Spec Delta

## MODIFIED Requirements

### Requirement: About window
About LeafMD SHALL show the app's name, icon, version and description, the developer (Jereme Hancock) with a link, an AI disclosure saying the project was created with the help of AI, a "Built With" section listing Rust, GTK, libadwaita, GtkSourceView and WebKitGTK with the versions LeafMD is running with plus comrak and ammonia, the GPL 3.0 license, a Website link to `https://leafmd.app/`, a GitHub link and a link for reporting issues.

#### Scenario: Checking library versions
- **WHEN** the user opens About and goes to the credits
- **THEN** the AI disclosure appears right under the developer, and the Built With section shows the running GTK version

#### Scenario: Visiting the website
- **WHEN** the user opens About and clicks Website
- **THEN** `https://leafmd.app/` opens in their web browser

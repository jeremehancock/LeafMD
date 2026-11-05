# Spec Delta

## MODIFIED Requirements

### Requirement: Same icons on every distro
LeafMD SHALL look the same whatever icon theme the system has chosen: its buttons and window controls SHALL use LeafMD's own icons, never the system's icon theme. Small icons GTK adds by itself, like the arrow beside Open, SHALL use the Adwaita icons from the GNOME runtime.

#### Scenario: A system with a different icon theme
- **WHEN** LeafMD runs on a system set to another icon theme, such as Papirus or Ubuntu's Yaru
- **THEN** the main menu button shows three lines, and every other icon matches what LeafMD shows on any other distro

#### Scenario: Maximizing a window
- **WHEN** the window isn't maximized
- **THEN** the maximize button shows two arrows pointing out to opposite corners, and once the user maximizes the window it shows a rounded square

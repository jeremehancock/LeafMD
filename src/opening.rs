//! The rule for which window a file opens in. It's kept apart from the windows
//! themselves so it can be tested without opening any.

/// What the rule needs to know about one window.
pub struct WindowFacts {
    /// One of its tabs shows the file, or is about to
    pub has_file: bool,
    /// It's showing the "No Document Open" page, with nothing on the way
    pub is_empty: bool,
}

/// Where a file should go.
#[derive(Debug, PartialEq)]
pub enum Target {
    /// It's already open there: just show it, by its position in the list.
    AlreadyOpen(usize),
    /// Open it in the window at this position in the list.
    Use(usize),
    /// There's no window to use, so make one.
    NewWindow,
}

/// `windows` is every LeafMD window, used longest ago first, so the last one is
/// the most recently used. `asked_from` is the window that asked (Open, Recent
/// files, a dropped file or a link), or None for a file from outside LeafMD, like
/// the file manager or the terminal.
pub fn choose_window(windows: &[WindowFacts], asked_from: Option<usize>) -> Target {
    // A file is never open twice, whoever asks
    if let Some(position) = windows.iter().position(|window| window.has_file) {
        return Target::AlreadyOpen(position);
    }
    if let Some(position) = asked_from {
        return Target::Use(position);
    }
    // An empty window is waiting for a document, so it goes first
    if let Some(position) = windows.iter().rposition(|window| window.is_empty) {
        return Target::Use(position);
    }
    match windows.len() {
        0 => Target::NewWindow,
        count => Target::Use(count - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::{Target, WindowFacts, choose_window};

    const WITH_DOCUMENT: WindowFacts = WindowFacts { has_file: false, is_empty: false };
    const EMPTY: WindowFacts = WindowFacts { has_file: false, is_empty: true };
    const HAS_THE_FILE: WindowFacts = WindowFacts { has_file: true, is_empty: false };

    #[test]
    fn with_leafmd_not_running_it_makes_a_window() {
        assert_eq!(choose_window(&[], None), Target::NewWindow);
    }

    #[test]
    fn a_file_from_outside_uses_an_empty_window() {
        assert_eq!(choose_window(&[EMPTY], None), Target::Use(0));
    }

    #[test]
    fn a_file_from_outside_becomes_a_tab_in_the_only_window() {
        assert_eq!(choose_window(&[WITH_DOCUMENT], None), Target::Use(0));
    }

    #[test]
    fn a_file_from_outside_goes_to_the_most_recently_used_window() {
        assert_eq!(choose_window(&[WITH_DOCUMENT, WITH_DOCUMENT], None), Target::Use(1));
    }

    #[test]
    fn an_empty_window_comes_before_the_most_recently_used_one() {
        assert_eq!(choose_window(&[EMPTY, WITH_DOCUMENT], None), Target::Use(0));
    }

    #[test]
    fn of_two_empty_windows_it_uses_the_most_recently_used() {
        assert_eq!(choose_window(&[EMPTY, EMPTY], None), Target::Use(1));
    }

    #[test]
    fn a_link_stays_in_its_window_even_with_an_empty_one_elsewhere() {
        assert_eq!(choose_window(&[WITH_DOCUMENT, EMPTY], Some(0)), Target::Use(0));
    }

    #[test]
    fn an_open_file_is_shown_rather_than_opened_again() {
        assert_eq!(choose_window(&[HAS_THE_FILE, EMPTY], None), Target::AlreadyOpen(0));
    }

    #[test]
    fn an_open_file_is_shown_even_when_another_window_asks() {
        assert_eq!(choose_window(&[HAS_THE_FILE, WITH_DOCUMENT], Some(1)), Target::AlreadyOpen(0));
    }
}

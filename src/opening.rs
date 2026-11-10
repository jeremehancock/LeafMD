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

/// What the rule for folders needs to know about one window.
pub struct FolderFacts {
    /// The side panel already shows this folder as the window's chosen folder
    pub has_folder: bool,
    /// It's showing the "No Document Open" page with nothing on the way, and
    /// no folder has been chosen in it
    pub is_empty: bool,
}

/// Where a folder from outside LeafMD (`leafmd .`) should go. Like VS Code, a
/// folder gets a window of its own, unless a window already shows it or one is
/// sitting empty. `windows` is in the same order as for `choose_window`.
pub fn choose_window_for_folder(windows: &[FolderFacts]) -> Target {
    if let Some(position) = windows.iter().position(|window| window.has_folder) {
        return Target::AlreadyOpen(position);
    }
    match windows.iter().rposition(|window| window.is_empty) {
        Some(position) => Target::Use(position),
        None => Target::NewWindow,
    }
}

/// Where a tab being brought back at startup goes, given the order saved when
/// the window closed and the tabs there now (their files, or None for a document
/// with no file). They can arrive in any order, because each file is read by
/// itself. It goes just before the first tab that was saved after it, or at the
/// end if there isn't one, so a new document someone just started stays last.
pub fn restored_tab_position(saved: &[String], tabs: &[Option<String>], uri: &str) -> usize {
    let place = |uri: &str| saved.iter().position(|saved| saved == uri);
    let mine = place(uri);
    tabs.iter()
        .position(|tab| {
            let theirs = tab.as_deref().and_then(place);
            matches!((theirs, mine), (Some(theirs), Some(mine)) if theirs > mine)
        })
        .unwrap_or(tabs.len())
}

#[cfg(test)]
mod tests {
    use super::{
        FolderFacts, Target, WindowFacts, choose_window, choose_window_for_folder,
        restored_tab_position,
    };

    const BUSY: FolderFacts = FolderFacts { has_folder: false, is_empty: false };
    const WAITING: FolderFacts = FolderFacts { has_folder: false, is_empty: true };
    const SHOWS_IT: FolderFacts = FolderFacts { has_folder: true, is_empty: false };

    #[test]
    fn a_folder_with_leafmd_not_running_makes_a_window() {
        assert_eq!(choose_window_for_folder(&[]), Target::NewWindow);
    }

    #[test]
    fn a_folder_gets_its_own_window_rather_than_joining_documents() {
        assert_eq!(choose_window_for_folder(&[BUSY, BUSY]), Target::NewWindow);
    }

    #[test]
    fn a_folder_uses_an_empty_window_the_most_recent_first() {
        assert_eq!(choose_window_for_folder(&[WAITING, BUSY]), Target::Use(0));
        assert_eq!(choose_window_for_folder(&[WAITING, WAITING]), Target::Use(1));
    }

    #[test]
    fn a_folder_already_showing_is_brought_forward() {
        assert_eq!(choose_window_for_folder(&[WAITING, SHOWS_IT]), Target::AlreadyOpen(1));
    }

    const WITH_DOCUMENT: WindowFacts = WindowFacts { has_file: false, is_empty: false };
    const EMPTY: WindowFacts = WindowFacts { has_file: false, is_empty: true };
    const HAS_THE_FILE: WindowFacts = WindowFacts { has_file: true, is_empty: false };

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|name| name.to_string()).collect()
    }

    fn tabs(list: &[Option<&str>]) -> Vec<Option<String>> {
        list.iter().map(|tab| tab.map(str::to_string)).collect()
    }

    #[test]
    fn restored_tabs_land_in_the_saved_order_however_they_arrive() {
        let saved = names(&["a", "b", "c", "d"]);
        let mut open = tabs(&[Some("c")]);
        // The selected file is first, then the rest in a muddled order
        for uri in ["d", "a", "b"] {
            let position = restored_tab_position(&saved, &open, uri);
            open.insert(position, Some(uri.to_string()));
        }
        assert_eq!(open, tabs(&[Some("a"), Some("b"), Some("c"), Some("d")]));
    }

    #[test]
    fn a_new_document_started_meanwhile_stays_after_the_restored_tabs() {
        let saved = names(&["a", "b"]);
        let open = tabs(&[Some("a"), None]);
        assert_eq!(restored_tab_position(&saved, &open, "b"), 2);
        let open = tabs(&[None, Some("b")]);
        assert_eq!(restored_tab_position(&saved, &open, "a"), 1);
    }

    #[test]
    fn a_missing_file_leaves_the_rest_in_order() {
        let saved = names(&["a", "b", "c"]);
        assert_eq!(restored_tab_position(&saved, &tabs(&[Some("c")]), "a"), 0);
    }

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

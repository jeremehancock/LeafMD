//! Remembers a few things between runs (the last view and the window size)
//! in a small text file at ~/.config/leafmd/state.ini.

use std::path::PathBuf;

use gtk::glib;

/// Which panes are showing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    /// Formatted view only
    Preview,
    /// Raw markdown only
    Source,
    /// Raw on the left, formatted on the right
    Split,
}

impl ViewMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ViewMode::Preview => "preview",
            ViewMode::Source => "source",
            ViewMode::Split => "split",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "preview" => Some(ViewMode::Preview),
            "source" => Some(ViewMode::Source),
            "split" => Some(ViewMode::Split),
            _ => None,
        }
    }
}

pub struct SavedState {
    pub view: ViewMode,
    pub width: i32,
    pub height: i32,
    pub maximized: bool,
}

impl Default for SavedState {
    fn default() -> Self {
        Self {
            view: ViewMode::Preview,
            width: 1000,
            height: 720,
            maximized: false,
        }
    }
}

const GROUP: &str = "window";

fn state_path() -> PathBuf {
    glib::user_config_dir().join("leafmd").join("state.ini")
}

impl SavedState {
    /// Reads the saved state. Anything missing or unreadable falls back to the default.
    pub fn load() -> Self {
        let mut state = Self::default();
        let file = glib::KeyFile::new();
        if file
            .load_from_file(state_path(), glib::KeyFileFlags::NONE)
            .is_err()
        {
            return state;
        }
        if let Some(view) = file
            .string(GROUP, "view")
            .ok()
            .and_then(|v| ViewMode::parse(&v))
        {
            state.view = view;
        }
        if let Ok(width) = file.integer(GROUP, "width")
            && width >= 360
        {
            state.width = width;
        }
        if let Ok(height) = file.integer(GROUP, "height")
            && height >= 300
        {
            state.height = height;
        }
        if let Ok(maximized) = file.boolean(GROUP, "maximized") {
            state.maximized = maximized;
        }
        state
    }

    pub fn save(&self) {
        update_state_file(|file| {
            file.set_string(GROUP, "view", self.view.as_str());
            file.set_integer(GROUP, "width", self.width);
            file.set_integer(GROUP, "height", self.height);
            file.set_boolean(GROUP, "maximized", self.maximized);
        });
    }
}

const RECENT_GROUP: &str = "recent";

/// How many recent files to remember.
pub const RECENT_LIMIT: usize = 10;

/// The recently opened files (as file:// addresses), newest first.
pub fn load_recent() -> Vec<String> {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return Vec::new();
    }
    // One address per line (file:// addresses can't contain a line break)
    file.string(RECENT_GROUP, "files")
        .map(|text| {
            text.lines()
                .filter(|uri| !uri.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

pub fn save_recent(uris: &[String]) {
    update_state_file(|file| file.set_string(RECENT_GROUP, "files", &uris.join("\n")));
}

/// The selected tab in the LeafMD window that was closed last. (Not simply the
/// newest recent file: a file opened after it may have been closed first.) An
/// empty value means there's nothing to reopen; see `tabs_to_reopen`.
pub fn load_last_file() -> Option<String> {
    let file = glib::KeyFile::new();
    file.load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .ok()?;
    file.string(RECENT_GROUP, "last")
        .ok()
        .map(|uri| uri.to_string())
}

/// All the tabs in that window, in order. None for a state file from before
/// LeafMD had tabs, which only has the last file.
pub fn load_saved_tabs() -> Option<Vec<String>> {
    let file = glib::KeyFile::new();
    file.load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .ok()?;
    let text = file.string(RECENT_GROUP, "tabs").ok()?;
    Some(text.lines().filter(|uri| !uri.is_empty()).map(str::to_string).collect())
}

/// Saves the tabs for next time. Older versions of LeafMD only read `last`, so
/// they reopen the selected tab by itself.
pub fn save_tabs(tabs: &TabList) {
    update_state_file(|file| {
        file.set_string(RECENT_GROUP, "tabs", &tabs.files.join("\n"));
        file.set_string(RECENT_GROUP, "last", &tabs.selected);
    });
}

/// Makes the next start show the "No Document Open" page, by saving nothing to
/// reopen. Used by Close Document and Clear Recent Files.
pub fn forget_last_file() {
    update_state_file(|file| {
        file.set_string(RECENT_GROUP, "tabs", "");
        file.set_string(RECENT_GROUP, "last", "");
    });
}

const PANEL_GROUP: &str = "side-panel";

/// Whether new windows open with the side panel showing. Closed unless the user
/// opened it last time they chose.
pub fn load_panel_open() -> bool {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return false;
    }
    panel_open_in(&file)
}

fn panel_open_in(file: &glib::KeyFile) -> bool {
    file.boolean(PANEL_GROUP, "open").unwrap_or(false)
}

/// Remembers the user opening or closing the side panel.
pub fn save_panel_open(open: bool) {
    update_state_file(|file| file.set_boolean(PANEL_GROUP, "open", open));
}

const EDITOR_GROUP: &str = "editor";

/// Whether the raw text shows line numbers. Off unless the user turned them on.
pub fn load_line_numbers() -> bool {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return false;
    }
    line_numbers_in(&file)
}

fn line_numbers_in(file: &glib::KeyFile) -> bool {
    file.boolean(EDITOR_GROUP, "line-numbers").unwrap_or(false)
}

/// Remembers the user turning line numbers on or off.
pub fn save_line_numbers(show: bool) {
    update_state_file(|file| file.set_boolean(EDITOR_GROUP, "line-numbers", show));
}

/// Some files that belong in tabs, and which one is selected.
#[derive(Debug, PartialEq)]
pub struct TabList {
    pub files: Vec<String>,
    pub selected: String,
}

/// The position of the nearest item that passes `keep`, starting at `from` and
/// looking to its right before its left at each distance.
fn nearest<T>(items: &[T], from: usize, keep: impl Fn(&T) -> bool) -> Option<usize> {
    (0..items.len())
        .flat_map(|distance| [from.checked_add(distance), from.checked_sub(distance)])
        .flatten()
        .filter(|&position| position < items.len())
        .find(|&position| keep(&items[position]))
}

/// What to remember when a window closes. `tabs` has each tab's file, or None for
/// a document that was never saved. Only files on the recent list count: after
/// Clear Recent Files, closing a file that's still open shouldn't bring it back.
/// None means there's nothing to remember, so what was saved before stays.
pub fn tabs_to_save(
    tabs: &[Option<String>],
    selected: Option<usize>,
    recent: &[String],
) -> Option<TabList> {
    let keep = |tab: &Option<String>| tab.as_ref().is_some_and(|uri| recent.contains(uri));
    let chosen = nearest(tabs, selected.unwrap_or(0), keep)?;
    Some(TabList {
        files: tabs.iter().filter(|tab| keep(tab)).flatten().cloned().collect(),
        selected: tabs[chosen].clone()?,
    })
}

/// Picks the files to open when LeafMD starts, from what was saved and the recent
/// list. There are three cases:
/// - nothing saved (say LeafMD didn't close normally): the newest recent file
/// - an empty value (the user closed the document or cleared the list): no files
/// - a file: the saved tabs, with that one selected. If it isn't one of them, an
///   older version of LeafMD saved it, and the tabs are out of date, so it opens
///   by itself.
///
/// Files that no longer exist are left out, and if that includes the selected
/// one, the nearest one left is selected instead.
pub fn tabs_to_reopen(
    saved_last: Option<&str>,
    saved_tabs: Option<&[String]>,
    recent: &[String],
    exists: impl Fn(&str) -> bool,
) -> Option<TabList> {
    let selected = match saved_last {
        None => recent.first()?.as_str(),
        Some("") => return None,
        Some(uri) => uri,
    };
    let files = match saved_tabs {
        Some(tabs) if saved_last.is_some() && tabs.iter().any(|uri| uri == selected) => tabs.to_vec(),
        _ => vec![selected.to_string()],
    };
    let from = files.iter().position(|uri| uri == selected)?;
    let chosen = nearest(&files, from, |uri| exists(uri))?;
    Some(TabList {
        selected: files[chosen].clone(),
        files: files.iter().filter(|uri| exists(uri)).cloned().collect(),
    })
}

/// Reads the state file, lets `change` edit it, and writes it back, so saving
/// one setting never wipes out the others.
fn update_state_file(change: impl FnOnce(&glib::KeyFile)) {
    let path = state_path();
    let file = glib::KeyFile::new();
    let _ = file.load_from_file(&path, glib::KeyFileFlags::KEEP_COMMENTS);
    change(&file);

    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(err) = file.save_to_file(&path) {
        eprintln!("LeafMD: could not save settings: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::{TabList, line_numbers_in, panel_open_in, tabs_to_reopen, tabs_to_save};
    use gtk::glib;

    fn key_file(text: &str) -> glib::KeyFile {
        let file = glib::KeyFile::new();
        file.load_from_data(text, glib::KeyFileFlags::NONE).expect("test state file reads");
        file
    }

    #[test]
    fn the_side_panel_starts_closed_unless_it_was_left_open() {
        assert!(!panel_open_in(&key_file("[window]\nview=split\n")));
        assert!(!panel_open_in(&key_file("[side-panel]\nopen=false\n")));
        assert!(!panel_open_in(&key_file("[side-panel]\nopen=maybe\n")));
        assert!(panel_open_in(&key_file("[side-panel]\nopen=true\n")));
    }

    #[test]
    fn line_numbers_are_off_unless_turned_on() {
        assert!(!line_numbers_in(&key_file("[window]\nview=source\n")));
        assert!(!line_numbers_in(&key_file("[editor]\nline-numbers=false\n")));
        assert!(!line_numbers_in(&key_file("[editor]\nline-numbers=maybe\n")));
        assert!(line_numbers_in(&key_file("[editor]\nline-numbers=true\n")));
    }

    fn list(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    fn tab_list(names: &[&str], selected: &str) -> Option<TabList> {
        Some(TabList { files: list(names), selected: selected.to_string() })
    }

    fn all_exist(_: &str) -> bool {
        true
    }

    #[test]
    fn nothing_saved_uses_the_newest_recent_file() {
        let recent = list(&["newest", "older"]);
        assert_eq!(tabs_to_reopen(None, None, &recent, all_exist), tab_list(&["newest"], "newest"));
        assert_eq!(tabs_to_reopen(None, None, &[], all_exist), None);
    }

    #[test]
    fn saved_tabs_come_back_in_order_with_the_same_one_selected() {
        let tabs = list(&["a", "b", "c"]);
        assert_eq!(
            tabs_to_reopen(Some("b"), Some(&tabs), &list(&["c"]), all_exist),
            tab_list(&["a", "b", "c"], "b")
        );
    }

    #[test]
    fn an_older_state_file_with_no_tabs_opens_the_last_file() {
        assert_eq!(
            tabs_to_reopen(Some("older"), None, &list(&["newest", "older"]), all_exist),
            tab_list(&["older"], "older")
        );
    }

    #[test]
    fn a_last_file_that_isnt_among_the_tabs_opens_by_itself() {
        // An older version of LeafMD ran in between, and only updated the last file
        let tabs = list(&["a", "b"]);
        assert_eq!(
            tabs_to_reopen(Some("c"), Some(&tabs), &[], all_exist),
            tab_list(&["c"], "c")
        );
    }

    #[test]
    fn an_empty_value_opens_nothing_even_with_recent_files() {
        let recent = list(&["newest"]);
        assert_eq!(tabs_to_reopen(Some(""), Some(&[]), &recent, all_exist), None);
        assert_eq!(tabs_to_reopen(Some(""), None, &recent, all_exist), None);
    }

    #[test]
    fn missing_files_are_left_out() {
        let tabs = list(&["a", "b", "c"]);
        assert_eq!(
            tabs_to_reopen(Some("c"), Some(&tabs), &[], |uri| uri != "a"),
            tab_list(&["b", "c"], "c")
        );
    }

    #[test]
    fn a_selected_file_thats_gone_selects_the_nearest_one_left() {
        let tabs = list(&["a", "b", "c"]);
        assert_eq!(
            tabs_to_reopen(Some("b"), Some(&tabs), &[], |uri| uri != "b"),
            tab_list(&["a", "c"], "c")
        );
        assert_eq!(
            tabs_to_reopen(Some("c"), Some(&tabs), &[], |uri| uri != "c"),
            tab_list(&["a", "b"], "b")
        );
    }

    #[test]
    fn nothing_left_opens_nothing() {
        let tabs = list(&["a", "b"]);
        assert_eq!(tabs_to_reopen(Some("a"), Some(&tabs), &[], |_| false), None);
        assert_eq!(tabs_to_reopen(None, None, &list(&["a"]), |_| false), None);
    }

    fn open(names: &[Option<&str>]) -> Vec<Option<String>> {
        names.iter().map(|name| name.map(str::to_string)).collect()
    }

    #[test]
    fn saving_keeps_the_order_and_the_selected_tab() {
        let tabs = open(&[Some("a"), Some("b"), Some("c")]);
        assert_eq!(
            tabs_to_save(&tabs, Some(1), &list(&["c", "b", "a"])),
            tab_list(&["a", "b", "c"], "b")
        );
    }

    #[test]
    fn a_never_saved_tab_is_left_out_and_the_nearest_file_is_selected() {
        let tabs = open(&[Some("notes"), None]);
        assert_eq!(tabs_to_save(&tabs, Some(1), &list(&["notes"])), tab_list(&["notes"], "notes"));
    }

    #[test]
    fn files_off_the_recent_list_are_left_out() {
        // Clear Recent Files, then saving only todo puts just that back on the list
        let tabs = open(&[Some("notes"), Some("todo")]);
        assert_eq!(tabs_to_save(&tabs, Some(0), &list(&["todo"])), tab_list(&["todo"], "todo"));
    }

    #[test]
    fn a_window_with_no_saved_file_remembers_nothing() {
        assert_eq!(tabs_to_save(&open(&[None]), Some(0), &list(&["a"])), None);
        assert_eq!(tabs_to_save(&[], None, &list(&["a"])), None);
        assert_eq!(tabs_to_save(&open(&[Some("a")]), Some(0), &[]), None);
    }
}

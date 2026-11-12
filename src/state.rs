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

/// The side panel's pages, by the names LeafMD knows them by
pub const PANEL_PAGES: [&str; 2] = ["files", "headings"];

/// Which page new windows open the side panel on: the one the user chose
/// last, or Files.
pub fn load_panel_page() -> &'static str {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return PANEL_PAGES[0];
    }
    panel_page_in(&file)
}

fn panel_page_in(file: &glib::KeyFile) -> &'static str {
    let saved = file.string(PANEL_GROUP, "page").ok();
    PANEL_PAGES
        .into_iter()
        .find(|page| saved.as_deref() == Some(*page))
        .unwrap_or(PANEL_PAGES[0])
}

/// Remembers the page the user chose in the side panel.
pub fn save_panel_page(page: &str) {
    update_state_file(|file| file.set_string(PANEL_GROUP, "page", page));
}

/// The side panel's width when nobody has resized it, in pixels
pub const PANEL_WIDTH: i32 = 240;
/// How narrow and how wide dragging its edge can make the side panel
pub const PANEL_WIDTH_MIN: i32 = 200;
pub const PANEL_WIDTH_MAX: i32 = 400;

/// How wide new windows make the side panel: the width the user dragged it to
/// last, or 240.
pub fn load_panel_width() -> i32 {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return PANEL_WIDTH;
    }
    panel_width_in(&file)
}

/// A saved width that isn't a number, or is outside what dragging allows
/// (say someone edited the file by hand), counts as no width saved.
fn panel_width_in(file: &glib::KeyFile) -> i32 {
    file.integer(PANEL_GROUP, "width")
        .ok()
        .filter(|width| (PANEL_WIDTH_MIN..=PANEL_WIDTH_MAX).contains(width))
        .unwrap_or(PANEL_WIDTH)
}

/// Remembers the width the user dragged the side panel to.
pub fn save_panel_width(width: i32) {
    update_state_file(|file| file.set_integer(PANEL_GROUP, "width", width));
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

/// Whether misspelt words in the raw text are underlined. On unless the user
/// turned it off.
pub fn load_spell_check() -> bool {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return true;
    }
    spell_check_in(&file)
}

fn spell_check_in(file: &glib::KeyFile) -> bool {
    file.boolean(EDITOR_GROUP, "spell-check").unwrap_or(true)
}

/// Remembers the user turning spell checking on or off.
pub fn save_spell_check(on: bool) {
    update_state_file(|file| file.set_boolean(EDITOR_GROUP, "spell-check", on));
}

/// The language the user chose for spell checking, such as "en_GB", or None
/// if they never chose one (spell checking then follows the system's).
pub fn load_spell_check_language() -> Option<String> {
    let file = glib::KeyFile::new();
    file.load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .ok()?;
    spell_check_language_in(&file)
}

fn spell_check_language_in(file: &glib::KeyFile) -> Option<String> {
    file.string(EDITOR_GROUP, "spell-check-language")
        .ok()
        .map(|language| language.to_string())
        .filter(|language| !language.is_empty())
}

/// Remembers the language the user chose for spell checking.
pub fn save_spell_check_language(language: &str) {
    update_state_file(|file| file.set_string(EDITOR_GROUP, "spell-check-language", language));
}

const PREFERENCES_GROUP: &str = "preferences";

/// Which of the two fonts in Preferences.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontFor {
    /// The raw text editor
    Raw,
    /// The formatted view
    Formatted,
}

impl FontFor {
    fn key(self) -> &'static str {
        match self {
            FontFor::Raw => "raw-font",
            FontFor::Formatted => "formatted-font",
        }
    }
}

/// The font the user chose in Preferences, as a font description such as
/// "Fira Code 12". None means they never chose one (or pressed Reset), so the
/// starting font is used.
pub fn load_font(which: FontFor) -> Option<String> {
    let file = glib::KeyFile::new();
    file.load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .ok()?;
    font_in(&file, which)
}

fn font_in(file: &glib::KeyFile, which: FontFor) -> Option<String> {
    let font = file.string(PREFERENCES_GROUP, which.key()).ok()?;
    let font = font.trim();
    (!font.is_empty()).then(|| font.to_string())
}

/// Remembers a font chosen in Preferences, or with None forgets it (Reset).
pub fn save_font(which: FontFor, font: Option<&str>) {
    update_state_file(|file| set_font_in(file, which, font));
}

fn set_font_in(file: &glib::KeyFile, which: FontFor, font: Option<&str>) {
    match font {
        Some(font) => file.set_string(PREFERENCES_GROUP, which.key(), font),
        None => {
            let _ = file.remove_key(PREFERENCES_GROUP, which.key());
        }
    }
}

/// The choices for Style in Preferences: follow the desktop's light or dark
/// style, or always be light, or always dark. The first is the starting one.
pub const STYLES: [&str; 3] = ["system", "light", "dark"];

/// The style chosen in Preferences, or "system" if none was (or the saved
/// value isn't one LeafMD knows).
pub fn load_style() -> &'static str {
    let file = glib::KeyFile::new();
    if file
        .load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .is_err()
    {
        return STYLES[0];
    }
    style_in(&file)
}

fn style_in(file: &glib::KeyFile) -> &'static str {
    let saved = file.string(PREFERENCES_GROUP, "style").unwrap_or_default();
    STYLES
        .into_iter()
        .find(|style| *style == saved.trim())
        .unwrap_or(STYLES[0])
}

/// Remembers the style chosen in Preferences.
pub fn save_style(style: &str) {
    update_state_file(|file| file.set_string(PREFERENCES_GROUP, "style", style));
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
    use super::{
        FontFor, TabList, font_in, line_numbers_in, spell_check_in, spell_check_language_in, panel_open_in, panel_page_in, panel_width_in, set_font_in, style_in,
        tabs_to_reopen, tabs_to_save,
    };
    use gtk::glib;

    fn key_file(text: &str) -> glib::KeyFile {
        let file = glib::KeyFile::new();
        file.load_from_data(text, glib::KeyFileFlags::NONE).expect("test state file reads");
        file
    }

    #[test]
    fn the_side_panel_opens_on_the_page_chosen_last_or_files() {
        assert_eq!(panel_page_in(&key_file("[side-panel]\npage=headings\n")), "headings");
        assert_eq!(panel_page_in(&key_file("[side-panel]\nopen=true\n")), "files");
        assert_eq!(panel_page_in(&key_file("[side-panel]\npage=outline\n")), "files");
    }

    #[test]
    fn the_side_panel_opens_at_the_width_chosen_last_or_240() {
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=320\n")), 320);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=200\n")), 200);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=400\n")), 400);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nopen=true\n")), 240);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=wide\n")), 240);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=900\n")), 240);
        assert_eq!(panel_width_in(&key_file("[side-panel]\nwidth=100\n")), 240);
    }

    #[test]
    fn the_side_panel_starts_closed_unless_it_was_left_open() {
        assert!(!panel_open_in(&key_file("[window]\nview=split\n")));
        assert!(!panel_open_in(&key_file("[side-panel]\nopen=false\n")));
        assert!(!panel_open_in(&key_file("[side-panel]\nopen=maybe\n")));
        assert!(panel_open_in(&key_file("[side-panel]\nopen=true\n")));
    }

    #[test]
    fn spell_check_is_on_unless_turned_off() {
        assert!(spell_check_in(&key_file("[window]\nview=source\n")));
        assert!(spell_check_in(&key_file("[editor]\nspell-check=true\n")));
        assert!(!spell_check_in(&key_file("[editor]\nspell-check=false\n")));
    }

    #[test]
    fn spell_check_language_only_when_chosen() {
        assert_eq!(spell_check_language_in(&key_file("[editor]\nline-numbers=true\n")), None);
        assert_eq!(spell_check_language_in(&key_file("[editor]\nspell-check-language=\n")), None);
        assert_eq!(
            spell_check_language_in(&key_file("[editor]\nspell-check-language=es_ES\n")),
            Some("es_ES".to_string())
        );
    }

    #[test]
    fn line_numbers_are_off_unless_turned_on() {
        assert!(!line_numbers_in(&key_file("[window]\nview=source\n")));
        assert!(!line_numbers_in(&key_file("[editor]\nline-numbers=false\n")));
        assert!(!line_numbers_in(&key_file("[editor]\nline-numbers=maybe\n")));
        assert!(line_numbers_in(&key_file("[editor]\nline-numbers=true\n")));
    }

    #[test]
    fn fonts_start_unchosen() {
        let file = key_file("[window]\nview=split\n");
        assert_eq!(font_in(&file, FontFor::Raw), None);
        assert_eq!(font_in(&file, FontFor::Formatted), None);
        assert_eq!(font_in(&key_file("[preferences]\nraw-font=\n"), FontFor::Raw), None);
    }

    #[test]
    fn a_chosen_font_comes_back_and_reset_forgets_it() {
        let file = key_file("[window]\nview=split\n");
        set_font_in(&file, FontFor::Raw, Some("Fira Code 12"));
        assert_eq!(font_in(&file, FontFor::Raw).as_deref(), Some("Fira Code 12"));
        assert_eq!(font_in(&file, FontFor::Formatted), None);
        set_font_in(&file, FontFor::Raw, None);
        assert_eq!(font_in(&file, FontFor::Raw), None);
        // Forgetting a font that was never chosen is fine too
        set_font_in(&file, FontFor::Formatted, None);
    }

    #[test]
    fn the_style_follows_the_system_unless_light_or_dark_was_chosen() {
        assert_eq!(style_in(&key_file("[window]\nview=split\n")), "system");
        assert_eq!(style_in(&key_file("[preferences]\nstyle=dark\n")), "dark");
        assert_eq!(style_in(&key_file("[preferences]\nstyle=light\n")), "light");
        assert_eq!(style_in(&key_file("[preferences]\nstyle=purple\n")), "system");
        assert_eq!(style_in(&key_file("[preferences]\nstyle=\n")), "system");
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

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

/// The file in the LeafMD window that was closed last, which is the one to
/// reopen next time. (Not simply the newest recent file: open README, click a
/// link to LICENSE, close LICENSE and then README, and README should come back.)
/// An empty value means there's nothing to reopen; see `file_to_reopen`.
pub fn load_last_file() -> Option<String> {
    let file = glib::KeyFile::new();
    file.load_from_file(state_path(), glib::KeyFileFlags::NONE)
        .ok()?;
    file.string(RECENT_GROUP, "last")
        .ok()
        .map(|uri| uri.to_string())
}

pub fn save_last_file(uri: &str) {
    update_state_file(|file| file.set_string(RECENT_GROUP, "last", uri));
}

/// Makes the next start show the "No Document Open" page, by saving an empty
/// last file. Used by Close Document and Clear Recent Files.
pub fn forget_last_file() {
    save_last_file("");
}

/// Picks the file to open when LeafMD starts, from the saved last file and the
/// recent list. There are three cases:
/// - nothing saved (say LeafMD didn't close normally): the newest recent file
/// - an empty value (the user closed the document or cleared the list): no file
/// - a file: that file
pub fn file_to_reopen(saved_last: Option<&str>, recent: &[String]) -> Option<String> {
    match saved_last {
        None => recent.first().cloned(),
        Some("") => None,
        Some(uri) => Some(uri.to_string()),
    }
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
    use super::file_to_reopen;

    fn recent() -> Vec<String> {
        vec!["file:///newest.md".to_string(), "file:///older.md".to_string()]
    }

    #[test]
    fn nothing_saved_uses_the_newest_recent_file() {
        assert_eq!(file_to_reopen(None, &recent()).as_deref(), Some("file:///newest.md"));
        assert_eq!(file_to_reopen(None, &[]), None);
    }

    #[test]
    fn a_saved_file_wins_over_the_recent_list() {
        assert_eq!(
            file_to_reopen(Some("file:///older.md"), &recent()).as_deref(),
            Some("file:///older.md")
        );
    }

    #[test]
    fn an_empty_value_opens_nothing_even_with_recent_files() {
        assert_eq!(file_to_reopen(Some(""), &recent()), None);
    }
}

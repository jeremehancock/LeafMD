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
        let file = glib::KeyFile::new();
        file.set_string(GROUP, "view", self.view.as_str());
        file.set_integer(GROUP, "width", self.width);
        file.set_integer(GROUP, "height", self.height);
        file.set_boolean(GROUP, "maximized", self.maximized);

        let path = state_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Err(err) = file.save_to_file(&path) {
            eprintln!("LeafMD: could not save settings: {err}");
        }
    }
}

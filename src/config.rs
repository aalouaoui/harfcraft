use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};
pub const DEFAULT_THEME: &str = "Default Dark";
pub const RECENT_PROJECTS_LIMIT: usize = 100;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub active_theme: String,
    pub last_open_project: Option<PathBuf>,
    pub recent_projects: Vec<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            active_theme: DEFAULT_THEME.to_string(),
            last_open_project: None,
            recent_projects: Vec::new(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("harfcraft").join("config.json"))
    }

    // TODO: switch to a result type and handle errors
    pub fn load() -> Self {
        let Some(config_path) = Self::config_path() else {
            return Self::default();
        };
        if !config_path.exists() {
            return Self::default();
        }
        let Ok(file) = File::open(config_path) else {
            return Self::default();
        };
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    }

    // TODO: switch to a result type and handle errors
    pub fn save(&self) {
        let Some(config_path) = Self::config_path() else {
            return;
        };
        if let Some(parent) = config_path.parent() {
            _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self).unwrap_or_default();
        _ = fs::write(config_path, json);
    }

    pub fn open_project(&mut self, path: PathBuf) {
        self.last_open_project = Some(path.clone());
        self.recent_projects.retain(|p| p != &path);
        self.recent_projects.insert(0, path);
        self.recent_projects.truncate(RECENT_PROJECTS_LIMIT);
        self.save();
    }

    pub fn close_project(&mut self) {
        self.last_open_project = None;
        self.save();
    }

    pub fn change_theme(&mut self, theme: impl Into<String>) {
        self.active_theme = theme.into();
        self.save();
    }
}

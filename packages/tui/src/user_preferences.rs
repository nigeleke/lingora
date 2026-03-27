use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct UserPreferences {
    pub(crate) theme_name: Option<String>,
}

fn config_file_path() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|dirs| dirs.config_local_dir().join("lingora-tui.toml"))
}

impl UserPreferences {
    pub fn load() -> Self {
        config_file_path()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    fn persist(&self) {
        if let Some(path) = config_file_path()
            && let Ok(content) = toml::to_string_pretty(self)
        {
            std::fs::write(path, content).ok();
        };
    }

    pub fn theme(&self) -> Option<&String> {
        self.theme_name.as_ref()
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        self.theme_name = Some(String::from(theme_name));
        self.persist();
    }
}

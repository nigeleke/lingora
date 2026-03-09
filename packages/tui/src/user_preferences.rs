use std::{path::PathBuf, time::Duration};

use ratatui_themes::ThemeName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct UserPreferences {
    pub(crate) theme: ThemeName,
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

    pub fn theme(&self) -> ThemeName {
        self.theme
    }

    pub fn set_theme(&mut self, theme: ThemeName) {
        self.theme = theme;
        self.persist();
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: user_system_theme(),
        }
    }
}

fn user_system_theme() -> ThemeName {
    match termbg::theme(Duration::from_millis(500)) {
        Ok(theme) => match theme {
            termbg::Theme::Light => ThemeName::CatppuccinLatte,
            termbg::Theme::Dark => ThemeName::CatppuccinMocha,
        },
        Err(_) => ThemeName::CatppuccinMocha,
    }
}

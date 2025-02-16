use crate::config::Settings;

use std::path::PathBuf;

#[derive(Clone)]
pub struct State {
    reference_path: PathBuf,
    selected_target_path: Option<PathBuf>,
}

impl State {
    pub fn reference_path(&self) -> &PathBuf {
        &self.reference_path
    }

    pub fn target_path(&self) -> Option<&PathBuf> {
        self.selected_target_path.as_ref()
    }

    pub fn set_target_path(&mut self, path: PathBuf) {
        self.selected_target_path = Some(path);
    }
}

impl From<&Settings> for State {
    fn from(value: &Settings) -> Self {
        Self {
            reference_path: value.reference().clone(),
            selected_target_path: None,
        }
    }
}

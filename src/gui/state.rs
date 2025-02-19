use std::path::PathBuf;

use crate::{config::Settings, domain::Identifier};

#[derive(Clone)]
pub struct State {
    reference_path: PathBuf,
    selected_target_path: Option<PathBuf>,
    selected_identifier: Option<Identifier>,
    error: Option<String>,
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

    pub fn identifier(&self) -> Option<&Identifier> {
        self.selected_identifier.as_ref()
    }

    pub fn set_identifier(&mut self, identifier: &Identifier) {
        self.selected_identifier = Some(identifier.clone());
    }

    pub fn error_string(&self) -> String {
        self.error.clone().unwrap_or("".into())
    }

    pub fn set_error_string(&mut self, s: &str) {
        self.error = Some(s.into());
    }
}

impl From<&Settings> for State {
    fn from(value: &Settings) -> Self {
        Self {
            reference_path: value.reference().clone(),
            selected_target_path: None,
            selected_identifier: None,
            error: None,
        }
    }
}

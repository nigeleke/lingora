use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{domain::Locale, error::LingoraError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FluentFile {
    pub(crate) path: PathBuf,
    pub(crate) locale: Locale,
}

impl FluentFile {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl TryFrom<&Path> for FluentFile {
    type Error = LingoraError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let is_fluent_ext = |path: &Path| path.extension() == Some(OsStr::new("ftl"));
        let is_fluent_file = |path: &Path| path.is_file() && is_fluent_ext(path);

        if is_fluent_file(path) {
            let locale = Locale::try_from(path)?;
            let path = path.to_path_buf();
            Ok(Self { path, locale })
        } else {
            Err(LingoraError::InvalidFluentPath(path.to_path_buf()))
        }
    }
}

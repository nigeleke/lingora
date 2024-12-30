use super::error::Error;
use super::language::Language;

use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LanguageFile {
    Editable(Language, PathBuf),
    Locked(Error),
}

impl From<&PathBuf> for LanguageFile {
    fn from(value: &PathBuf) -> Self {
        let validated = Language::try_from(value)
            .map(|l| LanguageFile::Editable(l, value.to_owned()))
            .map_err(LanguageFile::Locked);
        match validated {
            Ok(vl) => vl,
            Err(vl) => vl,
        }
    }
}

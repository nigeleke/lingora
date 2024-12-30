use super::error::Error;

use oxilangtag::LanguageTag;
use serde::Deserialize;

use std::{convert::TryFrom, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Language(LanguageTag<String>);

impl Language {
    pub fn primary_language(&self) -> &str {
        self.0.primary_language()
    }
}

impl Default for Language {
    fn default() -> Self {
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        Language(LanguageTag::parse_and_normalize(locale.as_str()).unwrap())
    }
}

impl TryFrom<&str> for Language {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        LanguageTag::parse_and_normalize(value)
            .map(|l| Self(l))
            .map_err(|e| Error::InvalidLanguage(format!("{e}: {value}")))
    }
}

impl TryFrom<&PathBuf> for Language {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let stem = value.file_stem().ok_or(Error::InvalidLanguage(format!(
            "from filename: {}",
            value.display()
        )))?;
        Language::try_from(stem)
    }
}

impl TryFrom<&std::ffi::OsStr> for Language {
    type Error = Error;

    fn try_from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        let as_str = value
            .to_str()
            .ok_or(Error::InvalidLanguage(value.to_string_lossy().to_string()))?;
        Language::try_from(as_str)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default_language_will_be_system_locale() {
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        let expected_language =
            Language(LanguageTag::parse_and_normalize(locale.as_str()).unwrap());
        assert_eq!(Language::default(), expected_language);
    }
}

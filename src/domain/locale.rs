use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::*;
use unic_langid::LanguageIdentifier;

#[derive(Debug, Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LocaleError {
    #[error("invalid locale: {0}")]
    InvalidLocale(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Locale(LanguageIdentifier);

impl Default for Locale {
    fn default() -> Self {
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        Locale(LanguageIdentifier::from_bytes(locale.as_bytes()).unwrap())
    }
}

impl FromStr for Locale {
    type Err = LocaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LanguageIdentifier::from_bytes(s.as_bytes())
            .map(Self)
            .map_err(|e| LocaleError::InvalidLocale(format!("{e}: '{s}'")))
    }
}

impl TryFrom<&PathBuf> for Locale {
    type Error = LocaleError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let stem = value.file_stem().ok_or(LocaleError::InvalidLocale(format!(
            "from filename: {}",
            value.display()
        )))?;
        Locale::try_from(stem)
    }
}

impl TryFrom<&std::ffi::OsStr> for Locale {
    type Error = LocaleError;

    fn try_from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        let as_str = value.to_str().ok_or(LocaleError::InvalidLocale(
            value.to_string_lossy().to_string(),
        ))?;
        Locale::from_str(as_str)
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_locale_is_system_locale() {
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        let expected_locale =
            Locale(LanguageIdentifier::from_bytes(locale.as_str().as_bytes()).unwrap());
        assert_eq!(Locale::default(), expected_locale);
    }

    #[test]
    fn is_created_from_valid_str() {
        let locale = Locale::from_str("en-GB").unwrap();
        assert_eq!(
            locale,
            Locale(LanguageIdentifier::from_bytes("en-GB".as_bytes()).unwrap())
        );
    }

    #[test]
    fn is_not_created_from_invalid_str() {
        let error = Locale::from_str("this-is-not-valid").unwrap_err();
        assert!(matches!(error, LocaleError::InvalidLocale(_)));
    }
}

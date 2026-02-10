use std::{
    path::{Component, Path},
    str::FromStr,
};

use icu_locale_core::{
    LanguageIdentifier,
    subtags::{Language, Region, Script},
};
use serde::{Deserialize, Serialize};

use crate::{domain::LanguageRoot, error::LingoraError};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale(LanguageIdentifier);

pub trait HasLocale {
    fn locale(&self) -> &Locale;

    fn language_root(&self) -> LanguageRoot {
        LanguageRoot::from(self.locale())
    }
}

impl Locale {
    pub fn language(&self) -> &Language {
        &self.0.language
    }

    pub fn script(&self) -> Option<&Script> {
        self.0.script.as_ref()
    }

    pub fn region(&self) -> Option<&Region> {
        self.0.region.as_ref()
    }

    pub fn has_variants(&self) -> bool {
        !self.0.variants.is_empty()
    }
}

impl Default for Locale {
    fn default() -> Self {
        println!("{:?}", sys_locale::get_locale());
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        println!("'{locale}'");
        Locale(LanguageIdentifier::from_str(&locale).unwrap())
    }
}

impl FromStr for Locale {
    type Err = LingoraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Call for BCP47 validation...
        let _tags = icu_locale_core::Locale::try_from_str(s)
            .map_err(|e| LingoraError::InvalidLocale(format!("{e}: '{s}'")))?;

        let locale = s
            .parse::<LanguageIdentifier>()
            .map_err(|e| LingoraError::InvalidLocale(format!("{e}: '{s}'")))?;

        Ok(Locale(locale))
    }
}

impl TryFrom<&Path> for Locale {
    type Error = LingoraError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let locale_from_osstr = |c: &std::ffi::OsStr| Locale::from_str(&c.to_string_lossy()).ok();

        let locale_from_path_segment = |c: Component| match c {
            Component::Normal(name) => locale_from_osstr(name),
            _ => None,
        };

        let invalid_locale = || {
            LingoraError::InvalidLocale(format!(
                "No valid locale found in path: {}",
                value.display()
            ))
        };

        value
            .file_stem()
            .and_then(locale_from_osstr)
            .or_else(|| {
                value
                    .parent()?
                    .components()
                    .rev()
                    .filter_map(locale_from_path_segment)
                    .next()
            })
            .ok_or_else(invalid_locale)
    }
}

impl TryFrom<&std::ffi::OsStr> for Locale {
    type Error = LingoraError;

    fn try_from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        let as_str = value.to_str().ok_or(LingoraError::InvalidLocale(
            value.to_string_lossy().to_string(),
        ))?;
        Locale::from_str(as_str)
    }
}

impl Ord for Locale {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Locale {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        let expected_locale = Locale(LanguageIdentifier::from_str(&locale).unwrap());
        assert_eq!(Locale::default(), expected_locale);
    }

    #[test]
    fn is_created_from_valid_str() {
        let locale = Locale::from_str("en-GB").unwrap();
        assert_eq!(
            locale,
            Locale(LanguageIdentifier::from_str("en-GB").unwrap())
        );
    }

    #[test]
    fn is_not_created_from_invalid_str() {
        let error = Locale::from_str("this-is-not-valid").unwrap_err();
        assert!(matches!(error, LingoraError::InvalidLocale(_)));
    }
}

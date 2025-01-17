use super::{error::Error, prelude::PrimaryLanguage};

use serde::Deserialize;
use unic_langid::LanguageIdentifier;

use std::collections::HashSet;
use std::{convert::TryFrom, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Locale(LanguageIdentifier);

impl Locale {
    pub fn primary_language(&self) -> PrimaryLanguage {
        self.0.language.into()
    }

    pub fn fallbacks(&self) -> HashSet<Locale> {
        let mut fallbacks = HashSet::new();
        let langid = &self.0;
        let variants = langid.variants().cloned().collect::<Vec<_>>();
        let region = langid.region;
        let script = langid.script;
        let primary = langid.language;
        fallbacks.insert(LanguageIdentifier::from_parts(
            primary,
            script,
            region,
            variants.as_slice(),
        ));
        fallbacks.insert(LanguageIdentifier::from_parts(primary, script, region, &[]));
        fallbacks.insert(LanguageIdentifier::from_parts(primary, script, None, &[]));
        fallbacks.insert(LanguageIdentifier::from_parts(primary, None, None, &[]));
        fallbacks.remove(langid);

        fallbacks.into_iter().map(Locale::from).collect()
    }
}

impl Default for Locale {
    fn default() -> Self {
        let locale = sys_locale::get_locale().unwrap_or("en".into());
        Locale(LanguageIdentifier::from_bytes(locale.as_bytes()).unwrap())
    }
}

impl TryFrom<&str> for Locale {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        LanguageIdentifier::from_bytes(value.as_bytes())
            .map(Self)
            .map_err(|e| Error::InvalidLanguage(format!("{value}. Reason: {e}")))
    }
}

impl TryFrom<&PathBuf> for Locale {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let stem = value.file_stem().ok_or(Error::InvalidLanguage(format!(
            "from filename: {}",
            value.display()
        )))?;
        Locale::try_from(stem)
    }
}

impl TryFrom<&std::ffi::OsStr> for Locale {
    type Error = Error;

    fn try_from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        let as_str = value
            .to_str()
            .ok_or(Error::InvalidLanguage(value.to_string_lossy().to_string()))?;
        Locale::try_from(as_str)
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(value: LanguageIdentifier) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Locale {
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
            Locale(LanguageIdentifier::from_bytes(locale.as_str().as_bytes()).unwrap());
        assert_eq!(Locale::default(), expected_language);
    }
}

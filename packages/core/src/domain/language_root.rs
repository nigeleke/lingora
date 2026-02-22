use icu_locale_core::subtags::{Language, Script};

use crate::{domain::Locale, error::LingoraError};

/// The **primary language root** of a locale - used to group locales that share
/// the same base language (with or without explicit script).
///
/// Variants:
/// - `ImplicitScript(Language)`: No script was specified (most common: Latin script implied)
/// - `Scripted(Language, Script)`: An explicit script is present (e.g. `zh-Hans`, `sr-Cyrl`)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LanguageRoot {
    /// Language without an explicit script subtag (implies the default script,
    /// usually Latin for Western languages).
    ImplicitScript(Language),

    /// Language with an explicit script subtag (important for languages like
    /// Chinese, Serbian, Uzbek that have multiple scripts in active use).
    Scripted(Language, Script),
}

impl From<&Locale> for LanguageRoot {
    fn from(value: &Locale) -> Self {
        let language = *value.language();
        let script = value.script().cloned();

        match script {
            None => Self::ImplicitScript(language),
            Some(script) => Self::Scripted(language, script),
        }
    }
}

impl std::str::FromStr for LanguageRoot {
    type Err = LingoraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locale = Locale::from_str(s)?;
        Ok(Self::from(&locale))
    }
}

impl std::fmt::Display for LanguageRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImplicitScript(language) => language.fmt(f),
            Self::Scripted(language, script) => write!(f, "{}-{}", language, script),
        }
    }
}

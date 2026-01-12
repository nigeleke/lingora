use unic_langid::subtags::{Language, Script};

use crate::{domain::Locale, error::LingoraError};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LanguageRoot {
    ImplicitScript(Language),
    Scripted(Language, Script),
}

impl From<&Locale> for LanguageRoot {
    fn from(value: &Locale) -> Self {
        let language = value.language().clone();
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

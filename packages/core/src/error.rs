use thiserror::*;

#[derive(Debug, Error)]
pub enum LingoraError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),

    #[error(transparent)]
    Args(#[from] clap::Error),

    #[error(transparent)]
    Fluent(#[from] fluent4rs::prelude::Fluent4rsError),

    #[error(transparent)]
    Syn(#[from] syn::Error),

    #[error("invalid locale: {0}")]
    InvalidLocale(String),

    #[error("invalid fluent file path: {0}")]
    InvalidFluentPath(std::path::PathBuf),

    #[error("invalid rust file path: {0}")]
    InvalidRustPath(std::path::PathBuf),

    #[error(
        "multiple locales resolve to the same language root(s) making graceful fallback impossible for {0}"
    )]
    AmbiguousLanguageRoots(String),

    #[error("no translation file(s) for required locales: {0}")]
    MissingTranslationFiles(String),

    #[error("no primary locales found for provided locales: {0}")]
    MissingPrimaryLocales(String),

    #[error("malformed identifier literal {0}")]
    MalformedIdentifierLiteral(String),
}

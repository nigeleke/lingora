use thiserror::*;

#[derive(Debug, Error)]
pub enum LingoraError {
    #[error("cannot find reference file: {0}")]
    CannotFindReferenceFile(String),

    #[error("ambiguous reference files:\n  {0}")]
    AmbiguousReferenceFiles(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),

    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),

    #[error(transparent)]
    Fluent(#[from] fluent4rs::prelude::Fluent4rsError),

    #[error("invalid locale: {0}")]
    InvalidLocale(String),
}

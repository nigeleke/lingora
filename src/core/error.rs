use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Error)]
pub enum Error {
    #[error("invalid language: {0}")]
    InvalidLanguage(String),

    #[error("invalid config file: {0}")]
    InvalidConfigFile(String),

    #[error("translation file error: {0}")]
    FluentFileTraversalFailed(String),

    #[error("duplicate language file for {0}")]
    DuplicateLanguageFile(String),
}

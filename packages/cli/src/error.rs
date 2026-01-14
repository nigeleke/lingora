use lingora_core::prelude::LingoraError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Lingora(#[from] LingoraError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("File {0} has no parent")]
    NoParent(String),

    #[error("Integrity errors detected")]
    IntegrityErrorsDetected,
}

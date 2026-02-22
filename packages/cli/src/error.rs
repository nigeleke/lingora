use lingora_core::prelude::LingoraError;
use thiserror::Error;

/// Main error type for the `lingora-cli` binary.
#[derive(Debug, Error)]
pub enum CliError {
    /// Any error that originated from `lingora-core`.
    #[error(transparent)]
    Lingora(#[from] LingoraError),

    /// Low-level I/O error (e.g. cannot read `Lingora.toml`, cannot write output,
    /// permission denied, disk full).
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A path provided or discovered (e.g. config file, output destination)
    #[error("File {0} has no parent")]
    NoParent(String),

    /// The audit completed successfully (in the sense there werw no fatal errors),
    /// but **issues were found** (missing translations, redundant keys, macro mismatches,
    /// etc.).
    ///
    /// This variant is used to signal a non-zero exit code in CI/pre-commit hooks
    /// while still allowing structured output to be printed.
    ///
    /// It is **not** an error in the traditional sense, but an outcome
    /// indicating "localization is not perfect".
    #[error("Integrity errors detected")]
    IntegrityErrorsDetected,
}

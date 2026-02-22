use thiserror::Error;

/// Top-level error type used throughout the `lingora-tui` binary.
#[derive(Debug, Error)]
pub enum TuiError {
    /// Any error originating from the core `lingora-common` library.
    #[error(transparent)]
    Lingora(#[from] lingora_core::prelude::LingoraError),

    /// Low-level error from the `fluent4rs` parser while reading or parsing `.ftl` files.
    #[error(transparent)]
    Fluent4rs(#[from] fluent4rs::prelude::Fluent4rsError),

    /// I/O error during terminal operations, file reading, or configuration loading.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

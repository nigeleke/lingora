use thiserror::Error;

#[derive(Debug, Error)]
pub enum TuiError {
    #[error(transparent)]
    Lingora(#[from] lingora_common::LingoraError),

    #[error(transparent)]
    Fluent4rs(#[from] fluent4rs::prelude::Fluent4rsError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

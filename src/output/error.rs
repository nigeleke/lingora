use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("write failed: {0}")]
    WriteFailed(String),
}

pub(super) type Result<T> = std::result::Result<T, WriterError>;

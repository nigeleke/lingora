use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("internal problem: {0}; raise issue")]
    InternalIssue(String),

    #[error("write failed: {0}")]
    WriteFailed(String),
}

pub(super) type Result<T> = std::result::Result<T, WriterError>;

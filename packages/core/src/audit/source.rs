use std::sync::Arc;

use crate::{
    error::LingoraError,
    fluent::{FluentDocument, FluentFile},
    rust::RustFile,
};

#[derive(Clone, Debug)]
pub enum Source {
    FluentFile(FluentFile),
    FluentDocument(Result<FluentDocument, Arc<LingoraError>>),
    Rust(RustFile),
}

impl From<FluentDocument> for Source {
    fn from(value: FluentDocument) -> Self {
        Self::FluentDocument(value)
    }
}

impl From<FluentFile> for Source {
    fn from(value: FluentFile) -> Self {
        Self::FluentFile(value)
    }
}

impl From<RustFile> for Source {
    fn from(value: RustFile) -> Self {
        Self::Rust(value)
    }
}

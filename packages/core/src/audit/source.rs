use crate::{fluent::QualfiedFluentFile, rust::RustFile};

#[derive(Clone, Debug)]
pub enum Source {
    Fluent(QualfiedFluentFile),
    Rust(RustFile),
}

impl From<QualfiedFluentFile> for Source {
    fn from(value: QualfiedFluentFile) -> Self {
        Self::Fluent(value)
    }
}

impl From<RustFile> for Source {
    fn from(value: RustFile) -> Self {
        Self::Rust(value)
    }
}

use crate::{fluent::QualifiedFluentFile, rust::RustFile};

#[derive(Clone, Debug)]
pub enum Source {
    Fluent(QualifiedFluentFile),
    Rust(RustFile),
}

impl From<QualifiedFluentFile> for Source {
    fn from(value: QualifiedFluentFile) -> Self {
        Self::Fluent(value)
    }
}

impl From<RustFile> for Source {
    fn from(value: RustFile) -> Self {
        Self::Rust(value)
    }
}

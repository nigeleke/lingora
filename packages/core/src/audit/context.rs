use crate::{fluent::QualfiedFluentFile, rust::RustFile};

#[derive(Debug)]
enum Source<'a> {
    Fluent(&'a QualfiedFluentFile),
    Rust(&'a RustFile),
}

#[derive(Debug)]
pub enum ContextKind {
    FluentIntegrity,
    CanonicalToPrimary,
    PrimaryToVariant,
    CanonicalToRustSource,
}

#[derive(Debug)]
pub enum ContextTarget<'a> {
    Single { source: Source<'a> },
    Pair { left: Source<'a>, right: Source<'a> },
}

#[derive(Debug)]
pub struct Context<'a> {
    pub(super) kind: ContextKind,
    pub(super) target: ContextTarget<'a>,
}

impl<'a> Context<'a> {
    pub fn fluent_file(source: &'a QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::FluentIntegrity,
            target: ContextTarget::Single {
                source: Source::Fluent(source),
            },
        }
    }

    pub fn canonical_to_primary(
        canonical: &'a QualfiedFluentFile,
        primary: &'a QualfiedFluentFile,
    ) -> Self {
        Self {
            kind: ContextKind::CanonicalToPrimary,
            target: ContextTarget::Pair {
                left: Source::Fluent(canonical),
                right: Source::Fluent(primary),
            },
        }
    }

    pub fn primary_to_variant(
        primary: &'a QualfiedFluentFile,
        variant: &'a QualfiedFluentFile,
    ) -> Self {
        Self {
            kind: ContextKind::PrimaryToVariant,
            target: ContextTarget::Pair {
                left: Source::Fluent(primary),
                right: Source::Fluent(variant),
            },
        }
    }

    pub fn canonical_to_rust(canonical: &'a QualfiedFluentFile, rust: &'a RustFile) -> Self {
        Self {
            kind: ContextKind::CanonicalToRustSource,
            target: ContextTarget::Pair {
                left: Source::Fluent(canonical),
                right: Source::Rust(rust),
            },
        }
    }

    pub fn fluent_single(&self) -> Option<&'a QualfiedFluentFile> {
        match &self.target {
            ContextTarget::Single {
                source: Source::Fluent(file),
            } => Some(*file),
            _ => None,
        }
    }

    pub fn fluent_pair(&self) -> Option<(&'a QualfiedFluentFile, &'a QualfiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Fluent(l),
                right: Source::Fluent(r),
            } => Some((*l, *r)),
            _ => None,
        }
    }

    pub fn fluent_rust_pair(&self) -> Option<(&'a QualfiedFluentFile, &'a RustFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Fluent(f),
                right: Source::Rust(r),
            } => Some((*f, *r)),
            _ => None,
        }
    }
}

use crate::{fluent::QualfiedFluentFile, rust::RustFile};

#[derive(Debug)]
enum Source<'a> {
    Fluent(&'a QualfiedFluentFile),
    Rust(&'a RustFile),
}

#[derive(Debug)]
pub enum ContextKind {
    All,
    Base,
    CanonicalToPrimary,
    BaseToVariant,
    RustToCanonical,
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
    pub fn all(source: &'a QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::All,
            target: ContextTarget::Single {
                source: Source::Fluent(source),
            },
        }
    }

    pub fn base(source: &'a QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::Base,
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

    pub fn base_to_variant(base: &'a QualfiedFluentFile, variant: &'a QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::BaseToVariant,
            target: ContextTarget::Pair {
                left: Source::Fluent(base),
                right: Source::Fluent(variant),
            },
        }
    }

    pub fn rust_to_canonical(rust: &'a RustFile, canonical: &'a QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::RustToCanonical,
            target: ContextTarget::Pair {
                left: Source::Rust(rust),
                right: Source::Fluent(canonical),
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

    pub fn rust_fluent_pair(&self) -> Option<(&'a RustFile, &'a QualfiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Rust(r),
                right: Source::Fluent(f),
            } => Some((*r, *f)),
            _ => None,
        }
    }
}

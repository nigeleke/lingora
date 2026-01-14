use crate::{
    audit::Source,
    domain::{LanguageRoot, Locale},
    fluent::QualfiedFluentFile,
    rust::RustFile,
};

#[derive(Clone, Debug)]
pub enum ContextKind {
    All,
    Base,
    CanonicalToPrimary,
    BaseToVariant,
    RustToCanonical,
}

#[derive(Clone, Debug)]
pub enum ContextTarget {
    Single { source: Source },
    Pair { left: Source, right: Source },
}

#[derive(Clone, Debug)]
pub struct Context {
    kind: ContextKind,
    target: ContextTarget,
}

// Constructors...
impl Context {
    pub fn all(source: QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::All,
            target: ContextTarget::Single {
                source: Source::Fluent(source),
            },
        }
    }

    pub fn base(source: QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::Base,
            target: ContextTarget::Single {
                source: Source::Fluent(source),
            },
        }
    }

    pub fn canonical_to_primary(
        canonical: QualfiedFluentFile,
        primary: QualfiedFluentFile,
    ) -> Self {
        Self {
            kind: ContextKind::CanonicalToPrimary,
            target: ContextTarget::Pair {
                left: Source::Fluent(canonical),
                right: Source::Fluent(primary),
            },
        }
    }

    pub fn base_to_variant(base: QualfiedFluentFile, variant: QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::BaseToVariant,
            target: ContextTarget::Pair {
                left: Source::Fluent(base),
                right: Source::Fluent(variant),
            },
        }
    }

    pub fn rust_to_canonical(rust: RustFile, canonical: QualfiedFluentFile) -> Self {
        Self {
            kind: ContextKind::RustToCanonical,
            target: ContextTarget::Pair {
                left: Source::Rust(rust),
                right: Source::Fluent(canonical),
            },
        }
    }
}

// Accessors
impl Context {
    #[inline(always)]
    pub fn kind(&self) -> &ContextKind {
        &self.kind
    }

    pub fn fluent_single(&self) -> Option<QualfiedFluentFile> {
        match &self.target {
            ContextTarget::Single {
                source: Source::Fluent(file),
            } => Some(file.clone()),
            _ => None,
        }
    }

    pub fn fluent_pair(&self) -> Option<(QualfiedFluentFile, QualfiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Fluent(l),
                right: Source::Fluent(r),
            } => Some((l.clone(), r.clone())),
            _ => None,
        }
    }

    pub fn rust_fluent_pair(&self) -> Option<(RustFile, QualfiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Rust(r),
                right: Source::Fluent(f),
            } => Some((r.clone(), f.clone())),
            _ => None,
        }
    }

    pub fn locale(&self) -> &Locale {
        match &self.target {
            ContextTarget::Single {
                source: Source::Fluent(file),
            } => file.locale(),
            ContextTarget::Pair {
                left: Source::Fluent(_),
                right: Source::Fluent(file),
            } => file.locale(),
            ContextTarget::Pair {
                left: Source::Rust(_),
                right: Source::Fluent(file),
            } => file.locale(),
            ContextTarget::Single {
                source: Source::Rust(_),
            }
            | ContextTarget::Pair {
                right: Source::Rust(_),
                ..
            } => unreachable!(),
        }
    }
}

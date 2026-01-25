use crate::{
    audit::{Source, Workspace},
    domain::Locale,
    error::LingoraError,
    fluent::{FluentDocument, FluentFile},
    rust::RustFile,
};

#[derive(Clone, Debug)]
pub enum ContextKind {
    Workspace,
    FluentFile,
    AllLocales,
    BaseLocales,
    CanonicalToPrimary,
    BaseToVariant,
    RustFileToCanonical,
}

#[derive(Clone, Debug)]
pub enum ContextTarget {
    Workspace { workspace: Workspace },
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
    pub fn new_workspace_context(workspace: Workspace) -> Self {
        Self {
            kind: ContextKind::Workspace,
            target: ContextTarget::Workspace { workspace },
        }
    }

    pub fn new_fluent_file_context(source: FluentFile) -> Self {
        Self {
            kind: ContextKind::FluentFile,
            target: ContextTarget::Single {
                source: Source::FluentFile(source),
            },
        }
    }

    pub fn new_locale_context(document: Result<FluentDocument, LingoraError>) -> Self {
        Self {
            kind: ContextKind::FluentDocument,
            target: ContextTarget::Single {
                source: Source::FluentDocument(source),
            },
        }
    }

    pub fn new_base_context(source: QualifiedFluentFile) -> Self {
        Self {
            kind: ContextKind::Base,
            target: ContextTarget::Single {
                source: Source::Fluent(source),
            },
        }
    }

    pub fn new_canonical_to_primary_context(
        canonical: QualifiedFluentFile,
        primary: QualifiedFluentFile,
    ) -> Self {
        Self {
            kind: ContextKind::CanonicalToPrimary,
            target: ContextTarget::Pair {
                left: Source::Fluent(canonical),
                right: Source::Fluent(primary),
            },
        }
    }

    pub fn new_base_to_variant_context(
        base: QualifiedFluentFile,
        variant: QualifiedFluentFile,
    ) -> Self {
        Self {
            kind: ContextKind::BaseToVariant,
            target: ContextTarget::Pair {
                left: Source::Fluent(base),
                right: Source::Fluent(variant),
            },
        }
    }

    pub fn new_rust_to_canonical_context(rust: RustFile, canonical: QualifiedFluentFile) -> Self {
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

    pub fn workspace(&self) -> Option<&Workspace> {
        match &self.target {
            ContextTarget::Workspace { workspace } => Some(workspace),
            _ => None,
        }
    }

    pub fn fluent_single(&self) -> Option<&QualifiedFluentFile> {
        match &self.target {
            ContextTarget::Single {
                source: Source::Fluent(file),
            } => Some(file),
            _ => None,
        }
    }

    pub fn fluent_pair(&self) -> Option<(&QualifiedFluentFile, &QualifiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Fluent(l),
                right: Source::Fluent(r),
            } => Some((l, r)),
            _ => None,
        }
    }

    pub fn rust_fluent_pair(&self) -> Option<(&RustFile, &QualifiedFluentFile)> {
        match &self.target {
            ContextTarget::Pair {
                left: Source::Rust(r),
                right: Source::Fluent(f),
            } => Some((r, f)),
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
            ContextTarget::Workspace { .. }
            | ContextTarget::Single { .. }
            | ContextTarget::Pair { .. } => unreachable!(),
        }
    }
}

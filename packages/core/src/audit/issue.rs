use crate::{audit::Context, domain::Locale, fluent::QualifiedIdentifier};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditKind {
    Workspace(String),
    InvalidSyntax(String),
    DuplicateDefinition,
    InvalidReference,
    MissingTranslation,
    RedundantTranslation,
    SignatureMismatch,
}

#[derive(Clone, Debug)]
pub struct AuditIssue {
    context: Context,
    identifier: Option<QualifiedIdentifier>,
    kind: AuditKind,
}

// Constructors...
impl AuditIssue {
    pub fn workspace(context: &Context, s: &str) -> Self {
        Self {
            context: context.clone(),
            identifier: None,
            kind: AuditKind::Workspace(String::from(s)),
        }
    }

    pub fn invalid_syntax(context: &Context, error: &str) -> Self {
        Self {
            context: context.clone(),
            identifier: None,
            kind: AuditKind::InvalidSyntax(String::from(error)),
        }
    }

    pub fn duplicate_definition(context: &Context, identifier: QualifiedIdentifier) -> Self {
        Self {
            context: context.clone(),
            identifier: Some(identifier.clone()),
            kind: AuditKind::DuplicateDefinition,
        }
    }

    pub fn invalid_reference(context: &Context, identifier: &QualifiedIdentifier) -> Self {
        Self {
            context: context.clone(),
            identifier: Some(identifier.clone()),
            kind: AuditKind::InvalidReference,
        }
    }

    pub fn missing_translation(context: &Context, identifier: &QualifiedIdentifier) -> Self {
        Self {
            context: context.clone(),
            identifier: Some(identifier.clone()),
            kind: AuditKind::MissingTranslation,
        }
    }

    pub fn redundant_translation(context: &Context, identifier: &QualifiedIdentifier) -> Self {
        Self {
            context: context.clone(),
            identifier: Some(identifier.clone()),
            kind: AuditKind::RedundantTranslation,
        }
    }

    pub fn signature_mismatch(context: &Context, identifier: &QualifiedIdentifier) -> Self {
        Self {
            context: context.clone(),
            identifier: Some(identifier.clone()),
            kind: AuditKind::SignatureMismatch,
        }
    }
}

// Accessors...
impl AuditIssue {
    // pub fn context(&self) -> &Context {
    //     &self.context
    // }

    pub fn kind(&self) -> &AuditKind {
        &self.kind
    }

    pub fn identifier(&self) -> Option<&QualifiedIdentifier> {
        self.identifier.as_ref()
    }

    pub fn locale(&self) -> &Locale {
        self.context.locale()
    }
}

use std::path::PathBuf;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::{ParsedFluentFile, QualifiedIdentifier},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    ParseError,
    MissingBase,
    UndefinedBase,
    DuplicateIdentifier,
    InvalidReference,
    MissingTranslation,
    RedundantTranslation,
    SignatureMismatch,
}

#[cfg(test)]
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::ParseError => "parse_error",
            Kind::MissingBase => "missing_base",
            Kind::UndefinedBase => "undefined_base",
            Kind::DuplicateIdentifier => "duplicate_identifier",
            Kind::InvalidReference => "invalid_reference",
            Kind::MissingTranslation => "missing_translation",
            Kind::RedundantTranslation => "redundant_translation",
            Kind::SignatureMismatch => "signature_mismatch",
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Subject {
    Workspace,
    File(PathBuf),
    Locale(Locale),
    Document(Locale),
    Entry(Locale, QualifiedIdentifier),
    LanguageRoot(LanguageRoot),
}

#[cfg(test)]
impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Workspace => String::from("workspace"),
            Subject::File(path) => format!("file({})", path.display()),
            Subject::Locale(locale) => format!("locale({locale})"),
            Subject::Document(locale) => format!("document({locale})"),
            Subject::Entry(locale, id) => format!("entry({locale}, {})", id.to_meta_string()),
            Subject::LanguageRoot(root) => format!("language_root({root})"),
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct AuditIssue {
    kind: Kind,
    subject: Subject,
    message: String,
}

// Constructors...
impl AuditIssue {
    fn new(kind: Kind, subject: Subject, message: String) -> Self {
        Self {
            kind,
            subject,
            message,
        }
    }

    pub fn parse_error(file: &ParsedFluentFile) -> Self {
        Self::new(
            Kind::ParseError,
            Subject::File(file.path().to_path_buf()),
            file.error_description(),
        )
    }

    pub fn missing_base_translation(locale: &Locale) -> Self {
        Self::new(
            Kind::MissingBase,
            Subject::Locale(locale.clone()),
            format!("the locale {locale} is required but no files have been found"),
        )
    }

    pub fn undefined_base_locale(root: &LanguageRoot, locales: &[Locale]) -> Self {
        let locales = locales
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Self::new(
            Kind::UndefinedBase,
            Subject::LanguageRoot(root.clone()),
            format!("no base locale is explicitly defined for '{locales}'"),
        )
    }

    pub fn duplicate_identifier(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::DuplicateIdentifier,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("multiple definitions for {}", identifier.to_meta_string()),
        )
    }

    pub fn invalid_reference(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::InvalidReference,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("invalid reference {}", identifier.to_meta_string()),
        )
    }

    pub fn missing_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::MissingTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("missing translation {}", identifier.to_meta_string()),
        )
    }

    pub fn redundant_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::RedundantTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("redundant translation {}", identifier.to_meta_string()),
        )
    }

    pub fn signature_mismatch(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::SignatureMismatch,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("signature mismatch {}", identifier.to_meta_string()),
        )
    }
}

// Accessors...
impl AuditIssue {
    #[cfg(test)]
    pub(crate) fn kind(&self) -> &Kind {
        &self.kind
    }

    #[cfg(test)]
    pub(crate) fn subject(&self) -> &Subject {
        &self.subject
    }
}

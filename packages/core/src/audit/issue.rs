use std::path::PathBuf;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::{ParsedFluentFile, QualifiedIdentifier},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Subject {
    File(PathBuf),
    Locale(Locale),
    Entry(Locale, QualifiedIdentifier),
    LanguageRoot(LanguageRoot),
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
            format!("no files found for required locale {locale}"),
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
            format!("missing base locale/s for '{locales}'"),
        )
    }

    pub fn duplicate_identifier(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::DuplicateIdentifier,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("multiple definitions for '{}'", identifier.to_meta_string()),
        )
    }

    pub fn invalid_reference(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::InvalidReference,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("invalid reference '{}'", identifier.to_meta_string()),
        )
    }

    pub fn missing_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::MissingTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("missing translation '{}'", identifier.to_meta_string()),
        )
    }

    pub fn redundant_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::RedundantTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("redundant translation '{}'", identifier.to_meta_string()),
        )
    }

    pub fn signature_mismatch(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::SignatureMismatch,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("signature mismatch '{}'", identifier.to_meta_string()),
        )
    }
}

// Accessors...
impl AuditIssue {
    pub fn locale(&self) -> Option<Locale> {
        match &self.subject {
            Subject::File(path) => Locale::try_from(path.as_path()).ok(),
            Subject::Locale(locale) => Some(locale.clone()),
            Subject::Entry(locale, _identifier) => Some(locale.clone()),
            Subject::LanguageRoot(_) => None,
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.kind
    }

    #[cfg(test)]
    pub(crate) fn subject(&self) -> &Subject {
        &self.subject
    }
}

impl std::fmt::Display for AuditIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

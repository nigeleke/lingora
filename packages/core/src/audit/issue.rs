use std::path::PathBuf;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::{ParsedFluentFile, QualifiedIdentifier},
    rust::ParsedRustFile,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    ParseError,
    MissingBase,
    UndefinedBase,
    DuplicateIdentifier,
    InvalidReference,
    MissingTranslation,
    RedundantTranslation,
    SignatureMismatch,
    MalformedIdentifierLiteral,
    UndefinedIdentifierLiteral,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subject {
    FluentFile(PathBuf),
    RustFile(PathBuf),
    Locale(Locale),
    Entry(Locale, QualifiedIdentifier),
    LanguageRoot(LanguageRoot),
}

impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::FluentFile(path_buf) => path_buf.display().fmt(f),
            Subject::RustFile(path_buf) => path_buf.display().fmt(f),
            Subject::Locale(locale) => locale.fmt(f),
            Subject::Entry(locale, qualified_identifier) => {
                write!(f, "{locale} :: {}", qualified_identifier.to_meta_string())
            }
            Subject::LanguageRoot(language_root) => language_root.fmt(f),
        }
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

    pub fn parse_fluent_file_error(file: &ParsedFluentFile) -> Self {
        Self::new(
            Kind::ParseError,
            Subject::FluentFile(file.path().to_path_buf()),
            file.error_description(),
        )
    }

    pub fn parse_rust_file_error(file: &ParsedRustFile) -> Self {
        Self::new(
            Kind::ParseError,
            Subject::FluentFile(file.path().to_path_buf()),
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

    pub fn undefined_identifier_literal(
        path: &ParsedRustFile,
        identifier: &QualifiedIdentifier,
    ) -> Self {
        Self::new(
            Kind::UndefinedIdentifierLiteral,
            Subject::RustFile(path.path().to_path_buf()),
            format!(
                "identifier literal {} is not defined in the canonical document",
                identifier.to_meta_string()
            ),
        )
    }

    pub fn malformed_identifier_literal(path: &ParsedRustFile, error: &str) -> Self {
        Self::new(
            Kind::MalformedIdentifierLiteral,
            Subject::RustFile(path.path().to_path_buf()),
            format!("malformed identifier literal: {error}"),
        )
    }
}

// Accessors...
impl AuditIssue {
    pub fn locale(&self) -> Option<Locale> {
        match &self.subject {
            Subject::FluentFile(path) => Locale::try_from(path.as_path()).ok(),
            Subject::RustFile(_) => None,
            Subject::Locale(locale) => Some(locale.clone()),
            Subject::Entry(locale, _identifier) => Some(locale.clone()),
            Subject::LanguageRoot(_) => None,
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

impl std::fmt::Display for AuditIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

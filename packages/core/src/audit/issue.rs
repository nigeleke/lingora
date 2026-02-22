use std::path::PathBuf;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::{ParsedFluentFile, QualifiedIdentifier},
    rust::ParsedRustFile,
};

/// Classification of the kind of localization / translation problems found during audit.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// Parsing of a `.ftl` or `.rs` file failed (syntax error, invalid structure, etc)
    ParseError,

    /// No translation file(s) were found for a locale that is explicitly required
    /// (e.g. listed as primary/canonical in config or CLI)
    MissingBase,

    /// A locale appears in variant lists but no corresponding primary/base locale
    /// document exists for its language root.
    UndefinedBase,

    /// The same message/term identifier is defined more than once in the same document.
    DuplicateIdentifier,

    /// A reference, e.g. `{ $term }`, points to a non-existent identifier.
    InvalidReference,

    /// A key present in the canonical document is missing from a primary.
    MissingTranslation,

    /// A key is present in a primary (or variant) is not required as it is not
    /// defined in the canonical (or primary).
    RedundantTranslation,

    /// The placeholder/argument signature differs between canonical and primary or
    /// primary and variant, e.g. different number or names of variables.
    SignatureMismatch,

    /// A string literal used in a `t!`, `te!`, or `tid!` macro does not conform to
    /// valid Fluent identifier syntax.
    MalformedIdentifierLiteral,

    /// A string literal used in a `dioxus_i18n` macro refers to an identifier that
    /// does **not** exist in the canonical Fluent document.
    UndefinedIdentifierLiteral,
}

/// The entity affected by or associated with an `AuditIssue`.
///
/// Used to group, filter, and display issues meaningfully (e.g. by file, by locale,
/// by message key).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subject {
    /// A Fluent translation file (`.ftl`) that failed parsing or contains issues.
    FluentFile(PathBuf),

    /// A Rust source file (`.rs`) that failed parsing or contains invalid macro usage.
    RustFile(PathBuf),

    /// A locale that is missing required files or has no base document.
    Locale(Locale),

    /// A specific message/term/attribute entry in a given locale.
    Entry(Locale, QualifiedIdentifier),

    /// A language root has configuration or fallback problems.
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

/// A single localization problem discovered during Fluent/Rust source analysis.
///
/// Each issue has:
/// - a `Kind` (what kind of problem)
/// - a `Subject` (what entity is affected)
/// - a human-readable `message` (for display in CLI/TUI/reports)
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

    /// Fluent file failed to parse (syntax error, invalid AST, etc.).
    pub fn parse_fluent_file_error(file: &ParsedFluentFile) -> Self {
        Self::new(
            Kind::ParseError,
            Subject::FluentFile(file.path().to_path_buf()),
            file.error_description(),
        )
    }

    /// Rust file failed to parse (used when scanning for `dioxus_i18n` macros).
    pub fn parse_rust_file_error(file: &ParsedRustFile) -> Self {
        Self::new(
            Kind::ParseError,
            Subject::FluentFile(file.path().to_path_buf()),
            file.error_description(),
        )
    }

    /// Required locale has no translation files at all.
    pub fn missing_base_translation(locale: &Locale) -> Self {
        Self::new(
            Kind::MissingBase,
            Subject::Locale(locale.clone()),
            format!("no files found for required locale {locale}"),
        )
    }

    /// Variants exist for a language root, but no primary/base file was found.
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

    /// Same identifier defined multiple times in one document.
    pub fn duplicate_identifier(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::DuplicateIdentifier,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("multiple definitions for '{}'", identifier.to_meta_string()),
        )
    }

    /// Reference to non-existent message/term/attribute.
    pub fn invalid_reference(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::InvalidReference,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("invalid reference '{}'", identifier.to_meta_string()),
        )
    }

    /// Key exists in canonical but is missing here.
    pub fn missing_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::MissingTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("missing translation '{}'", identifier.to_meta_string()),
        )
    }

    /// Key exists here but not in canonical / primary (depending on context).
    pub fn redundant_translation(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::RedundantTranslation,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("redundant translation '{}'", identifier.to_meta_string()),
        )
    }

    /// Number or names of placeholders differ from canonical.
    pub fn signature_mismatch(locale: &Locale, identifier: &QualifiedIdentifier) -> Self {
        Self::new(
            Kind::SignatureMismatch,
            Subject::Entry(locale.clone(), identifier.clone()),
            format!("signature mismatch '{}'", identifier.to_meta_string()),
        )
    }

    /// String literal in `t!`/`te!`/`tid!` refers to non-existent key in canonical.
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

    /// String literal in Rust macro is not a valid Fluent identifier.
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
    /// Extract the locale most relevant to this issue (if any).
    pub fn locale(&self) -> Option<Locale> {
        match &self.subject {
            Subject::FluentFile(path) => Locale::try_from(path.as_path()).ok(),
            Subject::RustFile(_) => None,
            Subject::Locale(locale) => Some(locale.clone()),
            Subject::Entry(locale, _identifier) => Some(locale.clone()),
            Subject::LanguageRoot(_) => None,
        }
    }

    /// Human-readable description of the problem.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// The entity this issue pertains to (file, locale, specific entry, etc.).
    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    /// The kind/category of this issue.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

impl std::fmt::Display for AuditIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

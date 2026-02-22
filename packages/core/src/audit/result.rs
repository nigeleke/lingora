use std::collections::HashMap;

use fluent4rs::ast::Entry;

use crate::{
    audit::{AuditIssue, Workspace},
    domain::{HasLocale, Locale},
    fluent::{FluentDocument, QualifiedIdentifier},
};

/// The classification of a Fluent document's role within the workspace during analysis.
#[derive(Clone, Copy, Debug)]
pub enum DocumentRole {
    /// The reference document against which all others are compared.
    Canonical,

    /// A primary (non-variant) translation document for a language root.
    Primary,

    /// A regional or script variant that depends on a primary for fallback.
    Variant,

    /// A document whose locale was not expected or configured in the workspace.
    Orphan,
}

/// A Fluent document that has been parsed, assigned a role, and is ready for analysis.
#[derive(Clone, Debug)]
pub struct AuditedDocument {
    document: FluentDocument,
    role: DocumentRole,
}

impl AuditedDocument {
    /// Constructs an `AuditedDocument` from a parsed `FluentDocument` and its assigned role.
    pub fn from_fluent_document(role: DocumentRole, document: &FluentDocument) -> Self {
        let document = document.clone();
        Self { document, role }
    }

    /// Returns the role assigned to this document in the audit.
    pub fn role(&self) -> DocumentRole {
        self.role
    }

    /// Returns an iterator over all **qualified identifiers** defined in this document.
    pub fn identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.document.entry_identifiers()
    }

    /// Returns an iterator over all AST `Entry` nodes matching the given identifier.
    pub fn entries(&self, identifier: &QualifiedIdentifier) -> impl Iterator<Item = &Entry> {
        self.document.entries(identifier)
    }
}

impl HasLocale for AuditedDocument {
    fn locale(&self) -> &Locale {
        self.document.locale()
    }
}

/// The complete result of running an audit over a workspace.
///
/// Contains:
/// - All discovered localization issues
/// - The set of parsed and classified documents, indexed by locale
/// - A reference to the original workspace configuration
///
/// This type is what `AuditEngine` returns and what CLI/TUI renderers consume.
#[derive(Debug)]
pub struct AuditResult {
    issues: Vec<AuditIssue>,
    documents: HashMap<Locale, AuditedDocument>,
    workspace: Workspace,
}

impl AuditResult {
    pub(crate) fn new(
        issues: Vec<AuditIssue>,
        nodes: Vec<AuditedDocument>,
        workspace: &Workspace,
    ) -> Self {
        let documents = nodes
            .iter()
            .cloned()
            .map(|node| (node.locale().clone(), node))
            .collect::<HashMap<_, _>>();
        let workspace = workspace.clone();

        Self {
            issues,
            documents,
            workspace,
        }
    }

    /// Returns `true` if no issues were found (clean audit).
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns the workspace configuration used for this audit.
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Returns an iterator over all discovered issues.
    pub fn issues(&self) -> impl Iterator<Item = &AuditIssue> {
        self.issues.iter()
    }

    /// Returns the canonical (reference) locale used as the baseline.
    pub fn canonical_locale(&self) -> &Locale {
        self.workspace.canonical_locale()
    }

    /// Returns an iterator over all locales for which a document was found.
    pub fn document_locales(&self) -> impl Iterator<Item = &Locale> {
        self.documents.keys()
    }

    /// Returns the `AuditedDocument` for the given locale, if one was parsed.
    pub fn document(&self, locale: &Locale) -> Option<&AuditedDocument> {
        self.documents.get(locale)
    }
}

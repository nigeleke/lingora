use std::collections::HashMap;

use crate::{
    audit::{AuditIssue, Workspace},
    domain::{HasLocale, Locale},
    fluent::FluentDocument,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum DocumentRole {
    Canonical,
    Primary,
    Variant,
    Orphan,
}

#[derive(Clone, Debug)]
pub struct AuditedDocument {
    document: FluentDocument,
    role: DocumentRole,
}

impl AuditedDocument {
    pub fn from_document(role: DocumentRole, document: &FluentDocument) -> Self {
        let document = document.clone();
        Self { document, role }
    }
}

impl HasLocale for AuditedDocument {
    fn locale(&self) -> &Locale {
        self.document.locale()
    }
}

#[derive(Debug)]
pub struct AuditResult {
    issues: Vec<AuditIssue>,
    documents: HashMap<Locale, AuditedDocument>,
    workspace: Workspace,
}

impl AuditResult {
    pub(crate) fn new(
        issues: &[AuditIssue],
        nodes: &[AuditedDocument],
        workspace: &Workspace,
    ) -> Self {
        let issues = Vec::from(issues);
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

    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn issues(&self) -> impl Iterator<Item = &AuditIssue> {
        self.issues.iter()
    }

    pub fn document_locales(&self) -> impl Iterator<Item = &Locale> {
        self.documents.keys()
    }
}

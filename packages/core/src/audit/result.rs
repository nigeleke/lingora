use std::collections::HashMap;

use crate::{
    audit::{AuditIssue, Workspace},
    domain::{HasLocale, LanguageRoot, Locale},
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
pub(crate) struct DocumentNode {
    document: FluentDocument,
    role: DocumentRole,
    root: LanguageRoot,
}

impl DocumentNode {
    pub fn from_document(role: DocumentRole, document: &FluentDocument) -> Self {
        let document = document.clone();
        let root = document.language_root();
        Self {
            document,
            role,
            root,
        }
    }

    pub fn locale(&self) -> &Locale {
        self.document.locale()
    }
}

#[derive(Debug)]
pub struct AuditResult {
    issues: Vec<AuditIssue>,
    documents: HashMap<Locale, DocumentNode>,
    workspace: Workspace,
}

impl AuditResult {
    pub(crate) fn new(
        issues: &[AuditIssue],
        nodes: &[DocumentNode],
        workspace: &Workspace,
    ) -> Self {
        let issues = Vec::from(issues);
        let documents = nodes
            .into_iter()
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

    pub fn issues(&self) -> &[AuditIssue] {
        &self.issues
    }
}

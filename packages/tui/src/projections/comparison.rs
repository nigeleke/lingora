use std::{collections::HashMap, rc::Rc};

use fluent4rs::ast::Entry;
use lingora_core::prelude::{AuditIssue, AuditResult, AuditedDocument, QualifiedIdentifier};

use crate::projections::{LocaleNode, LocaleNodeId, LocaleNodeKind, LocalesHierarchy};

#[derive(Debug)]
pub struct Comparison {
    reference: Option<LocaleNodeId>,
    target: Option<LocaleNodeId>,
    audit_result: Rc<AuditResult>,
    locales_hierarchy: LocalesHierarchy,
    entries: HashMap<QualifiedIdentifier, (Vec<Entry>, Vec<Entry>)>,
    issues: Vec<AuditIssue>,
}

impl Comparison {
    pub fn from_reference(
        reference: Option<LocaleNodeId>,
        audit_result: Rc<AuditResult>,
        locales_hierarchy: LocalesHierarchy,
    ) -> Self {
        let mut comparison = Self {
            reference: None,
            target: None,
            audit_result,
            locales_hierarchy,
            entries: HashMap::default(),
            issues: Vec::default(),
        };
        comparison.update_with_reference_and_target(reference, None);
        comparison
    }

    pub fn update_with_reference_and_target(
        &mut self,
        reference: Option<LocaleNodeId>,
        target: Option<LocaleNodeId>,
    ) {
        if reference != self.reference || target != self.target {
            self.reference = reference;
            self.target = target;
            self.update();
        }
    }

    fn document(&self, node: &LocaleNode) -> Option<&AuditedDocument> {
        match node.kind() {
            LocaleNodeKind::Locale { locale } => Some(locale),
            _ => None,
        }
        .and_then(|locale| self.audit_result.document(locale))
    }

    fn update(&mut self) {
        let reference_document = self
            .reference
            .and_then(|id| self.locales_hierarchy.node(&id))
            .and_then(|node| self.document(node));

        let target_document = self
            .target
            .and_then(|id| self.locales_hierarchy.node(&id))
            .and_then(|node| self.document(node));

        let document_entries = |document: Option<&AuditedDocument>, id: &QualifiedIdentifier| {
            let entries = document.iter().flat_map(|d| d.entries(id)).cloned();
            Vec::from_iter(entries)
        };

        let entries = reference_document
            .iter()
            .chain(target_document.iter())
            .flat_map(|d| d.identifiers())
            .fold(HashMap::new(), |mut acc, id| {
                let reference_entries = document_entries(reference_document, &id);
                let target_entries = document_entries(target_document, &id);
                acc.insert(id.clone(), (reference_entries, target_entries));
                acc
            });

        self.issues = Vec::from_iter(self.audit_result.issues().cloned());
        self.entries = entries;
    }

    #[inline(always)]
    pub fn locale_node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
        self.locales_hierarchy.node(node_id)
    }

    #[inline(always)]
    pub fn locales_hierarchy(&self) -> &LocalesHierarchy {
        &self.locales_hierarchy
    }

    #[inline(always)]
    pub fn identifiers(&self) -> impl Iterator<Item = &QualifiedIdentifier> {
        self.entries.keys()
    }

    #[inline(always)]
    pub fn reference_entries(
        &self,
        identifier: Option<&QualifiedIdentifier>,
    ) -> impl Iterator<Item = &Entry> {
        self.entries(self.reference, identifier)
    }

    #[inline(always)]
    pub fn target_entries(
        &self,
        identifier: Option<&QualifiedIdentifier>,
    ) -> impl Iterator<Item = &Entry> {
        self.entries(self.target, identifier)
    }

    fn entries(
        &self,
        node_id: Option<LocaleNodeId>,
        identifier: Option<&QualifiedIdentifier>,
    ) -> impl Iterator<Item = &Entry> {
        identifier.into_iter().flat_map(move |identifier| {
            node_id
                .into_iter()
                .flat_map(|id| self.locales_hierarchy.node(&id))
                .flat_map(|node| {
                    matches!(node.kind(), LocaleNodeKind::Locale { .. })
                        .then(|| node)
                        .into_iter()
                })
                .flat_map(|node| {
                    if let LocaleNodeKind::Locale { locale } = node.kind() {
                        self.audit_result.document(locale)
                    } else {
                        None
                    }
                    .into_iter()
                })
                .flat_map(|doc| doc.entries(identifier))
        })
    }
}

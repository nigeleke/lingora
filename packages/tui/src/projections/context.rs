use std::rc::Rc;

use lingora_core::prelude::{
    AuditResult, AuditedDocument, FluentDocument, LingoraToml, Locale, Workspace,
};

use crate::{
    pages::AppViewState,
    projections::{HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind, LocalesHierarchy},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Context(Rc<ContextInner>);

impl Context {
    pub fn new(settings: &LingoraToml, audit_result: &AuditResult, state: &AppViewState) -> Self {
        let settings = settings.clone();
        let workspace = audit_result.workspace().clone();

        let locale_filter = state.locale_filter().to_owned();
        let locales_hierarchy = LocalesHierarchy::from(audit_result);

        let resolve_document = |get_id: fn(&_) -> Option<&LocaleNodeId>| {
            get_id(state)
                .and_then(|id| locales_hierarchy.node(id))
                .and_then(|node| match node.kind() {
                    LocaleNodeKind::Locale { locale } => {
                        Some(audit_result.document(locale).cloned())
                    }
                    _ => None,
                })
                .flatten()
        };

        let reference = resolve_document(|s| s.reference());
        let target = resolve_document(|s| s.target());

        let identifier_filter = state.identifier_filter().to_owned();

        let inner = ContextInner {
            settings,
            workspace,
            locale_filter,
            locales_hierarchy,
            reference,
            target,
            identifier_filter,
        };

        Self(Rc::new(inner))
    }

    pub fn settings(&self) -> &LingoraToml {
        &self.0.settings
    }

    pub fn workspace(&self) -> &Workspace {
        &self.0.workspace
    }

    pub fn locales(&self) -> impl Iterator<Item = &Locale> {
        self.0.workspace.locales()
    }

    pub fn canonical_locale(&self) -> &Locale {
        &self.0.workspace.canonical_locale()
    }

    pub fn is_canonical_locale(&self, locale: &Locale) -> bool {
        self.0.workspace.is_canonical_locale(locale)
    }

    pub fn is_primary_locale(&self, locale: &Locale) -> bool {
        self.0.workspace.is_primary_locale(locale)
    }

    pub fn is_orphan_locale(&self, locale: &Locale) -> bool {
        self.0.workspace.is_orphan_locale(locale)
    }

    pub fn locales_hierarchy(&self) -> &LocalesHierarchy {
        &self.0.locales_hierarchy
    }

    pub fn locale_node_ids(&self) -> impl Iterator<Item = &LocaleNodeId> {
        self.0.locales_hierarchy.nodes().keys()
    }

    pub fn locale_node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
        self.0.locales_hierarchy.node(node_id)
    }

    pub fn node_id_for_locale(&self, locale: &Locale) -> Option<&LocaleNodeId> {
        self.0.locales_hierarchy.node_id_for_locale(locale)
    }

    pub fn root_node_ids(&self) -> impl Iterator<Item = &LocaleNodeId> {
        self.0.locales_hierarchy.roots()
    }
}

impl HasSelectionPair for Context {
    type Item = AuditedDocument;

    fn reference(&self) -> Option<&Self::Item> {
        self.0.reference.as_ref()
    }

    fn target(&self) -> Option<&Self::Item> {
        self.0.target.as_ref()
    }
}

#[derive(Debug)]
struct ContextInner {
    settings: LingoraToml,
    workspace: Workspace,
    locale_filter: String,
    locales_hierarchy: LocalesHierarchy,
    reference: Option<AuditedDocument>,
    target: Option<AuditedDocument>,
    identifier_filter: String,
}

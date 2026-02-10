use std::collections::{BTreeMap, BTreeSet, HashMap};

use lingora_core::prelude::{AuditResult, LanguageRoot, Locale};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LocaleNodeId(pub usize);

impl LocaleNodeId {
    fn bump(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocaleNodeKind {
    WorkspaceRoot,
    LanguageRoot { language: LanguageRoot },
    Locale { locale: Locale },
}

#[derive(Clone, Debug)]
pub struct LocaleNode {
    kind: LocaleNodeKind,
    has_issues: bool,
    children: Vec<LocaleNodeId>,
}

impl LocaleNode {
    pub fn new(kind: LocaleNodeKind, has_issues: bool, children: &[LocaleNodeId]) -> Self {
        let children = Vec::from(children);
        Self {
            kind,
            has_issues,
            children,
        }
    }
    pub fn kind(&self) -> &LocaleNodeKind {
        &self.kind
    }

    pub fn has_issues(&self) -> bool {
        self.has_issues
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn children(&self) -> impl Iterator<Item = &LocaleNodeId> {
        self.children.iter()
    }
}

#[derive(Debug, Default)]
pub struct LocalesHierarchy {
    roots: Vec<LocaleNodeId>,
    nodes: HashMap<LocaleNodeId, LocaleNode>,
}

impl LocalesHierarchy {
    pub fn roots(&self) -> impl Iterator<Item = &LocaleNodeId> {
        self.roots.iter()
    }

    pub fn nodes(&self) -> &HashMap<LocaleNodeId, LocaleNode> {
        &self.nodes
    }

    pub fn node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
        self.nodes.get(node_id)
    }

    pub fn node_id_for_locale(&self, required_locale: &Locale) -> Option<&LocaleNodeId> {
        self.nodes.iter().find_map(|(id, node)| match node.kind() {
            LocaleNodeKind::WorkspaceRoot => None,
            LocaleNodeKind::LanguageRoot { .. } => None,
            LocaleNodeKind::Locale { locale } => (locale == required_locale).then_some(id),
        })
    }
}

impl From<&AuditResult> for LocalesHierarchy {
    fn from(audit_result: &AuditResult) -> Self {
        let issues = audit_result.issues().fold(BTreeMap::new(), |mut acc, i| {
            let locale = i.locale();
            acc.entry(locale).or_insert_with(Vec::new).push(i.clone());
            acc
        });

        let mut nodes = HashMap::new();
        let mut node_id = LocaleNodeId::default();
        let mut roots: Vec<LocaleNodeId> = Vec::new();

        let mut add_node = |node| {
            node_id.bump();
            nodes.insert(node_id, node);
            node_id
        };

        if let Some(_issues) = issues.get(&None) {
            let node_id = add_node(LocaleNode::new(LocaleNodeKind::WorkspaceRoot, true, &[]));
            roots.push(node_id);
        }

        audit_result
            .document_locales()
            .fold(BTreeMap::new(), |mut acc, locale| {
                let root = LanguageRoot::from(locale);
                acc.entry(root)
                    .or_insert(BTreeSet::new())
                    .insert(locale.clone());
                acc
            })
            .iter()
            .for_each(|(language, locales)| {
                let language = language.clone();
                let mut language_issues = false;

                let locale_node_ids = locales
                    .iter()
                    .map(|locale| {
                        let locale = locale.clone();
                        let locale_issues = issues
                            .get(&Some(locale.clone()))
                            .is_some_and(|issues| !issues.is_empty());
                        language_issues |= locale_issues;

                        add_node(LocaleNode::new(
                            LocaleNodeKind::Locale { locale },
                            locale_issues,
                            &[],
                        ))
                    })
                    .collect::<Vec<_>>();

                let node_id = add_node(LocaleNode::new(
                    LocaleNodeKind::LanguageRoot { language },
                    language_issues,
                    &locale_node_ids,
                ));
                roots.push(node_id);
            });

        Self { roots, nodes }
    }
}

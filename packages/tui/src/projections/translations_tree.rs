use std::collections::{BTreeMap, BTreeSet, HashMap};

use lingora_core::prelude::{AuditResult, LanguageRoot, Locale};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeId(usize);

impl NodeId {
    fn bump(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    WorkspaceRoot,
    LanguageRoot { language: LanguageRoot },
    Locale { locale: Locale },
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    kind: NodeKind,
    has_issues: bool,
    children: Vec<NodeId>,
}

impl TreeNode {
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn has_issues(&self) -> bool {
        self.has_issues
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn children(&self) -> impl Iterator<Item = &NodeId> {
        self.children.iter()
    }
}

pub struct TranslationsTree {
    roots: Vec<NodeId>,
    nodes: HashMap<NodeId, TreeNode>,
}

impl TranslationsTree {
    pub fn roots(&self) -> impl Iterator<Item = &NodeId> {
        self.roots.iter()
    }

    pub fn node(&self, node_id: &NodeId) -> Option<&TreeNode> {
        self.nodes.get(node_id)
    }
}

impl From<&AuditResult> for TranslationsTree {
    fn from(audit_result: &AuditResult) -> Self {
        let issues = audit_result.issues().fold(BTreeMap::new(), |mut acc, i| {
            let locale = i.locale();
            acc.entry(locale).or_insert_with(Vec::new).push(i.clone());
            acc
        });

        let mut nodes = HashMap::new();
        let mut node_id = NodeId::default();
        let mut roots: Vec<NodeId> = Vec::new();

        let mut add_node = |node| {
            node_id.bump();
            nodes.insert(node_id, node);
            node_id
        };

        if let Some(_issues) = issues.get(&None) {
            let node_id = add_node(TreeNode {
                kind: NodeKind::WorkspaceRoot,
                has_issues: true,
                children: vec![],
            });
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

                        add_node(TreeNode {
                            kind: NodeKind::Locale { locale },
                            has_issues: locale_issues,
                            children: vec![],
                        })
                    })
                    .collect::<Vec<_>>();

                let node_id = add_node(TreeNode {
                    kind: NodeKind::LanguageRoot { language },
                    has_issues: language_issues,
                    children: locale_node_ids,
                });
                roots.push(node_id);
            });

        TranslationsTree { roots, nodes }
    }
}

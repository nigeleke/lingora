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

#[derive(Debug)]
pub enum NodeKind {
    WorkspaceRoot,
    LanguageRoot { language: LanguageRoot },
    Locale { locale: Locale },
}

pub struct TreeNode {
    kind: NodeKind,
    has_issues: bool,
    children: Vec<NodeId>,
}

impl TreeNode {
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn children(&self) -> impl Iterator<Item = &NodeId> {
        self.children.iter().into_iter()
    }

    pub fn description(&self) -> String {
        match &self.kind {
            NodeKind::WorkspaceRoot => String::from("workspace"),
            NodeKind::LanguageRoot { language } => language.to_string(),
            NodeKind::Locale { locale } => locale.to_string(),
        }
    }
}

pub struct TranslationsTree {
    roots: Vec<NodeId>,
    nodes: HashMap<NodeId, TreeNode>,
}

impl TranslationsTree {
    pub fn roots(&self) -> impl Iterator<Item = &NodeId> {
        self.roots.iter().into_iter()
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

        let mut add_node = |node, as_root| {
            node_id.bump();
            nodes.insert(node_id, node);
            if as_root {
                roots.push(node_id);
            }
            node_id
        };

        if let Some(_issues) = issues.get(&None) {
            let _ = add_node(
                TreeNode {
                    kind: NodeKind::WorkspaceRoot,
                    has_issues: true,
                    children: vec![],
                },
                true,
            );
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
                            .map_or(false, |is| !is.is_empty());
                        language_issues |= locale_issues;

                        add_node(
                            TreeNode {
                                kind: NodeKind::Locale { locale },
                                has_issues: locale_issues,
                                children: vec![],
                            },
                            false,
                        )
                    })
                    .collect::<Vec<_>>();

                let _ = add_node(
                    TreeNode {
                        kind: NodeKind::LanguageRoot { language },
                        has_issues: language_issues,
                        children: locale_node_ids,
                    },
                    true,
                );
            });

        TranslationsTree { roots, nodes }
    }
}

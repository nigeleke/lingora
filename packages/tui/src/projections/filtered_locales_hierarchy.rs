use std::collections::{HashMap, HashSet};

use lingora_core::prelude::LanguageRoot;

use crate::projections::{LocaleNode, LocaleNodeId, LocaleNodeKind, LocalesHierarchy};

#[derive(Debug, Default)]
pub struct FilteredLocalesHierarchy {
    roots: Vec<LocaleNodeId>,
    nodes: HashMap<LocaleNodeId, LocaleNode>,
}

impl FilteredLocalesHierarchy {
    pub fn filter_from(hierarchy: &LocalesHierarchy, filter: &str) -> Self {
        let mut locales = hierarchy
            .nodes()
            .iter()
            .filter_map(|(id, node)| match node.kind() {
                LocaleNodeKind::Locale { locale }
                    if locale.to_string().to_ascii_lowercase().contains(&filter) =>
                {
                    Some((*id, locale.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        locales.sort_by_key(|l| l.1.clone());

        let locale_ids = locales.iter().map(|(id, _)| id).collect::<Vec<_>>();

        let languages = locales
            .iter()
            .map(|(_, l)| LanguageRoot::from(l))
            .collect::<HashSet<_>>();

        let mut roots = hierarchy
            .nodes()
            .iter()
            .filter_map(|(id, node)| match node.kind() {
                LocaleNodeKind::WorkspaceRoot => Some((*id, None)),
                LocaleNodeKind::LanguageRoot { language } if languages.contains(language) => {
                    Some((*id, Some(language)))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        roots.sort_by_key(|r| r.1);

        let roots = roots.iter().map(|r| r.0).collect::<Vec<_>>();

        let nodes = hierarchy
            .nodes()
            .iter()
            .filter_map(|(id, node)| match node.kind() {
                LocaleNodeKind::WorkspaceRoot if roots.contains(id) => Some((*id, node.clone())),
                LocaleNodeKind::LanguageRoot { .. } if roots.contains(id) => {
                    let kind = node.kind().clone();
                    let has_issues = node.has_issues();
                    let child_set: HashSet<LocaleNodeId> = node.children().copied().collect();
                    let children = locale_ids
                        .iter()
                        .copied()
                        .filter_map(|id| child_set.contains(id).then_some(*id))
                        .collect::<Vec<_>>();
                    Some((*id, LocaleNode::new(kind, has_issues, &children)))
                }
                LocaleNodeKind::Locale { .. } if locale_ids.contains(&id) => {
                    Some((*id, node.clone()))
                }
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        Self { roots, nodes }
    }

    pub fn roots(&self) -> impl Iterator<Item = &LocaleNodeId> {
        self.roots.iter()
    }

    pub fn nodes(&self) -> &HashMap<LocaleNodeId, LocaleNode> {
        &self.nodes
    }

    // pub fn node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
    //     self.nodes.get(node_id)
    // }

    // pub fn node_id_for_locale(&self, required_locale: &Locale) -> Option<&LocaleNodeId> {
    //     self.nodes.iter().find_map(|(id, node)| match node.kind() {
    //         LocaleNodeKind::WorkspaceRoot => None,
    //         LocaleNodeKind::LanguageRoot { .. } => None,
    //         LocaleNodeKind::Locale { locale } => (locale == required_locale).then_some(id),
    //     })
    // }
}

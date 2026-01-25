use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct NodeId(String);

pub enum IssueStatus {
    None,
    HasIssues,
}

pub enum NodeKind {
    WorkspaceRoot,
    LanguageRoot { language: String },
    Locale { locale: Locale },
    File { file: QualifiedFluentFile },
}

pub struct TreeNode {
    pub kind: NodeKind,
    pub issue_status: IssueStatus,
    pub children: Vec<NodeId>,
}

pub struct TranslationsTree {
    pub roots: Vec<NodeId>,
    pub nodes: HashMap<NodeId, TreeNode>,
}

// fn has_issues<'a>(issues: impl Iterator<Item = &'a AuditIssue>) -> bool

impl From<&AuditReport> for TranslationsTree {
    fn from(report: &AuditReport) -> Self {
        let mut nodes: HashMap<NodeId, TreeNode> = HashMap::new();
        let mut roots: Vec<NodeId> = Vec::new();

        let workspace_issues = report
            .issues
            .iter()
            .filter(|i| matches!(i.context.target, ContextTarget::Workspace { .. }))
            .count();

        if workspace_issues > 0 {
            let id = NodeId("workspace".into());

            nodes.insert(
                id.clone(),
                TreeNode {
                    kind: NodeKind::WorkspaceRoot,
                    issue_status: IssueStatus::HasIssues,
                    children: Vec::new(),
                },
            );

            roots.push(id);
        }

        // ---- group issues by locale ----

        let mut issues_by_locale: HashMap<Locale, usize> = HashMap::new();

        for issue in &report.issues {
            if let ContextTarget::Workspace { .. } = issue.context.target {
                continue;
            }

            let locale = issue.context.locale().clone();
            *issues_by_locale.entry(locale).or_insert(0) += 1;
        }

        // ---- group files by language root ----

        let mut files_by_language: HashMap<String, Vec<QualifiedFluentFile>> = HashMap::new();

        for file in &report.workspace.fluent_files {
            let lang = file.locale().language().to_string();
            files_by_language
                .entry(lang)
                .or_default()
                .push(file.clone());
        }

        // ---- build language → locale → file nodes ----

        for (language, files) in files_by_language {
            let lang_id = NodeId(format!("lang:{language}"));
            let mut locale_nodes: HashMap<Locale, Vec<NodeId>> = HashMap::new();

            // file nodes
            for file in files {
                let locale = file.locale().clone();
                let file_id = NodeId(format!("file:{}", file.path().display()));

                let has_issue = issues_by_locale.get(&locale).copied().unwrap_or(0) > 0;

                nodes.insert(
                    file_id.clone(),
                    TreeNode {
                        kind: NodeKind::File { file },
                        issue_status: if has_issue {
                            IssueStatus::HasIssues
                        } else {
                            IssueStatus::None
                        },
                        children: Vec::new(),
                    },
                );

                locale_nodes.entry(locale).or_default().push(file_id);
            }

            let mut lang_children = Vec::new();
            let mut lang_has_issues = false;

            // locale nodes
            for (locale, file_ids) in locale_nodes {
                let locale_id = NodeId(format!("locale:{language}:{locale}"));

                let has_issue = issues_by_locale.get(&locale).copied().unwrap_or(0) > 0;
                if has_issue {
                    lang_has_issues = true;
                }

                nodes.insert(
                    locale_id.clone(),
                    TreeNode {
                        kind: NodeKind::Locale { locale },
                        issue_status: if has_issue {
                            IssueStatus::HasIssues
                        } else {
                            IssueStatus::None
                        },
                        children: file_ids,
                    },
                );

                lang_children.push(locale_id);
            }

            nodes.insert(
                lang_id.clone(),
                TreeNode {
                    kind: NodeKind::LanguageRoot { language },
                    issue_status: if lang_has_issues {
                        IssueStatus::HasIssues
                    } else {
                        IssueStatus::None
                    },
                    children: lang_children,
                },
            );

            roots.push(lang_id);
        }

        Self { roots, nodes }
    }
}

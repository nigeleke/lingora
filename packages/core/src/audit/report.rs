use std::collections::BTreeMap;

use crate::{
    audit::{AuditIssue, AuditKind, Workspace},
    domain::Locale,
};

pub struct AuditReport {
    issues: Vec<AuditIssue>,
    workspace: Workspace,
}

impl AuditReport {
    pub fn new(issues: &[AuditIssue], workspace: &Workspace) -> Self {
        let issues = Vec::from(issues);
        let workspace = workspace.clone();
        Self { issues, workspace }
    }

    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn workspace_issues(&self) -> Vec<AuditIssue> {
        Vec::from_iter(self.issues.iter().filter_map(|issue| {
            matches!(issue.kind(), AuditKind::Workspace(_)).then_some(issue.clone())
        }))
    }

    pub fn issues_by_locale(&self) -> BTreeMap<Locale, Vec<AuditIssue>> {
        self.issues
            .iter()
            .filter(|issue| !matches!(issue.kind(), AuditKind::Workspace(_)))
            .fold(BTreeMap::new(), |mut acc, issue| {
                let locale = issue.locale().clone();
                let issue = issue.clone();
                acc.entry(locale).or_default().push(issue);
                acc
            })
    }
}

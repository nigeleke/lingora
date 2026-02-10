use lingora_core::prelude::{AuditIssue, Subject};

use crate::{
    pages::TranslationsState,
    projections::{HasSelectionPair, LocaleNodeKind},
};

pub struct FilteredIssues(Vec<AuditIssue>);

impl FilteredIssues {
    pub fn from_issues<'a>(
        issues: impl Iterator<Item = &'a AuditIssue>,
        state: &TranslationsState,
    ) -> Self {
        let filter = |issue: &AuditIssue| {
            if let Some(reference_node_id) = state.reference()
                && let Some(reference_node) = state.locale_node(reference_node_id)
                && let Some(target_node_id) = state.target()
                && let Some(target_node) = state.locale_node(target_node_id)
            {
                match issue.subject() {
                    Subject::FluentFile(_) | Subject::RustFile(_) => {
                        matches!(reference_node.kind(), LocaleNodeKind::WorkspaceRoot)
                            || matches!(target_node.kind(), LocaleNodeKind::WorkspaceRoot)
                    }
                    Subject::Entry(locale, _) | Subject::Locale(locale) => {
                        matches!(reference_node.kind(), LocaleNodeKind::Locale { locale: l } if l == locale)
                            || matches!(target_node.kind(), LocaleNodeKind::Locale { locale: l } if l == locale)
                    }
                    Subject::LanguageRoot(language) => {
                        matches!(reference_node.kind(), LocaleNodeKind::LanguageRoot { language: l } if l == language)
                            || matches!(reference_node.kind(), LocaleNodeKind::WorkspaceRoot)
                            || matches!(target_node.kind(), LocaleNodeKind::LanguageRoot { language: l } if l == language)
                            || matches!(target_node.kind(), LocaleNodeKind::WorkspaceRoot)
                    }
                }
            } else {
                true
            }
        };

        let issues = Vec::from_iter(issues.filter(|i| filter(i)).cloned());
        Self(issues)
    }

    pub fn issues(&self) -> &Vec<AuditIssue> {
        &self.0
    }
}

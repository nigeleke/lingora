use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{
    audit::AuditIssue,
    domain::{LanguageRoot, Locale},
};

#[derive(Debug, Clone)]
pub struct AuditReportContext {
    canonical: Locale,
    primaries: Vec<Locale>,
    language_roots: BTreeSet<LanguageRoot>,
}

impl AuditReportContext {
    pub fn new(
        canonical: &Locale,
        primaries: &[Locale],
        language_roots: &HashSet<LanguageRoot>,
    ) -> Self {
        Self {
            canonical: canonical.clone(),
            primaries: Vec::from(primaries),
            language_roots: BTreeSet::from_iter(language_roots.iter().cloned()),
        }
    }
}

pub struct AuditReport {
    issues: Vec<AuditIssue>,
    context: AuditReportContext,
}

impl AuditReport {
    pub fn new(issues: &[AuditIssue], context: AuditReportContext) -> Self {
        let issues = Vec::from(issues);
        Self { issues, context }
    }

    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn canonical_locale(&self) -> &Locale {
        &self.context.canonical
    }

    pub fn issues_by_locale(&self) -> BTreeMap<Locale, Vec<AuditIssue>> {
        self.issues.iter().fold(BTreeMap::new(), |mut acc, issue| {
            let locale = issue.locale().clone();
            let issue = issue.clone();
            acc.entry(locale).or_default().push(issue);
            acc
        })
    }
}

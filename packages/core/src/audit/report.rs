use std::collections::BTreeMap;

use crate::{audit::AuditIssue, domain::Locale};

#[derive(Debug, Clone)]
pub struct AuditReportContext {
    canonical: Locale,
    primaries: Vec<Locale>,
}

impl AuditReportContext {
    pub fn new(canonical: &Locale, primaries: &[Locale]) -> Self {
        Self {
            canonical: canonical.clone(),
            primaries: Vec::from(primaries),
        }
    }

    pub fn canonical(&self) -> &Locale {
        &self.canonical
    }

    pub fn primaries(&self) -> &[Locale] {
        &self.primaries
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

    pub fn context(&self) -> &AuditReportContext {
        &self.context
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

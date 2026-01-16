use std::{collections::BTreeMap, io};

use crate::{
    audit::{AuditIssue, AuditReport, Workspace},
    domain::Locale,
    error::LingoraError,
};

pub struct AnalysisRenderer {
    workspace: Workspace,
    workspace_issues: Vec<AuditIssue>,
    issues_by_locale: BTreeMap<Locale, Vec<AuditIssue>>,
}

impl<'a> AnalysisRenderer {
    pub fn new(report: &AuditReport) -> Self {
        let workspace = report.workspace().clone();
        let workspace_issues = report.workspace_issues();
        let issues_by_locale = report.issues_by_locale();
        Self {
            workspace,
            workspace_issues,
            issues_by_locale,
        }
    }

    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        self.workspace_issues
            .iter()
            .try_for_each(|issue| writeln!(out, "Workspace: {issue}"))?;
        self.render_language(out, "Canonical:", &self.workspace.canonical_locale())?;
        self.workspace
            .primary_locales()
            .try_for_each(|primary| self.render_language(out, "Primary:", primary))?;
        self.workspace
            .orphan_locales()
            .try_for_each(|locale| self.render_locale(out, "Orphaned:", locale))
    }

    fn render_language<W: io::Write>(
        &self,
        out: &mut W,
        title: &str,
        base: &Locale,
    ) -> Result<(), LingoraError> {
        writeln!(out, "Language:  {}", base.language())?;

        self.render_locale(out, title, base)?;

        let mut variants_locales = Vec::from_iter(self.workspace.variant_locales(base));

        if !variants_locales.is_empty() {
            variants_locales.sort();
            variants_locales
                .iter()
                .try_for_each(|v| self.render_locale(out, "Variant:", v))?
        }

        Ok(())
    }

    fn render_locale<W: io::Write>(
        &self,
        out: &mut W,
        title: &str,
        locale: &Locale,
    ) -> Result<(), LingoraError> {
        if let Some(issues) = self.issues_by_locale.get(locale) {
            writeln!(out, "{:10} {}", title, locale,)?;
            issues
                .iter()
                .try_for_each(|issue| writeln!(out, "{:11}{}", "", issue))?;
        } else {
            writeln!(out, "{:10} {} - Ok", title, locale)?;
        }

        Ok(())
    }
}

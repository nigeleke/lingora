use std::{collections::BTreeMap, io};

use crate::{
    audit::{AuditIssue, AuditResult, Workspace},
    domain::Locale,
    error::LingoraError,
};

pub struct AnalysisRenderer {
    workspace: Workspace,
    issues: BTreeMap<Option<Locale>, Vec<AuditIssue>>,
}

impl AnalysisRenderer {
    pub fn new(audit_result: &AuditResult) -> Self {
        let workspace = audit_result.workspace().clone();

        let issues = audit_result.issues().fold(BTreeMap::new(), |mut acc, i| {
            let locale = i.locale();
            acc.entry(locale).or_insert_with(Vec::new).push(i.clone());
            acc
        });

        Self { workspace, issues }
    }

    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        self.render_workspace(out)?;
        self.render_language(out, "Canonical:", self.workspace.canonical_locale())?;
        self.workspace
            .primary_locales()
            .try_for_each(|primary| self.render_language(out, "Primary:", primary))?;
        self.workspace
            .orphan_locales()
            .try_for_each(|locale| self.render_locale(out, "Orphaned:", locale))
    }

    fn render_workspace<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        if let Some(issues) = self.issues.get(&None) {
            writeln!(out, "Workspace:")?;

            let mut issues = issues.clone();
            issues.sort_by_key(|i| i.kind().clone());

            issues
                .iter()
                .try_for_each(|i| writeln!(out, "{:10} {}", "", i.message()))?;
        }

        Ok(())
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
        if let Some(issues) = self.issues.get(&Some(locale.clone())) {
            writeln!(out, "{:10} {}", title, locale,)?;

            let mut issues = issues.clone();
            issues.sort_by_key(|i| i.kind().clone());

            issues
                .iter()
                .try_for_each(|issue| writeln!(out, "{:11}{issue}", ""))?;
        } else {
            writeln!(out, "{:10} {} - Ok", title, locale)?;
        }

        Ok(())
    }
}

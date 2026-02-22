use std::{collections::BTreeMap, io};

use crate::{
    audit::{AuditIssue, AuditResult, Workspace},
    domain::Locale,
    error::LingoraError,
};

/// A hierarchical text renderer for `AuditResult`.
///
/// Produces structured console output showing:
/// - Workspace-level issues (parse errors, ambiguous roots, etc.)
/// - Canonical locale status
/// - Each primary language (grouped under its language root)
///   - The primary/base document
///   - Any regional/script variants of that language
/// - Orphaned locales (no matching base root)
///
/// Issues for each locale are sorted by `Kind` for consistent readability.
pub struct AnalysisRenderer {
    workspace: Workspace,
    issues: BTreeMap<Option<Locale>, Vec<AuditIssue>>,
}

impl AnalysisRenderer {
    /// Creates a new renderer from an `AuditResult`.
    ///
    /// Groups all issues by their associated locale (extracted via `AuditIssue::locale()`).
    /// Global/workspace issues (no locale) are stored under `None`.
    pub fn new(audit_result: &AuditResult) -> Self {
        let workspace = audit_result.workspace().clone();

        let issues = audit_result.issues().fold(BTreeMap::new(), |mut acc, i| {
            let locale = i.locale();
            acc.entry(locale).or_insert_with(Vec::new).push(i.clone());
            acc
        });

        Self { workspace, issues }
    }

    /// Renders the full audit report to the given writer.
    ///
    /// Order of output:
    /// 1. Workspace-level issues (if any)
    /// 2. Canonical locale
    /// 3. Each primary language group:
    ///    - Primary/base locale
    ///    - All variants (sorted)
    /// 4. Orphaned locales (sorted)
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
            let issues = issues.iter().fold(BTreeMap::new(), |mut acc, issue| {
                acc.entry(issue.subject())
                    .or_insert_with(Vec::new)
                    .push(issue);
                acc
            });

            issues.iter().try_for_each(|(subject, issues)| {
                writeln!(out, "Workspace: {subject}")?;

                issues
                    .iter()
                    .try_for_each(|issue| writeln!(out, "{:10} {}", "", issue.message()))
            })?;
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

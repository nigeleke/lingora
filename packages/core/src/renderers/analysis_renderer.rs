use std::{collections::BTreeMap, io};

use crate::{
    audit::{AuditIssue, AuditReport, AuditReportContext},
    domain::Locale,
    error::LingoraError,
};

pub struct AnalysisRenderer {
    context: AuditReportContext,
    issues_by_locale: BTreeMap<Locale, Vec<AuditIssue>>,
}

impl<'a> AnalysisRenderer {
    pub fn new(report: &AuditReport) -> Self {
        let context = report.context().clone();
        let issues_by_locale = report.issues_by_locale();
        Self {
            context,
            issues_by_locale,
        }
    }

    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        self.render_language(out, &self.context.canonical())?;
        self.context
            .primaries()
            .iter()
            .try_for_each(|primary| self.render_language(out, primary))
    }

    fn render_language<W: io::Write>(
        &self,
        out: &mut W,
        base: &Locale,
    ) -> Result<(), LingoraError> {
        writeln!(out, "Language: {}", base.language())?;

        self.render_locale(
            out,
            if base == self.context.canonical() {
                "Canonical"
            } else {
                "Primary"
            },
            base,
        )?;

        let mut variants = self
            .issues_by_locale
            .keys()
            .filter_map(|locale| {
                (locale.language() == base.language() && locale != base).then_some(locale)
            })
            .collect::<Vec<_>>();

        if !variants.is_empty() {
            variants.sort();
            variants
                .iter()
                .try_for_each(|v| self.render_locale(out, "Variant", v))?
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
            writeln!(out, "{}: {}", title, locale,)?;
            issues.iter().try_for_each(|issue| {
                writeln!(out, "{:?}: {:?}", issue.kind(), issue.identifier())
            })?;
        } else {
            writeln!(out, "{}: {} - Ok", title, locale)?;
        }

        Ok(())
    }
}

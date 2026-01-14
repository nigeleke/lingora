use std::{collections::BTreeMap, io};

use crate::{
    audit::{AuditIssue, AuditReport},
    config::LingoraToml,
    domain::Locale,
    error::LingoraError,
};

pub struct AnalysisRenderer {
    settings: LingoraToml,
    issues_by_locale: BTreeMap<Locale, Vec<AuditIssue>>,
}

impl<'a> AnalysisRenderer {
    pub fn new(settings: &LingoraToml, report: &AuditReport) -> Self {
        let settings = settings.clone();
        let issues_by_locale = report.issues_by_locale();
        Self {
            settings,
            issues_by_locale,
        }
    }

    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        self.render_language(out, &self.settings.lingora.canonical)?;
        self.settings
            .lingora
            .primaries
            .iter()
            .try_for_each(|primary| self.render_language(out, primary))
    }

    fn render_language<W: io::Write>(
        &self,
        out: &mut W,
        base: &Locale,
    ) -> Result<(), LingoraError> {
        writeln!(out, "Language: {}", base.language())?;
        self.render_locale(out, "Base", base);

        let mut variants = self
            .settings
            .lingora
            .fluent_sources
            .iter()
            .filter_map(|f| {
                Locale::try_from(f.as_path()).map_or(None, |locale| {
                    (locale.language() == base.language() && &locale != base).then_some(locale)
                })
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
            writeln!(
                out,
                "{}: {}{}",
                title,
                locale,
                if issues.is_empty() { " - Ok" } else { "" }
            )?;
            issues.iter().try_for_each(|issue| {
                writeln!(out, "{:?}: {:?}", issue.kind(), issue.identifier())
            })?;
        }

        Ok(())
    }
}

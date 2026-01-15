use std::collections::HashMap;

use crate::{
    audit::{AuditIssue, AuditRule, Context, ContextKind},
    domain::LanguageRoot,
};

pub struct BaseFilesUniqueRule;

impl AuditRule for BaseFilesUniqueRule {
    fn applies_to(&self, context: &Context) -> bool {
        matches!(context.kind(), ContextKind::Workspace)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some(workspace) = context.workspace() {
            let required_locales = Vec::from_iter(workspace.base_locales());

            let counts = workspace
                .fluent_files()
                .iter()
                .fold(HashMap::new(), |mut acc, file| {
                    let locale = file.locale();
                    if required_locales.contains(&locale) {
                        *acc.entry(LanguageRoot::from(locale)).or_insert(0) += 1;
                    }
                    acc
                });

            counts
                .into_iter()
                .filter_map(|(root, count)| (count > 1).then_some(root))
                .for_each(|l| {
                    issues.push(AuditIssue::workspace(
                        context,
                        &format!("multiple base files found for language root {l}"),
                    ))
                })
        }

        issues
    }
}

#[cfg(test)]
mod test {
    use std::{path::Path, str::FromStr};

    use super::*;
    use crate::{
        assert_issue,
        audit::{AuditKind, Workspace},
        domain::Locale,
        fluent::QualifiedFluentFile,
    };

    #[test]
    fn unique_base_locales_must_provide_for_root_language() {
        let provided_files = [
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualifiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        let canonical_locale = Locale::from_str("en-GB").unwrap();
        let primary_locales =
            [Locale::from_str("it-IT"), Locale::from_str("sr-Cryl-RS")].map(|l| l.unwrap());

        let workspace = Workspace::new(&provided_files, canonical_locale, &primary_locales, &[]);

        let context = Context::new_workspace_context(workspace);
        let rule = BaseFilesUniqueRule;
        let issues = rule.audit(&context);

        assert!(issues.is_empty());
    }

    #[test]
    fn multiple_base_locales_must_provide_for_root_language_is_an_error() {
        let provided_files = [
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualifiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        let canonical_locale = Locale::from_str("en-GB").unwrap();
        let primary_locales = [
            Locale::from_str("en-AU"),
            Locale::from_str("it-IT"),
            Locale::from_str("sr-Cryl-RS"),
            Locale::from_str("sr-Cryl-BA"),
        ]
        .map(|l| l.unwrap());

        let workspace = Workspace::new(&provided_files, canonical_locale, &primary_locales, &[]);

        let context = Context::new_workspace_context(workspace);
        let rule = BaseFilesUniqueRule;
        let issues = rule.audit(&context);

        assert_issue!(
            issues,
            AuditKind::Workspace("multiple base files found for language root en".into())
        );
        assert_issue!(
            issues,
            AuditKind::Workspace("multiple base files found for language root sr-Cryl".into())
        );
    }
}

use crate::audit::{AuditIssue, AuditRule, Context, ContextKind};

pub struct BaseFilesProvidedRule;

impl AuditRule for BaseFilesProvidedRule {
    fn applies_to(&self, context: &Context) -> bool {
        matches!(context.kind(), ContextKind::Workspace)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some(workspace) = context.workspace() {
            let required_locales = workspace.base_locales();

            let provided_locales = workspace
                .fluent_files()
                .iter()
                .map(|f| f.locale())
                .collect::<std::collections::HashSet<_>>();

            required_locales
                .filter(|l| !provided_locales.contains(l))
                .for_each(|l| {
                    issues.push(AuditIssue::workspace(
                        context,
                        &format!("missing fluent file for locale {l}"),
                    ))
                });
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
    fn base_locales_must_have_fluent_file() {
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
        let rule = BaseFilesProvidedRule;
        let issues = rule.audit(&context);

        assert!(issues.is_empty());
    }

    #[test]
    fn base_locales_without_fluent_file_is_an_error() {
        let provided_files = [
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
        ]
        .into_iter()
        .map(|p| QualifiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        let canonical_locale = Locale::from_str("en-GB").unwrap();
        let primary_locales =
            [Locale::from_str("it-IT"), Locale::from_str("sr-Cryl-RS")].map(|l| l.unwrap());

        let workspace = Workspace::new(&provided_files, canonical_locale, &primary_locales, &[]);

        let context = Context::new_workspace_context(workspace);
        let rule = BaseFilesProvidedRule;
        let issues = rule.audit(&context);

        assert_issue!(
            issues,
            AuditKind::Workspace("missing fluent file for locale sr-Cryl-RS".into())
        );
    }
}

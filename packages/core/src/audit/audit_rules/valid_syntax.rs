use crate::audit::{AuditIssue, AuditRule, Context, ContextKind};

pub struct ValidSyntaxRule;

impl AuditRule for ValidSyntaxRule {
    fn applies_to(&self, context: &crate::audit::Context) -> bool {
        matches!(context.kind(), ContextKind::All)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some(f) = context.fluent_single()
            && let Err(error) = f.resource()
        {
            issues.push(AuditIssue::invalid_syntax(context, &error.to_string()));
        }

        issues
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_issue, audit::AuditKind, test_support::qff};

    #[test]
    fn valid_syntactic_file_will_have_no_issues() {
        let file = qff(
            "en-GB",
            r#"
### Comment
## Comment
# Comment
matched-message = Reference matched message
-matched-term = Reference matched term
missing-message = Reference missing message
-missing-term = Reference missing term
"#,
        );

        let context = Context::new_all_context(file);
        let rule = ValidSyntaxRule;
        let issues = rule.audit(&context);

        assert!(issues.is_empty());
    }

    #[test]
    fn invalid_fluent_file_will_fail_integrity_test() {
        let file = qff(
            "en-GB",
            r#"
gobbledegook !@#$%^&*()_+=-
"#,
        );

        let context = Context::new_all_context(file);
        let rule = ValidSyntaxRule;
        let issues = rule.audit(&context);

        assert_issue!(issues, AuditKind::InvalidSyntax("Unwanted junk found in Fluent grammar: Invalid entries: gobbledegook !@#$%^&*()_+=-\n".into()));
    }
}

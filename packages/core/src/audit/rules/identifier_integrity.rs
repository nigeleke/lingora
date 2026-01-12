use crate::audit::{AuditIssue, AuditRule, Context, ContextKind};

pub struct IdentifierIntegrityRule;

impl AuditRule for IdentifierIntegrityRule {
    fn applies_to(&self, context: &Context) -> bool {
        matches!(context.kind, ContextKind::FluentIntegrity)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some(file) = context.fluent_single() {
            file.duplicate_identifiers().iter().for_each(|k| {
                issues.push(AuditIssue::DuplicateDefinition(k.to_normalized_string()))
            });
        }

        issues
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_support::qff;

    #[test]
    fn will_detect_duplicated_messages() {
        let file = qff(
            "en-GB",
            r#"
message1 = Message 1.1
message1 = Message 1.2
message2 = Message 2
"#,
        );

        let context = Context::fluent_file(&file);
        let rule = IdentifierIntegrityRule;
        let issues = rule.audit(&context);
        assert!(issues.contains(&AuditIssue::DuplicateDefinition("message1".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("message2".into())));
    }

    #[test]
    fn will_detect_duplicated_terms() {
        let file = qff(
            "en-GB",
            r#"
-term1 = Term 1.1
-term1 = Term 1.2
-term2 = Term 2
"#,
        );

        let context = Context::fluent_file(&file);
        let rule = IdentifierIntegrityRule;
        let issues = rule.audit(&context);
        assert!(issues.contains(&AuditIssue::DuplicateDefinition("-term1".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("-term2".into())));
    }

    #[test]
    fn will_detect_duplicated_attributes() {
        let file = qff(
            "en-GB",
            r#"
message1 =
    .attr1 = Attribute 1.1
    .attr1 = Attribute 1.2
    .attr2 = Attribute 2.1
message2 =
    .attr2 = Attribute 2.1
"#,
        );

        let context = Context::fluent_file(&file);
        let rule = IdentifierIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::DuplicateDefinition("message1 / .attr1".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("message1 / .attr2".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("message2 / .attr2".into())));
    }

    #[test]
    fn will_detect_duplicated_variants() {
        let file = qff(
            "en-GB",
            r#"
emails =
    { $unreadEmails ->
        [one] You have one unread email.
        [one] You have one more unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
emails3 =
    { $unreadEmails ->
        [other] You have unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
        );

        let context = Context::fluent_file(&file);
        let rule = IdentifierIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::DuplicateDefinition("emails / [one]".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("emails / [two]".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("emails / [other]".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("emails2 / [one]".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("emails2 / [two]".into())));
        assert!(!issues.contains(&AuditIssue::DuplicateDefinition("emails2 / [other]".into())));
        assert!(issues.contains(&AuditIssue::DuplicateDefinition("emails3 / [other]".into())));
    }
}

use std::collections::HashSet;

use super::emit_ordered;
use crate::audit::{AuditIssue, AuditRule, Context, ContextKind};

pub struct TranslationIntegrityRule;

impl AuditRule for TranslationIntegrityRule {
    fn applies_to(&self, context: &Context) -> bool {
        matches!(context.kind, ContextKind::CanonicalToPrimary)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some((canonical, primary)) = context.fluent_pair() {
            let canonical_entries = canonical.entry_identifiers().collect::<HashSet<_>>();
            let primary_entries = primary.entry_identifiers().collect::<HashSet<_>>();

            emit_ordered(canonical_entries.difference(&primary_entries), |id| {
                issues.push(AuditIssue::MissingTranslation(id.to_meta_string()))
            });

            emit_ordered(
                canonical_entries
                    .intersection(&primary_entries)
                    .filter(|id| canonical.signature(id) != primary.signature(id)),
                |id| issues.push(AuditIssue::SignatureMismatch(id.to_meta_string())),
            );

            emit_ordered(primary_entries.difference(&canonical_entries), |id| {
                issues.push(AuditIssue::RedundantTranslation(id.to_meta_string()))
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
    fn detects_untranslated_messages() {
        let canonical = qff(
            "en-AU",
            r#"
message1 = G'day en 1
message2 = G'day en 2
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
message1 = Buongiorno it 1
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(!issues.contains(&AuditIssue::MissingTranslation("message1".into())));
        assert!(issues.contains(&AuditIssue::MissingTranslation("message2".into())));
    }

    #[test]
    fn detects_redundant_messages() {
        let canonical = qff(
            "en-AU",
            r#"
message1 = G'day en 1
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
message1 = Buongiorno it 1
message2 = Buongiorno it 2
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(!issues.contains(&AuditIssue::RedundantTranslation("message1".into())));
        assert!(issues.contains(&AuditIssue::RedundantTranslation("message2".into())));
    }

    #[test]
    fn detects_untranslated_terms() {
        let canonical = qff(
            "en-AU",
            r#"
-term1 = G'day en 1
-term2 = G'day en 2
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
-term1 = Buongiorno it 1
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(!issues.contains(&AuditIssue::MissingTranslation("-term1".into())));
        assert!(issues.contains(&AuditIssue::MissingTranslation("-term2".into())));
    }

    #[test]
    fn detects_redundant_terms() {
        let canonical = qff(
            "en-AU",
            r#"
-term1 = G'day en 1
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
-term1 = Buongiorno it 1
-term2 = Buongiorno it 2
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(!issues.contains(&AuditIssue::RedundantTranslation("-term1".into())));
        assert!(issues.contains(&AuditIssue::RedundantTranslation("-term2".into())));
    }

    #[test]
    fn detects_untranslated_attributes() {
        let canonical = qff(
            "en-AU",
            r#"
message1 =
    .hello = G'day en 1
    .world = World 1
message2 =
    .hello = G'day en 2
    .world = World 2
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
message1 =
    .hello = Buongiorno it 1
message2 =
    .hello = Buongiorno it 2
    .world = Mondo 2
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("message1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("message2".into())));
    }

    #[test]
    fn detects_redundant_attributes() {
        let canonical = qff(
            "en-AU",
            r#"
message1 =
    .hello = G'day en 1
message2 =
    .hello = G'day en 2
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
message1 =
    .hello = Buongiorno it 1
    .world = Mondo 1
message2 =
    .hello = Buongiorno it 2
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("message1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("message2".into())));
    }

    #[test]
    fn detects_untranslated_variants() {
        let canonical = qff(
            "en-AU",
            r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
emails1 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
emails2 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        [two] Hai due email non lette.
        *[other] Hai { $unreadEmails } email non lette.
    }
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("emails1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("emails2".into())));
    }

    #[test]
    fn detects_redundant_variants() {
        let canonical = qff(
            "en-AU",
            r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
emails1 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        [two] Hai due email non lette.
        *[other] Hai { $unreadEmails } email non lette.
    }
emails2 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("emails1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("emails2".into())));
    }

    #[test]
    fn detects_mismatched_default_variants() {
        let canonical = qff(
            "en-AU",
            r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
emails1 =
    { $unreadEmails ->
        *[one] Hai un'email non letta.
        [other] Hai { $unreadEmails } email non lette.
    }
emails2 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("emails1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("emails2".into())));
    }

    #[test]
    fn detects_mismatched_variables() {
        let canonical = qff(
            "en-AU",
            r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
        );
        let primary = qff(
            "it-IT",
            r#"
emails1 =
    { $readEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
emails2 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
"#,
        );

        let context = Context::canonical_to_primary(&canonical, &primary);
        let rule = TranslationIntegrityRule;
        let issues = rule.audit(&context);

        assert!(issues.contains(&AuditIssue::SignatureMismatch("emails1".into())));
        assert!(!issues.contains(&AuditIssue::SignatureMismatch("emails2".into())));
    }
}

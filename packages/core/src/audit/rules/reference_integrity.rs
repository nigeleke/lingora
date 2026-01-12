use std::collections::HashSet;

use super::emit_ordered;
use crate::audit::{AuditIssue, AuditRule, Context, ContextKind};

pub struct ReferenceIntegrityRule;

impl AuditRule for ReferenceIntegrityRule {
    fn applies_to(&self, context: &Context) -> bool {
        matches!(context.kind, ContextKind::Base)
    }

    fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        if let Some(file) = context.fluent_single() {
            let identifiers = file.identifiers().collect::<HashSet<_>>();
            let references = file.references().collect::<HashSet<_>>();

            emit_ordered(references.difference(&identifiers), |id| {
                issues.push(AuditIssue::InvalidReference(id.to_meta_string()))
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
    fn will_detect_invalid_message_references() {
        let file = qff(
            "en-GB",
            r#"
message1 = Message 1
message11 = { message1 }
message21 = { message2 }
"#,
        );

        let context = Context::base(&file);
        let rule = ReferenceIntegrityRule;
        let issues = rule.audit(&context);
        assert!(!issues.contains(&AuditIssue::InvalidReference("message1".into())));
        assert!(issues.contains(&AuditIssue::InvalidReference("message2".into())));
    }

    #[test]
    fn will_detect_invalid_term_references() {
        let file = qff(
            "en-GB",
            r#"
-term1 = Message 1
message11 = { -term1 }
message21 = { -term2 }
"#,
        );

        let context = Context::base(&file);
        let rule = ReferenceIntegrityRule;
        let issues = rule.audit(&context);
        assert!(!issues.contains(&AuditIssue::InvalidReference("-term1".into())));
        assert!(issues.contains(&AuditIssue::InvalidReference("-term2".into())));
    }

    #[test]
    fn will_detect_invalid_attribute_references() {
        let file = qff(
            "en-GB",
            r#"
message1 = Message 1
    .attr1 = Attribute 1.1
message11 = { message1.attr1 }
message12 = { message1.attr2 }
"#,
        );

        let context = Context::base(&file);
        let rule = ReferenceIntegrityRule;
        let issues = rule.audit(&context);
        assert!(!issues.contains(&AuditIssue::InvalidReference("message1 / .attr1".into())));
        assert!(issues.contains(&AuditIssue::InvalidReference("message1 / .attr2".into())));
    }
}

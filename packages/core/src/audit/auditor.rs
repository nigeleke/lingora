use crate::audit::{audit_rules::*, context::Context, issue::AuditIssue};

pub struct Auditor {
    rules: Vec<Box<dyn AuditRule>>,
}

impl Auditor {
    pub fn audit(&self, context: &Context) -> Vec<AuditIssue> {
        self.rules
            .iter()
            .filter(|r| r.applies_to(context))
            .fold(Vec::new(), |mut acc, r| {
                acc.extend(r.audit(context));
                acc
            })
    }
}

impl Default for Auditor {
    fn default() -> Self {
        Self {
            rules: vec![
                Box::new(BaseFilesProvidedRule),
                Box::new(BaseFilesUniqueRule),
                Box::new(NoOrphanLocalesRule),
                Box::new(ValidSyntaxRule),
                Box::new(IdentifierIntegrityRule),
                Box::new(ReferenceIntegrityRule),
                Box::new(TranslationIntegrityRule),
            ],
        }
    }
}

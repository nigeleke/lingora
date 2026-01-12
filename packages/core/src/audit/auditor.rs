use crate::audit::{
    context::Context,
    issue::AuditIssue,
    rules::{AuditRule, IdentifierIntegrityRule, TranslationIntegrityRule, ValidSyntaxRule},
};

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
                Box::new(ValidSyntaxRule),
                Box::new(IdentifierIntegrityRule),
                Box::new(TranslationIntegrityRule),
            ],
        }
    }
}

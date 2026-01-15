use crate::audit::{AuditIssue, context::Context};

pub trait AuditRule {
    fn applies_to(&self, context: &Context) -> bool;
    fn audit(&self, context: &Context) -> Vec<AuditIssue>;
}

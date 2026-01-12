use crate::audit::AuditIssue;

pub struct AuditReport(Vec<AuditIssue>);

impl AuditReport {
    pub fn new(issues: &[AuditIssue]) -> Self {
        Self(Vec::from(issues))
    }
}

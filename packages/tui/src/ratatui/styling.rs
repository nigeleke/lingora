use lingora_core::prelude::AuditResult;

use crate::ratatui::{FocusStyling, LocaleStyling, TextStyling};

pub struct Styling {
    pub locale: LocaleStyling,
    pub focus: FocusStyling,
    pub text: TextStyling,
}

impl Styling {
    pub fn from_audit_result(audit_result: &AuditResult) -> Self {
        Styling {
            locale: LocaleStyling::from_audit_result(audit_result),
            focus: FocusStyling::default(),
            text: TextStyling::default(),
        }
    }
}

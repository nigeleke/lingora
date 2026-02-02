use std::rc::Rc;

use lingora_core::prelude::{AuditResult, AuditedDocument};

use crate::projections::locales_hierarchy::LocaleNodeId;

#[derive(Debug)]
pub struct Comparison {
    audit_result: Rc<AuditResult>,
    reference: Option<LocaleNodeId>,
    reference_document: Option<AuditedDocument>,
    target: Option<LocaleNodeId>,
    target_document: Option<AuditedDocument>,
    count: i32,
}

impl Comparison {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        Self {
            audit_result,
            reference: Default::default(),
            reference_document: Default::default(),
            target: Default::default(),
            target_document: Default::default(),
            count: 0,
        }
    }

    pub fn update(
        &mut self,
        reference: Option<LocaleNodeId>,
        reference_document: Option<AuditedDocument>,
        target: Option<LocaleNodeId>,
        target_document: Option<AuditedDocument>,
    ) {
        if reference != self.reference || target != self.target {
            self.reference = reference;
            self.reference_document = reference_document;
            self.target = target;
            self.target_document = target_document;
            self.count += 1;
        }
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

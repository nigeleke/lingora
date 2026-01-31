pub use crate::projections::locales_hierarchy::LocaleNodeId;

#[derive(Debug, Default)]
pub struct Comparison {
    reference: Option<LocaleNodeId>,
    target: Option<LocaleNodeId>,
    pub result: i32,
}

impl Comparison {
    pub fn update(&mut self, reference: Option<LocaleNodeId>, target: Option<LocaleNodeId>) {
        if reference != self.reference || target != self.target {
            self.reference = reference;
            self.target = target;
            self.result += 1
        }
    }
}

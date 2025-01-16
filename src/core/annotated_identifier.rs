use super::{identifier::Identifier, identifier_origin::IdentifierOrigin};

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotatedIdentifier {
    identifier: Identifier,
    origins: HashSet<IdentifierOrigin>,
}

impl AnnotatedIdentifier {
    pub fn from<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            origins: HashSet::new(),
        }
    }

    pub fn with_origin(mut self, origin: IdentifierOrigin) -> Self {
        self.origins.insert(origin);
        self
    }

    pub fn name(&self) -> String {
        self.identifier.to_string()
    }

    pub fn css_class(&self) -> String {
        let is_reference = self.origins.contains(&IdentifierOrigin::Reference);
        let is_target = self.origins.contains(&IdentifierOrigin::Target);
        let is_target_fallback = self.origins.contains(&IdentifierOrigin::TargetFallback);
        match (is_target_fallback, is_target, is_reference) {
            (false, false, false) => "",
            (false, false, true) => "missing-target",
            (false, true, false) => "superfluous-target",
            (false, true, true) => "ok",
            (true, false, false) => "superfluous-target-fallback",
            (true, false, true) => "ok-uses-target-fallback",
            (true, true, false) => "superfluous-target",
            (true, true, true) => "ok",
        }
        .into()
    }
}

impl PartialOrd for AnnotatedIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.identifier.partial_cmp(&other.identifier)
    }
}

impl Ord for AnnotatedIdentifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

use super::{
    identifier::Identifier, identifier_origin::IdentifierOrigin, prelude::AnnotatedIdentifierState,
};

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

    pub fn state(&self) -> AnnotatedIdentifierState {
        let is_reference = self.origins.contains(&IdentifierOrigin::Reference);
        let is_target = self.origins.contains(&IdentifierOrigin::Target);
        let is_target_fallback = self.origins.contains(&IdentifierOrigin::TargetFallback);
        match (is_target_fallback, is_target, is_reference) {
            (false, false, false) => unreachable!(),
            (false, false, true) => AnnotatedIdentifierState::MissingTarget,
            (false, true, false) => AnnotatedIdentifierState::SuperfluousTarget,
            (false, true, true) => AnnotatedIdentifierState::Ok,
            (true, false, false) => AnnotatedIdentifierState::SuperfluousTargetFallback,
            (true, false, true) => AnnotatedIdentifierState::OkUsingTargetFallback,
            (true, true, false) => AnnotatedIdentifierState::SuperfluousTarget,
            (true, true, true) => AnnotatedIdentifierState::Ok,
        }
    }
}

impl PartialOrd for AnnotatedIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnnotatedIdentifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

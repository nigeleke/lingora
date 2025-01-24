use super::prelude::{AnnotatedIdentifierState, EntryOrigin, Identifier};

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct AnnotatedIdentifier {
    identifier: Identifier,
    origins: HashSet<EntryOrigin>,
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

    pub fn with_origin(mut self, origin: EntryOrigin) -> Self {
        self.origins.insert(origin);
        self
    }

    pub fn name(&self) -> String {
        self.identifier.to_string()
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn state(&self) -> AnnotatedIdentifierState {
        let is_reference = self.origins.contains(&EntryOrigin::Reference);
        let is_target = self.origins.contains(&EntryOrigin::Target);
        let is_target_fallback = self.origins.contains(&EntryOrigin::TargetFallback);
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

impl PartialEq for AnnotatedIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Eq for AnnotatedIdentifier {}

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

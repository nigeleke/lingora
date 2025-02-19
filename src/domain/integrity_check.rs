use std::collections::{HashMap, HashSet};

use fluent4rs::prelude::*;

use super::integrity_warning::IntegrityWarning;

pub struct IntegrityCheck(Vec<IntegrityWarning>);

impl IntegrityCheck {
    pub fn warnings(&self) -> &[IntegrityWarning] {
        self.0.as_slice()
    }
}

impl From<&Resource> for IntegrityCheck {
    fn from(value: &Resource) -> Self {
        let mut visitor = EntryVisitor::default();
        Walker::walk(value, &mut visitor);

        let conflicting_names: HashSet<String> = visitor
            .identifier_names
            .iter()
            .filter_map(|(id, count)| (*count > 1).then_some(String::from(id)))
            .collect::<HashSet<_>>();

        let ambiguous_names = visitor
            .identifiers
            .iter()
            .filter_map(|(id, count)| (*count > 1).then_some(id.to_string()))
            .filter(|s| !conflicting_names.contains(s))
            .filter(|s| !conflicting_names.contains(&format!("-{s}")));

        let invalid_message_references = visitor
            .message_references
            .iter()
            .filter(|id| !visitor.identifiers.contains_key(id));

        let invalid_term_references = visitor
            .term_references
            .iter()
            .filter(|id| !visitor.identifiers.contains_key(id));

        type IW = IntegrityWarning;

        let warnings = conflicting_names
            .iter()
            .map(|s| IW::IdentifierConflict(s.to_string()))
            .chain(ambiguous_names.map(IW::MessageTermConflict))
            .chain(invalid_message_references.map(|s| IW::InvalidMessageReference(s.to_string())))
            .chain(invalid_term_references.map(|s| IW::InvalidTermReference(s.to_string())))
            .collect::<Vec<_>>();

        Self(warnings)
    }
}

type IdentifierNameMap = HashMap<String, usize>;
type IdentifierMap = HashMap<Identifier, usize>;
type IdentifierSet = HashSet<Identifier>;

#[derive(Default)]
struct EntryVisitor {
    identifier_names: IdentifierNameMap,
    identifiers: IdentifierMap,
    message_references: IdentifierSet,
    term_references: IdentifierSet,
}

impl Visitor for EntryVisitor {
    fn visit_message(&mut self, message: &Message) {
        *self
            .identifier_names
            .entry(message.identifier_name())
            .or_insert(0) += 1;
        *self
            .identifiers
            .entry(message.identifier().to_owned())
            .or_insert(0) += 1;
    }

    fn visit_term(&mut self, term: &Term) {
        *self
            .identifier_names
            .entry(term.identifier_name())
            .or_insert(0) += 1;
        *self
            .identifiers
            .entry(term.identifier().to_owned())
            .or_insert(0) += 1;
    }

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        self.message_references
            .insert(reference.identifier().to_owned());
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        self.term_references
            .insert(reference.identifier().to_owned());
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::domain::FluentFile;

    #[test]
    fn no_integrity_issues_in_valid_file() {
        let file = PathBuf::from("tests/data/fluent_file/valid.ftl");
        let file = FluentFile::try_from(&file).expect("accessible test file");
        let resource = file.resource();
        let check = IntegrityCheck::from(resource);
        assert!(check.warnings().is_empty());
    }

    #[test]
    fn duplicated_identifier() {
        let file = PathBuf::from("tests/data/fluent_file/duplicated_identifier.ftl");
        let file = FluentFile::try_from(&file).expect("accessible test file");
        let resource = file.resource();
        let check = IntegrityCheck::from(resource);
        let warnings = check.warnings();
        assert_eq!(warnings.len(), 4);

        let expected_warnings = [
            IntegrityWarning::MessageTermConflict("duplicated-identifier1".into()),
            IntegrityWarning::IdentifierConflict("duplicated-identifier2".into()),
            IntegrityWarning::IdentifierConflict("duplicated-message".into()),
            IntegrityWarning::IdentifierConflict("-duplicated-term".into()),
        ];
        assert!(expected_warnings.iter().all(|ew| warnings.contains(ew)));
    }

    #[test]
    fn invalid_references() {
        let file = PathBuf::from("tests/data/fluent_file/invalid_references.ftl");
        let file = FluentFile::try_from(&file).expect("accessible test file");
        let resource = file.resource();
        let check = IntegrityCheck::from(resource);
        let warnings = check.warnings();
        assert_eq!(warnings.len(), 2);

        let expected_warnings = [
            IntegrityWarning::InvalidMessageReference("message-none".into()),
            IntegrityWarning::InvalidTermReference("term-none".into()),
        ];
        assert!(expected_warnings.iter().all(|ew| warnings.contains(ew)));
    }
}

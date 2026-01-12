use std::collections::HashMap;

use fluent4rs::{ast::*, prelude::*};

use crate::domain::IntegrityWarning;

impl CrossCheck {
    pub fn new(reference: &Resource, target: &Resource) -> Self {
        let mut reference_visitor = IdentifierUsageVisitor::default();
        Walker::walk(reference, &mut reference_visitor);

        let mut target_visitor = IdentifierUsageVisitor::default();
        Walker::walk(target, &mut target_visitor);

        let reference_names = reference_visitor.identifier_names();
        let missing = reference_names
            .iter()
            .filter(|id| !target_visitor.identifier_names().contains(id));

        let target_names = target_visitor.identifier_names();
        let superfluous = target_names
            .iter()
            .filter(|id| !reference_visitor.identifier_names().contains(id));

        let warnings = missing
            .map(|s| IntegrityWarning::MissingTranslation(s.to_owned()))
            .chain(superfluous.map(|s| IntegrityWarning::SuperfluousTranslation(s.to_owned())))
            .collect::<Vec<_>>();

        Self(warnings)
    }

    pub fn warnings(&self) -> &[IntegrityWarning] {
        self.0.as_slice()
    }
}

type IdentifierNameMap = HashMap<String, Entry>;

#[derive(Default)]
struct IdentifierUsageVisitor {
    entries: IdentifierNameMap,
}

impl IdentifierUsageVisitor {
    fn identifier_names(&self) -> Vec<String> {
        self.entries.keys().map(String::from).collect::<Vec<_>>()
    }

    fn add_attribute_identifiers(&mut self, entry: &Entry) {
        let (parent_name, attributes) = match entry {
            Entry::Message(message) => (message.identifier_name(), message.attributes()),
            Entry::Term(term) => (term.identifier_name(), term.attributes()),
            Entry::CommentLine(_) => unreachable!(),
        };

        for attribute in attributes {
            let name = format!("{}{}", parent_name, attribute.identifier_name());
            self.entries.insert(name, entry.to_owned());
        }
    }
}

impl Visitor for IdentifierUsageVisitor {
    fn visit_entry(&mut self, entry: &Entry) {
        match entry {
            Entry::Message(message) => {
                self.entries
                    .insert(message.identifier_name(), entry.to_owned());
                self.add_attribute_identifiers(entry);
            }
            Entry::Term(term) => {
                self.entries
                    .insert(term.identifier_name(), entry.to_owned());
                self.add_attribute_identifiers(entry);
            }
            Entry::CommentLine(_) => {}
        }
    }

    fn visit_attribute(&mut self, _attribute: &Attribute) {}
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::domain::FluentFile;

    fn cross_check_for(reference: &str, target: &str) -> CrossCheck {
        let reference = PathBuf::from(reference);
        let reference = FluentFile::try_from(reference.as_path()).expect("accessible test file");
        let reference = reference.resource();

        let target = PathBuf::from(target);
        let target = FluentFile::try_from(target.as_path()).expect("accessible test file");
        let target = target.resource();

        CrossCheck::new(reference, target)
    }

    #[test]
    fn will_not_report_matching_entrys() {
        let warnings = cross_check_for(
            "tests/data/cross_check/reference_matching.ftl",
            "tests/data/cross_check/target_matching.ftl",
        );

        let warnings = warnings.warnings();
        assert!(warnings.is_empty());
    }

    #[test]
    fn will_detect_missing_entry_in_target() {
        let warnings = cross_check_for(
            "tests/data/cross_check/reference_missing.ftl",
            "tests/data/cross_check/target_matching.ftl",
        );

        let warnings = warnings.warnings();
        assert_eq!(warnings.len(), 2);

        let expected_warnings = [
            IntegrityWarning::MissingTranslation("missing-message".into()),
            IntegrityWarning::MissingTranslation("-missing-term".into()),
        ];
        assert!(expected_warnings.iter().all(|ew| warnings.contains(ew)));
    }

    #[test]
    fn will_detect_superfluous_entry_in_target() {
        let warnings = cross_check_for(
            "tests/data/cross_check/reference_matching.ftl",
            "tests/data/cross_check/target_superfluous.ftl",
        );

        let warnings = warnings.warnings();
        assert_eq!(warnings.len(), 2);

        let expected_warnings = [
            IntegrityWarning::SuperfluousTranslation("superfluous-message".into()),
            IntegrityWarning::SuperfluousTranslation("-superfluous-term".into()),
        ];
        assert!(expected_warnings.iter().all(|ew| warnings.contains(ew)));
    }
}

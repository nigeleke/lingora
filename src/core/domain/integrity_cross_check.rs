use super::integrity_warning::IntegrityWarning;

use fluent4rs::prelude::*;

use std::collections::HashMap;

// #[derive(Clone, Debug)]
pub struct IntegrityCrossCheck(Vec<IntegrityWarning>);

impl IntegrityCrossCheck {
    pub fn new(reference: &Resource, target: &Resource) -> Self {
        let mut reference_visitor = EntryVisitor::default();
        Walker::walk(reference, &mut reference_visitor);

        let mut target_visitor = EntryVisitor::default();
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
struct EntryVisitor {
    entries: IdentifierNameMap,
}

impl EntryVisitor {
    fn identifier_names(&self) -> Vec<String> {
        self.entries.keys().map(String::from).collect::<Vec<_>>()
    }
}

impl Visitor for EntryVisitor {
    fn visit_entry(&mut self, entry: &Entry) {
        match entry {
            Entry::Message(message) => {
                self.entries
                    .insert(message.identifier_name(), entry.to_owned());
            }
            Entry::Term(term) => {
                self.entries
                    .insert(term.identifier_name(), entry.to_owned());
            }
            Entry::CommentLine(_) => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::domain::FluentFile;
    use std::path::PathBuf;

    fn cross_check_for(reference: &str, target: &str) -> IntegrityCrossCheck {
        let reference = PathBuf::from(reference);
        let reference = FluentFile::try_from(&reference).expect("accessible test file");
        let reference = reference.resource();

        let target = PathBuf::from(target);
        let target = FluentFile::try_from(&target).expect("accessible test file");
        let target = target.resource();

        IntegrityCrossCheck::new(&reference, &target)
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

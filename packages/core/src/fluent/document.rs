use std::cell::OnceCell;

use fluent4rs::{ast::*, prelude::Walker};

use crate::{
    domain::{HasLocale, Locale},
    fluent::{Definitions, ParsedFluentFile, QualifiedIdentifier, Signature},
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FluentDocument {
    locale: Locale,
    resource: Resource,
    analysis: OnceCell<Definitions>,
}

impl FluentDocument {
    pub fn from_parsed_files(locale: &Locale, files: &[ParsedFluentFile]) -> Self {
        let entries = files
            .iter()
            .filter(|f| f.locale() == locale)
            .filter_map(|f| f.resource())
            .flat_map(|r| r.entries().into_iter().cloned())
            .collect::<Vec<_>>();

        let locale = locale.clone();
        let resource = Resource::from(entries);
        let analysis = OnceCell::default();

        Self {
            locale,
            resource,
            analysis,
        }
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    fn definitions(&self) -> &Definitions {
        self.analysis.get_or_init(|| {
            let mut analysis = Definitions::default();
            Walker::walk(&self.resource, &mut analysis);
            analysis
        })
    }

    pub fn entry_identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().entry_identifiers()
    }

    pub fn duplicate_identifier_names(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().duplicate_identifiers().into_iter()
    }

    pub fn invalid_references(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().invalid_references().into_iter()
    }

    pub fn signature(&self, identifier: &QualifiedIdentifier) -> Option<&Signature> {
        self.definitions().signature(identifier)
    }

    pub fn entries(&self, identifier: &QualifiedIdentifier) -> impl Iterator<Item = &Entry> {
        self.definitions().entries(identifier)
    }
}

impl HasLocale for FluentDocument {
    fn locale(&self) -> &Locale {
        &self.locale
    }
}

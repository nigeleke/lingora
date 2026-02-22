use std::cell::OnceCell;

use fluent4rs::{ast::*, prelude::Walker};

use crate::{
    domain::{HasLocale, Locale},
    fluent::{Definitions, ParsedFluentFile, QualifiedIdentifier, Signature},
};

/// A normalized, representation of all Fluent translations for **one locale**.
///
/// `FluentDocument` aggregates entries from one or more `ParsedFluentFile`s belonging
/// to the same locale (e.g. split files like `en-GB/auth.ftl`, `en-GB/ui.ftl`, `en-GB/errors.ftl` all for `en-GB`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FluentDocument {
    locale: Locale,
    resource: Resource,
    analysis: OnceCell<Definitions>,
}

impl FluentDocument {
    /// Constructs a `FluentDocument` by merging entries from all `ParsedFluentFile`s
    /// that match the given locale.
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

    /// Returns the locale this document represents.
    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    /// Returns a reference to the `Definitions` analysis.
    fn definitions(&self) -> &Definitions {
        self.analysis.get_or_init(|| {
            let mut analysis = Definitions::default();
            Walker::walk(&self.resource, &mut analysis);
            analysis
        })
    }

    /// Returns an iterator over all **top-level** qualified identifiers defined in this document.
    pub fn entry_identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().entry_identifiers()
    }

    /// Returns an iterator over **all** identifiers used or defined (including placeholders
    /// and references inside patterns).
    pub fn all_identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().all_identifiers()
    }

    /// Returns an iterator over identifiers that appear more than once in the document.
    pub fn duplicate_identifier_names(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().duplicate_identifiers().into_iter()
    }

    /// Returns an iterator over identifiers that are referenced, e.g. `{ $var }`,
    /// but **not defined** elsewhere in the document.
    pub fn invalid_references(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().invalid_references().into_iter()
    }

    /// Returns the placeholder signature (arguments/variables) for the given identifier,
    /// if it is defined in this document.
    pub fn signature(&self, identifier: &QualifiedIdentifier) -> Option<&Signature> {
        self.definitions().signature(identifier)
    }

    /// Returns an iterator over all AST `Entry` nodes that define the given identifier.
    pub fn entries(&self, identifier: &QualifiedIdentifier) -> impl Iterator<Item = &Entry> {
        self.definitions().entries(identifier)
    }
}

impl HasLocale for FluentDocument {
    fn locale(&self) -> &Locale {
        &self.locale
    }
}

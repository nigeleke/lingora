use std::collections::HashSet;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::FluentFile,
    rust::RustFile,
};

/// The complete set of files and configuration used as input to an audit.
///
/// A `Workspace` gathers:
/// - All discovered Fluent translation files (`.ftl`)
/// - The designated canonical (reference) locale
/// - Explicitly configured primary locales
/// - Rust source files to scan for `dioxus_i18n`
#[derive(Clone, Debug)]
pub struct Workspace {
    fluent_files: Vec<FluentFile>,
    canonical: Locale,
    primaries: Vec<Locale>,
    rust_files: Vec<RustFile>,
}

impl Workspace {
    /// Creates a new `Workspace` from its constituent parts.
    pub fn new(
        fluent_files: Vec<FluentFile>,
        canonical: Locale,
        primaries: Vec<Locale>,
        rust_files: Vec<RustFile>,
    ) -> Self {
        Self {
            fluent_files,
            canonical,
            primaries,
            rust_files,
        }
    }

    /// Returns a slice of all Fluent files in the workspace.
    pub fn fluent_files(&self) -> &[FluentFile] {
        &self.fluent_files
    }

    /// Returns an iterator over all **unique** locales found in the Fluent files.
    pub fn locales(&self) -> impl Iterator<Item = &Locale> {
        self.fluent_files
            .iter()
            .map(|f| f.locale())
            .collect::<HashSet<_>>()
            .into_iter()
    }

    /// Returns an iterator over all unique **language roots** present in the workspace.
    pub fn language_roots(&self) -> impl Iterator<Item = LanguageRoot> {
        self.fluent_files
            .iter()
            .map(|f| LanguageRoot::from(f.locale()))
            .collect::<HashSet<_>>()
            .into_iter()
    }

    /// Returns an iterator over all locales that belong to the given language root.
    pub fn locales_by_language_root(&self, root: &LanguageRoot) -> impl Iterator<Item = &Locale> {
        self.fluent_files.iter().filter_map(|f| {
            let locale = f.locale();
            (LanguageRoot::from(locale) == *root).then_some(locale)
        })
    }

    /// Returns an iterator over all Fluent files for the exact given locale,
    /// sorted by file path (stable order).
    pub fn fluent_files_by_locale(&self, locale: &Locale) -> impl Iterator<Item = &FluentFile> {
        let mut files = self
            .fluent_files()
            .iter()
            .filter(|f| f.locale() == locale)
            .collect::<Vec<_>>();
        files.sort_by_key(|f| f.path());
        files.into_iter()
    }

    /// Returns the configured canonical (reference) locale.
    pub fn canonical_locale(&self) -> &Locale {
        &self.canonical
    }

    /// Returns `true` if the given locale is the canonical one.
    pub fn is_canonical_locale(&self, locale: &Locale) -> bool {
        locale == self.canonical_locale()
    }

    /// Returns an iterator over all explicitly configured primary locales.
    pub fn primary_locales(&self) -> impl Iterator<Item = &Locale> {
        self.primaries.iter()
    }

    /// Returns `true` if the given locale is one of the configured primaries.
    pub fn is_primary_locale(&self, locale: &Locale) -> bool {
        self.primary_locales().any(|p| p == locale)
    }

    /// Returns an iterator over all **base** locales — i.e. canonical + primaries.
    pub fn base_locales(&self) -> impl Iterator<Item = &Locale> {
        self.primaries
            .iter()
            .chain(std::iter::once(&self.canonical))
    }

    /// Returns `true` if the given locale is a base locale (canonical or primary).
    pub fn is_base_locale(&self, locale: &Locale) -> bool {
        self.base_locales().any(|p| p == locale)
    }

    /// Returns an iterator over all **variant** locales for the given base locale.
    pub fn variant_locales(&self, base: &Locale) -> impl Iterator<Item = &Locale> {
        let base_root = LanguageRoot::from(base);
        self.fluent_files.iter().filter_map(move |f| {
            let locale = f.locale();
            let root = LanguageRoot::from(locale);
            (base_root == root && base != locale).then_some(locale)
        })
    }

    /// Returns an iterator over all **orphan** locales.
    pub fn orphan_locales(&self) -> impl Iterator<Item = &Locale> {
        let base_roots = Vec::from_iter(self.base_locales().map(LanguageRoot::from));
        self.fluent_files.iter().filter_map(move |f| {
            let locale = f.locale();
            let root = LanguageRoot::from(locale);
            (!base_roots.contains(&root)).then_some(locale)
        })
    }

    /// Returns `true` if the given locale is an orphan (no matching base root).
    pub fn is_orphan_locale(&self, locale: &Locale) -> bool {
        self.orphan_locales().any(|p| p == locale)
    }

    /// Returns a slice of all Rust files included in the workspace.
    pub fn rust_files(&self) -> &[RustFile] {
        &self.rust_files
    }
}

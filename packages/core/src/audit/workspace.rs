use std::collections::BTreeSet;

use crate::{
    domain::{LanguageRoot, Locale},
    fluent::QualifiedFluentFile,
    rust::RustFile,
};

#[derive(Clone, Debug)]
pub struct Workspace {
    fluent_files: Vec<QualifiedFluentFile>,
    canonical: Locale,
    primaries: Vec<Locale>,
    rust_files: Vec<RustFile>,
}

impl Workspace {
    pub fn new(
        fluent_files: &[QualifiedFluentFile],
        canonical: Locale,
        primaries: &[Locale],
        rust_files: &[RustFile],
    ) -> Self {
        let fluent_files = Vec::from(fluent_files);
        let primaries = Vec::from(primaries);
        let rust_files = Vec::from(rust_files);

        Self {
            fluent_files,
            canonical,
            primaries,
            rust_files,
        }
    }

    pub fn fluent_files(&self) -> &[QualifiedFluentFile] {
        &self.fluent_files
    }

    pub fn language_roots(&self) -> impl Iterator<Item = LanguageRoot> {
        let roots = self
            .fluent_files
            .iter()
            .map(|f| LanguageRoot::from(f.locale()))
            .collect::<BTreeSet<_>>();
        roots.into_iter()
    }

    pub fn locales_by_language_root(&self, root: &LanguageRoot) -> impl Iterator<Item = &Locale> {
        let locales = self
            .fluent_files
            .iter()
            .filter_map(|f| {
                let locale = f.locale();
                (&LanguageRoot::from(locale) == root).then_some(locale)
            })
            .collect::<BTreeSet<_>>();
        locales.into_iter()
    }

    pub fn fluent_files_by_locale(
        &self,
        locale: &Locale,
    ) -> impl Iterator<Item = &QualifiedFluentFile> {
        let mut files = self
            .fluent_files()
            .iter()
            .filter_map(|f| (f.locale() == locale).then_some(f))
            .collect::<Vec<_>>();
        files.sort_by_key(|f| f.path());
        files.into_iter()
    }

    pub fn canonical_locale(&self) -> &Locale {
        &self.canonical
    }

    pub fn primary_locales(&self) -> impl Iterator<Item = &Locale> {
        self.primaries.iter()
    }

    pub fn base_locales(&self) -> impl Iterator<Item = &Locale> {
        self.primaries
            .iter()
            .chain(std::iter::once(&self.canonical))
    }

    pub fn variant_locales(&self, base: &Locale) -> impl Iterator<Item = &Locale> {
        let base_root = LanguageRoot::from(base);
        self.fluent_files.iter().filter_map(move |f| {
            let locale = f.locale();
            let root = LanguageRoot::from(locale);
            (base_root == root && base != locale).then_some(locale)
        })
    }

    pub fn orphan_locales(&self) -> impl Iterator<Item = &Locale> {
        let base_roots = Vec::from_iter(self.base_locales().map(LanguageRoot::from));
        self.fluent_files.iter().filter_map(move |f| {
            let locale = f.locale();
            let root = LanguageRoot::from(locale);
            (!base_roots.contains(&root)).then_some(locale)
        })
    }

    pub fn rust_files(&self) -> &[RustFile] {
        &self.rust_files
    }
}

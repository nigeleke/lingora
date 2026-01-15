use crate::{domain::Locale, fluent::QualifiedFluentFile, rust::RustFile};

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

    pub fn rust_files(&self) -> &[RustFile] {
        &self.rust_files
    }
}

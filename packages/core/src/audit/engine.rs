use std::{ffi::OsStr, path::PathBuf};

use walkdir::WalkDir;

use crate::{
    audit::{AuditResult, Pipeline, Workspace},
    config::LingoraToml,
    error::LingoraError,
    fluent::FluentFile,
    rust::RustFile,
};

/// The main engine that drives the Lingora audit process.
///
/// `AuditEngine` is responsible for:
/// 1. Discovering Fluent (`.ftl`) and Rust (`.rs`) files from configured paths
/// 2. Building a `Workspace` model
/// 3. Running the analysis `Pipeline` (parsing → document collection → classification → auditing)
/// 4. Returning a complete `AuditResult` containing issues and classified documents
#[derive(Debug)]
pub struct AuditEngine {
    workspace: Workspace,
}

impl AuditEngine {
    /// Executes the full audit pipeline and returns the result.
    ///
    /// Steps performed (via `Pipeline`):
    /// - Parse all Fluent and Rust files
    /// - Aggregate entries into `FluentDocument`s per locale
    /// - Classify documents as Canonical / Primary / Variant / Orphan
    /// - Compare canonical vs targets (missing keys, redundants, signatures, etc.)
    /// - Validate Rust macro usage against canonical identifiers
    ///
    /// Returns `Ok(AuditResult)` on success, even if issues are found (use `AuditResult::is_ok()` to check cleanliness).
    pub fn run(&self) -> Result<AuditResult, LingoraError> {
        let workspace = &self.workspace;

        let fluent_files = workspace.fluent_files();
        let rust_files = workspace.rust_files();

        let canonical_locale = workspace.canonical_locale();
        let primary_locales = Vec::from_iter(workspace.primary_locales().cloned());

        let audit_result = Pipeline::default()
            .parse_files(fluent_files, rust_files)?
            .collect_documents_by_locale()
            .classify_documents(canonical_locale, &primary_locales)
            .audit()
            .get_result(workspace);

        Ok(audit_result)
    }
}

impl TryFrom<&LingoraToml> for AuditEngine {
    type Error = LingoraError;

    fn try_from(settings: &LingoraToml) -> Result<Self, Self::Error> {
        let fluent_files = collate_fluent_files(&settings.lingora.fluent_sources)?;

        let canonical = settings.lingora.canonical.clone();
        let primaries = settings.lingora.primaries.clone();

        let rust_files = collate_rust_files(&settings.dioxus_i18n.rust_sources)?;

        let workspace = Workspace::new(fluent_files, canonical, primaries, rust_files);

        Ok(AuditEngine { workspace })
    }
}

fn collate_fluent_files(fluent_paths: &[PathBuf]) -> Result<Vec<FluentFile>, LingoraError> {
    collate_files(fluent_paths, "ftl")
        .map(|p| FluentFile::try_from(p.as_path()))
        .collect()
}

fn collate_rust_files(rust_paths: &[PathBuf]) -> Result<Vec<RustFile>, LingoraError> {
    collate_files(rust_paths, "rs")
        .map(|p| RustFile::try_from(p.as_path()))
        .collect()
}

fn collate_files(paths: &[PathBuf], ext: &str) -> impl Iterator<Item = PathBuf> {
    let ext = Some(OsStr::new(ext));
    paths
        .iter()
        .fold(Vec::new(), |mut acc, path| {
            if path.is_file() && path.extension() == ext {
                acc.push(path.clone());
            } else if path.is_dir() {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter_map(|e| {
                        (e.file_type().is_file() && e.path().extension() == ext)
                            .then_some(e.path().to_path_buf())
                    })
                    .for_each(|p| acc.push(p));
            };

            acc
        })
        .into_iter()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;

    #[test]
    fn fluent_files_will_be_collated_from_provided_paths() {
        let paths = &[Path::new("./tests/data/i18n").to_path_buf()];
        let files = collate_fluent_files(paths).unwrap();

        let expected_files = [
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/fr/fr-FR.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cyrl/sr-Cyrl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cyrl/sr-Cyrl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| FluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        assert_eq!(files.len(), expected_files.len());

        expected_files
            .iter()
            .for_each(|f| assert!(files.contains(f)));
    }

    #[test]
    fn audit_engine_should_produce_a_report() {
        let toml = LingoraToml::try_from(Path::new("./tests/data/toml/Lingora_audit_engine.toml"))
            .unwrap();
        let engine = AuditEngine::try_from(&toml).unwrap();
        assert!(engine.run().is_ok());
    }
}

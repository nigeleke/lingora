use std::path::PathBuf;

use walkdir::WalkDir;

use crate::{
    audit::{AuditReport, AuditReportContext, Auditor, Context, Workspace},
    config::LingoraToml,
    domain::LanguageRoot,
    error::LingoraError,
    fluent::{FluentFile, QualifiedFluentFile},
    rust::RustFile,
};

#[derive(Debug)]
pub struct AuditEngine {
    workspace: Workspace,
}

impl AuditEngine {
    pub fn run(&self) -> Result<AuditReport, LingoraError> {
        let workspace = &self.workspace;

        let auditor = Auditor::default();
        let mut issues = Vec::new();

        let workspace_context = Context::new_workspace_context(workspace.clone());
        issues.extend(auditor.audit(&workspace_context));

        let contexts = self.contexts();
        let issues = contexts.iter().fold(Vec::new(), |mut acc, c| {
            let issues = auditor.audit(c);
            acc.extend(issues);
            acc
        });

        let report_context = AuditReportContext::new(
            workspace.canonical_locale(),
            &Vec::from_iter(workspace.primary_locales().cloned()),
        );
        Ok(AuditReport::new(&issues, report_context))
    }

    fn contexts(&self) -> Vec<Context> {
        let workspace = &self.workspace;

        let all_file_contexts = workspace
            .fluent_files()
            .iter()
            .map(|f| Context::new_all_context(f.clone()));

        let parsed_files = workspace
            .fluent_files()
            .iter()
            .filter(|f| f.is_well_formed())
            .collect::<Vec<_>>();

        let canonical_file = parsed_files
            .iter()
            .find(|f| f.locale() == workspace.canonical_locale());
        let primary_locales = Vec::from_iter(workspace.primary_locales());
        let primary_files = parsed_files
            .iter()
            .filter(|f| primary_locales.contains(&f.locale()));

        let base_files = canonical_file.into_iter().chain(primary_files.clone());
        let base_contexts = base_files
            .clone()
            .map(|f| Context::new_base_context((*f).clone()));

        let canonical_contexts = canonical_file.into_iter().flat_map(|canonical_file| {
            let canonical_to_primary = primary_files.clone().map(move |primary| {
                Context::new_canonical_to_primary_context(
                    (*canonical_file).clone(),
                    (*primary).clone(),
                )
            });

            let rust_to_canonical = workspace.rust_files().iter().map(move |f| {
                Context::new_rust_to_canonical_context(f.clone(), (*canonical_file).clone())
            });

            canonical_to_primary.chain(rust_to_canonical)
        });

        let variant_contexts = base_files.flat_map(|base| {
            let base_root = LanguageRoot::from(base.locale());

            parsed_files.iter().filter_map(move |variant| {
                let variant_root = LanguageRoot::from(variant.locale());
                (base != variant && base_root == variant_root).then_some(
                    //
                    Context::new_base_to_variant_context((*base).clone(), (*variant).clone()),
                )
            })
        });

        all_file_contexts
            .chain(base_contexts)
            .chain(canonical_contexts)
            .chain(variant_contexts)
            .collect()
    }
}

impl TryFrom<&LingoraToml> for AuditEngine {
    type Error = LingoraError;

    fn try_from(settings: &LingoraToml) -> Result<Self, Self::Error> {
        let fluent_files = collate_fluent_files(&settings.lingora.fluent_sources)?;

        let canonical = settings.lingora.canonical.clone();
        let primaries = settings.lingora.primaries.clone();

        let rust_files = collate_rust_files(&settings.dioxus_i18n.rust_sources)?;

        let workspace = Workspace::new(&fluent_files, canonical, &primaries, &rust_files);

        Ok(AuditEngine { workspace })
    }
}

fn collate_fluent_files(
    fluent_paths: &[PathBuf],
) -> Result<Vec<QualifiedFluentFile>, LingoraError> {
    fluent_paths
        .iter()
        .try_fold(Vec::new(), |mut acc, path| {
            if path.is_file() {
                let fluent = FluentFile::try_from(path.as_path())?;
                acc.push(fluent);
            } else if path.is_dir() {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file())
                    .for_each(|entry| {
                        if let Ok(fluent) = FluentFile::try_from(entry.path()) {
                            acc.push(fluent);
                        }
                    });
            }
            Ok(acc)
        })
        .map(|files| files.into_iter().map(QualifiedFluentFile::from).collect())
}

fn collate_rust_files(rust_sources: &[PathBuf]) -> Result<Vec<RustFile>, LingoraError> {
    rust_sources.iter().try_fold(Vec::new(), |mut acc, path| {
        if path.is_file() {
            if let Ok(rust) = RustFile::try_from(path.as_path()) {
                acc.push(rust);
            }
        } else if path.is_dir() {
            WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
                .for_each(|entry| {
                    if let Ok(rust) = RustFile::try_from(entry.path()) {
                        acc.push(rust);
                    }
                });
        }
        Ok(acc)
    })
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
            Path::new("./tests/data/i18n/en/en.ftl"),
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/fr/fr-FR.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualifiedFluentFile::try_from(p).unwrap())
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
        assert!(matches!(engine.run(), Ok(_)));
    }
}

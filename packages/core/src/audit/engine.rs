use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use walkdir::WalkDir;

use crate::{
    audit::{AuditReport, Auditor, Context},
    config::LingoraToml,
    domain::{LanguageRoot, Locale},
    error::LingoraError,
    fluent::{FluentFile, QualfiedFluentFile},
    rust::RustFile,
};

#[derive(Debug)]
pub struct AuditEngine {
    files: Vec<QualfiedFluentFile>,
    language_roots: HashSet<LanguageRoot>,
    canonical: Locale,
    primaries: Vec<Locale>,
    rust_files: Vec<RustFile>,
}

impl AuditEngine {
    pub fn run(&self) -> Result<AuditReport, LingoraError> {
        let auditor = Auditor::default();
        let contexts = self.contexts();

        let issues = contexts.iter().fold(Vec::new(), |mut acc, c| {
            let issues = auditor.audit(c);
            acc.extend(issues);
            acc
        });

        Ok(AuditReport::new(&issues))
    }

    fn contexts(&self) -> Vec<Context<'_>> {
        let all_file_contexts = self.files.iter().map(|f| Context::all(f));

        let parsed_files = self
            .files
            .iter()
            .filter(|f| f.document.is_ok())
            .collect::<Vec<_>>();

        let canonical_file = parsed_files.iter().find(|f| f.locale() == &self.canonical);
        let primary_files = parsed_files
            .iter()
            .filter(|f| self.primaries.contains(f.locale()));

        let base_files = canonical_file.into_iter().chain(primary_files.clone());
        let base_contexts = base_files.clone().map(|f| Context::base(f));

        let canonical_contexts = canonical_file.into_iter().flat_map(|canonical_file| {
            let canonical_to_primary = primary_files
                .clone()
                .map(move |primary| Context::canonical_to_primary(&canonical_file, primary));

            let rust_to_canonical = self
                .rust_files
                .iter()
                .map(move |f| Context::rust_to_canonical(f, &canonical_file));

            canonical_to_primary.chain(rust_to_canonical)
        });

        let variant_contexts = base_files.flat_map(|base| {
            let base_root = LanguageRoot::from(base.locale());
            parsed_files.iter().filter_map(move |variant| {
                let variant_root = LanguageRoot::from(variant.locale());
                (base != variant && base_root == variant_root)
                    .then_some(Context::base_to_variant(base, variant))
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
        let files = collate_fluent_files(settings.lingora.fluent_sources.as_slice())?;

        let canonical = settings.lingora.canonical.clone();
        let primaries = settings.lingora.primaries.clone();

        let required_locales = collate_required_locales(&canonical, &primaries)?;
        let language_roots = collate_language_roots(&required_locales)?;
        let _ = check_required_locales_have_fluent_files(&required_locales, &files)?;
        let _ = check_fluent_files_can_fallback(&files, &language_roots)?;
        let rust_files = collate_rust_files(&settings.dioxus_i18n.rust_sources)?;

        Ok(AuditEngine {
            files,
            language_roots,
            canonical,
            primaries,
            rust_files,
        })
    }
}

fn collate_fluent_files(fluent_paths: &[PathBuf]) -> Result<Vec<QualfiedFluentFile>, LingoraError> {
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
        .map(|files| files.into_iter().map(QualfiedFluentFile::from).collect())
}

fn collate_required_locales(
    canonical: &Locale,
    primaries: &[Locale],
) -> Result<Vec<Locale>, LingoraError> {
    let mut locales = Vec::from(primaries);
    locales.push(canonical.clone());
    Ok(locales)
}

fn collate_language_roots(
    required_locales: &[Locale],
) -> Result<HashSet<LanguageRoot>, LingoraError> {
    let counts =
        required_locales
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, locale| {
                let root = LanguageRoot::from(locale);
                *acc.entry(root).or_insert(0) += 1;
                acc
            });

    let duplicates = counts
        .into_iter()
        .filter_map(|(root, count)| (count > 1).then_some(root))
        .collect::<Vec<_>>();

    if duplicates.is_empty() {
        let roots = required_locales
            .iter()
            .map(LanguageRoot::from)
            .collect::<HashSet<_>>();
        Ok(roots)
    } else {
        let duplicates = duplicates
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(LingoraError::AmbiguousLanguageRoots(duplicates))
    }
}

fn check_required_locales_have_fluent_files(
    required_locales: &[Locale],
    files: &[QualfiedFluentFile],
) -> Result<(), LingoraError> {
    let provided_locales = files
        .iter()
        .map(|f| f.locale())
        .collect::<std::collections::HashSet<_>>();

    let missing = required_locales
        .iter()
        .filter(|l| !provided_locales.contains(l))
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    if missing.is_empty() {
        Ok(())
    } else {
        Err(LingoraError::MissingTranslationFiles(missing))
    }
}

fn check_fluent_files_can_fallback(
    files: &[QualfiedFluentFile],
    roots: &HashSet<LanguageRoot>,
) -> Result<(), LingoraError> {
    let by_root = files.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<LanguageRoot, Vec<Locale>>, file| {
            let root = LanguageRoot::from(file.locale());
            acc.entry(root).or_default().push(file.locale().clone());
            acc
        },
    );

    let missing = by_root
        .into_iter()
        .filter(|(root, _)| !roots.contains(root))
        .flat_map(|(_, locales)| locales)
        .collect::<Vec<Locale>>();

    if missing.is_empty() {
        Ok(())
    } else {
        let missing = missing
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(LingoraError::MissingPrimaryLocales(missing))
    }
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
    use std::{path::Path, str::FromStr};

    use super::*;

    #[test]
    fn fluent_files_will_be_collated_from_provided_paths() {
        let paths = &[Path::new("./tests/data/i18n").to_path_buf()];
        let files = collate_fluent_files(paths).unwrap();
        assert_eq!(files.len(), 6);

        let expected_files = [
            Path::new("./tests/data/i18n/en/en.ftl"),
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualfiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        expected_files
            .iter()
            .for_each(|f| assert!(files.contains(f)));
    }

    #[test]
    fn required_locales_will_be_collated_from_canonical_and_primaries() {
        let locales = collate_required_locales(
            &Locale::from_str("en-GB").unwrap(),
            &[
                Locale::from_str("it-IT").unwrap(),
                Locale::from_str("sr-Cryl-RS").unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(locales.len(), 3);

        assert!(locales.contains(&Locale::from_str("en-GB").unwrap()));
        assert!(locales.contains(&Locale::from_str("it-IT").unwrap()));
        assert!(locales.contains(&Locale::from_str("sr-Cryl-RS").unwrap()));
    }

    #[test]
    fn language_roots_must_be_unique() {
        let required_locales = [
            Locale::from_str("it-IT").unwrap(),
            Locale::from_str("en-GB").unwrap(),
            Locale::from_str("sr-Cryl-RS").unwrap(),
        ];

        let expected_roots = HashSet::from([
            LanguageRoot::from_str("it").unwrap(),
            LanguageRoot::from_str("en").unwrap(),
            LanguageRoot::from_str("sr-Cryl").unwrap(),
        ]);

        let actual_roots = collate_language_roots(&required_locales).unwrap();

        assert_eq!(actual_roots, expected_roots);
    }

    #[test]
    fn duplicated_language_roots_is_an_error() {
        let required_locales = [
            Locale::from_str("it-IT").unwrap(),
            Locale::from_str("en-GB").unwrap(),
            Locale::from_str("en-AU").unwrap(),
            Locale::from_str("sr-Cryl-RS").unwrap(),
            Locale::from_str("sr-Cryl-BA").unwrap(),
        ];

        assert!(matches!(
            collate_language_roots(&required_locales),
            Err(LingoraError::AmbiguousLanguageRoots(_))
        ))
    }

    #[test]
    fn required_locales_must_have_fluent_file() {
        let required_locales = [Locale::from_str("en-GB").unwrap()];
        let provided_files = [Path::new("./tests/data/i18n/en/en-GB.ftl")]
            .into_iter()
            .map(|p| QualfiedFluentFile::try_from(p).unwrap())
            .collect::<Vec<_>>();

        assert!(matches!(
            check_required_locales_have_fluent_files(&required_locales, &provided_files),
            Ok(())
        ))
    }

    #[test]
    fn required_locales_missing_fluent_file_is_an_error() {
        let required_locales = [Locale::from_str("it-IT").unwrap()];
        let provided_files = [Path::new("./tests/data/i18n/en/en-GB.ftl")]
            .into_iter()
            .map(|p| QualfiedFluentFile::try_from(p).unwrap())
            .collect::<Vec<_>>();

        assert!(matches!(
            check_required_locales_have_fluent_files(&required_locales, &provided_files),
            Err(LingoraError::MissingTranslationFiles(_))
        ));
    }

    #[test]
    fn fluent_files_must_have_primary_fallback() {
        let roots = [
            Locale::from_str("en-GB").unwrap(),
            Locale::from_str("it-IT").unwrap(),
            Locale::from_str("sr-Cryl-RS").unwrap(),
        ]
        .iter()
        .map(LanguageRoot::from)
        .collect::<HashSet<_>>();

        let files = [
            Path::new("./tests/data/i18n/en/en-GB.ftl"),
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-RS.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualfiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        assert!(matches!(
            check_fluent_files_can_fallback(&files, &roots),
            Ok(())
        ));
    }

    #[test]
    fn fluent_files_without_primary_fallback_is_an_error() {
        let roots = [Locale::from_str("en-GB").unwrap()]
            .iter()
            .map(LanguageRoot::from)
            .collect::<HashSet<_>>();

        let files = [
            Path::new("./tests/data/i18n/en/en-AU.ftl"),
            Path::new("./tests/data/i18n/it/it-IT.ftl"),
            Path::new("./tests/data/i18n/sr-Cryl/sr-Cryl-BA.ftl"),
        ]
        .into_iter()
        .map(|p| QualfiedFluentFile::try_from(p).unwrap())
        .collect::<Vec<_>>();

        assert!(matches!(
            check_fluent_files_can_fallback(&files, &roots),
            Err(LingoraError::MissingPrimaryLocales(_))
        ));
    }

    #[test]
    fn audit_engine_should_produce_a_report() {
        let toml = LingoraToml::try_from(Path::new("./tests/data/toml/Lingora_audit_engine.toml"))
            .unwrap();
        let engine = AuditEngine::try_from(&toml).unwrap();
        assert!(matches!(engine.run(), Ok(_)));
    }
}

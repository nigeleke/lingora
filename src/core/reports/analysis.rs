use crate::core::config::Settings;
use crate::core::domain::{FluentFile, IntegrityCheck, IntegrityCrossCheck, IntegrityWarning};

use thiserror::*;

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("failed to read fluent file {0}")]
    FailedToReadFluentFile(String),
}

#[cfg_attr(test, derive(Debug))]
pub struct Analysis {
    reference_path: PathBuf,
    checks: HashMap<PathBuf, Vec<IntegrityWarning>>,
}

impl Analysis {
    pub fn reference_path(&self) -> &PathBuf {
        &self.reference_path
    }

    pub fn target_paths_by_locale(&self) -> Vec<&PathBuf> {
        let mut keys = self
            .checks
            .keys()
            .filter(|p| *p != &self.reference_path)
            .collect::<Vec<_>>();
        keys.sort_by(|lhs, rhs| lhs.file_stem().cmp(&rhs.file_stem()));
        keys
    }

    pub fn check(&self, path: &PathBuf) -> Option<&[IntegrityWarning]> {
        println!("Analysis::check {:?} in {:#?}", path, self.checks);
        self.checks.get(path).map(|iw| iw.as_slice())
    }

    pub fn is_ok(&self) -> bool {
        self.checks.values().filter(|v| !v.is_empty()).count() == 0
    }
}

impl TryFrom<&Settings> for Analysis {
    type Error = AnalysisError;

    fn try_from(value: &Settings) -> std::result::Result<Self, Self::Error> {
        println!("TryFrom::Analysis::try_from: {:?}", value);

        let reference_path = value.reference();

        let check = |path: &PathBuf| {
            let file = FluentFile::try_from(path)
                .map_err(|e| AnalysisError::FailedToReadFluentFile(e.to_string()))?;
            let resource = file.resource();
            let check = IntegrityCheck::from(resource);
            Ok((resource.to_owned(), Vec::from(check.warnings())))
        };

        println!("TryFrom::Analysis::try_from:1");
        let (reference_resource, reference_check) = check(reference_path)?;
        println!(
            "TryFrom::Analysis::try_from:2: {:?} {:?}",
            reference_resource, reference_path
        );

        let mut checks =
            value
                .target_files()
                .into_iter()
                .try_fold(HashMap::new(), |mut acc, target_path| {
                    let (target_resource, mut target_check) = check(&target_path)?;
                    let cross_check =
                        IntegrityCrossCheck::new(&reference_resource, &target_resource);
                    target_check.extend(Vec::from(cross_check.warnings()));
                    acc.insert(target_path.to_owned(), target_check);
                    Ok(acc)
                })?;

        checks.insert(reference_path.to_path_buf(), reference_check);

        Ok(Self {
            reference_path: reference_path.to_owned(),
            checks,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::core::domain::Locale;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn will_analyse_supplied_reference_and_target_files() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
reference = "tests/data/i18n/en/en-GB.ftl"
targets = ["tests/data/i18n/"]
"#,
        )
        .unwrap();

        println!("*** Test settings: {:?}", settings);

        let analysis = Analysis::try_from(&settings).unwrap();
        assert_eq!(
            analysis.reference_path,
            PathBuf::from("tests/data/i18n/en/en-GB.ftl"),
        );

        let expected_paths = [
            PathBuf::from("tests/data/i18n/en/en.ftl"),
            PathBuf::from("tests/data/i18n/en/en-AU.ftl"),
            PathBuf::from("tests/data/i18n/en/en-GB.ftl"),
            PathBuf::from("tests/data/i18n/it/it-IT.ftl"),
        ];

        assert_eq!(analysis.checks.len(), expected_paths.len());
        assert!(expected_paths
            .iter()
            .all(|p| analysis.checks.contains_key(p)));
    }

    #[test]
    fn will_not_analyse_invalid_file() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
reference = "tests/data/does-not-exist.ftl"
targets = ["tests/data/i18n/"]
"#,
        )
        .unwrap();

        let error = Analysis::try_from(&settings).unwrap_err();
        assert!(matches!(error, AnalysisError::FailedToReadFluentFile(_)));
    }
}

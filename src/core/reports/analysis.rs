use crate::core::domain::{FluentFile, IntegrityCheck, IntegrityCrossCheck, IntegrityWarning};

use thiserror::*;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::args::ResolvedArgs;

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
        self.checks.get(path).map(|iw| iw.as_slice())
    }

    pub fn is_ok(&self) -> bool {
        self.checks.values().filter(|v| !v.is_empty()).count() == 0
    }
}

impl TryFrom<&ResolvedArgs> for Analysis {
    type Error = AnalysisError;

    fn try_from(value: &ResolvedArgs) -> std::result::Result<Self, Self::Error> {
        let reference_path = value.reference();

        let check = |path: &PathBuf| {
            let file = FluentFile::try_from(path)
                .map_err(|e| AnalysisError::FailedToReadFluentFile(e.to_string()))?;
            let resource = file.resource();
            let check = IntegrityCheck::from(resource);
            Ok((resource.to_owned(), Vec::from(check.warnings())))
        };

        let (reference_resource, reference_check) = check(reference_path)?;

        let mut checks =
            value
                .targets()
                .iter()
                .try_fold(HashMap::new(), |mut acc, target_path| {
                    let (target_resource, mut target_check) = check(target_path)?;
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
    use super::*;
    use crate::core::args::{CommandLineArgs, ResolvedArgsBuilder};
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn will_analyse_supplied_reference_and_target_files() {
        let builder = ResolvedArgsBuilder::default();
        let args = "app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/";
        let args = CommandLineArgs::from_str(&args).unwrap();
        let args = builder.build(&args).unwrap();

        let analysis = Analysis::try_from(&args).expect("args should be valid");
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
        let builder = ResolvedArgsBuilder::default();
        let args = "app_name -r tests/data/does-not-exist.ftl -t tests/data/i18n/";
        let args = CommandLineArgs::from_str(&args).unwrap();
        let args = builder.build(&args).unwrap();

        let error = Analysis::try_from(&args).unwrap_err();
        assert!(matches!(error, AnalysisError::FailedToReadFluentFile(_)));
    }
}

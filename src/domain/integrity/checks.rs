use std::{collections::HashMap, ops::Index, path::PathBuf};

use thiserror::*;

use super::{Check, CrossCheck, Status, Warning};
use crate::{config::Settings, domain::FluentFile};

#[derive(Debug, Error)]
pub enum ChecksError {
    #[error("cannot create integrity checks from settings: reason {0}")]
    CannotCreateFromSettings(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Checks(HashMap<PathBuf, Vec<Warning>>);

impl Checks {
    pub fn are_ok(&self) -> bool {
        self.0.values().all(|ws| ws.is_empty())
    }

    pub fn paths(&self) -> Vec<&PathBuf> {
        self.0.keys().collect()
    }

    pub fn status(&self, path: &PathBuf) -> Status {
        let warnings = &self[path];
        let errors = warnings.iter().filter(|w| w.is_error()).count();
        if errors == 0 {
            let warnings = warnings.iter().filter(|w| w.is_warning()).count();
            if warnings == 0 {
                Status::Ok
            } else {
                Status::Warning
            }
        } else {
            Status::Error
        }
    }
}

impl TryFrom<&Settings> for Checks {
    type Error = ChecksError;

    fn try_from(value: &Settings) -> std::result::Result<Self, Self::Error> {
        let reference_path = value.reference();

        let check = |path: &PathBuf| {
            let file = FluentFile::try_from(path)
                .map_err(|e| ChecksError::CannotCreateFromSettings(e.to_string()))?;
            let resource = file.resource();
            let check = Check::from(resource);
            Ok((resource.to_owned(), Vec::from(check.warnings())))
        };

        let (reference_resource, reference_check) = check(reference_path)?;

        let mut checks =
            value
                .targets()
                .into_iter()
                .try_fold(HashMap::new(), |mut acc, target_path| {
                    let (target_resource, mut target_check) = check(&target_path)?;
                    let cross_check = CrossCheck::new(&reference_resource, &target_resource);
                    target_check.extend(Vec::from(cross_check.warnings()));
                    acc.insert(target_path.to_owned(), target_check);
                    Ok(acc)
                })?;

        checks.insert(reference_path.to_owned(), reference_check);

        Ok(Self(checks))
    }
}

impl Index<&PathBuf> for Checks {
    type Output = Vec<Warning>;

    fn index(&self, index: &PathBuf) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::domain::Locale;

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

        let analysis = Checks::try_from(&settings).unwrap();
        let expected_paths = [
            PathBuf::from("tests/data/i18n/en/en.ftl"),
            PathBuf::from("tests/data/i18n/en/en-AU.ftl"),
            PathBuf::from("tests/data/i18n/en/en-GB.ftl"),
            PathBuf::from("tests/data/i18n/it/it-IT.ftl"),
        ];

        assert_eq!(analysis.0.len(), expected_paths.len());
        assert!(expected_paths.iter().all(|p| analysis
            .0
            .iter()
            .map(|(p, _)| p)
            .collect::<Vec<_>>()
            .contains(&p)));
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

        let error = Checks::try_from(&settings).unwrap_err();
        assert!(matches!(error, ChecksError::CannotCreateFromSettings(_)));
    }
}

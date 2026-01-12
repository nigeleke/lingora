use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use unic_langid::{LanguageIdentifier, subtags::Language};

use super::integrity::{Checks as IntegrityChecks, IntegrityWarning, Status as IntegrityStatus};
use crate::utils::path_to_id;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Validated<T> {
    Valid(T),
    Invalid(String),
}

impl<T> PartialOrd for Validated<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Validated<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Validated::Valid(a), Validated::Valid(b)) => a.cmp(b),
            (Validated::Invalid(a), Validated::Invalid(b)) => a.cmp(b),
            (Validated::Valid(_), Validated::Invalid(_)) => Ordering::Less,
            (Validated::Invalid(_), Validated::Valid(_)) => Ordering::Greater,
        }
    }
}

impl<T> std::fmt::Display for Validated<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Validated::Valid(x) => x.to_string(),
                Validated::Invalid(e) => e.to_string(),
            }
        )
    }
}

pub type ValidatedLocale = Validated<LanguageIdentifier>;

impl From<&Path> for ValidatedLocale {
    fn from(value: &Path) -> Self {
        match path_to_id(value) {
            Ok(id) => Validated::Valid(id),
            Err(_) => {
                let name = invalid_filename_as_string(value);
                Validated::Invalid(name)
            }
        }
    }
}

pub type ValidatedLanguage = Validated<Language>;

pub type PathsByLocale = HashMap<ValidatedLocale, Vec<PathBuf>>;
pub type PathsByLocaleByLanguage = HashMap<ValidatedLanguage, PathsByLocale>;

#[derive(Clone, Debug, PartialEq)]
pub struct Analysis {
    checks: IntegrityChecks,
    locales: PathsByLocale,
    languages: PathsByLocaleByLanguage,
}

fn invalid_filename_as_string(path: &Path) -> String {
    path.file_name()
        .map_or(path.to_string_lossy(), |f| f.to_string_lossy())
        .to_string()
}

impl Analysis {
    pub fn paths(&self) -> Vec<&PathBuf> {
        self.checks.paths()
    }

    pub fn checks(&self, path: &Path) -> &Vec<IntegrityWarning> {
        &self.checks[&path.to_path_buf()]
    }

    pub fn status(&self, path: &Path) -> IntegrityStatus {
        self.checks.status(path)
    }

    pub fn paths_by_locale_by_language(&self) -> &PathsByLocaleByLanguage {
        &self.languages
    }

    pub fn paths_by_locale(&self, language: &ValidatedLanguage) -> &PathsByLocale {
        &self.languages[language]
    }

    pub fn is_ok(&self) -> bool {
        self.checks.are_ok()
    }
}

impl From<IntegrityChecks> for Analysis {
    fn from(value: IntegrityChecks) -> Self {
        let mut locales = PathsByLocale::new();
        let mut languages = PathsByLocaleByLanguage::new();

        let add_locale = |locales: &mut PathsByLocale, path: &Path| {
            let validated = ValidatedLocale::from(path);
            locales
                .entry(validated)
                .or_default()
                .push(path.to_path_buf());
        };

        let add_language =
            |languages: &mut PathsByLocaleByLanguage,
             (id, paths): (&ValidatedLocale, &Vec<PathBuf>)| {
                let validated = match &id {
                    Validated::Valid(id) => Validated::Valid(id.language),
                    Validated::Invalid(_) => Validated::Invalid("≪Unknown≫".into()),
                };

                languages
                    .entry(validated)
                    .or_default()
                    .insert(id.clone(), paths.clone());
            };

        value
            .paths()
            .into_iter()
            .for_each(|f| add_locale(&mut locales, f));

        locales.iter().for_each(|l| add_language(&mut languages, l));

        Self {
            checks: value,
            locales,
            languages,
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        config::Settings,
        domain::{IntegrityChecks, Locale},
    };

    fn create_analysis(settings: &str) -> Analysis {
        let settings = Settings::try_from_str(Locale::default(), settings).unwrap();
        let checks = IntegrityChecks::try_from(&settings).unwrap();
        Analysis::from(checks)
    }

    #[test]
    fn will_create_from_integrity_checks() {
        let analysis = create_analysis(
            r#"
[lingora]
reference = "tests/data/i18n/en/en-GB.ftl"
targets = ["tests/data/i18n/"]
"#,
        );

        assert_eq!(analysis.languages.keys().len(), 2);
    }

    #[test]
    fn will_provide_primary_languages() {
        let analysis = create_analysis(
            r#"
[lingora]
reference = "tests/data/i18n/en/en-GB.ftl"
targets = ["tests/data/i18n/"]
"#,
        );

        let languages = analysis.languages;
        assert_eq!(languages.len(), 2);
    }
}

use super::integrity_checks::IntegrityChecks;
use super::integrity_warning::IntegrityWarning;

use crate::utils::pb2id;

use unic_langid::{subtags::Language, LanguageIdentifier};

use std::fmt::Display;
use std::{collections::HashMap, path::PathBuf};

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

impl From<&PathBuf> for ValidatedLocale {
    fn from(value: &PathBuf) -> Self {
        match pb2id(value) {
            Ok(id) => Validated::Valid(id),
            Err(_) => {
                let name = invalid_filename_as_string(value);
                Validated::Invalid(name)
            }
        }
    }
}

pub type ValidatedLanguage = Validated<Language>;

type PathsByLocale = HashMap<ValidatedLocale, Vec<PathBuf>>;
type PathsByLocaleByLanguage = HashMap<ValidatedLanguage, PathsByLocale>;

#[derive(Clone, Debug, PartialEq)]
pub struct Analysis {
    checks: IntegrityChecks,
    locales: PathsByLocale,
    languages: PathsByLocaleByLanguage,
}

fn invalid_filename_as_string(path: &PathBuf) -> String {
    path.file_name()
        .map_or(path.to_string_lossy(), |f| f.to_string_lossy())
        .to_string()
}

impl Analysis {
    pub fn is_ok(&self) -> bool {
        self.checks.are_ok()
    }

    pub fn reference_path(&self) -> &PathBuf {
        self.checks.reference_path()
    }

    pub fn target_paths(&self) -> Vec<&PathBuf> {
        self.checks.target_paths()
    }

    pub fn check(&self, path: &PathBuf) -> Option<&[IntegrityWarning]> {
        self.checks.check(path)
    }

    pub fn paths_by_locale_by_language(&self) -> &PathsByLocaleByLanguage {
        &self.languages
    }

    pub fn paths_by_locale(&self, language: &ValidatedLanguage) -> &PathsByLocale {
        &self.languages[&language]
    }
}

impl From<IntegrityChecks> for Analysis {
    fn from(value: IntegrityChecks) -> Self {
        let checks = value;

        let mut locales = PathsByLocale::new();
        let mut languages = PathsByLocaleByLanguage::new();

        let add_locale = |locales: &mut PathsByLocale, path: &PathBuf| {
            let validated = ValidatedLocale::from(path);
            locales
                .entry(validated)
                .or_insert_with(Vec::new)
                .push(path.clone());
        };

        let add_language =
            |languages: &mut PathsByLocaleByLanguage,
             (id, paths): (&ValidatedLocale, &Vec<PathBuf>)| {
                let validated = match &id {
                    Validated::Valid(id) => Validated::Valid(id.language.clone()),
                    Validated::Invalid(_) => Validated::Invalid("≪Unknown≫".into()),
                };

                languages
                    .entry(validated)
                    .or_insert_with(PathsByLocale::new)
                    .insert(id.clone(), paths.clone());
            };

        checks
            .target_paths()
            .into_iter()
            .for_each(|p| add_locale(&mut locales, p));
        add_locale(&mut locales, checks.reference_path());

        locales.iter().for_each(|l| add_language(&mut languages, l));

        Self {
            checks,
            locales,
            languages,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Settings;
    use crate::domain::{IntegrityChecks, Locale};
    use pretty_assertions::assert_eq;

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

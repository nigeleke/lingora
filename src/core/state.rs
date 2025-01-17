use super::error::Error;
use super::fluent_file::FluentFile;
use super::identifier::Identifier;
use super::locale::Locale;
use super::prelude::AnnotatedIdentifier;
use super::primary_language::PrimaryLanguage;

use walkdir::{DirEntry, WalkDir};

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

type FluentFiles = HashMap<Locale, FluentFile>;
type PrimaryLanguages = HashMap<PrimaryLanguage, HashSet<Locale>>;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    fluent_files: FluentFiles,
    primary_languages: PrimaryLanguages,
    target_locale: Option<Locale>,
    selected_identifier: Option<AnnotatedIdentifier>,
}

impl State {
    pub fn primary_languages(&self) -> Vec<&PrimaryLanguage> {
        self.primary_languages.keys().collect()
    }

    pub fn target_locale(&self) -> Option<&Locale> {
        self.target_locale.as_ref()
    }

    pub fn set_target_locale(&mut self, language: &Locale) {
        self.target_locale = Some(language.clone())
    }

    pub fn languages(&self) -> &PrimaryLanguages {
        &self.primary_languages
    }

    pub fn locales(&self, primary: &PrimaryLanguage) -> &HashSet<Locale> {
        self.primary_languages.get(primary).unwrap()
    }

    pub fn identifiers(&self, locale: &Locale) -> HashSet<Identifier> {
        if let Some(file) = self.fluent_files.get(locale) {
            file.try_identifiers().unwrap_or(HashSet::default())
        } else {
            HashSet::new()
        }
    }

    pub fn set_selected_identifier(&mut self, identifier: &AnnotatedIdentifier) {
        self.selected_identifier = Some(identifier.clone());
    }

    pub fn selected_identifier(&self) -> Option<&AnnotatedIdentifier> {
        self.selected_identifier.as_ref()
    }
}

impl TryFrom<&PathBuf> for State {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let translation_files = find_fluent_files(value)?;

        let fluent_files = translation_files
            .iter()
            .try_fold(HashMap::default(), add_fluent_file)?;

        let primary_languages = translation_files
            .iter()
            .try_fold(HashMap::default(), add_primary_language)?;

        Ok(Self {
            fluent_files,
            primary_languages,
            target_locale: None,
            selected_identifier: None,
        })
    }
}

fn find_fluent_files(folder: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    if !folder.is_dir() {
        return Err(Error::FluentFileTraversalFailed(format!(
            "invalid root folder: {}",
            folder.display()
        )));
    }

    let fluent_files: Vec<PathBuf> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_fluent_file(entry))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(fluent_files)
}

fn is_fluent_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .path()
            .extension()
            .map(|ext| ext == "ftl")
            .unwrap_or(false)
}

fn add_fluent_file(mut files: FluentFiles, file: &PathBuf) -> Result<FluentFiles, Error> {
    let language = Locale::try_from(file)?;
    let error = Error::DuplicateLanguageFile(language.to_string());
    let language_file = FluentFile::from(file);
    let previous = files.insert(language, language_file);
    previous.map_or(Ok(files), |_| Err(error))
}

fn add_primary_language(
    mut languages: PrimaryLanguages,
    file: &PathBuf,
) -> Result<PrimaryLanguages, Error> {
    let language = Locale::try_from(file)?;
    let primary_language = language.primary_language();

    let validated_languages = languages
        .entry(primary_language)
        .or_insert_with(HashSet::default);

    validated_languages.insert(language);

    Ok(languages)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn state_will_be_set_from_root_path() {
        let path = PathBuf::from(format!("{}/tests/data/i18n/", env!("CARGO_MANIFEST_DIR")));
        let state = State::try_from(&path);
        assert!(state.is_ok(), "Unexpected: {state:?}");
    }

    #[test]
    fn state_errors_if_invalid_subtag() {
        let path = PathBuf::from(format!(
            "{}/tests/data/i18n_invalid_subtag/",
            env!("CARGO_MANIFEST_DIR")
        ));
        let state = State::try_from(&path);
        assert!(state.is_err(), "Unexpected: {state:?}");
    }

    #[test]
    fn state_try_from_fails_if_unknown_root() {
        let path = PathBuf::from(format!(
            "{}/tests/data/i18n_does_not_exist/",
            env!("CARGO_MANIFEST_DIR")
        ));
        let state = State::try_from(&path);
        assert!(state.is_err(), "Unexpected: {state:?}");
    }

    #[test]
    fn state_try_from_fails_when_duplicate_locales() {
        let path = PathBuf::from(format!(
            "{}/tests/data/i18n_duplicates/",
            env!("CARGO_MANIFEST_DIR")
        ));
        let state = State::try_from(&path);
        assert!(state.is_err(), "Unexpected: {state:?}");
    }

    #[test]
    fn primary_languages_will_be_derived_from_translation_files() {
        let path = PathBuf::from(format!("{}/tests/data/i18n/", env!("CARGO_MANIFEST_DIR")));
        let state = State::try_from(&path).unwrap();
        assert_eq!(state.primary_languages().len(), 2);
    }
}

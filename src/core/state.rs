use super::error::Error;
use super::language::Language;
use super::language_file::LanguageFile;
use super::primary_language::PrimaryLanguage;

use walkdir::{DirEntry, WalkDir};

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

type LanguageFiles = HashMap<Language, LanguageFile>;
type PrimaryLanguages = HashMap<PrimaryLanguage, HashSet<Language>>;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    language_files: LanguageFiles,
    primary_languages: PrimaryLanguages,
    target_language: Option<Language>,
}

impl State {
    pub fn primary_languages(&self) -> Vec<&PrimaryLanguage> {
        self.primary_languages.keys().collect()
    }

    pub fn target_language(&self) -> Option<&Language> {
        self.target_language.as_ref()
    }

    pub fn set_target_language(&mut self, language: Language) {
        self.target_language = Some(language)
    }

    pub fn languages(&self) -> &PrimaryLanguages {
        &self.primary_languages
    }
}

impl TryFrom<&PathBuf> for State {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let translation_files = find_ftl_files(value)?;

        let language_files = translation_files
            .iter()
            .try_fold(HashMap::default(), add_file_language_file)?;

        let primary_languages = translation_files
            .iter()
            .try_fold(HashMap::default(), add_file_to_primary_languages)?;

        Ok(Self {
            language_files,
            primary_languages,
            target_language: None,
        })
    }
}

fn find_ftl_files(folder: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    if !folder.is_dir() {
        return Err(Error::FluentFileTraversalFailed(format!(
            "invalid translation file root folder: {}",
            folder.display()
        )));
    }

    let ftl_files: Vec<PathBuf> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_ftl_file(entry))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(ftl_files)
}

fn is_ftl_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .path()
            .extension()
            .map(|ext| ext == "ftl")
            .unwrap_or(false)
}

fn add_file_language_file(
    mut files: LanguageFiles,
    file: &PathBuf,
) -> Result<LanguageFiles, Error> {
    let language = Language::try_from(file)?;
    let error = Error::DuplicateLanguageFile(language.to_string());
    let language_file = LanguageFile::from(file);
    let previous = files.insert(language, language_file);
    previous.map_or(Ok(files), |_| Err(error))
}

fn add_file_to_primary_languages(
    mut languages: PrimaryLanguages,
    file: &PathBuf,
) -> Result<PrimaryLanguages, Error> {
    let language = Language::try_from(file)?;
    let primary_language = PrimaryLanguage::from(&language);

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
    fn state_errors_if_unknown_root() {
        let path = PathBuf::from(format!(
            "{}/tests/data/i18n_does_not_exist/",
            env!("CARGO_MANIFEST_DIR")
        ));
        let state = State::try_from(&path);
        assert!(state.is_err(), "Unexpected: {state:?}");
    }

    #[test]
    fn state_errors_duplicate_languages() {
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

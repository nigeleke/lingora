use super::Arguments;

use crate::domain::Locale;

use serde::Serialize;
use thiserror::Error;
use toml::{value::Array, Table, Value};
use walkdir::WalkDir;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum InterimSettingsError {
    #[error("cannot read configuration file; reason: {0}")]
    CannotReadConfigFile(String),

    #[error("invalid configuation: {0}")]
    InvalidConfig(String),

    #[error("cannot find reference file: {0}")]
    CannotFindReferenceFile(String),

    #[error("ambiguous reference files: {0}")]
    AmbiguousReferenceFiles(String),
}

type Result<T> = std::result::Result<T, InterimSettingsError>;

#[derive(Debug, Serialize)]
pub struct InterimSettings {
    locale: Locale,
    table: Table,
}

const DEFAULT_TOML_FILE: &str = "Lingora.toml";
const DEFAULT_TOML_CONTENT: &str = include_str!("default_lingora.toml");

const LINGORA: &str = "lingora";
const LINGORA_ROOT: &str = "root";
const LINGORA_REFERENCE: &str = "reference";
const LINGORA_TARGETS: &str = "targets";
const DIOXUS_I18N: &str = "dioxus_i18n";
const DIOXUS_I18N_WITH_LOCALE: &str = "with_locale";
const DIOXUS_I18N_SHARES: &str = "shares";
const DIOXUS_I18N_FALLBACK: &str = "fallback";

impl InterimSettings {
    pub fn try_from_arguments(locale: Locale, arguments: &Arguments) -> Result<Self> {
        let toml_content = if let Some(path) = arguments.config() {
            std::fs::read_to_string(path)
                .map_err(|e| InterimSettingsError::CannotReadConfigFile(e.to_string()))?
        } else {
            let path = PathBuf::from(DEFAULT_TOML_FILE);

            let exists = std::fs::exists(&path)
                .map_err(|e| InterimSettingsError::CannotReadConfigFile(e.to_string()))?;

            if exists {
                std::fs::read_to_string(&path)
                    .map_err(|e| InterimSettingsError::CannotReadConfigFile(e.to_string()))?
            } else {
                DEFAULT_TOML_CONTENT.to_string()
            }
        };

        Self::try_from_str(locale, toml_content.as_str()).and_then(|settings| {
            settings
                .override_from_arguments(arguments)
                .with_defaulted_missing_entries()
        })
    }

    pub fn try_from_str(locale: Locale, s: &str) -> Result<Self> {
        let table = s
            .parse::<toml::Table>()
            .map_err(|e| InterimSettingsError::InvalidConfig(e.to_string()))?;

        Ok(Self { locale, table })
    }

    fn override_from_arguments(self, arguments: &Arguments) -> Self {
        self.with_defaulted_table(LINGORA)
            .with_overridden_root(arguments.root())
            .with_overridden_reference(arguments.reference())
            .with_overridden_targets(arguments.targets())
    }

    fn with_overridden_root(mut self, pathbuf: Option<&PathBuf>) -> Self {
        if let Some(pathbuf) = pathbuf {
            if let Value::Table(lingora) = &mut self.table[LINGORA] {
                lingora.insert(
                    LINGORA_ROOT.into(),
                    Value::String(pathbuf.to_string_lossy().into()),
                );
            }
        }

        self
    }

    fn with_overridden_reference(mut self, pathbuf: Option<&PathBuf>) -> Self {
        if let Some(pathbuf) = pathbuf {
            if let Value::Table(lingora) = &mut self.table[LINGORA] {
                lingora.insert(
                    LINGORA_REFERENCE.into(),
                    Value::String(pathbuf.to_string_lossy().into()),
                );
            }
        }

        self
    }

    fn with_overridden_targets(mut self, targets: Vec<&PathBuf>) -> Self {
        if !targets.is_empty() {
            if let Value::Table(lingora) = &mut self.table[LINGORA] {
                let targets = targets
                    .into_iter()
                    .map(|p| Value::String(p.to_string_lossy().into()))
                    .collect::<Vec<_>>();
                lingora.insert(LINGORA_TARGETS.into(), Value::Array(targets));
            }
        }

        self
    }

    pub fn with_defaulted_missing_entries(self) -> Result<Self> {
        Ok(self
            .with_defaulted_table(LINGORA)
            .with_defaulted_lingora_root()
            .with_defaulted_lingora_targets()
            .with_defaulted_lingora_reference()?
            .with_defaulted_table(DIOXUS_I18N)
            .with_defaulted_dioxus_i18n_with_locale()
            .with_defaulted_dioxus_i18n_shares()
            .with_defaulted_dioxus_i18n_fallback())
    }

    fn with_defaulted_table(mut self, table: &str) -> Self {
        self.table
            .entry(table)
            .or_insert_with(|| Value::Table(Table::new()));
        self
    }

    fn with_defaulted_lingora_root(mut self) -> Self {
        if let Value::Table(lingora) = &mut self.table[LINGORA] {
            lingora
                .entry(LINGORA_ROOT)
                .or_insert_with(|| Value::String("./i18n/".into()));
        } else {
            unreachable!()
        };
        self
    }

    fn with_defaulted_lingora_targets(mut self) -> Self {
        if let Value::Table(lingora) = &mut self.table[LINGORA] {
            let root = lingora.get(LINGORA_ROOT).unwrap().clone();
            lingora
                .entry(LINGORA_TARGETS)
                .or_insert_with(|| Value::Array(Array::from([root])));
        } else {
            unreachable!()
        };
        self
    }

    fn with_defaulted_lingora_reference(mut self) -> Result<Self> {
        let target_files = self.target_files();

        let reference_file = || {
            let reference_filename = &PathBuf::from(self.locale.to_string()).with_extension("ftl");

            let reference_files = target_files
                .into_iter()
                .filter(|p| p.file_name() == reference_filename.file_name())
                .collect::<Vec<_>>();

            type IE = InterimSettingsError;
            match reference_files.len() {
                0 => Err(IE::CannotFindReferenceFile(
                    reference_filename.to_string_lossy().into(),
                )),
                1 => Ok(reference_files[0].to_owned()),
                _ => Err(IE::AmbiguousReferenceFiles(format!(
                    "{:#?}",
                    reference_files
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                ))),
            }
        };

        if let Value::Table(lingora) = &mut self.table[LINGORA] {
            if !lingora.contains_key(LINGORA_REFERENCE) {
                let reference_file = reference_file()?;
                lingora
                    .entry(LINGORA_REFERENCE)
                    .or_insert_with(|| Value::String(reference_file.to_string_lossy().into()));
            }
        }

        Ok(self)
    }

    fn target_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        let walk_dir = |path: &PathBuf| {
            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path())
                .filter(|p| p.is_file())
        };

        let add_path = |path: &String| {
            let path = PathBuf::from(path);
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(walk_dir(&path));
            }
        };

        if let Value::Table(lingora) = &self.table[LINGORA] {
            if let Value::Array(array) = &lingora[LINGORA_TARGETS] {
                array
                    .iter()
                    .filter_map(|v| match v {
                        Value::String(s) => Some(s),
                        _ => None,
                    })
                    .for_each(add_path)
            }
        }

        files
    }

    fn with_defaulted_dioxus_i18n_with_locale(mut self) -> Self {
        if let Value::Table(dioxus_i18n) = &mut self.table[DIOXUS_I18N] {
            dioxus_i18n
                .entry(DIOXUS_I18N_WITH_LOCALE)
                .or_insert_with(|| Value::String("auto".into()));
        } else {
            unreachable!()
        };
        self
    }

    fn with_defaulted_dioxus_i18n_shares(mut self) -> Self {
        if let Value::Table(dioxus_i18n) = &mut self.table[DIOXUS_I18N] {
            dioxus_i18n
                .entry(DIOXUS_I18N_SHARES)
                .or_insert_with(|| Value::Array(Array::new()));
        } else {
            unreachable!()
        };
        self
    }

    fn with_defaulted_dioxus_i18n_fallback(mut self) -> Self {
        if let Value::Table(dioxus_i18n) = &mut self.table[DIOXUS_I18N] {
            dioxus_i18n
                .entry(DIOXUS_I18N_FALLBACK)
                .or_insert_with(|| Value::String(self.locale.to_string()));
        } else {
            unreachable!()
        };
        self
    }

    pub fn toml_table(&self) -> &Table {
        &self.table
    }
}

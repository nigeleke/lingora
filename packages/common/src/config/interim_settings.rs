use std::{
    fs::{exists, read_to_string},
    path::{Path, PathBuf},
};

use serde::Serialize;
use toml::{Table, Value, value::Array};
use walkdir::WalkDir;

use crate::{AnalysisArgs, LingoraError, Locale};

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
    pub fn try_from_args(locale: Locale, args: &AnalysisArgs) -> Result<Self, LingoraError> {
        // Read from the config file provided in the argumments, otherwise
        // use the default file, if it exists, otherwise
        // use application defaults.

        let toml_content = if let Some(path) = args.config() {
            read_to_string(path)?
        } else if exists(&PathBuf::from(DEFAULT_TOML_FILE))? {
            read_to_string(&DEFAULT_TOML_FILE)?
        } else {
            DEFAULT_TOML_CONTENT.to_string()
        };

        Self::try_from_str(locale, toml_content.as_str()).and_then(|settings| {
            settings
                .override_from_arguments(args)
                .with_defaulted_missing_entries()
        })
    }

    pub fn try_from_str(locale: Locale, s: &str) -> Result<Self, LingoraError> {
        let table = s.parse::<toml::Table>()?;
        Ok(Self { locale, table })
    }

    fn override_from_arguments(self, arguments: &AnalysisArgs) -> Self {
        self.with_defaulted_table(LINGORA)
            .with_overridden_root(arguments.root())
            .with_overridden_reference(arguments.reference())
            .with_overridden_targets(Vec::from_iter(arguments.targets()).as_slice())
    }

    fn with_overridden_root(mut self, path: Option<&Path>) -> Self {
        if let Some(path) = path
            && let Value::Table(lingora) = &mut self.table[LINGORA]
        {
            lingora.insert(
                LINGORA_ROOT.into(),
                Value::String(path.to_string_lossy().into()),
            );
        }

        self
    }

    fn with_overridden_reference(mut self, path: Option<&Path>) -> Self {
        if let Some(path) = path
            && let Value::Table(lingora) = &mut self.table[LINGORA]
        {
            lingora.insert(
                LINGORA_REFERENCE.into(),
                Value::String(path.to_string_lossy().into()),
            );
        }

        self
    }

    fn with_overridden_targets(mut self, targets: &[&Path]) -> Self {
        if !targets.is_empty()
            && let Value::Table(lingora) = &mut self.table[LINGORA]
        {
            let targets = targets
                .into_iter()
                .map(|p| Value::String(p.to_string_lossy().into()))
                .collect::<Vec<_>>();
            lingora.insert(LINGORA_TARGETS.into(), Value::Array(targets));
        }

        self
    }

    pub fn with_defaulted_missing_entries(self) -> Result<Self, LingoraError> {
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

    fn with_defaulted_lingora_reference(mut self) -> Result<Self, LingoraError> {
        let target_files = self.target_files();

        let reference_file = || {
            let reference_filename = &PathBuf::from(self.locale.to_string()).with_extension("ftl");

            let mut reference_files = target_files
                .iter()
                .filter(|p| p.file_name() == reference_filename.file_name())
                .map(|p| p.to_path_buf())
                .collect::<Vec<_>>();
            reference_files.sort();

            type IE = LingoraError;
            match reference_files.len() {
                0 => Err(IE::CannotFindReferenceFile(
                    reference_filename.to_string_lossy().into(),
                )),
                1 => Ok(reference_files[0].to_owned()),
                _ => Err(IE::AmbiguousReferenceFiles(
                    reference_files
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                        .join("\n  "),
                )),
            }
        };

        if let Value::Table(lingora) = &mut self.table[LINGORA]
            && !lingora.contains_key(LINGORA_REFERENCE)
        {
            let reference_file = reference_file()?;
            lingora
                .entry(LINGORA_REFERENCE)
                .or_insert_with(|| Value::String(reference_file.to_string_lossy().into()));
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
                .map(|p| p.to_path_buf())
        };

        let add_path = |path: &String| {
            let path = PathBuf::from(path);
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(walk_dir(&path));
            }
        };

        if let Value::Table(lingora) = &self.table[LINGORA]
            && let Value::Array(array) = &lingora[LINGORA_TARGETS]
        {
            array
                .iter()
                .filter_map(|v| match v {
                    Value::String(s) => Some(s),
                    _ => None,
                })
                .for_each(add_path)
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

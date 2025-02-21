use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;
use walkdir::WalkDir;

use super::{arguments::Arguments, interim_settings::InterimSettings};
use crate::domain::Locale;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("invalid settings {0}")]
    InvalidSettings(String),
}

type Result<T> = std::result::Result<T, SettingsError>;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Settings {
    lingora: LingoraSettings,
    dioxus_i18n: DioxusI18nSettings,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct LingoraSettings {
    root: PathBuf,
    reference: PathBuf,
    targets: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum WithLocale {
    IncludeStr,
    PathBuf,
    Auto,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct DioxusI18nSettings {
    with_locale: WithLocale,
    shares: Vec<(Locale, Locale)>,
    fallback: Locale,
}

impl Settings {
    pub fn try_from_arguments(locale: Locale, value: &Arguments) -> Result<Self> {
        let interim = InterimSettings::try_from_arguments(locale, value)
            .map_err(|e| SettingsError::InvalidSettings(e.to_string()))?;

        Self::try_from(&interim)
    }

    #[cfg(test)]
    pub fn try_from_str(locale: Locale, s: &str) -> Result<Self> {
        let interim = InterimSettings::try_from_str(locale, s)
            .and_then(|settings| settings.with_defaulted_missing_entries())
            .map_err(|e| SettingsError::InvalidSettings(e.to_string()))?;

        Self::try_from(&interim)
    }

    pub fn root(&self) -> &PathBuf {
        &self.lingora.root
    }

    pub fn reference(&self) -> &PathBuf {
        &self.lingora.reference
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        self.lingora.targets.iter().for_each(|p| {
            let reference = &self.lingora.reference;

            if p.is_file() && p != reference {
                files.push(p.clone());
            } else if p.is_dir() {
                let new_files = WalkDir::new(p)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| e.into_path())
                    .filter(|p| p.is_file() && p != reference)
                    .collect::<Vec<_>>();
                files.extend(new_files);
            }
        });

        files
    }

    pub fn with_locale(&self) -> &WithLocale {
        &self.dioxus_i18n.with_locale
    }

    pub fn shares(&self) -> &[(Locale, Locale)] {
        self.dioxus_i18n.shares.as_slice()
    }

    pub fn fallback(&self) -> &Locale {
        &self.dioxus_i18n.fallback
    }
}

impl TryFrom<&InterimSettings> for Settings {
    type Error = SettingsError;

    fn try_from(value: &InterimSettings) -> std::result::Result<Self, Self::Error> {
        let interim = toml::to_string(&value.toml_table())
            .map_err(|e| SettingsError::InvalidSettings(e.to_string()))?;

        let settings: Settings =
            toml::from_str(&interim).map_err(|e| SettingsError::InvalidSettings(e.to_string()))?;

        Ok(settings)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn will_create_from_valid_content() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
reference = "tests/data/i18n/en/en-GB.ftl"
targets = ["tests/data/i18n/"]
shares = [["it", "it-IT"]]
"#,
        );

        assert!(settings.is_ok());
    }

    #[test]
    fn will_fail_to_create_from_invalid_content() {
        let error = Settings::try_from_str(
            Locale::default(),
            r#"
reference = !@#$%^&*
targets =
"#,
        )
        .unwrap_err();

        assert!(matches!(error, SettingsError::InvalidSettings(_)));
    }

    #[test]
    fn have_reference_file_if_provided_in_command_line_arguments() {
        let args = Arguments::from_str("app_name -r path/to/en.ftl").unwrap();
        let settings = Settings::try_from_arguments(Locale::default(), &args).unwrap();
        assert_eq!(*settings.reference(), PathBuf::from("path/to/en.ftl"));
    }

    #[test]
    fn have_reference_file_if_not_provided_but_sys_locale_file_is_in_target_folder() {
        let args = Arguments::from_str("app_name -t tests/data/i18n/").unwrap();
        let settings =
            Settings::try_from_arguments(Locale::from_str("en-GB").unwrap(), &args).unwrap();
        let expected_file_name = PathBuf::from("en-GB.ftl");
        assert_eq!(
            settings.reference().file_name().unwrap(),
            expected_file_name.file_name().unwrap()
        );
    }

    #[test]
    fn fail_to_have_reference_file_if_not_provided_and_sys_locale_file_not_in_target_folder() {
        let args = Arguments::from_str("app_name -t tests/data/i18n_empty/").unwrap();
        let error = Settings::try_from_arguments(Locale::default(), &args).unwrap_err();
        assert!(matches!(error, SettingsError::InvalidSettings(_)));
        insta::with_settings!({filters => vec![
            (r"[a-z]{2}(-[A-Z]{2})?\.ftl", "<langid>.ftl")
        ]}, {
            insta::assert_snapshot!(error, @"invalid settings cannot find reference file: <langid>.ftl")
        });
    }

    #[test]
    fn fail_to_have_reference_file_if_not_provided_and_multiple_sys_locale_files_in_target_folder()
    {
        let args = Arguments::from_str("app_name -t tests/data/i18n_duplicates").unwrap();
        let error =
            Settings::try_from_arguments(Locale::from_str("en-GB").unwrap(), &args).unwrap_err();
        assert!(matches!(error, SettingsError::InvalidSettings(_)));
        insta::assert_snapshot!(error, @r#"
        invalid settings ambiguous reference files: [
            "tests/data/i18n_duplicates/one/en-GB.ftl",
            "tests/data/i18n_duplicates/two/en-GB.ftl",
        ]
        "#);
    }

    fn count_files(file_name: &str, files: &Vec<PathBuf>) -> usize {
        files
            .iter()
            .filter(|f| f.ends_with(PathBuf::from(file_name)))
            .count()
    }

    #[test]
    fn have_targets_when_explicitly_provided_command_line_arguments() {
        let args = Arguments::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let settings = Settings::try_from_arguments(Locale::default(), &args).unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn not_have_reference_in_targets_even_when_requested() {
        let args = Arguments::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let settings = Settings::try_from_arguments(Locale::default(), &args).unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn have_targets_when_implicitly_provided_command_line_arguments() {
        let args =
            Arguments::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/")
                .unwrap();
        let settings = Settings::try_from_arguments(Locale::default(), &args).unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn default_target_path_is_used_when_no_targets_provided() {
        let settings = Settings::try_from_str(
            Locale::from_str("en-GB").unwrap(),
            r#"
[lingora]
root = "./tests/data/i18n/"
"#,
        )
        .unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("i18n/en/en.ftl", &targets), 1);
        assert_eq!(count_files("i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn default_reference_path_is_used_when_no_targets_provided() {
        let settings = Settings::try_from_str(
            Locale::from_str("en-GB").unwrap(),
            r#"
[lingora]
root = "./tests/data/i18n/"
"#,
        )
        .unwrap();

        assert_eq!(
            settings.reference().file_name(),
            PathBuf::from("en-GB.ftl").file_name()
        );
    }
}

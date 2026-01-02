use std::path::{Path, PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::{AnalysisArgs, LingoraError, Locale, config::InterimSettings};

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Settings {
    lingora: LingoraSettings,
    dioxus_i18n: DioxusI18nSettings,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
struct LingoraSettings {
    root: PathBuf,
    reference: PathBuf,
    targets: Vec<PathBuf>,
}

impl std::fmt::Display for LingoraSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"root = "{}"
reference = "{}"
targets = [{}]
"#,
            self.root.display(),
            self.reference.display(),
            self.targets
                .iter()
                .map(|t| format!(r#""{}""#, t.display()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum WithLocale {
    IncludeStr,
    PathBuf,
    #[default]
    Auto,
}

impl std::fmt::Display for WithLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncludeStr => "includestr",
            Self::PathBuf => "pathbuf",
            Self::Auto => "auto",
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
struct DioxusI18nSettings {
    with_locale: WithLocale,
    shares: Vec<(Locale, Locale)>,
    fallback: Locale,
}

impl std::fmt::Display for DioxusI18nSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"with_locale = "{}"
shares = [{}]
fallback = "{}""#,
            self.with_locale,
            self.shares
                .iter()
                .map(|(source, target)| format!(r#"("{source}", "{target}")"#))
                .collect::<Vec<_>>()
                .join(", "),
            self.fallback
        )
    }
}

impl Settings {
    pub fn try_from_args(locale: Locale, value: &AnalysisArgs) -> Result<Self, LingoraError> {
        let interim = InterimSettings::try_from_args(locale, value)?;
        Self::try_from(&interim)
    }

    pub fn try_from_str(locale: Locale, s: &str) -> Result<Self, LingoraError> {
        let interim = InterimSettings::try_from_str(locale, s)
            .and_then(|settings| settings.with_defaulted_missing_entries())?;

        Self::try_from(&interim)
    }

    pub fn root(&self) -> &Path {
        self.lingora.root.as_path()
    }

    pub fn reference_path(&self) -> &Path {
        self.lingora.reference.as_path()
    }

    pub fn reference_locale(&self) -> Result<Locale, LingoraError> {
        Locale::try_from(self.lingora.reference.as_path()).map_err(LingoraError::from)
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
    type Error = LingoraError;

    fn try_from(value: &InterimSettings) -> std::result::Result<Self, Self::Error> {
        let interim = toml::to_string(&value.toml_table())?;

        let settings: Settings = toml::from_str(&interim)?;

        Ok(settings)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"[lingora]
{}
[dioxus_i18n]
{}"#,
            self.lingora, self.dioxus_i18n
        )
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

        assert!(matches!(error, LingoraError::TomlParse(_)));
    }

    #[test]
    fn have_reference_file_if_provided_in_command_line_arguments() {
        let args = AnalysisArgs::from_str("app_name -r path/to/en.ftl").unwrap();
        let settings = Settings::try_from_args(Locale::default(), &args).unwrap();
        assert_eq!(*settings.reference_path(), PathBuf::from("path/to/en.ftl"));
    }

    #[test]
    fn have_reference_file_if_not_provided_but_sys_locale_file_is_in_target_folder() {
        let args = AnalysisArgs::from_str("app_name -t tests/data/i18n/").unwrap();
        let settings = Settings::try_from_args(Locale::from_str("en-GB").unwrap(), &args).unwrap();
        let expected_file_name = PathBuf::from("en-GB.ftl");
        assert_eq!(
            settings.reference_path().file_name().unwrap(),
            expected_file_name.file_name().unwrap()
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn fail_to_have_reference_file_if_not_provided_and_sys_locale_file_not_in_target_folder() {
        let args = AnalysisArgs::from_str("app_name -t tests/data/i18n_empty/").unwrap();
        let error = Settings::try_from_args(Locale::default(), &args).unwrap_err();
        assert!(matches!(error, LingoraError::CannotFindReferenceFile(_)));
        insta::with_settings!({filters => vec![
            (r"[a-z]{2}(-[A-Z]{2})?\.ftl", "<langid>.ftl")
        ]}, {
            insta::assert_snapshot!(error, @"cannot find reference file: <langid>.ftl")
        });
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn fail_to_have_reference_file_if_not_provided_and_multiple_sys_locale_files_in_target_folder()
    {
        let args = AnalysisArgs::from_str("app_name -t tests/data/i18n_duplicates").unwrap();
        let error = Settings::try_from_args(Locale::from_str("en-GB").unwrap(), &args).unwrap_err();
        assert!(matches!(error, LingoraError::AmbiguousReferenceFiles(_)));
        insta::assert_snapshot!(error, @r"
        ambiguous reference files:
          tests/data/i18n_duplicates/one/en-GB.ftl
          tests/data/i18n_duplicates/two/en-GB.ftl
        ");
    }

    fn count_files(file_name: &str, files: &[PathBuf]) -> usize {
        files
            .iter()
            .filter(|f| f.ends_with(PathBuf::from(file_name)))
            .count()
    }

    #[test]
    fn have_targets_when_explicitly_provided_command_line_arguments() {
        let args = AnalysisArgs::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let settings = Settings::try_from_args(Locale::default(), &args).unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn not_have_reference_in_targets_even_when_requested() {
        let args = AnalysisArgs::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let settings = Settings::try_from_args(Locale::default(), &args).unwrap();

        let targets = settings.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", &targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", &targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", &targets), 1);
    }

    #[test]
    fn have_targets_when_implicitly_provided_command_line_arguments() {
        let args =
            AnalysisArgs::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/")
                .unwrap();
        let settings = Settings::try_from_args(Locale::default(), &args).unwrap();

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
            settings.reference_path().file_name(),
            PathBuf::from("en-GB.ftl").file_name()
        );
    }
}

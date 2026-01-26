use std::{fs, io, path::Path};

use lingora_core::prelude::*;

use crate::{args::CliArgs, error::CliError};

pub struct App {
    settings: LingoraToml,
    audit_result: AuditResult,
}

impl App {
    pub fn output_audit_report<W: io::Write>(&self, out: &mut W) -> Result<(), CliError> {
        let renderer = AnalysisRenderer::new(&self.audit_result);
        renderer.render(out)?;
        Ok(())
    }

    pub fn output_dioxus_i18n_config(&self, path: &Path) -> Result<(), CliError> {
        let base_path = path.parent();
        let mut file = fs::File::create_new(path)?;
        let workspace = self.audit_result.workspace();
        let renderer = DioxusI18nConfigRenderer::new(&self.settings, workspace, base_path);
        renderer.render(&mut file)?;
        Ok(())
    }

    pub fn exit_status(&self) -> Result<(), CliError> {
        if self.audit_result.is_ok() {
            Ok(())
        } else {
            Err(CliError::IntegrityErrorsDetected)
        }
    }
}

impl TryFrom<&LingoraToml> for App {
    type Error = CliError;

    fn try_from(settings: &LingoraToml) -> Result<Self, Self::Error> {
        let settings = settings.clone();

        let engine = AuditEngine::try_from(&settings)?;
        let audit_result = engine.run()?;

        Ok(Self {
            settings,
            audit_result,
        })
    }
}

impl TryFrom<&CliArgs> for App {
    type Error = CliError;

    fn try_from(value: &CliArgs) -> Result<Self, Self::Error> {
        let settings = LingoraToml::try_from(value.core_args())?;
        Self::try_from(&settings)
    }
}

// #[cfg(test)]
// mod test {
//     use std::{fs, str::FromStr};

//     use tempfile::TempPath;

//     use super::*;

//     fn do_output_analysis(settings: &LingoraToml) -> String {
//         let out_buffer = Vec::new();
//         let mut out = io::BufWriter::new(out_buffer);

//         let app = App::try_from(settings).unwrap();

//         app.output_audit_report(&mut out).unwrap();

//         let bytes = out.buffer();
//         String::from_utf8_lossy(bytes).to_string()
//     }

//     #[test]
//     fn app_will_output_checks_when_no_errors() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// reference = "tests/data/cross_check/reference_matching.ftl"
// targets = ["tests/data/cross_check/target_matching.ftl"]
// "#,
//         )
//         .unwrap();

//         let result = do_output_analysis(&settings);
//         insta::assert_snapshot!(result, @r"
//         Reference: tests/data/cross_check/reference_matching.ftl - Ok
//         Target: tests/data/cross_check/target_matching.ftl - Ok
//         ");
//     }

//     #[test]
//     fn app_will_output_checks_when_errors() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// fluent_sources = ["tests/data/cross_check/reference_missing.ftl"
// targets = ["tests/data/cross_check/target_redundant.ftl"]
// "#,
//         )
//         .unwrap();

//         let result = do_output_analysis(&settings);
//         insta::assert_snapshot!(result, @r"
//         Reference: tests/data/cross_check/reference_missing.ftl - Ok
//         Target: tests/data/cross_check/target_redundant.ftl
//             Missing translation: -missing-term
//                                  missing-message
//             Superfluous translation: -superfluous-term
//                                      superfluous-message
//         ");
//     }

//     fn create_temp_filepath() -> TempPath {
//         let file = tempfile::NamedTempFile::new().unwrap();

//         let temp_path = file.into_temp_path();
//         let path = temp_path.to_path_buf();
//         fs::remove_file(&path).expect("temporary file must be deleted");

//         temp_path
//     }

//     #[test]
//     fn will_output_dioxus_i18n_config_for_auto() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// root = "tests/data/i18n"
// reference = "tests/data/i18n/en/en-GB.ftl"
// [dioxus_i18n]
// with_locale = "auto"
// fallback = "en-GB"
// "#,
//         )
//         .unwrap();

//         let path = create_temp_filepath();
//         let app = App::try_from(&settings).unwrap();
//         app.output_dioxus_i18n_config(&path).unwrap();

//         let content = fs::read_to_string(path).unwrap();
//         insta::assert_snapshot!(content, @r#"
//         use dioxus_i18n::{prelude::*, *};
//         use unic_langid::{langid, LanguageIdentifier};
//         use std::path::PathBuf;

//         pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
//             I18nConfig::new(initial_language)
//                 .with_auto_locales(PathBuf::from("tests/data/i18n"))
//                 .with_fallback(langid!("en-GB"))
//         }
//         "#);
//     }

//     #[test]
//     #[cfg(not(target_os = "windows"))]
//     fn will_output_dioxus_i18n_config_for_pathbuf() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// root = "tests/data/i18n"
// reference = "tests/data/i18n/en/en-GB.ftl"
// [dioxus_i18n]
// with_locale = "pathbuf"
// fallback = "en-GB"
// "#,
//         )
//         .unwrap();

//         let path = create_temp_filepath();
//         let app = App::try_from(&settings).unwrap();
//         app.output_dioxus_i18n_config(&path).unwrap();

//         let content = fs::read_to_string(path).unwrap();
//         insta::assert_snapshot!(content, @r#"
//         use dioxus_i18n::{prelude::*, *};
//         use unic_langid::{langid, LanguageIdentifier};
//         use std::path::PathBuf;

//         pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
//             I18nConfig::new(initial_language)
//                 .with_locale((
//                     langid!("en-AU"),
//                     PathBuf::from("tests/data/i18n/en/en-AU.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-GB"),
//                     PathBuf::from("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en"),
//                     PathBuf::from("tests/data/i18n/en/en.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-IT"),
//                     PathBuf::from("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_fallback(langid!("en-GB"))
//         }
//         "#);
//     }

//     #[test]
//     #[cfg(not(target_os = "windows"))]
//     fn will_output_dioxus_i18n_config_for_include_str() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// fluent_sources = ["tests/data/i18n"]
// canonical = "en-GB"
// primaries = ["it-IT"]
// [dioxus_i18n]
// config_inclusion = "includestr"
// "#,
//         )
//         .unwrap();

//         let path = create_temp_filepath();
//         let app = App::try_from(&settings).unwrap();
//         app.output_dioxus_i18n_config(&path).unwrap();

//         let content = fs::read_to_string(path).unwrap();
//         insta::assert_snapshot!(content, @r#"
//         use dioxus_i18n::{prelude::*, *};
//         use unic_langid::{langid, LanguageIdentifier};

//         pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
//             I18nConfig::new(initial_language)
//                 .with_locale((
//                     langid!("en-AU"),
//                     include_str!("tests/data/i18n/en/en-AU.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-GB"),
//                     include_str!("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en"),
//                     include_str!("tests/data/i18n/en/en.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-IT"),
//                     include_str!("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_fallback(langid!("en-GB"))
//         }
//         "#);
//     }

//     #[test]
//     #[cfg(not(target_os = "windows"))]
//     fn will_output_dioxus_i18n_config_shares_for_pathbuf() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// fluent_sources = ["tests/data/i18n"]
// canonical = "en-GB"
// primaries = ["it-IT"]
// [dioxus_i18n]
// config_inclusion = "includestr"
// "#,
//         )
//         .unwrap();

//         let path = create_temp_filepath();
//         let app = App::try_from(&settings).unwrap();
//         app.output_dioxus_i18n_config(&path).unwrap();

//         let content = fs::read_to_string(path).unwrap();
//         insta::assert_snapshot!(content, @r#"
//         use dioxus_i18n::{prelude::*, *};
//         use unic_langid::{langid, LanguageIdentifier};
//         use std::path::PathBuf;

//         pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
//             I18nConfig::new(initial_language)
//                 .with_locale((
//                     langid!("en-AU"),
//                     PathBuf::from("tests/data/i18n/en/en-AU.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-GB"),
//                     PathBuf::from("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en"),
//                     PathBuf::from("tests/data/i18n/en/en.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-IT"),
//                     PathBuf::from("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-US"),
//                     PathBuf::from("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it"),
//                     PathBuf::from("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-CH"),
//                     PathBuf::from("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_fallback(langid!("en-GB"))
//         }
//         "#);
//     }

//     #[test]
//     #[cfg(not(target_os = "windows"))]
//     fn will_output_dioxus_i18n_config_shares_for_include_str() {
//         let settings = LingoraToml::from_str(
//             r#"
// [lingora]
// fluent_sources = ["tests/data/i18n"]
// canonical = "en-GB"
// primaries = ["it-IT"]
// [dioxus_i18n]
// config_inclusion = "includestr"
// "#,
//         )
//         .unwrap();

//         let path = create_temp_filepath();
//         let app = App::try_from(&settings).unwrap();
//         app.output_dioxus_i18n_config(&path).unwrap();

//         let content = fs::read_to_string(path).unwrap();
//         insta::assert_snapshot!(content, @r#"
//         use dioxus_i18n::{prelude::*, *};
//         use unic_langid::{langid, LanguageIdentifier};

//         pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
//             I18nConfig::new(initial_language)
//                 .with_locale((
//                     langid!("en-AU"),
//                     include_str!("tests/data/i18n/en/en-AU.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-GB"),
//                     include_str!("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en"),
//                     include_str!("tests/data/i18n/en/en.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-IT"),
//                     include_str!("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_locale((
//                     langid!("en-US"),
//                     include_str!("tests/data/i18n/en/en-GB.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it"),
//                     include_str!("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_locale((
//                     langid!("it-CH"),
//                     include_str!("tests/data/i18n/it/it-IT.ftl")
//                 ))
//                 .with_fallback(langid!("en-GB"))
//         }
//         "#);
//     }
// }

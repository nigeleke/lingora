use std::{path::PathBuf, sync::OnceLock};

use dioxus::prelude::*;
use thiserror::*;

use super::{
    config::{Arguments, Settings},
    domain::{Analysis, IntegrityChecks, Locale},
    gui::{App as AppComponent, AppProps as AppComponentProps},
    output::{AnalysisWriter, DioxusI18nConfigWriter, Writer},
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("integrity errors detected")]
    IntegrityErrorsDetected,

    #[error("internal problem: {0}; raise issue")]
    InternalIssue(String),

    #[error("output failed: {0}")]
    OutputFailed(String),
}

type Result<T> = std::result::Result<T, AppError>;

static SETTINGS: OnceLock<Settings> = OnceLock::new();
static ANALYSIS: OnceLock<Analysis> = OnceLock::new();

pub struct App {
    settings: Settings,
    analysis: Analysis,
}

impl App {
    pub fn try_from_arguments(locale: Locale, arguments: &Arguments) -> Result<Self> {
        let settings = Settings::try_from_arguments(locale, arguments)
            .map_err(|e| AppError::InvalidArguments(e.to_string()))?;
        Self::try_from_settings(&settings)
    }

    fn try_from_settings(settings: &Settings) -> Result<Self> {
        let checks = IntegrityChecks::try_from(settings)
            .map_err(|e| AppError::AnalysisFailed(e.to_string()))?;
        let analysis = Analysis::from(checks);

        Ok(Self {
            analysis,
            settings: settings.clone(),
        })
    }

    pub fn output_analysis(&self, writer: Writer) -> Result<()> {
        let writer = AnalysisWriter::new(self.settings.reference(), &self.analysis, writer);
        writer
            .write()
            .map_err(|e| AppError::OutputFailed(e.to_string()))
    }

    pub fn output_dioxus_i18n(&self, path: &PathBuf) -> Result<()> {
        let writer = DioxusI18nConfigWriter::new(&self.settings, path);
        writer
            .write()
            .map_err(|e| AppError::OutputFailed(e.to_string()))
    }

    pub fn show_gui(&self) {
        SETTINGS.set(self.settings.clone()).unwrap();
        ANALYSIS.set(self.analysis.clone()).unwrap();
        dioxus::launch(Self::gui_app);
    }

    fn gui_app() -> Element {
        let settings = SETTINGS.get().expect("Settings should be set").clone();
        let analysis = ANALYSIS.get().expect("Analyis should be set").clone();

        let builder = AppComponentProps::builder()
            .settings(settings)
            .analysis(analysis);

        let props = builder.build();
        AppComponent(props)
    }

    pub fn exit_status(&self) -> Result<()> {
        if self.analysis.is_ok() {
            Ok(())
        } else {
            Err(AppError::IntegrityErrorsDetected)
        }
    }
}

impl TryFrom<&Arguments> for App {
    type Error = AppError;

    fn try_from(value: &Arguments) -> std::result::Result<Self, Self::Error> {
        App::try_from_arguments(Locale::default(), value)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    fn do_output_analysis(settings: &Settings) -> String {
        let stdout_buffer = Vec::new();
        let stdout = Rc::new(RefCell::new(std::io::BufWriter::new(stdout_buffer)));

        let app = App::try_from_settings(settings).unwrap();

        app.output_analysis(stdout.clone()).unwrap();

        let stdout = stdout.borrow();
        let bytes = stdout.buffer();
        String::from_utf8_lossy(bytes).to_string()
    }

    #[test]
    fn app_will_output_checks_when_no_errors() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
reference = "tests/data/cross_check/reference_matching.ftl"
targets = ["tests/data/cross_check/target_matching.ftl"]
"#,
        )
        .unwrap();

        let result = do_output_analysis(&settings);
        insta::assert_snapshot!(result, @r"
        Reference: tests/data/cross_check/reference_matching.ftl - Ok
        Target: tests/data/cross_check/target_matching.ftl - Ok
        ");
    }

    #[test]
    fn app_will_output_checks_when_errors() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
reference = "tests/data/cross_check/reference_missing.ftl"
targets = ["tests/data/cross_check/target_superfluous.ftl"]
"#,
        )
        .unwrap();

        let result = do_output_analysis(&settings);
        insta::assert_snapshot!(result, @r"
        Reference: tests/data/cross_check/reference_missing.ftl - Ok
        Target: tests/data/cross_check/target_superfluous.ftl
            Missing translation: -missing-term
                                 missing-message
            Superfluous translation: -superfluous-term
                                     superfluous-message
        ");
    }

    #[test]
    fn will_output_dioxus_i18n_config_for_auto() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
root = "tests/data/i18n"
reference = "tests/data/i18n/en/en-GB.ftl"
[dioxus_i18n]
with_locale = "auto"
fallback = "en-GB"
"#,
        )
        .unwrap();

        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let app = App::try_from_settings(&settings).unwrap();
        app.output_dioxus_i18n(&path).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(content, @r#"
        use dioxus_i18n::{prelude::*, *};
        use unic_langid::{langid, LanguageIdentifier};
        use std::path::PathBuf;

        pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
            I18nConfig::new(initial_language)
                .with_auto_locales(PathBuf::from("tests/data/i18n"))
                .with_fallback(langid!("en-GB"))
        }
        "#);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_for_pathbuf() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
root = "tests/data/i18n"
reference = "tests/data/i18n/en/en-GB.ftl"
[dioxus_i18n]
with_locale = "pathbuf"
fallback = "en-GB"
"#,
        )
        .unwrap();

        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let app = App::try_from_settings(&settings).unwrap();
        app.output_dioxus_i18n(&path).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(content, @r#"
        use dioxus_i18n::{prelude::*, *};
        use unic_langid::{langid, LanguageIdentifier};
        use std::path::PathBuf;

        pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
            I18nConfig::new(initial_language)
                .with_locale((
                    langid!("en-AU"),
                    PathBuf::from("tests/data/i18n/en/en-AU.ftl")
                ))
                .with_locale((
                    langid!("en-GB"),
                    PathBuf::from("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("en"),
                    PathBuf::from("tests/data/i18n/en/en.ftl")
                ))
                .with_locale((
                    langid!("it-IT"),
                    PathBuf::from("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_fallback(langid!("en-GB"))
        }
        "#);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_for_include_str() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
root = "tests/data/i18n"
reference = "tests/data/i18n/en/en-GB.ftl"
[dioxus_i18n]
with_locale = "includestr"
fallback = "en-GB"
"#,
        )
        .unwrap();

        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let app = App::try_from_settings(&settings).unwrap();
        app.output_dioxus_i18n(&path).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(content, @r#"
        use dioxus_i18n::{prelude::*, *};
        use unic_langid::{langid, LanguageIdentifier};


        pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
            I18nConfig::new(initial_language)
                .with_locale((
                    langid!("en-AU"),
                    include_str!("tests/data/i18n/en/en-AU.ftl")
                ))
                .with_locale((
                    langid!("en-GB"),
                    include_str!("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("en"),
                    include_str!("tests/data/i18n/en/en.ftl")
                ))
                .with_locale((
                    langid!("it-IT"),
                    include_str!("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_fallback(langid!("en-GB"))
        }
        "#);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_shares_for_pathbuf() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
root = "tests/data/i18n"
reference = "tests/data/i18n/en/en-GB.ftl"
[dioxus_i18n]
with_locale = "pathbuf"
fallback = "en-GB"
shares = [["en-US", "en-GB"], ["it", "it-IT"], ["it-CH", "it-IT"]]
"#,
        )
        .unwrap();

        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let app = App::try_from_settings(&settings).unwrap();
        app.output_dioxus_i18n(&path).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(content, @r#"
        use dioxus_i18n::{prelude::*, *};
        use unic_langid::{langid, LanguageIdentifier};
        use std::path::PathBuf;

        pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
            I18nConfig::new(initial_language)
                .with_locale((
                    langid!("en-AU"),
                    PathBuf::from("tests/data/i18n/en/en-AU.ftl")
                ))
                .with_locale((
                    langid!("en-GB"),
                    PathBuf::from("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("en"),
                    PathBuf::from("tests/data/i18n/en/en.ftl")
                ))
                .with_locale((
                    langid!("it-IT"),
                    PathBuf::from("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_locale((
                    langid!("en-US"),
                    PathBuf::from("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("it"),
                    PathBuf::from("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_locale((
                    langid!("it-CH"),
                    PathBuf::from("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_fallback(langid!("en-GB"))
        }
        "#);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_shares_for_include_str() {
        let settings = Settings::try_from_str(
            Locale::default(),
            r#"
[lingora]
root = "tests/data/i18n"
reference = "tests/data/i18n/en/en-GB.ftl"
[dioxus_i18n]
with_locale = "includestr"
fallback = "en-GB"
shares = [["en-US", "en-GB"], ["it", "it-IT"], ["it-CH", "it-IT"]]
"#,
        )
        .unwrap();

        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let app = App::try_from_settings(&settings).unwrap();
        app.output_dioxus_i18n(&path).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(content, @r#"
        use dioxus_i18n::{prelude::*, *};
        use unic_langid::{langid, LanguageIdentifier};


        pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
            I18nConfig::new(initial_language)
                .with_locale((
                    langid!("en-AU"),
                    include_str!("tests/data/i18n/en/en-AU.ftl")
                ))
                .with_locale((
                    langid!("en-GB"),
                    include_str!("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("en"),
                    include_str!("tests/data/i18n/en/en.ftl")
                ))
                .with_locale((
                    langid!("it-IT"),
                    include_str!("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_locale((
                    langid!("en-US"),
                    include_str!("tests/data/i18n/en/en-GB.ftl")
                ))
                .with_locale((
                    langid!("it"),
                    include_str!("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_locale((
                    langid!("it-CH"),
                    include_str!("tests/data/i18n/it/it-IT.ftl")
                ))
                .with_fallback(langid!("en-GB"))
        }
        "#);
    }
}

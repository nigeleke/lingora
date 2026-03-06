use std::{fs, io, path::Path};

use lingora_core::prelude::*;

use crate::{args::CliArgs, error::CliError};

/// High-level application context for `lingora-cli`.
///
/// `App` owns:
/// - the loaded configuration (`LingoraToml`)
/// - the result of the full audit (`AuditResult`)
///
/// This struct acts as the bridge between parsed CLI arguments, core engine execution,
/// and output/rendering logic.
pub struct App {
    settings: LingoraToml,
    audit_result: AuditResult,
}

impl App {
    /// Renders the audit report using `AnalysisRenderer` to the given writer.
    ///
    /// This is the primary way to produce the human-readable report in standard mode.
    /// The renderer groups issues hierarchically (workspace → canonical → primaries → variants → orphans).
    ///
    /// # Errors
    /// Returns `CliError::Io` if writing to the output fails.
    pub fn output_audit_report<W: io::Write>(&self, out: &mut W) -> Result<(), CliError> {
        let renderer = AnalysisRenderer::new(&self.audit_result);
        renderer.render(out)?;
        Ok(())
    }

    /// Generates `dioxus_i18n::I18nConfig` Rust code and writes it to the specified file.
    ///
    /// - Uses `DioxusI18nConfigRenderer` with the current `settings` and `workspace`
    /// - Computes relative paths from the **parent directory** of the target file
    ///   (so `include_str!` or `PathBuf::from` paths are correct relative to the generated file)
    /// - Creates the file (fails if it already exists — use `create_new` for safety)
    ///
    /// # Arguments
    /// * `path` — destination path (e.g. `src/i18n_config.rs`)
    ///
    /// # Errors
    /// - `CliError::Io` on file creation/write failure
    /// - Propagates any renderer errors (rare, usually path resolution)
    pub fn output_dioxus_i18n_config(&self, path: &Path) -> Result<(), CliError> {
        let base_path = path.parent();
        let mut file = fs::File::create_new(path)?;
        let workspace = self.audit_result.workspace();
        let renderer = DioxusI18nConfigRenderer::new(&self.settings, workspace, base_path);
        renderer.render(&mut file)?;
        Ok(())
    }

    /// Returns `Ok(())` if the audit found **no issues**, otherwise returns
    /// `Err(CliError::IntegrityErrorsDetected)`.
    ///
    /// Determines the final exit code in `main`:
    /// - `0` → everything is perfect
    /// - non-zero → issues were found (even if parsing/execution succeeded)
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

#[cfg(test)]
mod test {
    use std::{env, fs, str::FromStr};

    use tempfile::TempPath;

    use super::*;

    fn do_output_analysis(settings: &LingoraToml) -> String {
        let out_buffer = Vec::new();
        let mut out = io::BufWriter::new(out_buffer);

        let app = App::try_from(settings).unwrap();

        app.output_audit_report(&mut out).unwrap();

        let bytes = out.buffer();
        String::from_utf8_lossy(bytes).to_string()
    }

    fn with_filters(f: impl FnOnce()) {
        let mut settings = insta::Settings::clone_current();
        let manifest_dir = regex::escape(env!("CARGO_MANIFEST_DIR"));
        let manifest_dir = Path::new(&manifest_dir)
            .parent()
            .expect("require CARGO_MANIFEST_DIR parent")
            .display()
            .to_string();
        settings.add_filter(&manifest_dir, "...");
        settings.bind(f)
    }

    #[test]
    fn app_will_output_checks_when_no_errors() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n/en", "../core/tests/data/i18n/it"]
canonical = "en-GB"
primaries = ["it-IT"]
"#,
        )
        .unwrap();

        let result = do_output_analysis(&settings);

        with_filters(|| {
            insta::assert_snapshot!(result, @r"
            Language:  en
            Canonical: en-GB - Ok
            Variant:   en-AU - Ok
            Language:  it
            Primary:   it-IT - Ok
            ");
        })
    }

    #[test]
    fn app_will_output_checks_when_errors() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
"#,
        )
        .unwrap();

        let result = do_output_analysis(&settings);

        with_filters(|| {
            insta::assert_snapshot!(result, @r"
            Language:  en
            Canonical: en-GB - Ok
            Variant:   en-AU - Ok
            Language:  fr
            Primary:   fr-FR
                       missing translation 'en'
                       missing translation 'en-AU'
                       missing translation 'en-GB'
            Language:  it
            Primary:   it-IT - Ok
            Language:  sr
            Primary:   sr-Cyrl-RS
                       missing translation 'en-GB'
                       redundant translation '-en-GB'
            Variant:   sr-Cyrl-BA - Ok
            ");
        });
    }

    fn create_temp_filepath() -> TempPath {
        let file = tempfile::NamedTempFile::new().unwrap();

        let temp_path = file.into_temp_path();
        let path = temp_path.to_path_buf();
        fs::remove_file(&path).expect("temporary file must be deleted");

        temp_path
    }

    #[test]
    fn will_output_dioxus_i18n_config_for_auto() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n_semantic"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
[dioxus_i18n]
config_inclusion = "auto"
"#,
        )
        .unwrap();

        let path = create_temp_filepath();
        let app = App::try_from(&settings).unwrap();
        app.output_dioxus_i18n_config(&path).unwrap();

        let content = fs::read_to_string(path).unwrap();

        with_filters(|| {
            insta::assert_snapshot!(content, @r#"
            use dioxus_i18n::{prelude::*, *};
            use unic_langid::{langid, LanguageIdentifier};
            use std::path::PathBuf;

            pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
                I18nConfig::new(initial_language)
                    .with_auto_locales(PathBuf::from(".../core/tests/data/i18n_semantic"))
                    .with_locale(langid!("en"), PathBuf::from(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl"))
                    .with_locale(langid!("it"), PathBuf::from(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl"))
                    .with_fallback(langid!("en-GB"))
            }
            "#);
        });
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_for_pathbuf() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n_semantic"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
[dioxus_i18n]
config_inclusion = "pathbuf"
"#,
        )
        .unwrap();

        let path = create_temp_filepath();
        let app = App::try_from(&settings).unwrap();
        app.output_dioxus_i18n_config(&path).unwrap();

        let content = fs::read_to_string(path).unwrap();

        with_filters(|| {
            insta::assert_snapshot!(content, @r#"
            use dioxus_i18n::{prelude::*, *};
            use unic_langid::{langid, LanguageIdentifier};
            use std::path::PathBuf;

            pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
                I18nConfig::new(initial_language)
                    .with_locale((
                        langid!("en-AU"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/en/en-AU/errors.ftl")
                    ))
                    .with_locale((
                        langid!("en-GB"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl")
                    ))
                    .with_locale((
                        langid!("it-IT"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl")
                    ))
                    .with_locale(langid!("en"), PathBuf::from(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl"))
                    .with_locale(langid!("it"), PathBuf::from(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl"))
                    .with_fallback(langid!("en-GB"))
            }
            "#);
        });
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_for_include_str() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n_semantic"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
[dioxus_i18n]
config_inclusion = "includestr"
"#,
        )
        .unwrap();

        let path = create_temp_filepath();
        let app = App::try_from(&settings).unwrap();
        app.output_dioxus_i18n_config(&path).unwrap();

        let content = fs::read_to_string(path).unwrap();

        with_filters(|| {
            insta::assert_snapshot!(content, @r#"
            use dioxus_i18n::{prelude::*, *};
            use unic_langid::{langid, LanguageIdentifier};


            pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
                I18nConfig::new(initial_language)
                    .with_locale((
                        langid!("en-AU"),
                        include_str!(".../core/tests/data/i18n_semantic/en/en-AU/errors.ftl")
                    ))
                    .with_locale((
                        langid!("en-GB"),
                        include_str!(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl")
                    ))
                    .with_locale((
                        langid!("it-IT"),
                        include_str!(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl")
                    ))
                    .with_locale(langid!("en"), include_str!(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl"))
                    .with_locale(langid!("it"), include_str!(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl"))
                    .with_fallback(langid!("en-GB"))
            }
            "#);
        });
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_shares_for_pathbuf() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n_semantic"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
[dioxus_i18n]
config_inclusion = "pathbuf"
"#,
        )
        .unwrap();

        let path = create_temp_filepath();
        let app = App::try_from(&settings).unwrap();
        app.output_dioxus_i18n_config(&path).unwrap();

        let content = fs::read_to_string(path).unwrap();

        with_filters(|| {
            insta::assert_snapshot!(content, @r#"
            use dioxus_i18n::{prelude::*, *};
            use unic_langid::{langid, LanguageIdentifier};
            use std::path::PathBuf;

            pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
                I18nConfig::new(initial_language)
                    .with_locale((
                        langid!("en-AU"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/en/en-AU/errors.ftl")
                    ))
                    .with_locale((
                        langid!("en-GB"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl")
                    ))
                    .with_locale((
                        langid!("it-IT"),
                        PathBuf::from(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl")
                    ))
                    .with_locale(langid!("en"), PathBuf::from(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl"))
                    .with_locale(langid!("it"), PathBuf::from(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl"))
                    .with_fallback(langid!("en-GB"))
            }
            "#);
        });
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn will_output_dioxus_i18n_config_shares_for_include_str() {
        let settings = LingoraToml::from_str(
            r#"
[lingora]
fluent_sources = ["../core/tests/data/i18n_semantic"]
canonical = "en-GB"
primaries = ["fr-FR", "it-IT", "sr-Cyrl-RS"]
[dioxus_i18n]
config_inclusion = "includestr"
"#,
        )
        .unwrap();

        let path = create_temp_filepath();
        let app = App::try_from(&settings).unwrap();
        app.output_dioxus_i18n_config(&path).unwrap();

        let content = fs::read_to_string(path).unwrap();

        with_filters(|| {
            insta::assert_snapshot!(content, @r#"
            use dioxus_i18n::{prelude::*, *};
            use unic_langid::{langid, LanguageIdentifier};


            pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
                I18nConfig::new(initial_language)
                    .with_locale((
                        langid!("en-AU"),
                        include_str!(".../core/tests/data/i18n_semantic/en/en-AU/errors.ftl")
                    ))
                    .with_locale((
                        langid!("en-GB"),
                        include_str!(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl")
                    ))
                    .with_locale((
                        langid!("it-IT"),
                        include_str!(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl")
                    ))
                    .with_locale(langid!("en"), include_str!(".../core/tests/data/i18n_semantic/en/en-GB/errors.ftl"))
                    .with_locale(langid!("it"), include_str!(".../core/tests/data/i18n_semantic/it/it-IT/errors.ftl"))
                    .with_fallback(langid!("en-GB"))
            }
            "#);
        });
    }
}

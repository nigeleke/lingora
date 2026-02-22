use std::path::PathBuf;

use clap::*;

use crate::{
    config::config_inclusion_style::ConfigInclusionStyle, domain::Locale, error::LingoraError,
};

/// Core command-line arguments shared between `lingora-cli` and `lingora-tui`.
///
/// These arguments represent settings that can be overridden when running
/// `lingora-cli` and `lingora-tui` from the command line. They allow users to:
///
/// - Point to a specific configuration file (or otherwise rely on auto-discovery of `Lingora.toml`)
/// - Supplement or override the translation files and locales defined by the configuration.
/// - Enable Rust source scanning for `dioxus_i18n` macro usage
/// - Control how generated `I18nConfig` code includes locales
#[derive(Debug, Parser)]
pub struct CoreArgs {
    /// Config file.
    /// The config file contains attributes, most of which can also be overridden
    /// by command line arguments.
    /// If no config file is provided and `Lingora.toml` exists in the current
    /// working directory then that will be used. If `Lingora.toml` doesn't exist
    /// then sensible defaults will be used.
    #[clap(long, default_value = None)]
    pub(crate) config: Option<PathBuf>,

    /// Paths to translation files or folders containing translation files.
    /// Files or folders provided here will be added to those defined in
    /// the 'Lingora.toml' configuration file.
    #[clap(long, value_delimiter = ',')]
    pub(crate) fluent_sources: Vec<PathBuf>,

    /// Canonical locale.
    /// This is expected to be defined in the Lingora.toml file, but can be
    /// overriddem here.
    #[clap(long, default_value = None)]
    pub(crate) canonical: Option<Locale>,

    /// Primary locales for the translation.
    /// Supported translations are expected to be defined in the Lingora.toml
    /// file. Any primaries defined here will be included in the analysis in
    /// addition to those in the Lingora.toml config file.
    #[arg(long, value_delimiter = ',')]
    pub(crate) primaries: Vec<Locale>,

    /// The paths for rust source files that may use dioxus_i18n macros.
    /// If provided Lingora will analyse the source files for usage of the
    /// `dioxus_i18n::t!`, `te!` and `tid!` macros. If parsable (const str)
    /// it check that the identifier exists in the reference file and produce
    /// a warning if it is missing.
    #[clap(long, value_delimiter = ',')]
    pub(crate) rust_sources: Vec<PathBuf>,

    /// Define which dioxus_i18n::I18nConfig with_locale method will be used
    /// during rendering of the config.rs code.
    #[clap(long, default_value = None)]
    pub(crate) config_inclusion: Option<ConfigInclusionStyle>,
}

impl std::str::FromStr for CoreArgs {
    type Err = LingoraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split_whitespace();
        let args = CoreArgs::try_parse_from(value)?;
        Ok(args)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn no_config_file_by_default() {
        let args = CoreArgs::from_str("").unwrap();
        assert_eq!(args.config, None);
    }

    #[test]
    fn user_can_provide_config_file() {
        let args = CoreArgs::from_str("app_name --config=tests/data/i18n/Lingora.toml").unwrap();
        assert_eq!(
            args.config,
            Some(PathBuf::from("tests/data/i18n/Lingora.toml"))
        );
    }

    #[test]
    fn no_additional_fluent_sources_by_default() {
        let args = CoreArgs::from_str("").unwrap();
        assert!(args.fluent_sources.is_empty());
    }

    #[test]
    fn user_can_provide_fluents_sources() {
        let args = CoreArgs::from_str("app_name --fluent-sources=tests/data/i18n").unwrap();
        assert_eq!(args.fluent_sources, [PathBuf::from("tests/data/i18n")]);
    }

    #[test]
    fn no_canonical_locale_by_default() {
        let args = CoreArgs::from_str("").unwrap();
        assert_eq!(args.canonical, None);
    }

    #[test]
    fn user_can_provide_canonical_locale() {
        let args = CoreArgs::from_str("app_name --canonical=en-GB").unwrap();
        assert_eq!(args.canonical, Locale::from_str("en-GB").ok());
    }

    #[test]
    fn no_additional_primary_locales_by_default() {
        let args = CoreArgs::from_str("").unwrap();
        assert!(args.primaries.is_empty())
    }

    #[test]
    fn user_can_provide_additional_primary_locale() {
        let args = CoreArgs::from_str("app_name --primaries=it-IT").unwrap();
        assert_eq!(args.primaries, [Locale::from_str("it-IT").unwrap()])
    }

    #[test]
    fn user_can_provide_multiple_additional_primary_locales() {
        let args = CoreArgs::from_str("app_name --primaries=en-GB,en-US,it-IT").unwrap();
        assert_eq!(
            args.primaries,
            [
                Locale::from_str("en-GB").unwrap(),
                Locale::from_str("en-US").unwrap(),
                Locale::from_str("it-IT").unwrap()
            ]
        )
    }
}

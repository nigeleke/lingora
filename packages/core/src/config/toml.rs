use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    config::{args::CoreArgs, config_inclusion_style::ConfigInclusionStyle},
    domain::Locale,
    error::LingoraError,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct EngineSettings {
    pub(crate) fluent_sources: Vec<PathBuf>,
    pub(crate) canonical: Locale,
    pub(crate) primaries: Vec<Locale>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct DioxusI18nSettings {
    pub(crate) rust_sources: Vec<PathBuf>,
    pub(crate) config_inclusion: ConfigInclusionStyle,
}

/// Top-level deserialized structure of a `Lingora.toml` configuration file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LingoraToml {
    pub(crate) lingora: EngineSettings,
    pub(crate) dioxus_i18n: DioxusI18nSettings,
}

impl std::str::FromStr for LingoraToml {
    type Err = LingoraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = toml::from_str(s)?;
        Ok(toml)
    }
}

impl TryFrom<&Path> for LingoraToml {
    type Error = LingoraError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }
}

impl TryFrom<&CoreArgs> for LingoraToml {
    type Error = LingoraError;

    fn try_from(args: &CoreArgs) -> Result<Self, Self::Error> {
        let default_toml_path = Path::new("Lingora.toml");

        let mut toml = if let Some(requested_toml_path) = &args.config {
            Self::try_from(requested_toml_path.as_path())
        } else if std::fs::exists(default_toml_path).unwrap_or(false) {
            Self::try_from(default_toml_path)
        } else {
            Ok(Self::default())
        }?;

        toml.lingora
            .fluent_sources
            .append(&mut args.fluent_sources.clone());

        if let Some(locale) = &args.canonical {
            toml.lingora.canonical = locale.clone();
        }

        toml.lingora.primaries.append(&mut args.primaries.clone());

        toml.dioxus_i18n
            .rust_sources
            .append(&mut args.rust_sources.clone());

        if let Some(style) = &args.config_inclusion {
            toml.dioxus_i18n.config_inclusion = *style;
        }

        Ok(toml)
    }
}

impl Default for LingoraToml {
    fn default() -> Self {
        let fluent_sources = vec![Path::new("./i18n").to_path_buf()];
        let canonical = Locale::default();
        let primaries = Vec::default();
        let rust_sources = vec![Path::new("./src").to_path_buf()];
        let config_inclusion = ConfigInclusionStyle::Auto;

        Self {
            lingora: EngineSettings {
                fluent_sources,
                canonical,
                primaries,
            },
            dioxus_i18n: DioxusI18nSettings {
                rust_sources,
                config_inclusion,
            },
        }
    }
}

impl std::fmt::Display for LingoraToml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let toml =
            toml::to_string_pretty(self).unwrap_or("failed to re-create lingora toml text".into());
        toml.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use std::{path::Path, str::FromStr};

    use crate::{
        config::toml::{ConfigInclusionStyle, LingoraToml},
        domain::Locale,
        error::LingoraError,
    };

    #[test]
    fn will_load_from_toml_file() {
        let path = Path::new("./tests/data/toml/Lingora.toml");
        let toml = LingoraToml::try_from(path).expect("failed to load from toml");
        assert_eq!(toml.lingora.fluent_sources, [Path::new("./i18n")]);
        assert_eq!(toml.lingora.canonical, Locale::from_str("en-GB").unwrap());
        assert!(toml.lingora.primaries.is_empty());
        assert_eq!(toml.dioxus_i18n.rust_sources, [Path::new("./src")]);
        assert_eq!(
            toml.dioxus_i18n.config_inclusion,
            ConfigInclusionStyle::Auto
        );
    }

    #[test]
    fn will_not_load_from_non_existing_toml_file() {
        let path = Path::new("./non_existing_lingora.toml");
        let error = LingoraToml::try_from(path).expect_err("failed to detect error");
        assert!(matches!(error, LingoraError::Io(_)));
    }

    #[test]
    fn will_not_load_from_invalid_toml_file() {
        let path = Path::new("./tests/data/toml/Lingora_error.toml");
        let error = LingoraToml::try_from(path).expect_err("failed to detect error");
        assert!(matches!(error, LingoraError::TomlParse(_)));
    }

    #[test]
    fn will_default_when_no_file_provided() {
        let toml = LingoraToml::default();
        assert_eq!(toml.lingora.fluent_sources, [Path::new("./i18n")]);
        assert_eq!(toml.lingora.canonical, Locale::default());
        assert!(toml.lingora.primaries.is_empty());
        assert_eq!(toml.dioxus_i18n.rust_sources, [Path::new("./src")]);
        assert_eq!(
            toml.dioxus_i18n.config_inclusion,
            ConfigInclusionStyle::Auto
        );
    }

    #[test]
    fn will_load_from_str() {
        let content = std::fs::read_to_string(Path::new("./tests/data/toml/Lingora.toml"))
            .expect("read test file");
        let toml = LingoraToml::from_str(&content).expect("failed to parse toml");
        assert_eq!(toml.lingora.fluent_sources, [Path::new("./i18n")]);
        assert_eq!(toml.lingora.canonical, Locale::from_str("en-GB").unwrap());
        assert!(toml.lingora.primaries.is_empty());
        assert_eq!(toml.dioxus_i18n.rust_sources, [Path::new("./src")]);
        assert_eq!(
            toml.dioxus_i18n.config_inclusion,
            ConfigInclusionStyle::Auto
        );
    }

    #[test]
    fn will_load_from_args_config() {
        use crate::config::args::CoreArgs;
        let args =
            CoreArgs::from_str("app_name --config=./tests/data/toml/Lingora_args.toml").unwrap();
        let toml = LingoraToml::try_from(&args).unwrap();
        assert_eq!(toml.lingora.fluent_sources, [Path::new("./args/i18n")]);
        assert_eq!(toml.lingora.canonical, Locale::from_str("de-DE").unwrap());
        assert_eq!(toml.lingora.primaries, [Locale::from_str("en-AU").unwrap()]);
        assert_eq!(toml.dioxus_i18n.rust_sources, [Path::new("./args/src")]);
        assert_eq!(
            toml.dioxus_i18n.config_inclusion,
            ConfigInclusionStyle::IncludeStr
        );
    }

    #[test]
    fn will_load_from_args_overridden() {
        use crate::config::args::CoreArgs;
        let args =
            CoreArgs::from_str("app_name --config=./tests/data/toml/Lingora_args.toml --fluent-sources=./also/i18n,./also/branding --canonical=ja-JP --primaries=sk-SK,sr-Cryl-RS,bn-IN --rust-sources=./also/src --config-inclusion=pathbuf").unwrap();
        let toml = LingoraToml::try_from(&args).unwrap();
        assert_eq!(
            toml.lingora.fluent_sources,
            [
                Path::new("./args/i18n"),
                Path::new("./also/i18n"),
                Path::new("./also/branding")
            ]
        );
        assert_eq!(toml.lingora.canonical, Locale::from_str("ja-JP").unwrap());
        assert_eq!(
            toml.lingora.primaries,
            [
                Locale::from_str("en-AU").unwrap(),
                Locale::from_str("sk-SK").unwrap(),
                Locale::from_str("sr-Cryl-RS").unwrap(),
                Locale::from_str("bn-IN").unwrap()
            ]
        );
        assert_eq!(
            toml.dioxus_i18n.rust_sources,
            [Path::new("./args/src"), Path::new("./also/src")]
        );
        assert_eq!(
            toml.dioxus_i18n.config_inclusion,
            ConfigInclusionStyle::PathBuf
        );
    }
}

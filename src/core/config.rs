use super::error::Error;
use super::locale::Locale;

use serde::Deserialize;

use std::{env, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Config {
    root_path: PathBuf,
    reference_locale: Locale,
}

impl Config {
    const DEFAULT_ROOT_PATH: &str = "i18n";

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn reference_locale(&self) -> &Locale {
        &self.reference_locale
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_path: env::current_dir().unwrap().join(Self::DEFAULT_ROOT_PATH),
            reference_locale: Locale::default(),
        }
    }
}

impl TryFrom<&std::path::PathBuf> for Config {
    type Error = Error;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let content =
            std::fs::read_to_string(value).map_err(|e| Error::InvalidConfigFile(e.to_string()))?;
        let mut config = toml::from_str::<Config>(&content)
            .map_err(|e| Error::InvalidConfigFile(e.to_string()))?;
        config.root_path = config
            .root_path
            .canonicalize()
            .map_err(|e| Error::InvalidConfigFile(e.to_string()))?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default_config_base_path_will_be_current_working_directory_i18n() {
        let config = Config::default();
        let expected_root_path = std::env::current_dir()
            .unwrap()
            .join(Config::DEFAULT_ROOT_PATH);
        assert_eq!(config.root_path(), &expected_root_path);
    }

    #[test]
    fn default_config_reference_locale_will_be_system_language() {
        let config = Config::default();
        let expected_language = Locale::default();
        assert_eq!(config.reference_locale(), &expected_language);
    }

    #[test]
    fn config_is_read_from_file() {
        let path = PathBuf::from(format!(
            "{}/tests/data/config-test.toml",
            env!("CARGO_MANIFEST_DIR")
        ));
        let config = Config::try_from(&path).unwrap();

        let expected_root_path =
            PathBuf::from(format!("{}/tests/data/i18n/", env!("CARGO_MANIFEST_DIR")));
        assert_eq!(config.root_path(), &expected_root_path);

        let expected_reference_language = Locale::try_from("jp").unwrap();
        assert_eq!(config.reference_locale(), &expected_reference_language);
    }
}

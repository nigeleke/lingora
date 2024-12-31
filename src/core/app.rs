use super::cli::Cli;
use super::config::Config;
use super::error::Error;
use super::prelude::Language;
use super::primary_language::PrimaryLanguage;
use super::state::State;

use std::env;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn try_new(args: &Cli) -> Result<Self, Error> {
        let requested_config = args.config();
        let default_config =
            env::current_dir().map(|p| PathBuf::from(p.join("Lingora").with_extension("toml")));
        let config = match (requested_config, default_config) {
            (Some(requested), _) => Config::try_from(requested)?,
            (None, Ok(default)) if default.exists() => Config::try_from(&default)?,
            (None, _) => Config::default(),
        };
        let state = State::try_from(config.root_path())?;
        dioxus::logger::tracing::info!("State: {state:?}");
        Ok(Self { config, state })
    }

    pub fn root_path(&self) -> PathBuf {
        self.config.root_path().clone()
    }

    pub fn reference_language(&self) -> Language {
        self.config.reference_language().clone()
    }

    pub fn target_language(&self) -> Option<Language> {
        self.state.target_language().cloned()
    }

    pub fn set_target_language(&mut self, language: Language) {
        self.state.set_target_language(&language)
    }

    pub fn primary_languages(&self) -> Vec<PrimaryLanguage> {
        self.state
            .primary_languages()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn locales(&self, primary: &PrimaryLanguage) -> Vec<Language> {
        self.state
            .locales(primary)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::Language;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn app_will_use_default_config() {
        let args = "".into();
        let app = App::try_new(&args).unwrap();
        let config = Config::default();
        assert_eq!(app.root_path(), *config.root_path());
        assert_eq!(app.reference_language(), *config.reference_language());
    }

    #[test]
    fn app_will_use_user_defined_config() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        let path = PathBuf::from(TOML_FILE);
        let config = Config::try_from(&path).unwrap();
        assert_eq!(app.root_path(), *config.root_path());
        assert_eq!(app.reference_language(), *config.reference_language());
    }

    #[test]
    fn app_will_use_root_path() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        assert_eq!(
            app.root_path(),
            PathBuf::from(format!("{}/tests/data/i18n/", env!("CARGO_MANIFEST_DIR")))
        );
    }

    #[test]
    fn app_will_use_reference_language() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        assert_eq!(app.reference_language(), Language::try_from("jp").unwrap());
    }

    #[test]
    fn app_has_no_default_target_language() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        assert_eq!(app.target_language(), None);
    }

    #[test]
    fn app_has_selected_target_language() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let mut app = App::try_new(&args).unwrap();
        let target = Language::try_from("jp").unwrap();
        app.set_target_language(target.clone());
        assert_eq!(app.target_language(), Some(target));
    }

    #[test]
    fn app_will_provide_primary_languages() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        let mut expected = [
            PrimaryLanguage::from(&Language::try_from("en").unwrap()),
            PrimaryLanguage::from(&Language::try_from("it").unwrap()),
        ];
        expected.sort();
        let mut actual = app
            .primary_languages()
            .into_iter()
            .map(|l| l.clone())
            .collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn app_will_provide_locales_for_a_primary_language() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        let mut expected = vec![
            Language::try_from("en").unwrap(),
            Language::try_from("en-AU").unwrap(),
            Language::try_from("en-GB").unwrap(),
        ];
        expected.sort();
        let primary = PrimaryLanguage::from(&Language::try_from("en").unwrap());
        let mut actual = app.locales(&primary);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn languages_have_optional_fallback() {}

    #[test]
    #[ignore]
    fn language_fallback_can_be_defined() {}

    #[test]
    #[ignore]
    fn language_fallback_can_be_cleared() {}

    #[test]
    #[ignore]
    fn language_fallbacks_cannot_be_circular() {}

    #[test]
    #[ignore]
    fn an_identifier_list_will_be_merged_from_reference_target_and_fallback_language_files() {}

    #[test]
    #[ignore]
    fn identifier_categorises_reference_target_and_fallback_sources() {}

    #[test]
    #[ignore]
    fn missing_target_identifiers_will_be_marked() {}

    #[test]
    #[ignore]
    fn unrequired_target_identifiers_will_be_marked() {}

    #[test]
    #[ignore]
    fn identifiers_can_be_renamed() {}

    #[test]
    #[ignore]
    fn reference_and_target_values_will_be_shown() {}

    #[test]
    #[ignore]
    fn reference_values_will_be_editable() {}

    #[test]
    #[ignore]
    fn target_values_will_be_editable() {}

    #[test]
    #[ignore]
    fn reference_and_target_value_placeholders_will_be_cross_checked() {}

    #[test]
    #[ignore]
    fn description_of_usage_will_be_shown_from_reference() {}
}

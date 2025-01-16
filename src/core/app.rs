use super::annotated_identifier::AnnotatedIdentifier;
use super::cli::Cli;
use super::config::Config;
use super::error::Error;
use super::identifier_origin::IdentifierOrigin;
use super::prelude::Locale;
use super::primary_language::PrimaryLanguage;
use super::state::State;

use std::collections::HashSet;
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
        Ok(Self { config, state })
    }

    pub fn root_path(&self) -> PathBuf {
        self.config.root_path().clone()
    }

    pub fn reference_locale(&self) -> Locale {
        self.config.reference_locale().clone()
    }

    pub fn target_locale(&self) -> Option<Locale> {
        self.state.target_locale().cloned()
    }

    pub fn set_target_locale(&mut self, language: Locale) {
        self.state.set_target_locale(&language)
    }

    pub fn primary_languages(&self) -> Vec<PrimaryLanguage> {
        self.state
            .primary_languages()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn locales(&self, primary: &PrimaryLanguage) -> Vec<Locale> {
        self.state
            .locales(primary)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn identifiers(&self) -> Vec<AnnotatedIdentifier> {
        let reference_identifiers = self.state.identifiers(self.config.reference_locale());

        let target_identifiers = self
            .state
            .target_locale()
            .map_or(HashSet::new(), |locale| self.state.identifiers(locale));

        let target_fallback_identifiers =
            self.state.target_locale().map_or(HashSet::new(), |locale| {
                locale
                    .fallbacks()
                    .into_iter()
                    .fold(HashSet::new(), |mut acc, locale| {
                        let identifiers = self.state.identifiers(&locale);
                        acc.extend(identifiers.into_iter());
                        acc
                    })
            });

        let mut all_identifiers = HashSet::new();
        all_identifiers.extend(reference_identifiers.iter());
        all_identifiers.extend(target_identifiers.iter());
        all_identifiers.extend(target_fallback_identifiers.iter());

        all_identifiers
            .iter()
            .map(|&id| {
                let is_reference = reference_identifiers.contains(id);
                let is_target = target_identifiers.contains(id);
                let is_target_fallback = target_fallback_identifiers.contains(id);
                let mut ai = AnnotatedIdentifier::from(id.clone());
                if is_reference {
                    ai = ai.with_origin(IdentifierOrigin::Reference)
                }
                if is_target {
                    ai = ai.with_origin(IdentifierOrigin::Target)
                }
                if is_target_fallback {
                    ai = ai.with_origin(IdentifierOrigin::TargetFallback)
                }
                ai
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::Locale;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn app_will_use_default_config() {
        let args = "".into();
        let app = App::try_new(&args).unwrap();
        let config = Config::default();
        assert_eq!(app.root_path(), *config.root_path());
        assert_eq!(app.reference_locale(), *config.reference_locale());
    }

    #[test]
    fn app_will_use_user_defined_config() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        let path = PathBuf::from(TOML_FILE);
        let config = Config::try_from(&path).unwrap();
        assert_eq!(app.root_path(), *config.root_path());
        assert_eq!(app.reference_locale(), *config.reference_locale());
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
    fn app_will_use_reference_locale() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        assert_eq!(app.reference_locale(), Locale::try_from("jp").unwrap());
    }

    #[test]
    fn app_has_no_default_target_locale() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        assert_eq!(app.target_locale(), None);
    }

    #[test]
    fn app_has_selected_target_locale() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let mut app = App::try_new(&args).unwrap();
        let target = Locale::try_from("jp").unwrap();
        app.set_target_locale(target.clone());
        assert_eq!(app.target_locale(), Some(target));
    }

    #[test]
    fn app_will_provide_primary_languages() {
        const TOML_FILE: &str = "./tests/data/app-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();
        let app = App::try_new(&args).unwrap();
        let mut expected = [
            Locale::try_from("en").unwrap().primary_language(),
            Locale::try_from("it").unwrap().primary_language(),
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
            Locale::try_from("en").unwrap(),
            Locale::try_from("en-AU").unwrap(),
            Locale::try_from("en-GB").unwrap(),
        ];
        expected.sort();
        let primary = Locale::try_from("en").unwrap().primary_language();
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
    // TODO: Add fallback...
    fn an_identifier_list_will_be_merged_from_reference_target_and_fallback_locales() {
        const TOML_FILE: &str = "./tests/data/identifiers-test.toml";
        let args = format!("lingora --config={TOML_FILE}").as_str().into();

        let mut app = App::try_new(&args).unwrap();
        app.set_target_locale(Locale::try_from("it-IT").unwrap());

        let mut expected = vec![
            AnnotatedIdentifier::from("ref-1").with_origin(IdentifierOrigin::Reference),
            AnnotatedIdentifier::from("ref-2").with_origin(IdentifierOrigin::Target),
            AnnotatedIdentifier::from("ref-3")
                .with_origin(IdentifierOrigin::Target)
                .with_origin(IdentifierOrigin::Reference),
            AnnotatedIdentifier::from("ref-4").with_origin(IdentifierOrigin::TargetFallback),
            AnnotatedIdentifier::from("ref-5")
                .with_origin(IdentifierOrigin::TargetFallback)
                .with_origin(IdentifierOrigin::Reference),
            AnnotatedIdentifier::from("ref-6")
                .with_origin(IdentifierOrigin::TargetFallback)
                .with_origin(IdentifierOrigin::Target),
            AnnotatedIdentifier::from("ref-7")
                .with_origin(IdentifierOrigin::TargetFallback)
                .with_origin(IdentifierOrigin::Target)
                .with_origin(IdentifierOrigin::Reference),
        ];
        expected.sort();

        let mut actual = app.identifiers();
        actual.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn identifier_categorises_reference_target_and_fallback_origin() {}

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

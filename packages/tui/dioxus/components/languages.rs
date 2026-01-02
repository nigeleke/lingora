use std::path::PathBuf;

use dioxus::prelude::{document::*, *};

use super::scrollable::Scrollable;
use crate::{
    domain::{Analysis, ValidatedLanguage, ValidatedLocale},
    tui::state::State,
};

#[component]
pub fn Languages() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/languages.css") }
        Scrollable {
            LanguageTree { }
        }
    }
}

#[component]
fn LanguageTree() -> Element {
    let analysis = use_context::<Signal<Analysis>>();

    let analysis = analysis.read();
    let paths = analysis.paths_by_locale_by_language();

    let mut languages = Vec::from_iter(paths.keys());
    languages.sort();

    rsx! {
        div {
            class: "languages-tree",
            ul {
                for language in languages.into_iter() {
                    LanguageGroup { language: language.clone() }
                    Locales { language: language.clone() }
                }
            }
        }
    }
}

#[component]
fn LanguageGroup(language: ValidatedLanguage) -> Element {
    rsx! {
        li {
            key: "{language.to_string()}",
            "{language.to_string()}"
        }
    }
}

#[component]
fn Locales(language: ValidatedLanguage) -> Element {
    let analysis = use_context::<Signal<Analysis>>();

    let analysis = analysis.read();
    let paths = analysis.paths_by_locale(&language);

    let mut locales = Vec::from_iter(paths.keys());
    locales.sort();

    rsx! {
        ul {
            for locale in locales {
                for (index, path) in paths[locale].iter().enumerate() {
                    Locale { locale: locale.clone(), path: path.clone(), index, is_unique: paths[locale].len() == 1 }
                }
            }
        }
    }
}

#[component]
fn Locale(locale: ValidatedLocale, path: PathBuf, index: usize, is_unique: bool) -> Element {
    let mut state = use_context::<Signal<State>>();
    let analysis = use_context::<Signal<Analysis>>();

    let select_path = |path: PathBuf| {
        move |_| {
            state.write().set_target_path(path.clone());
        }
    };

    let select_path_on_enter = |path: PathBuf| {
        move |event: Event<KeyboardData>| {
            if event.data.key() == Key::Enter {
                state.write().set_target_path(path.clone());
            }
        }
    };

    let disambiguator = if is_unique {
        "".to_string()
    } else {
        let parent = path.parent().map_or("".into(), |p| p.to_string_lossy());
        format!(" ({}) - {}{}", index, parent, std::path::MAIN_SEPARATOR)
    };
    let description = format!("{}{}", locale, disambiguator);

    let status = analysis.read().status(&path);
    let css_class = status.css_class();

    rsx! {
        li {
            class: if state.read().target_path() == Some(&path) { "selected" },
            class: if state.read().reference_path() == &path { "reference" },
            class: "{css_class}",
            tabindex: "0",
            role: "button",
            key: "{path.to_string_lossy()}",
            onclick: select_path(path.clone()),
            onkeypress: select_path_on_enter(path.clone()),
            div {
                span { {description} }
                span {}
            }
        }
    }
}

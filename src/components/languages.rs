use crate::core::prelude::{CoreApp, Locale, PrimaryLanguage};

use dioxus::prelude::{document::*, *};

#[component]
pub fn Languages() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/languages.css") }
        div {
            class: "languages",
            LanguageTree { }
        }
    }
}

#[component]
fn LanguageTree() -> Element {
    let app = use_context::<Signal<CoreApp>>();

    let mut languages = app.read().primary_languages();
    languages.sort();

    rsx! {
        div {
            class: "languages-tree",
            ul {
                for language in languages.into_iter() {
                    Language { language: language.clone() }
                    Locales { language }
                }
            }
        }
    }
}

#[component]
fn Language(language: PrimaryLanguage) -> Element {
    rsx! {
        li {
            key: "{language.to_string()}",
            "{language.to_string()}"
        }
    }
}

#[component]
fn Locales(language: PrimaryLanguage) -> Element {
    let mut app = use_context::<Signal<CoreApp>>();

    let mut locales = app.read().locales(&language);
    locales.sort();

    let select_locale = |locale: &Locale| {
        let locale = locale.clone();
        move |_| {
            app.write().set_target_locale(locale.clone());
        }
    };

    dioxus::logger::tracing::info!("target_locale: {:?}", app.read().target_locale());
    dioxus::logger::tracing::info!("reference_locale: {:?}", app.read().reference_locale());

    rsx! {
        ul {
            for locale in locales {
                li {
                    class: if app.read().target_locale().map_or(false, |tl| tl == locale) { "selected" },
                    class: if app.read().reference_locale() == locale { "reference" },
                    tabindex: "0",
                    role: "button",
                    key: "{locale.to_string()}",
                    onclick: select_locale(&locale),
                    "{locale.to_string()}"
                }
            }
        }
    }
}

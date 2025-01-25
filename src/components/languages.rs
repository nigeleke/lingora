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

    let select_locale_on_enter = |locale: &Locale| {
        let locale = locale.clone();
        move |event: Event<KeyboardData>| {
            if event.data.key() == Key::Enter {
                app.write().set_target_locale(locale.clone());
            }
        }
    };

    rsx! {
        ul {
            for locale in locales {
                li {
                    class: if app.read().target_locale() == Some(locale.clone()) { "selected" },
                    class: if app.read().reference_locale() == locale { "reference" },
                    tabindex: "0",
                    role: "button",
                    key: "{locale.to_string()}",
                    onclick: select_locale(&locale),
                    onkeypress: select_locale_on_enter(&locale),
                    "{locale.to_string()}"
                }
            }
        }
    }
}

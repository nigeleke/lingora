use crate::core::prelude::{CoreApp, Locale};

use dioxus::prelude::{document::*, *};

#[component]
pub fn Languages() -> Element {
    let app = use_context::<Signal<CoreApp>>();

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/languages.css") }
        LanguageTree { app }
    }
}

#[component]
fn LanguageTree(app: Signal<CoreApp>) -> Element {
    let mut languages = app.read().primary_languages();
    languages.sort();

    let select_locale = |locale: &Locale| {
        let locale = locale.clone();
        move |_| {
            app.write().set_target_locale(locale.clone());
        }
    };

    rsx! {
        div {
            class: "languages",
            ul {
                for language in languages.iter() {
                    li {
                        key: "{language.to_string()}",
                        "{language.to_string()}"
                    }
                    {
                        let app = app.read();
                        let mut locales = app.locales(language);
                        locales.sort();
                        rsx! {
                            ul {
                                for locale in locales {
                                    li {
                                        class: if Some(locale.clone()) == app.target_locale() { "selected" },
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
                }
            }
        }
    }
}

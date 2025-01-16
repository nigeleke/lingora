use crate::core::prelude::*;

use dioxus::prelude::{document::*, *};

#[component]
pub fn Identifiers() -> Element {
    let app = use_context::<Signal<CoreApp>>();
    let mut identifiers = app.read().identifiers();
    identifiers.sort();

    let mut selected_identifier = use_signal(|| None);

    let select_identifier = |identifier: &AnnotatedIdentifier| {
        let identifier = identifier.clone();
        move |_| {
            selected_identifier.set(Some(identifier.clone()));
        }
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/identifiers.css") }
        div {
            class: "identifiers",
            ul {
                for identifier in identifiers {
                    li {
                        class: "{identifier.css_class()}",
                        class: if Some(identifier.clone()) == selected_identifier() { "selected" },
                        tabindex: "0",
                        role: "button",
                        key: "{identifier.name()}",
                        onclick: select_identifier(&identifier),
                        div {
                            span { {identifier.name()} }
                            span { }
                        }
                    }
                }
            }
        }
    }
}

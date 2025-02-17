use super::scrollable::Scrollable;

use crate::{
    domain::{FluentFile, Identifier},
    gui::state::State,
};

use dioxus::prelude::{document::*, *};

use std::collections::HashSet;

#[component]
pub fn Identifiers() -> Element {
    let filter = use_signal(|| "".to_string());

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/identifiers.css") }
        div {
            class: "identifiers-outer",
            Filter { filter }
            Scrollable {
                IdentifiersList { filter }
            }
        }
    }
}

#[component]
fn Filter(filter: Signal<String>) -> Element {
    let update_filter = move |event: Event<FormData>| filter.set(event.value());

    rsx! {
        div {
            class: "identifiers-filter",
            input {
                r#type: "text",
                placeholder: "❔Search",
                oninput: update_filter,
            }
        }
    }
}

#[component]
fn IdentifiersList(filter: Signal<String>) -> Element {
    let mut state = use_context::<Signal<State>>();

    let mut reference_file = use_signal(|| Ok(FluentFile::default()));
    use_effect(move || reference_file.set(FluentFile::try_from(state.read().reference_path())));

    let mut reference_identifiers = use_signal(HashSet::new);
    use_effect(move || match &*reference_file.read() {
        Ok(file) => reference_identifiers.set(file.identifiers()),
        Err(e) => state.write().set_error_string(&e.to_string()),
    });

    let mut target_file = use_signal(|| None);
    use_effect(move || target_file.set(state.read().target_path().map(FluentFile::try_from)));

    let mut target_identifiers = use_signal(HashSet::new);
    use_effect(move || {
        if let Some(file) = &*target_file.read() {
            match file {
                Ok(file) => target_identifiers.set(file.identifiers()),
                Err(e) => state.write().set_error_string(&e.to_string()),
            }
        }
    });

    let mut identifiers = use_signal(Vec::new);
    use_effect(move || {
        let all_identifiers = reference_identifiers()
            .into_iter()
            .chain(target_identifiers().into_iter())
            .filter(|id| {
                id.name()
                    .to_ascii_lowercase()
                    .contains(&filter.read().to_ascii_lowercase())
            })
            .collect::<HashSet<_>>();

        let mut sorted = Vec::from_iter(all_identifiers);
        sorted.sort();

        identifiers.set(sorted);
    });

    rsx! {
        div {
            class: "identifiers-list",
            ul {
                for identifier in &*identifiers.read() {
                    IdentifierItem { identifier: identifier.clone() }
                }
            }
        }

    }
}

#[component]
fn IdentifierItem(identifier: Identifier) -> Element {
    let mut state = use_context::<Signal<State>>();

    let select_identifier = |identifier: &Identifier| {
        let identifier = identifier.clone();
        move |_| {
            state.write().set_identifier(&identifier);
        }
    };

    let select_identifier_on_enter = |identifier: &Identifier| {
        let identifier = identifier.clone();
        move |event: Event<KeyboardData>| {
            if event.data.key() == Key::Enter {
                state.write().set_identifier(&identifier);
            }
        }
    };

    rsx! {
        li {
            // class: "{identifier.state().css_class()}",
            class: if Some(&identifier) == state.read().identifier() { "selected" },
            tabindex: "0",
            role: "button",
            key: "{identifier.name()}",
            onclick: select_identifier(&identifier),
            onkeypress: select_identifier_on_enter(&identifier),
            div {
                span { {identifier.name()} }
                span { }
            }
        }

    }
}

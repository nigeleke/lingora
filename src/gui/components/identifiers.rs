use std::collections::HashSet;

use dioxus::prelude::{document::*, *};

use super::scrollable::Scrollable;
use crate::{
    domain::{FluentFile, Identifier},
    gui::state::State,
};

#[derive(Clone, Default)]
struct Filter {
    name: String,
    ok: bool,
    warnings: bool,
    errors: bool,
}

#[component]
pub fn Identifiers() -> Element {
    let filter = use_signal(Filter::default);

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/identifiers.css") }
        div {
            class: "identifiers-outer",
            FilterView { filter }
            Scrollable {
                IdentifiersTree { filter }
            }
        }
    }
}

#[component]
fn FilterView(filter: Signal<Filter>) -> Element {
    let update_filter = move |event: Event<FormData>| filter.write().name = event.value();
    let update_ok = move |event: Event<FormData>| filter.write().ok = event.checked();
    let update_warnings = move |event: Event<FormData>| filter.write().warnings = event.checked();
    let update_errors = move |event: Event<FormData>| filter.write().errors = event.checked();

    rsx! {
        div {
            class: "identifiers-filter",
            input {
                r#type: "text",
                placeholder: "🔎",
                oninput: update_filter,
            }
            label {
                class: "both",
                input {
                    r#type: "checkbox",
                    checked: filter.read().ok,
                    onchange: update_ok,
                }
                span { "●" }
                span { "Ok" }
            }
            label {
                class: "superfluous-target",
                input {
                    r#type: "checkbox",
                    checked: filter.read().warnings,
                    onchange: update_warnings,
                }
                span { "●" }
                span { "Warning" }
            }
            label {
                class: "missing-target",
                input {
                    r#type: "checkbox",
                    checked: filter.read().errors,
                    onchange: update_errors,
                }
                span { "●" }
                span { "Error" }
            }
        }
    }
}

#[component]
fn IdentifiersTree(filter: Signal<Filter>) -> Element {
    let mut state = use_context::<Signal<State>>();

    let default_all = !(filter.read().ok || filter.read().warnings || filter.read().errors);

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
                id.path()
                    .to_ascii_lowercase()
                    .contains(&filter.read().name.to_ascii_lowercase())
            })
            .filter(|id| {
                match (
                    reference_identifiers.read().contains(id),
                    target_identifiers.read().contains(id),
                ) {
                    (true, true) => filter.read().ok || default_all,
                    (true, false) => filter.read().errors || default_all,
                    (false, true) => filter.read().warnings || default_all,
                    (false, false) => unreachable!(),
                }
            })
            .collect::<HashSet<_>>();

        let mut sorted = Vec::from_iter(all_identifiers);
        sorted.sort();

        identifiers.set(sorted);
    });

    rsx! {
        div {
            class: "identifiers-tree",
            ul {
                for identifier in &*identifiers.read() {
                    IdentifierItem {
                        identifier: identifier.clone(),
                        in_reference: reference_identifiers.read().contains(identifier),
                        in_target: target_identifiers.read().contains(identifier)
                    }
                }
            }
        }
    }
}

#[component]
fn IdentifierItem(identifier: Identifier, in_reference: bool, in_target: bool) -> Element {
    let mut state = use_context::<Signal<State>>();

    let css_class = match (in_reference, in_target) {
        (false, false) => "",
        (false, true) => "superfluous-target",
        (true, false) => "missing-target",
        (true, true) => "both",
    };

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
            class: "{css_class}",
            class: if Some(&identifier) == state.read().identifier() { "selected" },
            tabindex: "0",
            role: "button",
            key: "{identifier.path()}",
            onclick: select_identifier(&identifier),
            onkeypress: select_identifier_on_enter(&identifier),
            div {
                span { {identifier.path()} }
                span { }
            }
        }

    }
}

use super::scrollable::Scrollable;

use crate::domain::Analysis;
use crate::gui::state::State;

use dioxus::prelude::{document::*, *};

#[component]
pub fn Identifiers() -> Element {
    let analysis = use_context::<Signal<Analysis>>();
    let state = use_context::<Signal<State>>();

    let mut search_text = use_signal(|| "".to_string());
    let update_search_text = move |event: Event<FormData>| search_text.set(event.value());

    let mut errors_filter = use_signal(|| false);
    let toggle_errors_filter = move |event: Event<FormData>| errors_filter.set(event.checked());

    let mut warnings_filter = use_signal(|| false);
    let toggle_warnings_filter = move |event: Event<FormData>| warnings_filter.set(event.checked());

    let mut ok_filter = use_signal(|| false);
    let toggle_ok_filter = move |event: Event<FormData>| ok_filter.set(event.checked());

    // let mut state_filter = use_signal(HashSet::<AnnotatedIdentifierState>::new);

    // use_effect(move || {
    //     state_filter.write().clear();

    //     let all_by_default = !errors_filter() && !warnings_filter() && !ok_filter();

    //     if all_by_default || errors_filter() {
    //         state_filter
    //             .write()
    //             .insert(AnnotatedIdentifierState::MissingTarget);
    //     }

    //     if all_by_default || warnings_filter() {
    //         state_filter
    //             .write()
    //             .insert(AnnotatedIdentifierState::SuperfluousTarget);
    //         state_filter
    //             .write()
    //             .insert(AnnotatedIdentifierState::SuperfluousTargetFallback);
    //     }

    //     if all_by_default || ok_filter() {
    //         state_filter.write().insert(AnnotatedIdentifierState::Ok);
    //         state_filter
    //             .write()
    //             .insert(AnnotatedIdentifierState::OkUsingTargetFallback);
    //     }
    // });

    // let identifiers = analysis.read().identifiers();
    // let mut identifiers = identifiers
    //     .iter()
    //     .filter(|id| {
    //         id.name()
    //             .to_ascii_lowercase()
    //             .contains(&search_text.read().to_ascii_lowercase())
    //     })
    //     .filter(|id| state_filter.read().contains(&id.state()))
    //     .collect::<Vec<_>>();
    // identifiers.sort();

    // let select_identifier = |identifier: &AnnotatedIdentifier| {
    //     let identifier = identifier.clone();
    //     move |_| {
    //         app.write().set_selected_identifier(&identifier);
    //     }
    // };

    // let select_identifier_on_enter = |identifier: &AnnotatedIdentifier| {
    //     let identifier = identifier.clone();
    //     move |event: Event<KeyboardData>| {
    //         if event.data.key() == Key::Enter {
    //             app.write().set_selected_identifier(&identifier);
    //         }
    //     }
    // };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/identifiers.css") }
        div {
            class: "identifiers",
            div {
                class: "identifiers-filter",
                input {
                    r#type: "text",
                    placeholder: "❔Search",
                    oninput: update_search_text,
                }
                label {
                    input {
                        r#type: "checkbox",
                        onchange: toggle_errors_filter,
                    }
                    "errors",
                }
                label {
                    input {
                        r#type: "checkbox",
                        onchange: toggle_warnings_filter,
                    }
                    "warnings",
                }
                label {
                    input {
                        r#type: "checkbox",
                        onchange: toggle_ok_filter,
                    }
                    "ok",
                }
            }
            Scrollable {
                ul {
                    // for identifier in identifiers {
                    //     li {
                    //         class: "{identifier.state().css_class()}",
                    //         class: if Some(&identifier) == app.read().selected_identifier().as_ref() { "selected" },
                    //         tabindex: "0",
                    //         role: "button",
                    //         key: "{identifier.name()}",
                    //         onclick: select_identifier(identifier),
                    //         onkeypress: select_identifier_on_enter(identifier),
                    //         div {
                    //             span { {identifier.name()} }
                    //             span { }
                    //         }
                    //     }
                    // }
                }

            }
        }
    }
}

use std::collections::HashSet;

use dioxus::prelude::{document::*, *};
use fluent4rs::prelude::Entry;

use super::scrollable::Scrollable;
use crate::{domain::FluentFile, gui::state::State};

#[component]
pub fn Translation() -> Element {
    let state = use_context::<Signal<State>>();

    let mut identifier = use_signal(|| None);
    let mut identifier_name = use_signal(|| "".to_string());

    use_effect(move || {
        identifier.set(state.read().identifier().cloned());
        identifier_name.set(identifier().map_or(" ".to_string(), |i| i.to_string()));
    });

    let mut reference_usages = use_signal(HashSet::default);
    use_effect(move || {
        reference_usages.write().clear();
        if let Ok(file) = FluentFile::try_from(state.read().reference_path()) {
            if let Some(identifier) = &*identifier.read() {
                reference_usages.set(file.identifier_usage(identifier));
            }
        }
    });

    let mut target_usages = use_signal(HashSet::default);
    use_effect(move || {
        target_usages.write().clear();
        if let Some(target_path) = state.read().target_path() {
            if let Ok(file) = FluentFile::try_from(target_path) {
                if let Some(identifier) = &*identifier.read() {
                    target_usages.set(file.identifier_usage(identifier));
                }
            }
        }
    });

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/translation.css") }
        div {
            class: "translation",
            IdentifierName { name: identifier_name }
            TranslationComparison {
                reference: reference_usages(),
                target: target_usages()
            }
        }
    }
}

#[component]
fn IdentifierName(name: String) -> Element {
    rsx! { p { strong { {name} } } }
}

#[component]
fn TranslationComparison(reference: HashSet<Entry>, target: HashSet<Entry>) -> Element {
    const REFERENCE: &str = "Reference";
    const TARGET: &str = "Target";

    let entries_string = |set: &HashSet<Entry>| {
        let mut entries = set.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        entries.sort();
        entries.join("\n")
    };

    let left_entries_string = entries_string(&reference);
    let right_entries_string = entries_string(&target);

    rsx! {
        div {
            class: "translation-comparison",
            SideBySide {
                class: "",
                left: REFERENCE,
                right: TARGET
            }
            SideBySide {
                class: "translation-content",
                left: left_entries_string,
                right: right_entries_string,
            }
        }
    }
}

#[component]
fn SideBySide(class: String, left: String, right: String) -> Element {
    rsx! {
        div {
            class: "lhs",
            class: if !class.is_empty() {"{class}"},
            Scrollable { {left} }
        }
        span {}
        div {
            class: "rhs",
            class: if !class.is_empty() {"{class}"},
            Scrollable { {right} }
        }
    }
}

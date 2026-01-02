use std::collections::HashSet;

use dioxus::prelude::{document::*, *};

use super::scrollable::Scrollable;
use crate::{
    domain::{Analysis, IntegrityWarning},
    tui::state::State,
};

#[component]
pub fn Warnings() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/warnings.css") }
        div {
           class: "warnings",
           Scrollable {
               WarningsPanel {}
           }
        }
    }
}

#[component]
fn WarningsPanel() -> Element {
    let analysis = use_context::<Signal<Analysis>>();
    let state = use_context::<Signal<State>>();

    let mut warnings = use_signal(Vec::new);
    use_effect(move || {
        let analysis = analysis.read();

        let add_category = |mut categories: HashSet<String>, warning: &IntegrityWarning| {
            categories.insert(warning.category_str().to_string());
            categories
        };

        let reference_path = state.read().reference_path().to_owned();
        let reference_warnings = analysis.checks(&reference_path).iter();

        let target_path = state.read().target_path().map(|p| p.to_owned());
        let target_warnings = if let Some(path) = target_path {
            analysis.checks(&path).iter()
        } else {
            [].iter()
        };

        let identifier_name = state
            .read()
            .identifier()
            .map_or("".into(), |i| i.to_string());
        let categories = reference_warnings
            .chain(target_warnings)
            .filter(|w| w.value_str() == identifier_name)
            .fold(HashSet::new(), add_category);
        let mut categories = Vec::from_iter(categories);
        categories.sort();

        warnings.set(categories);
    });

    rsx! {
        div {
            class: "warnings-panel",
            for warning in warnings.read().iter() {
                WarningPill { text: warning }
            }
        }
    }
}

#[component]
fn WarningPill(text: String) -> Element {
    rsx! {
        div {
            class: "warnings-pill",
            {text}
        }
    }
}

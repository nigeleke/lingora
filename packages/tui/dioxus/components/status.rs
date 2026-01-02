use dioxus::prelude::{document::*, *};

use crate::{config::Settings, domain::ValidatedLocale, tui::state::State};

#[component]
pub fn Status() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/status.css") }
        div {
            class: "status",
            RootPath {}
            Error {}
            Translation {}
        }
    }
}

#[component]
fn RootPath() -> Element {
    let settings = use_context::<Signal<Settings>>();
    rsx! {
        div { {settings.read().root().display().to_string()} }
    }
}

#[component]
fn Error() -> Element {
    let state = use_context::<Signal<State>>();
    rsx! {
        div {
            class: "status-error",
            {state.read().error_string()}
        }
    }
}

#[component]
fn Translation() -> Element {
    let state = use_context::<Signal<State>>();

    let source = ValidatedLocale::from(state.read().reference_path());
    let target = state.read().target_path().map(ValidatedLocale::from);

    rsx! {
        span {
            {source.to_string()}
            " ⤅ "
            {target.map_or("≪no target≫".to_string(), |l| l.to_string())}
        }
    }
}

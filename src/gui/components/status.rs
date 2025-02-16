use crate::gui::state::State;
use crate::{config::Settings, domain::ValidatedLocale};

use dioxus::prelude::{document::*, *};

#[component]
pub fn Status() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/status.css") }
        div {
            class: "status",
            RootPath {}
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

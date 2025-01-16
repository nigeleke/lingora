use crate::core::prelude::{CoreApp, Locale};

use dioxus::prelude::{document::*, *};

use std::path::PathBuf;

#[component]
pub fn State() -> Element {
    let app = use_context::<Signal<CoreApp>>();

    rsx! {
        div {
            class: "state",
            Link { rel: "stylesheet", href: asset!("/assets/css/state.css") }
            RootPath { path: app.read().root_path() }
            Translation {
                source: app.read().reference_locale(),
                target: app.read().target_locale(),
            }
        }
    }
}

#[component]
fn RootPath(path: PathBuf) -> Element {
    rsx! {
        span { {path.display().to_string()} }
    }
}

#[component]
fn Translation(source: Locale, target: Option<Locale>) -> Element {
    rsx! {
        span {
            {source.to_string()}
            " ⤅ "
            {target.map_or("≪no target≫".to_string(), |l| l.to_string())}
        }
    }
}

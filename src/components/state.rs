use crate::{
    core::prelude::{CoreApp, CoreError},
    prelude::Language,
};

use dioxus::prelude::{document::*, *};
use dioxus_elements::select::form;

use std::path::PathBuf;

#[component]
pub fn State() -> Element {
    let app = use_context::<Result<CoreApp, CoreError>>();

    rsx! {
        div {
            class: "state",
            Link { rel: "stylesheet", href: asset!("/assets/css/state.css") }
            if let Ok(app) = app {
                Status { app }
            } else if let Err(error) = app {
                ErrorText { error }
            }
        }
    }
}

#[component]
fn Status(app: CoreApp) -> Element {
    rsx! {
        div {
            class: "state_status",
            RootPath { path: app.root_path().clone() }
            Translation {
                source: app.reference_language().clone(),
                target: app.target_language().cloned(),
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
fn Translation(source: Language, target: Option<Language>) -> Element {
    rsx! {
        span {
            {source.to_string()}
            " ⤅ "
            {format!("Target {:?}", target)}
        }
    }
}

#[component]
fn ErrorText(error: CoreError) -> Element {
    rsx! {
        span {
            class: "state_error_text",
            {error.to_string()}
        }
    }
}

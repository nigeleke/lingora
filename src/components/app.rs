use super::*;

use crate::core::prelude::{Cli, CoreApp};

use clap::Parser;
use dioxus::prelude::{document::*, *};

#[component]
pub fn App() -> Element {
    let cli = Cli::parse();
    let app = CoreApp::try_new(&cli);

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        if let Ok(app) = app {
            Link { rel: "stylesheet", href: asset!("/assets/css/app.css") }
            Workspace { app }
        } else if let Err(error) = app {
            WarningText { text: error.to_string() }
        }
    }
}

#[component]
fn Workspace(app: CoreApp) -> Element {
    let app = use_signal(|| app);
    provide_context(app);

    rsx! {
        div {
            class: "workspace",
            Languages {}
            Identifiers {}
            Translation {}
            Description {}
        }
        State { }
    }
}

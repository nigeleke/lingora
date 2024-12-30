use super::*;

use crate::core::prelude::{Cli, CoreApp};

use clap::Parser;
use dioxus::prelude::{document::*, *};

#[component]
pub fn App() -> Element {
    let cli = Cli::parse();
    let app = CoreApp::try_new(&cli);
    let _ = provide_context(app);

    let var_name = rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        Languages {}
        Identifiers {}
        Translation {}
        Description {}
        State {}
    };
    var_name
}

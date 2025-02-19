use dioxus::prelude::{document::*, *};

use super::{components::Workspace, state::State};
use crate::{config::Settings, domain::Analysis};

#[component]
pub fn App(settings: Settings, analysis: Analysis) -> Element {
    let settings = use_signal(|| settings);
    let analysis = use_signal(|| analysis);

    provide_context(settings);
    provide_context(analysis);

    let state = State::from(&*settings.read());
    let state = use_signal(|| state);
    provide_context(state);

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        Workspace { }
    }
}

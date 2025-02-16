use dioxus::prelude::{document::*, *};

#[component]
pub fn Description() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/description.css") }
        div {
           class: "description",
           "Description"
        }
    }
}

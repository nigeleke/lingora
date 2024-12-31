use dioxus::prelude::*;

#[component]
pub fn Identifiers() -> Element {
    rsx! {
        div {
            class: "identifiers",
            "Identifiers"
        }
    }
}

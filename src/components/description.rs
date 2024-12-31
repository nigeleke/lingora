use dioxus::prelude::*;

#[component]
pub fn Description() -> Element {
    rsx! {
        div {
           class: "description",
           "Description"
        }
    }
}

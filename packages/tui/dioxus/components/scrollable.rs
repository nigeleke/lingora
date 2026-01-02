use dioxus::prelude::{document::*, *};

#[component]
pub fn Scrollable(children: Element) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/scrollable.css") }
        div {
            class: "scrollable",
            {children}
        }
    }
}

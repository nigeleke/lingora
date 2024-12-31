use dioxus::prelude::*;

#[component]
pub fn WarningText(text: String) -> Element {
    rsx! {
        span {
            style: "color: var(--warning-colour)",
            {text}
        }
    }
}

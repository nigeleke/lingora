use dioxus::prelude::{document::*, *};

use super::{
    identifiers::Identifiers, languages::Languages, status::Status, translation::Translation,
    warnings::Warnings,
};

#[component]
pub fn Workspace() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/workspace.css") }
        div {
                class: "workspace",
                WorkspaceItem { Languages {} }
                WorkspaceItem { Identifiers {} }
                WorkspaceItem { Translation {} }
                WorkspaceItem { Warnings {} }
        }
        Status { }
    }
}

#[component]
fn WorkspaceItem(children: Element) -> Element {
    rsx! {
        section {
            class: "workspace-item",
            {children}
        }
    }
}

use dioxus::prelude::{document::*, *};

use super::{
    description::Description, identifiers::Identifiers, languages::Languages, status::Status,
    translation::Translation,
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
                WorkspaceItem { Description {} }
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

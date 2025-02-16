use super::description::Description;
use super::identifiers::Identifiers;
use super::languages::Languages;
use super::status::Status;
use super::translation::Translation;

use dioxus::prelude::{document::*, *};

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

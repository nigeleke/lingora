use lingora_core::prelude::{LanguageRoot, Workspace};
use ratatui::prelude::*;

pub fn language_root_span<'a>(root: &LanguageRoot, workspace: &Workspace) -> Span<'a> {
    let span = Span::from(root.to_string());

    if &LanguageRoot::from(workspace.canonical_locale()) == root {
        span.light_yellow()
    } else {
        span
    }
}

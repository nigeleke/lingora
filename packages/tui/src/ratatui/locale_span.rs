use lingora_core::prelude::{Locale, Workspace};
use ratatui::prelude::*;

pub fn locale_span<'a>(locale: &Locale, workspace: &Workspace) -> Span<'a> {
    if locale == workspace.canonical_locale() {
        Span::from(locale.to_string()).light_yellow()
    } else {
        Span::from(locale.to_string())
    }
}

pub fn canonical_locale_span<'a>(workspace: &Workspace) -> Span<'a> {
    locale_span(workspace.canonical_locale(), workspace)
}

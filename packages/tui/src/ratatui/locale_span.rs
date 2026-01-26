use lingora_core::prelude::{Locale, Workspace};
use ratatui::prelude::*;

pub fn locale_span_str<'a>(locale: &str, workspace: &Workspace) -> Span<'a> {
    if locale == workspace.canonical_locale().to_string() {
        Span::from(locale.to_string()).light_yellow()
    } else {
        Span::from(locale.to_string())
    }
}

pub fn locale_span<'a>(locale: &Locale, workspace: &Workspace) -> Span<'a> {
    locale_span_str(&locale.to_string(), workspace)
}

pub fn canonical_locale_span<'a>(workspace: &Workspace) -> Span<'a> {
    locale_span(workspace.canonical_locale(), workspace)
}

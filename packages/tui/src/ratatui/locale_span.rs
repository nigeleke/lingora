use lingora_core::prelude::{Locale, Workspace};
use ratatui::prelude::*;

pub fn locale_span<'a>(locale: &Locale, workspace: &Workspace) -> Span<'a> {
    let span = Span::from(locale.to_string());

    if workspace.is_canonical_locale(locale) {
        span.light_yellow().underlined()
    } else if workspace.is_primary_locale(locale) {
        span.light_yellow()
    } else if workspace.is_orphan_locale(locale) {
        span.gray().italic()
    } else {
        span
    }
}

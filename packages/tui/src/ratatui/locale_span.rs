use lingora_core::prelude::Locale;
use ratatui::prelude::*;

use crate::projections::Context;

pub fn locale_span<'a>(locale: &Locale, context: &Context) -> Span<'a> {
    let span = Span::from(locale.to_string());

    if context.is_canonical_locale(locale) {
        span.light_yellow().underlined()
    } else if context.is_primary_locale(locale) {
        span.light_yellow()
    } else if context.is_orphan_locale(locale) {
        span.gray().italic()
    } else {
        span
    }
}

use lingora_core::prelude::LanguageRoot;
use ratatui::prelude::*;

use crate::projections::Context;

pub fn language_root_span<'a>(root: &LanguageRoot, context: &Context) -> Span<'a> {
    let span = Span::from(root.to_string());

    if &LanguageRoot::from(context.canonical_locale()) == root {
        span.light_yellow()
    } else {
        span
    }
}

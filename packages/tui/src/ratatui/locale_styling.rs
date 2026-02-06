use std::collections::HashSet;

use lingora_core::prelude::{AuditResult, LanguageRoot, Locale};
use ratatui::prelude::*;

pub struct LocaleStyling {
    canonical: Locale,
    primaries: HashSet<Locale>,
    orphans: HashSet<Locale>,
}

impl LocaleStyling {
    pub fn from_audit_result(audit_result: &AuditResult) -> Self {
        let canonical = audit_result.workspace().canonical_locale().clone();
        let primaries = audit_result
            .workspace()
            .primary_locales()
            .cloned()
            .collect();
        let orphans = audit_result.workspace().orphan_locales().cloned().collect();

        Self {
            canonical,
            primaries,
            orphans,
        }
    }

    pub fn locale_span<'a>(&self, locale: &Locale) -> Span<'a> {
        let span = Span::from(locale.to_string());

        if locale == &self.canonical {
            span.light_yellow().underlined()
        } else if self.primaries.contains(locale) {
            span.light_yellow()
        } else if self.orphans.contains(locale) {
            span.gray().italic()
        } else {
            span
        }
    }

    pub fn language_root_span<'a>(&self, root: &LanguageRoot) -> Span<'a> {
        let span = Span::from(root.to_string());

        if &LanguageRoot::from(&self.canonical) == root {
            span.light_yellow()
        } else {
            span
        }
    }
}

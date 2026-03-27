use lingora_core::prelude::{LanguageRoot, Locale, Workspace};
use rat_focus::{FocusFlag, HasFocus};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
};
use tca_ratatui::{TcaTheme, TcaThemeCursor};

use crate::user_preferences::UserPreferences;

pub struct LingoraTheme {
    base: TcaTheme,
    canonical: Locale,
    primaries: Vec<Locale>,
    orphans: Vec<Locale>,
}

impl LingoraTheme {
    pub fn new(workspace: &Workspace) -> Self {
        let base = TcaTheme::default();
        let canonical = workspace.canonical_locale().clone();
        let primaries = Vec::from_iter(workspace.primary_locales().cloned());
        let orphans = Vec::from_iter(workspace.orphan_locales().cloned());

        Self {
            base,
            canonical,
            primaries,
            orphans,
        }
    }

    #[inline]
    pub fn base(&self) -> &TcaTheme {
        &self.base
    }

    #[inline]
    pub fn set_base(&mut self, base: TcaTheme) {
        UserPreferences::load().set_theme(&base.meta.name);
        self.base = base;
    }

    #[inline]
    pub fn next_theme(&mut self) {
        let mut cursor = TcaThemeCursor::with_all_themes();
        cursor.set_current(&self.base.meta.name);
        cursor.next().into_iter().for_each(|theme| {
            self.set_base(theme.clone());
        });
    }

    #[inline]
    pub fn previous_theme(&mut self) {
        let mut cursor = TcaThemeCursor::with_all_themes();
        cursor.set_current(&self.base.meta.name);
        cursor.prev().into_iter().for_each(|theme| {
            self.set_base(theme.clone());
        });
    }

    #[inline]
    pub fn default_style(&self) -> Style {
        let ui = &self.base.ui;
        Style::default().bg(ui.bg_primary).fg(ui.fg_primary)
    }

    #[inline]
    pub fn error(&self) -> Style {
        self.default_style().fg(self.base.semantic.error)
    }

    #[inline]
    pub fn warning(&self) -> Style {
        self.default_style().fg(self.base.semantic.warning)
    }

    #[inline]
    pub fn success(&self) -> Style {
        self.default_style().fg(self.base.semantic.success)
    }

    #[inline]
    pub fn highlight(&self) -> Style {
        self.default_style().fg(self.base.semantic.highlight)
    }

    #[inline]
    pub fn highlight_span<'a>(&self, s: &'a str) -> Span<'a> {
        Span::from(s).style(self.highlight())
    }

    pub fn language_root_span<'a>(&self, root: &LanguageRoot) -> Span<'a> {
        let span = Span::from(root.to_string());

        if &LanguageRoot::from(&self.canonical) == root {
            span.style(self.default_style().fg(self.base.semantic.highlight))
        } else {
            span
        }
    }

    pub fn locale_span<'a>(&self, locale: &Locale) -> Span<'a> {
        let semantics = &self.base.semantic;
        let span = Span::from(locale.to_string());

        if locale == &self.canonical {
            span.style(self.default_style().fg(semantics.highlight).underlined())
        } else if self.primaries.contains(locale) {
            span.style(self.default_style().fg(semantics.highlight))
        } else if self.orphans.contains(locale) {
            span.style(self.default_style().fg(semantics.warning).italic())
        } else {
            span
        }
    }

    #[inline]
    pub fn placeholder(&self) -> Style {
        self.default_style().fg(self.base.ui.fg_muted)
    }

    pub fn focus_block<'a>(&self, focus: &FocusFlag) -> Block<'a> {
        let (color, modifier) = if focus.is_focused() {
            (self.base.ui.border_primary, Modifier::BOLD)
        } else {
            (self.base.ui.border_muted, Modifier::empty())
        };

        Block::new()
            .borders(Borders::ALL)
            .border_style(self.default_style().fg(color).add_modifier(modifier))
    }

    #[inline]
    pub fn selection(&self) -> Style {
        self.default_style().bg(self.base.ui.selection_bg)
    }

    #[inline]
    pub fn muted(&self) -> Style {
        self.default_style().fg(self.base.ui.fg_muted)
    }
}

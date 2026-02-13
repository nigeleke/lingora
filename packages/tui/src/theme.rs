use lingora_core::prelude::{LanguageRoot, Locale, Workspace};
use rat_focus::{FocusFlag, HasFocus};
use ratatui::{
    style::Modifier,
    text::Span,
    widgets::{Block, Borders},
};
use ratatui_themes::{Style, Theme, ThemeName};

pub struct LingoraTheme {
    base: Theme,
    canonical: Locale,
    primaries: Vec<Locale>,
    orphans: Vec<Locale>,
}

impl LingoraTheme {
    pub fn new(base: ThemeName, workspace: &Workspace) -> Self {
        let base = Theme::new(base);
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
    pub fn set_base(&mut self, base: ThemeName) {
        self.base = Theme::new(base);
    }

    #[inline]
    pub fn default_style(&self) -> Style {
        let palette = self.base.palette();
        Style::default().bg(palette.bg).fg(palette.fg)
    }

    #[inline]
    pub fn error(&self) -> Style {
        Style::default().fg(self.base.palette().error)
    }

    #[inline]
    pub fn warning(&self) -> Style {
        Style::default().fg(self.base.palette().warning)
    }

    #[inline]
    pub fn success(&self) -> Style {
        Style::default().fg(self.base.palette().success)
    }

    #[inline]
    pub fn accent(&self) -> Style {
        Style::default().fg(self.base.palette().accent)
    }

    #[inline]
    pub fn accent_span<'a>(&self, s: &'a str) -> Span<'a> {
        Span::from(s).style(self.accent())
    }

    pub fn language_root_span<'a>(&self, root: &LanguageRoot) -> Span<'a> {
        let span = Span::from(root.to_string());

        if &LanguageRoot::from(&self.canonical) == root {
            span.style(Style::default().fg(self.base.palette().accent))
        } else {
            span
        }
    }

    pub fn locale_span<'a>(&self, locale: &Locale) -> Span<'a> {
        let palette = self.base.palette();
        let span = Span::from(locale.to_string());

        if locale == &self.canonical {
            span.style(Style::default().fg(palette.accent).underlined())
        } else if self.primaries.contains(locale) {
            span.style(Style::default().fg(palette.accent))
        } else if self.orphans.contains(locale) {
            span.style(Style::default().fg(palette.warning).italic())
        } else {
            span
        }
    }

    #[inline]
    pub fn placeholder(&self) -> Style {
        Style::default().fg(self.base.palette().muted)
    }

    pub fn focus_block<'a>(&self, focus: &FocusFlag) -> Block<'a> {
        let palette = self.base.palette();

        let (color, modifier) = if focus.is_focused() {
            (palette.accent, Modifier::BOLD)
        } else {
            (palette.muted, Modifier::empty())
        };

        Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color).add_modifier(modifier))
    }

    #[inline]
    pub fn selection(&self) -> Style {
        Style::default().bg(self.base.palette().selection)
    }

    #[inline]
    pub fn muted(&self) -> Style {
        Style::default().fg(self.base.palette().muted)
    }
}

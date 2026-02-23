use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::{AuditResult, DocumentRole, LanguageRoot, LingoraToml};
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::*};
use ratatui_themes::ThemeName;
use strum::VariantArray;

use crate::{
    components::Cursor,
    pages::{
        DioxusI18nConfig, DioxusI18nConfigState, Help, Settings, SettingsState, Translations,
        TranslationsState,
    },
    projections::{HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind},
    theme::LingoraTheme,
};

#[derive(Debug, Default)]
enum RunState {
    #[default]
    Running,
    Quit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, VariantArray)]
enum Page {
    #[default]
    Translations,
    DioxusI18nConfig,
    Settings,
    Help,
}

impl Page {
    pub fn next(&self) -> Self {
        let index = Page::VARIANTS.iter().position(|x| x == self).unwrap();
        let index = (index + 1) % Page::VARIANTS.len();
        Page::VARIANTS[index]
    }

    pub fn previous(&self) -> Self {
        let index = Page::VARIANTS.iter().position(|x| x == self).unwrap();
        let index = (index + Page::VARIANTS.len() - 1) % Page::VARIANTS.len();
        Page::VARIANTS[index]
    }
}

#[derive(Debug)]
pub struct AppViewState {
    run_state: RunState,
    theme: LingoraTheme,
    page: Page,
    translations_state: TranslationsState,
    dioxus_i18n_config_state: DioxusI18nConfigState,
    settings_state: SettingsState,
}

impl AppViewState {
    pub fn new(settings: &LingoraToml, theme: LingoraTheme, audit_result: Rc<AuditResult>) -> Self {
        Self {
            run_state: RunState::default(),
            theme,
            page: Page::default(),
            translations_state: TranslationsState::new(audit_result.clone()),
            dioxus_i18n_config_state: DioxusI18nConfigState::new(
                settings,
                audit_result.workspace(),
            ),
            settings_state: SettingsState::new(settings),
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.run_state, RunState::Running)
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::PageDown => self.set_page(self.page.next()),
            KeyCode::PageUp => self.set_page(self.page.previous()),
            KeyCode::Right if self.page == Page::Help => {
                self.theme.next_theme();
                Outcome::Changed
            }
            KeyCode::Left if self.page == Page::Help => {
                self.theme.previous_theme();
                Outcome::Changed
            }
            KeyCode::F(1) => self.set_page(Page::Help),
            _ => Outcome::Continue,
        }
    }

    #[inline]
    fn quit(&mut self) -> Outcome {
        self.run_state = RunState::Quit;
        Outcome::Changed
    }

    #[inline]
    fn set_page(&mut self, page: Page) -> Outcome {
        self.page = page;
        Outcome::Changed
    }

    #[inline]
    pub fn locale_filter(&self) -> &str {
        self.translations_state.locale_filter()
    }

    #[inline]
    pub fn identifier_filter(&self) -> &str {
        self.translations_state.identifier_filter()
    }

    #[inline]
    pub fn set_theme(&mut self, theme: ThemeName) {
        self.theme.set_base(theme);
    }
}

impl HasSelectionPair for AppViewState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<&Self::Item> {
        self.translations_state.reference()
    }

    fn target(&self) -> Option<&Self::Item> {
        self.translations_state.target()
    }
}

impl HasFocus for AppViewState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.translations_state);
        builder.widget(&self.dioxus_i18n_config_state);
        builder.widget(&self.settings_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HasScreenCursor for AppViewState {
    fn screen_cursor(&self) -> Cursor {
        if self.page == Page::Translations {
            self.translations_state.screen_cursor()
        } else {
            Cursor::None
        }
    }
}

impl HandleEvent<Event, Regular, Outcome> for AppViewState {
    fn handle(&mut self, event: &Event, _qualifier: Regular) -> Outcome {
        match event {
            Event::Key(event) => self.handle_key_event(event),
            _ => Outcome::Continue,
        }
        .or_else(|| match self.page {
            Page::Translations => self.translations_state.handle(event, Regular),
            Page::DioxusI18nConfig => self.dioxus_i18n_config_state.handle(event, Regular),
            Page::Settings => self.settings_state.handle(event, Regular),
            Page::Help => Outcome::Continue,
        })
    }
}

pub struct AppView<'a> {
    audit_result: &'a AuditResult,
}

impl<'a> AppView<'a> {
    pub fn new(audit_result: &'a AuditResult) -> Self {
        Self { audit_result }
    }
}

impl<'a> StatefulWidget for &mut AppView<'a> {
    type State = AppViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let reference = state.translations_state.reference();
        let target = state.translations_state.target();

        let footer_style = if is_valid_footer(
            self.audit_result,
            &state.translations_state,
            reference,
            target,
        )
        .unwrap_or_default()
        {
            state.theme.success()
        } else {
            state.theme.warning()
        };

        let node_span = |node: Option<&LocaleNode>| {
            if let Some(node) = node {
                match &node.kind() {
                    LocaleNodeKind::WorkspaceRoot => Span::from("workspace"),
                    LocaleNodeKind::LanguageRoot { language } => {
                        state.theme.language_root_span(language)
                    }
                    LocaleNodeKind::Locale { locale } => state.theme.locale_span(locale),
                }
            } else {
                Span::from("-")
            }
        };

        let title = Line::from(vec![
            Span::from(" Lingora - "),
            state
                .theme
                .locale_span(self.audit_result.canonical_locale()),
            Span::from(" "),
        ])
        .centered();

        let footer_left =
            Line::from(vec![state.theme.accent_span("F1"), Span::from(" - Help")]).left_aligned();

        let reference =
            node_span(reference.and_then(|id| state.translations_state.locale_node(id)));
        let target = node_span(target.and_then(|id| state.translations_state.locale_node(id)));

        let footer_right = Line::from(vec![
            state.theme.accent_span("Reference: "),
            reference.style(footer_style),
            state.theme.accent_span(" Target: "),
            target.style(footer_style),
            Span::from("  "),
        ])
        .right_aligned();

        Block::new()
            .title(title)
            .title_bottom(footer_left)
            .title_bottom(footer_right)
            .style(state.theme.default_style())
            .render(area, buf);

        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        match state.page {
            Page::Translations => {
                Translations::new(&state.theme, self.audit_result).render(
                    area,
                    buf,
                    &mut state.translations_state,
                );
            }
            Page::DioxusI18nConfig => {
                DioxusI18nConfig::new(&state.theme).render(
                    area,
                    buf,
                    &mut state.dioxus_i18n_config_state,
                );
            }
            Page::Settings => {
                Settings::new(&state.theme).render(area, buf, &mut state.settings_state);
            }
            Page::Help => Help::new(&state.theme).render(area, buf),
        };
    }
}

fn is_valid_footer(
    audit_result: &AuditResult,
    translations_state: &TranslationsState,
    reference: Option<&LocaleNodeId>,
    target: Option<&LocaleNodeId>,
) -> Option<bool> {
    let reference_node = translations_state.locale_node(reference?)?;
    let target_node = translations_state.locale_node(target?)?;

    let (reference_locale, target_locale) = match (reference_node.kind(), target_node.kind()) {
        (
            LocaleNodeKind::Locale { locale: reference },
            LocaleNodeKind::Locale { locale: target },
        ) => (reference, target),
        _ => return None,
    };

    let reference_doc = audit_result.document(reference_locale)?;
    let target_doc = audit_result.document(target_locale)?;

    let same_root = LanguageRoot::from(reference_locale) == LanguageRoot::from(target_locale);

    let result = match (reference_doc.role(), target_doc.role()) {
        (DocumentRole::Canonical, DocumentRole::Primary) => true,
        (DocumentRole::Canonical | DocumentRole::Primary, DocumentRole::Variant) => same_root,
        _ => false,
    };

    Some(result)
}

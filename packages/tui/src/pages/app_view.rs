use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::{AuditResult, LingoraToml};
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::*};
use strum::VariantArray;

use crate::{
    pages::{
        DioxusI18nConfig, DioxusI18nConfigState, Help, Settings, SettingsState, Translations,
        TranslationsState,
    },
    projections::{HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind},
    ratatui::{Cursor, Styling},
};

#[derive(Debug, Default)]
enum RunState {
    #[default]
    Uninitialized,
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
    page: Page,
    translations_state: TranslationsState,
    dioxus_i18n_config_state: DioxusI18nConfigState,
    settings_state: SettingsState,
}

impl AppViewState {
    pub fn new(settings: &LingoraToml, audit_result: Rc<AuditResult>) -> Self {
        Self {
            run_state: RunState::default(),
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
        matches!(self.run_state, RunState::Uninitialized | RunState::Running)
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::PageDown => self.set_page(self.page.next()),
            KeyCode::PageUp => self.set_page(self.page.previous()),
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
        self.translations_state.screen_cursor()
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
    styling: &'a Styling,
    audit_result: &'a AuditResult,
}

impl<'a> AppView<'a> {
    pub fn new(styling: &'a Styling, audit_result: &'a AuditResult) -> Self {
        Self {
            styling,
            audit_result,
        }
    }
}

impl<'a> StatefulWidget for &mut AppView<'a> {
    type State = AppViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let locale_styling = &self.styling.locale;

        let reference = state.translations_state.reference();
        let target = state.translations_state.target();

        let node_span = |node: Option<&LocaleNode>| {
            if let Some(node) = node {
                match &node.kind() {
                    LocaleNodeKind::WorkspaceRoot => Span::from("workspace"),
                    LocaleNodeKind::LanguageRoot { language } => {
                        locale_styling.language_root_span(&language)
                    }
                    LocaleNodeKind::Locale { locale } => locale_styling.locale_span(&locale),
                }
            } else {
                Span::from("-")
            }
        };

        let title = Line::from(vec![
            Span::from(" Lingora - "),
            locale_styling.locale_span(self.audit_result.canonical_locale()),
            Span::from(" "),
        ])
        .centered();

        let footer_left =
            Line::from(vec![Span::from("F1").blue(), Span::from(" - Help")]).left_aligned();

        let reference =
            node_span(reference.and_then(|id| state.translations_state.locale_node(&id)));
        let target = node_span(target.and_then(|id| state.translations_state.locale_node(&id)));

        let footer_right = Line::from(vec![
            Span::from("Reference: ").light_blue(),
            reference,
            Span::from(" Target: ").light_blue(),
            target,
            Span::from("  "),
        ])
        .right_aligned();

        Block::new()
            .title(title)
            .title_bottom(footer_left)
            .title_bottom(footer_right)
            .render(area, buf);

        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        match state.page {
            Page::Translations => {
                Translations::new(self.styling, self.audit_result).render(
                    area,
                    buf,
                    &mut state.translations_state,
                );
            }
            Page::DioxusI18nConfig => {
                DioxusI18nConfig.render(area, buf, &mut state.dioxus_i18n_config_state);
            }
            Page::Settings => {
                Settings.render(area, buf, &mut state.settings_state);
            }
            Page::Help => Help.render(area, buf),
        };
    }
}

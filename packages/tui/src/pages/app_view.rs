use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use lingora_core::prelude::{AuditResult, LingoraToml};
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::*};
use strum::VariantArray;

use crate::{
    pages::{
        DioxusI18nConfig, DioxusI18nConfigState, Settings, SettingsState, Translations,
        TranslationsState,
    },
    ratatui::{Cursor, locale_span},
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

#[derive(Debug, Default)]
pub struct AppViewState {
    run_state: RunState,
    page: Page,
    translations_state: TranslationsState,
    dioxus_i18n_config_state: DioxusI18nConfigState,
    settings_state: SettingsState,
}

impl AppViewState {
    pub fn is_running(&self) -> bool {
        matches!(self.run_state, RunState::Running)
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::PageDown => self.set_page(self.page.next()),
            KeyCode::PageUp => self.set_page(self.page.previous()),
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

    fn handle_mouse_event(&mut self, _event: &MouseEvent) -> Outcome {
        Outcome::Continue
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
            Event::Mouse(event) => self.handle_mouse_event(event),
            _ => Outcome::Continue,
        }
        .or_else(|| match self.page {
            Page::Translations => self.translations_state.handle(event, Regular),
            Page::DioxusI18nConfig => self.dioxus_i18n_config_state.handle(event, Regular),
            Page::Settings => self.settings_state.handle(event, Regular),
        })
    }
}

pub struct AppView {
    settings: Rc<LingoraToml>,
    audit_result: Rc<AuditResult>,
}

impl AppView {
    pub fn new(settings: LingoraToml, audit_result: AuditResult) -> Self {
        let settings = Rc::new(settings);
        let audit_result = Rc::new(audit_result);
        Self {
            settings,
            audit_result,
        }
    }
}

impl StatefulWidget for &mut AppView {
    type State = AppViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let settings = &self.settings;
        let audit_result = &self.audit_result;
        let workspace = &audit_result.workspace();

        let title = vec![
            Span::from(" Lingora - "),
            locale_span(workspace.canonical_locale(), workspace),
            Span::from(" "),
        ];

        let footer_left = vec![
            Span::from("PgUp/PgDn").blue(),
            Span::from(" - Page up/down   "),
            Span::from("Tab/Shift+Tab").blue(),
            Span::from(" - Change focus   "),
            Span::from("↑/↓ ").blue(),
            Span::from(" - Select   "),
            Span::from("F1").blue(),
            Span::from(" - Help"),
        ];

        Block::new()
            .title(Line::from(title).centered())
            .title_bottom(Line::from(footer_left).left_aligned())
            .render(area, buf);

        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        match state.page {
            Page::Translations => {
                Translations::new(audit_result.clone()).render(
                    area,
                    buf,
                    &mut state.translations_state,
                );
            }
            Page::DioxusI18nConfig => {
                DioxusI18nConfig::new(settings, audit_result.workspace()).render(
                    area,
                    buf,
                    &mut state.dioxus_i18n_config_state,
                );
            }
            Page::Settings => {
                Settings::new(settings).render(area, buf, &mut state.settings_state);
            }
        };
    }
}

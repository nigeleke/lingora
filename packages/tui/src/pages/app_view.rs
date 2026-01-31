use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
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
    projections::{
        Comparison, Context, HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind,
    },
    ratatui::{Cursor, language_root_span, locale_span},
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
    comparison: Comparison,
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

    #[inline]
    fn handle_mouse_event(&mut self, _event: &MouseEvent) -> Outcome {
        Outcome::Continue
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

    fn reference(&self) -> Option<Self::Item> {
        self.translations_state.reference()
    }

    fn target(&self) -> Option<Self::Item> {
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
    context: Context,
}

impl From<Context> for AppView {
    fn from(context: Context) -> Self {
        Self { context }
    }
}

impl StatefulWidget for &mut AppView {
    type State = AppViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let context = &self.context;

        let reference = state.translations_state.reference().or_else(|| {
            context
                .node_id_for_locale(context.canonical_locale())
                .cloned()
        });
        let target = state.translations_state.target();

        state.comparison.update(reference, target);

        let node_span = |node: Option<&LocaleNode>| {
            if let Some(node) = node {
                match &node.kind() {
                    LocaleNodeKind::WorkspaceRoot => Span::from("workspace"),
                    LocaleNodeKind::LanguageRoot { language } => {
                        language_root_span(&language, &context)
                    }
                    LocaleNodeKind::Locale { locale } => locale_span(&locale, &context),
                }
            } else {
                Span::from("-")
            }
        };

        let title = Line::from(vec![
            Span::from(" Lingora - "),
            locale_span(context.canonical_locale(), &context),
            Span::from(" "),
        ])
        .centered();

        let footer_left = Line::from(vec![
            Span::from("PgUp/PgDn").blue(),
            Span::from(" - Page up/down   "),
            Span::from("Tab/Shift+Tab").blue(),
            Span::from(" - Change focus   "),
            Span::from("↑/↓").blue(),
            Span::from(" - Select target   "),
            Span::from("<sp>").blue(),
            Span::from(" - Set reference   "),
            Span::from("F1").blue(),
            Span::from(" - Help"),
        ])
        .left_aligned();

        let reference = node_span(reference.and_then(|id| context.locale_node(&id)));
        let target = node_span(target.and_then(|id| context.locale_node(&id)));

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
                Translations::from(context.clone()).render(
                    area,
                    buf,
                    &mut state.translations_state,
                );
            }
            Page::DioxusI18nConfig => {
                DioxusI18nConfig::from(context.clone()).render(
                    area,
                    buf,
                    &mut state.dioxus_i18n_config_state,
                );
            }
            Page::Settings => {
                Settings::new(context.settings()).render(area, buf, &mut state.settings_state);
            }
        };
    }
}

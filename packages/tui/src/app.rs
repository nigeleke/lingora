use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};
use lingora_core::prelude::*;
use ratatui::{DefaultTerminal, prelude::*, widgets::Block};
use tui_scrollview::ScrollViewState;

use crate::{
    args::TuiArgs,
    error::TuiError,
    pages::{DioxusI18nConfig, Settings, Translations},
    state::{Page, RunState, UiState},
};

pub struct App {
    settings: LingoraToml,
    report: AuditReport,
    ui_state: UiState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), TuiError> {
        while matches!(self.ui_state.run_state, RunState::Running) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), TuiError> {
        match event::read()? {
            Event::Key(event) => self.handle_key_events(event),
            Event::Mouse(event) => self.handle_mouse_events(event),
            _ => Ok(()),
        }
    }

    fn handle_key_events(&mut self, event: KeyEvent) -> Result<(), TuiError> {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::PageDown => self.next_page(),
            KeyCode::PageUp => self.previous_page(),
            KeyCode::Tab => self.next_focus(),
            KeyCode::BackTab => self.previous_focus(),
            _ => {}
        }

        Ok(())
    }

    #[inline]
    fn quit(&mut self) {
        self.ui_state.run_state = RunState::Quit;
    }

    #[inline]
    fn next_page(&mut self) {
        self.ui_state.page = self.ui_state.page.next();
    }

    #[inline]
    fn previous_page(&mut self) {
        self.ui_state.page = self.ui_state.page.previous();
    }

    #[inline]
    fn next_focus(&mut self) {
        self.ui_state.focused_widget = self.ui_state.focused_widget.next();
    }

    #[inline]
    fn previous_focus(&mut self) {
        self.ui_state.focused_widget = self.ui_state.focused_widget.previous();
    }

    fn handle_mouse_events(&mut self, _event: MouseEvent) -> Result<(), TuiError> {
        Ok(())
    }
}

impl TryFrom<&LingoraToml> for App {
    type Error = TuiError;

    fn try_from(settings: &LingoraToml) -> Result<Self, Self::Error> {
        let settings = settings.clone();

        let engine = AuditEngine::try_from(&settings)?;
        let report = engine.run()?;

        let ui_state = UiState::default();

        Ok(Self {
            settings,
            report,
            ui_state,
        })
    }
}

impl TryFrom<&TuiArgs> for App {
    type Error = TuiError;

    fn try_from(value: &TuiArgs) -> Result<Self, Self::Error> {
        let settings = LingoraToml::try_from(value.core_args())?;
        Self::try_from(&settings)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let canonical_locale = self.report.canonical_locale().to_string();

        let title = vec![
            Span::from(" Lingora - "),
            Span::from(canonical_locale).light_yellow(),
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
        match self.ui_state.page {
            Page::Translations => {
                Translations::new(&self.report, &self.ui_state).render(area, buf);
            }
            Page::DioxusI18nConfig => {
                let mut scroll_state = ScrollViewState::default();
                DioxusI18nConfig::new(&self.settings, &self.ui_state).render(
                    area,
                    buf,
                    &mut scroll_state,
                );
            }
            Page::Settings => {
                let mut scroll_state = ScrollViewState::default();
                Settings::new(&self.settings, &self.ui_state).render(area, buf, &mut scroll_state);
            }
        };
    }
}

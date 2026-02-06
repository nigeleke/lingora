use std::rc::Rc;

use crossterm::event;
use lingora_core::prelude::*;
use rat_event::{HandleEvent, Regular};
use rat_text::HasScreenCursor;
use ratatui::{DefaultTerminal, prelude::*};

use crate::{
    args::TuiArgs,
    error::TuiError,
    pages::{AppView, AppViewState},
    ratatui::Styling,
};

pub struct App {
    settings: LingoraToml,
    audit_result: Rc<AuditResult>,
    styling: Styling,
    state: AppViewState,
}

impl App {
    pub fn new(settings: LingoraToml, audit_result: AuditResult) -> Self {
        let styling = Styling::from_audit_result(&audit_result);
        let audit_result = Rc::new(audit_result);
        let state = AppViewState::new(&settings, audit_result.clone());

        Self {
            settings,
            audit_result,
            styling,
            state,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), TuiError> {
        while self.state.is_running() {
            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut view = AppView::new(&self.styling, &self.audit_result);

        frame.render_stateful_widget(&mut view, frame.area(), &mut self.state);
        if let Some(cursor) = self.state.screen_cursor() {
            frame.set_cursor_position(cursor);
        }
    }

    fn handle_events(&mut self) -> Result<(), TuiError> {
        let event = event::read()?;
        self.state.handle(&event, Regular);
        Ok(())
    }
}

impl TryFrom<LingoraToml> for App {
    type Error = TuiError;

    fn try_from(settings: LingoraToml) -> Result<Self, Self::Error> {
        let engine = AuditEngine::try_from(&settings)?;
        let audit_result = engine.run()?;

        Ok(App::new(settings, audit_result))
    }
}

impl TryFrom<&TuiArgs> for App {
    type Error = TuiError;

    fn try_from(value: &TuiArgs) -> Result<Self, Self::Error> {
        let settings = LingoraToml::try_from(value.core_args())?;
        Self::try_from(settings)
    }
}

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
    projections::Context,
};

pub struct App {
    settings: Rc<LingoraToml>,
    audit_result: Rc<AuditResult>,
    state: AppViewState,
}

impl App {
    pub fn new(settings: Rc<LingoraToml>, audit_result: Rc<AuditResult>) -> Self {
        let state = AppViewState::new(audit_result.clone());
        Self {
            settings,
            audit_result,
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
        let context = Context::new(&self.settings, &self.audit_result, &self.state);

        let mut view = AppView::from(context);

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

impl TryFrom<&LingoraToml> for App {
    type Error = TuiError;

    fn try_from(settings: &LingoraToml) -> Result<Self, Self::Error> {
        let settings = Rc::new(settings.clone());

        let engine = AuditEngine::try_from(&*settings)?;
        let audit_result = Rc::new(engine.run()?);

        Ok(App::new(settings, audit_result))
    }
}

impl TryFrom<&TuiArgs> for App {
    type Error = TuiError;

    fn try_from(value: &TuiArgs) -> Result<Self, Self::Error> {
        let settings = LingoraToml::try_from(value.core_args())?;
        Self::try_from(&settings)
    }
}

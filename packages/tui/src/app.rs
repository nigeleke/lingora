use crossterm::event;
use lingora_core::prelude::*;
use rat_event::{HandleEvent, Regular};
use ratatui::{DefaultTerminal, prelude::*};

use crate::{
    args::TuiArgs,
    error::TuiError,
    pages::{AppView, AppViewState},
};

pub struct App {
    view: AppView,
    state: AppViewState,
}

impl App {
    pub fn new(settings: LingoraToml, audit_result: AuditResult) -> Self {
        let view = AppView::new(settings, audit_result);
        let state = AppViewState::default();
        Self { view, state }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), TuiError> {
        while self.state.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget(&mut self.view, frame.area(), &mut self.state);
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
        let settings = settings.clone();

        let engine = AuditEngine::try_from(&settings)?;
        let audit_result = engine.run()?;

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

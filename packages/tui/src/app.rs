use std::rc::Rc;

use crossterm::event;
use lingora_core::prelude::*;
use rat_event::{HandleEvent, Regular};
use rat_text::HasScreenCursor;
use ratatui::{DefaultTerminal, prelude::*};
use ratatui_themes::ThemeName;

use crate::{
    args::TuiArgs,
    error::TuiError,
    pages::{AppView, AppViewState},
    theme::LingoraTheme,
};

/// The main application state and driver for the interactive terminal user interface.
///
/// `App` owns:
/// - The visual theme (`LingoraTheme`)
/// - The `AuditResult` (shared across widgets/views)
/// - The current application view state (`AppViewState`)
///
/// Responsibilities:
/// - Initialize from configuration and audit result
/// - Run the main event/draw loop
/// - Delegate rendering to `AppView` (stateful widget)
/// - Forward keyboard/mouse events to the view state
/// - Manage cursor visibility and position
pub struct App {
    audit_result: Rc<AuditResult>,
    state: AppViewState,
}

impl App {
    /// Creates a new `App` instance from settings and a completed audit result.
    ///
    /// - Initializes the theme
    /// - Wraps the audit result in `Rc` for shared access
    /// - Creates initial view state from settings and result
    pub fn new(settings: LingoraToml, audit_result: AuditResult) -> Self {
        let theme = LingoraTheme::new(ThemeName::Dracula, audit_result.workspace());
        let audit_result = Rc::new(audit_result);
        let state = AppViewState::new(&settings, theme, audit_result.clone());

        Self {
            audit_result,
            state,
        }
    }

    /// Replaces the base theme and returns `self` (builder-style).
    pub fn set_theme(mut self, theme: ThemeName) -> Self {
        self.state.set_theme(theme);
        self
    }

    /// Runs the main TUI event/draw loop until the user quits.
    ///
    /// Loop steps:
    /// 1. Draw current frame using `AppView` widget
    /// 2. Read next crossterm event
    /// 3. Handle event (keyboard, mouse, resize) via `AppViewState`
    /// 4. Repeat until `state.is_running()` returns `false` (usually on 'q' or Ctrl+C)
    ///
    /// # Errors
    /// Propagates terminal I/O or event reading failures as `TuiError::Io`.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), TuiError> {
        while self.state.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut view = AppView::new(&self.audit_result);

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
        Self::try_from(settings).map(|app| app.set_theme(value.theme()))
    }
}

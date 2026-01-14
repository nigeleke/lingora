use lingora_core::prelude::AuditReport;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

use crate::{
    focus_border_type,
    state::{FocusableWidget, UiState},
};

pub struct Identifiers<'a> {
    report: &'a AuditReport,
    ui_state: &'a UiState,
}

impl<'a> Identifiers<'a> {
    pub fn new(report: &'a AuditReport, ui_state: &'a UiState) -> Self {
        Self { report, ui_state }
    }
}

impl Widget for &Identifiers<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ui_state = &self.ui_state;

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);

        Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(
                ui_state,
                FocusableWidget::IdentifierFilter
            ))
            .render(chunks[0], buf);

        Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(
                ui_state,
                FocusableWidget::IdentifierList
            ))
            .render(chunks[1], buf);
    }
}

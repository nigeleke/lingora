use lingora_core::prelude::AuditReport;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    components::{Identifiers, Locales},
    state::UiState,
};

pub struct Translations<'a> {
    report: &'a AuditReport,
    ui_state: &'a UiState,
}

impl<'a> Translations<'a> {
    pub fn new(report: &'a AuditReport, ui_state: &'a UiState) -> Self {
        Self { report, ui_state }
    }
}

impl Widget for &Translations<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::horizontal(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Min(0),
        ])
        .split(area);

        Locales::new(self.report, self.ui_state).render(chunks[0], buf);
        Identifiers::new(&self.report, self.ui_state).render(chunks[1], buf);
        Paragraph::new("Entries").render(chunks[2], buf);
    }
}

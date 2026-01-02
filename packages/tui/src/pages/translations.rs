use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    GlobalContext,
    components::{Identifiers, Locales},
    state::UiState,
};

pub struct Translations<'a> {
    context: &'a GlobalContext,
    ui_state: &'a UiState,
}

impl<'a> Translations<'a> {
    pub fn new(context: &'a GlobalContext, ui_state: &'a UiState) -> Self {
        Self { context, ui_state }
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

        Locales::new(self.context, self.ui_state).render(chunks[0], buf);
        Identifiers::new(self.context, self.ui_state).render(chunks[1], buf);
        Paragraph::new("Entries").render(chunks[2], buf);
    }
}

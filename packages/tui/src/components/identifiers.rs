use crossterm::event::Event;
use lingora_core::prelude::QualifiedIdentifier;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::prelude::*;

use crate::{
    components::{
        Cursor, IdentifierFilter, IdentifierFilterState, IdentifierList, IdentifierListState,
    },
    theme::LingoraTheme,
};

#[derive(Debug, Default)]
pub struct IdentifiersState {
    filter_state: IdentifierFilterState,
    list_state: IdentifierListState,
}

impl IdentifiersState {
    #[inline(always)]
    pub fn filter(&self) -> &str {
        self.filter_state.text()
    }

    #[inline]
    pub fn selected(&self) -> Option<&QualifiedIdentifier> {
        self.list_state.selected()
    }
}

impl HasFocus for IdentifiersState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.filter_state);
        builder.widget(&self.list_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HasScreenCursor for IdentifiersState {
    fn screen_cursor(&self) -> Cursor {
        self.filter_state.screen_cursor()
    }
}

impl HandleEvent<Event, Regular, Outcome> for IdentifiersState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.filter_state
            .handle(event, qualifier)
            .or_else(|| self.list_state.handle(event, qualifier))
    }
}

pub struct Identifiers<'a> {
    theme: &'a LingoraTheme,
    identifiers: Vec<QualifiedIdentifier>,
}

impl<'a> Identifiers<'a> {
    pub fn new(
        theme: &'a LingoraTheme,
        identifiers: impl Iterator<Item = QualifiedIdentifier>,
    ) -> Self {
        let identifiers = Vec::from_iter(identifiers);
        Self { theme, identifiers }
    }
}

impl StatefulWidget for &Identifiers<'_> {
    type State = IdentifiersState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let filter = state.filter_state.text().to_ascii_lowercase();
        let filtered_identifiers = self
            .identifiers
            .iter()
            .filter(|id| id.to_meta_string().to_ascii_lowercase().contains(&filter));

        let filter = IdentifierFilter::new(self.theme);
        let list = IdentifierList::new(self.theme, filtered_identifiers.cloned());

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        list.render(chunks[1], buf, &mut state.list_state);
    }
}

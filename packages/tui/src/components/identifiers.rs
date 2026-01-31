use crossterm::event::Event;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::prelude::*;

use crate::{
    components::{IdentifierFilter, IdentifierFilterState, IdentifierList, IdentifierListState},
    projections::Context,
    ratatui::Cursor,
};

#[derive(Debug, Default)]
pub struct IdentifiersState {
    filter_state: IdentifierFilterState,
    list_state: IdentifierListState,
}

impl IdentifiersState {
    #[inline]
    pub fn filter(&self) -> &str {
        self.filter_state.text()
    }
}

impl HasFocus for IdentifiersState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.filter_state);
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

pub struct Identifiers {
    context: Context,
}

impl From<Context> for Identifiers {
    fn from(context: Context) -> Self {
        Self { context }
    }
}

impl StatefulWidget for &Identifiers {
    type State = IdentifiersState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let filter = IdentifierFilter::from(self.context.clone());
        let list = IdentifierList::from(self.context.clone());

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        list.render(chunks[1], buf, &mut state.list_state);
    }
}

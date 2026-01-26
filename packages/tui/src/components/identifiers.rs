use std::rc::Rc;

use crossterm::event::Event;
use lingora_core::prelude::AuditResult;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::prelude::*;

use crate::components::{
    IdentifierFilter, IdentifierFilterState, IdentifierList, IdentifierListState,
};

#[derive(Debug, Default)]
pub struct IdentifiersState {
    filter_state: IdentifierFilterState,
    list_state: IdentifierListState,
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

impl HandleEvent<Event, Regular, Outcome> for IdentifiersState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.filter_state
            .handle(event, qualifier)
            .or_else(|| self.list_state.handle(event, qualifier))
    }
}

pub struct Identifiers {
    audit_result: Rc<AuditResult>,
}

impl Identifiers {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        Self { audit_result }
    }
}

impl StatefulWidget for &Identifiers {
    type State = IdentifiersState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let filter = IdentifierFilter;
        let list = IdentifierList;

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        list.render(chunks[1], buf, &mut state.list_state);
    }
}

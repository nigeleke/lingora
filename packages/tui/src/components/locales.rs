use std::rc::Rc;

use crossterm::event::Event;
use lingora_core::prelude::AuditResult;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::StatefulWidget};

use crate::components::{LocaleFilter, LocaleFilterState, LocaleTree, LocaleTreeState};

#[derive(Debug, Default)]
pub struct LocalesState {
    pub filter_state: LocaleFilterState,
    pub tree_state: LocaleTreeState,
}

impl HasFocus for LocalesState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.filter_state);
        builder.widget(&self.tree_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HandleEvent<Event, Regular, Outcome> for LocalesState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.filter_state
            .handle(event, qualifier)
            .or_else(|| self.tree_state.handle(event, qualifier))
    }
}

pub struct Locales {
    audit_result: Rc<AuditResult>,
}

impl Locales {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        Self { audit_result }
    }
}

impl StatefulWidget for &Locales {
    type State = LocalesState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let filter = LocaleFilter;
        let tree = LocaleTree::new(self.audit_result.clone());

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        tree.render(chunks[1], buf, &mut state.tree_state);
    }
}

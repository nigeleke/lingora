use std::rc::Rc;

use crossterm::event::Event;
use lingora_core::prelude::{AuditResult, Locale};
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::StatefulWidget};

use crate::{
    components::{LocaleFilter, LocaleFilterState, LocaleTree, LocaleTreeState},
    projections::translations_tree::{NodeKind, TranslationsTree},
    ratatui::Cursor,
};

#[derive(Debug, Default)]
pub struct LocalesState {
    filter_state: LocaleFilterState,
    tree_state: LocaleTreeState,
    selected_locale: Option<NodeKind>,
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

impl HasScreenCursor for LocalesState {
    fn screen_cursor(&self) -> Cursor {
        self.filter_state.screen_cursor()
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
    tree_model: Rc<TranslationsTree>,
}

impl Locales {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        let tree_model = Rc::new(TranslationsTree::from(&*audit_result));
        Self {
            audit_result,
            tree_model,
        }
    }
}

impl StatefulWidget for &Locales {
    type State = LocalesState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        // state.selected_locale = state.tree_state.selected_node().map(|n| n.kind());

        let filter = LocaleFilter;
        let tree = LocaleTree::new(self.tree_model.clone(), self.audit_result.clone());

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        tree.render(chunks[1], buf, &mut state.tree_state);
    }
}

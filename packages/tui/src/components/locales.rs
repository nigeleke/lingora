use crossterm::event::Event;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::StatefulWidget};

use crate::{
    components::{LocaleFilter, LocaleFilterState, LocaleTree, LocaleTreeState},
    projections::{FilteredLocalesHierarchy, HasSelectionPair, LocaleNodeId, LocalesHierarchy},
    ratatui::{Cursor, Styling},
};

#[derive(Debug)]
pub struct LocalesState {
    filter_state: LocaleFilterState,
    tree_state: LocaleTreeState,
}

impl LocalesState {
    pub fn new(
        reference_node_id: Option<LocaleNodeId>,
        nodes: impl IntoIterator<Item = LocaleNodeId>,
    ) -> Self {
        let filter_state = LocaleFilterState::default();
        let tree_state = LocaleTreeState::new(reference_node_id, nodes);
        Self {
            filter_state,
            tree_state,
        }
    }

    pub fn filter(&self) -> &str {
        self.filter_state.text()
    }
}

impl HasSelectionPair for LocalesState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<&Self::Item> {
        self.tree_state.reference()
    }

    fn target(&self) -> Option<&Self::Item> {
        self.tree_state.target()
    }
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

pub struct Locales<'a> {
    styling: &'a Styling,
    locales_hierarchy: &'a LocalesHierarchy,
}

impl<'a> Locales<'a> {
    pub fn new(styling: &'a Styling, locales_hierarchy: &'a LocalesHierarchy) -> Self {
        Self {
            styling,
            locales_hierarchy,
        }
    }
}

impl StatefulWidget for &Locales<'_> {
    type State = LocalesState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let filtered_hierarchy = FilteredLocalesHierarchy::filter_from(
            self.locales_hierarchy,
            state.filter_state.text(),
        );

        let filter = LocaleFilter::new(&self.styling.focus, &self.styling.text);
        let tree = LocaleTree::new(
            &self.styling.focus,
            &self.styling.locale,
            &filtered_hierarchy,
        );

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);
        filter.render(chunks[0], buf, &mut state.filter_state);
        tree.render(chunks[1], buf, &mut state.tree_state);
    }
}

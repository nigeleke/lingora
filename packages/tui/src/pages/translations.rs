use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::AuditResult;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{Focus, FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    components::{Identifiers, IdentifiersState, Locales, LocalesState},
    projections::{Comparison, HasSelectionPair, LocaleNode, LocaleNodeId, LocalesHierarchy},
    ratatui::{Cursor, Styling},
};

#[derive(Debug)]
pub struct TranslationsState {
    focus: Option<Focus>,
    locales_state: LocalesState,
    identifiers_state: IdentifiersState,
    comparison: Comparison,
}

impl TranslationsState {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        let canonical_locale = audit_result.workspace().canonical_locale();

        let locales_hierachy = LocalesHierarchy::from(&*audit_result);

        let reference_node_id = locales_hierachy
            .node_id_for_locale(canonical_locale)
            .copied();
        let nodes = locales_hierachy.nodes().keys().copied();
        let locales_state = LocalesState::new(reference_node_id, nodes);
        let identifiers_state = IdentifiersState::default();

        let comparison =
            Comparison::from_reference(reference_node_id, audit_result, locales_hierachy);

        Self {
            focus: None,
            locales_state,
            identifiers_state,
            comparison,
        }
    }

    pub fn rebuild_focus(&mut self) {
        let mut builder = FocusBuilder::new(self.focus.take());
        self.build(&mut builder);
        self.focus = Some(builder.build());
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match event.code {
            KeyCode::Tab => self.focus_next(),
            KeyCode::BackTab => self.focus_prev(),
            _ => Outcome::Continue,
        }
    }

    #[inline]
    fn focus_next(&mut self) -> Outcome {
        let focus = self.focus.as_mut().expect("focus");
        focus.next();
        Outcome::Unchanged
    }

    #[inline]
    fn focus_prev(&mut self) -> Outcome {
        let focus = self.focus.as_mut().expect("focus");
        focus.prev();
        Outcome::Unchanged
    }

    #[inline]
    pub fn locale_filter(&self) -> &str {
        self.locales_state.filter()
    }

    #[inline]
    pub fn locale_node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
        self.comparison.locale_node(node_id)
    }

    #[inline]
    pub fn identifier_filter(&self) -> &str {
        self.identifiers_state.filter()
    }
}

impl HasSelectionPair for TranslationsState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<&Self::Item> {
        self.locales_state.reference()
    }

    fn target(&self) -> Option<&Self::Item> {
        self.locales_state.target()
    }
}

impl HasFocus for TranslationsState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.locales_state);
        builder.widget(&self.identifiers_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HasScreenCursor for TranslationsState {
    fn screen_cursor(&self) -> Cursor {
        self.locales_state
            .screen_cursor()
            .or_else(|| self.identifiers_state.screen_cursor())
    }
}

impl HandleEvent<Event, Regular, Outcome> for TranslationsState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.rebuild_focus();

        match event {
            Event::Key(event) => self.handle_key_event(event),
            _ => Outcome::Continue,
        }
        .or_else(|| self.locales_state.handle(event, qualifier))
        .or_else(|| self.identifiers_state.handle(event, qualifier))
    }
}

pub struct Translations<'a> {
    styling: &'a Styling,
    audit_result: &'a AuditResult,
}

impl<'a> Translations<'a> {
    pub fn new(styling: &'a Styling, audit_result: &'a AuditResult) -> Self {
        Self {
            styling,
            audit_result,
        }
    }
}

impl<'a> StatefulWidget for &Translations<'a> {
    type State = TranslationsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state
            .comparison
            .update_with_reference_and_target(state.reference().copied(), state.target().copied());

        let chunks = Layout::horizontal(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(30),
            Constraint::Min(0),
        ])
        .split(area);

        Locales::new(self.styling, state.comparison.locales_hierarchy()).render(
            chunks[0],
            buf,
            &mut state.locales_state,
        );
        Identifiers::new(self.styling, state.comparison.identifiers()).render(
            chunks[1],
            buf,
            &mut state.identifiers_state,
        );
        Paragraph::new(format!("Entries {}", 42)).render(chunks[2], buf);
    }
}

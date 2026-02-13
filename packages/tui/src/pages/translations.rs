use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::AuditResult;
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{Focus, FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::prelude::*;

use crate::{
    components::{
        Cursor, Entries, EntriesState, Identifiers, IdentifiersState, Issues, IssuesState, Locales,
        LocalesState,
    },
    projections::{
        Comparison, FilteredIssues, HasSelectionPair, LocaleNode, LocaleNodeId, LocalesHierarchy,
    },
    theme::LingoraTheme,
};

#[derive(Debug)]
pub struct TranslationsState {
    focus: Option<Focus>,
    locales_state: LocalesState,
    identifiers_state: IdentifiersState,
    reference_entries_state: EntriesState,
    target_entries_state: EntriesState,
    issues_state: IssuesState,
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

        let reference_entries_state = EntriesState::default();
        let target_entries_state = EntriesState::default();
        let issues_state = IssuesState::default();

        let comparison =
            Comparison::from_reference(reference_node_id, audit_result, locales_hierachy);

        Self {
            focus: None,
            locales_state,
            identifiers_state,
            reference_entries_state,
            target_entries_state,
            issues_state,
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

    #[inline(always)]
    pub fn locale_filter(&self) -> &str {
        self.locales_state.filter()
    }

    #[inline(always)]
    pub fn locale_node(&self, node_id: &LocaleNodeId) -> Option<&LocaleNode> {
        self.comparison.locale_node(node_id)
    }

    #[inline(always)]
    pub fn identifier_filter(&self) -> &str {
        self.identifiers_state.filter()
    }
}

impl HasSelectionPair for TranslationsState {
    type Item = LocaleNodeId;

    #[inline(always)]
    fn reference(&self) -> Option<&Self::Item> {
        self.locales_state.reference()
    }

    #[inline(always)]
    fn target(&self) -> Option<&Self::Item> {
        self.locales_state.target()
    }
}

impl HasFocus for TranslationsState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.locales_state);
        builder.widget(&self.identifiers_state);
        builder.widget(&self.reference_entries_state);
        builder.widget(&self.target_entries_state);
        builder.widget(&self.issues_state);
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
        .or_else(|| self.reference_entries_state.handle(event, qualifier))
        .or_else(|| self.target_entries_state.handle(event, qualifier))
        .or_else(|| self.issues_state.handle(event, qualifier))
    }
}

pub struct Translations<'a> {
    theme: &'a LingoraTheme,
    audit_result: &'a AuditResult,
}

impl<'a> Translations<'a> {
    pub fn new(theme: &'a LingoraTheme, audit_result: &'a AuditResult) -> Self {
        Self {
            theme,
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

        let main_columns = Layout::horizontal(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Min(0),
        ])
        .split(area);

        let comparison_outer = Layout::vertical(vec![Constraint::Min(0), Constraint::Length(10)])
            .split(main_columns[2]);

        let comparison_inner =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(comparison_outer[0]);

        Locales::new(self.theme, state.comparison.locales_hierarchy()).render(
            main_columns[0],
            buf,
            &mut state.locales_state,
        );

        Identifiers::new(self.theme, state.comparison.identifiers().cloned()).render(
            main_columns[1],
            buf,
            &mut state.identifiers_state,
        );

        Entries::new(
            self.theme,
            state
                .comparison
                .reference_entries(state.identifiers_state.selected()),
        )
        .render(comparison_inner[0], buf, &mut state.reference_entries_state);

        Entries::new(
            self.theme,
            state
                .comparison
                .target_entries(state.identifiers_state.selected()),
        )
        .render(comparison_inner[1], buf, &mut state.target_entries_state);

        let filtered_issues = FilteredIssues::from_issues(self.audit_result.issues(), state);

        Issues::new(self.theme, filtered_issues.issues().to_owned()).render(
            comparison_outer[1],
            buf,
            &mut state.issues_state,
        );
    }
}

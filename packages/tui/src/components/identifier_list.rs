use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::QualifiedIdentifier;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};

use crate::ratatui::FocusStyling;

#[derive(Debug, Default)]
pub struct IdentifierListState {
    focus_flag: FocusFlag,
    list_state: ListState,
    target: Option<usize>,
    area: Rect,
}

impl IdentifierListState {
    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match &event.code {
            KeyCode::Up => {
                self.list_state.select_previous();
                self.target = self.list_state.selected();
                Outcome::Unchanged
            }
            KeyCode::Down => {
                self.list_state.select_next();
                self.target = self.list_state.selected();
                Outcome::Unchanged
            }
            _ => Outcome::Continue,
        }
    }
}

impl HasFocus for IdentifierListState {
    fn build(&self, builder: &mut rat_focus::FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.focus_flag.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }
}

impl HandleEvent<Event, Regular, Outcome> for IdentifierListState {
    fn handle(&mut self, event: &Event, _qualifier: Regular) -> Outcome {
        if self.focus_flag.is_focused() {
            match event {
                Event::Key(event) => self.handle_key_event(event),
                _ => Outcome::Continue,
            }
        } else {
            Outcome::Continue
        }
    }
}

pub struct IdentifierList<'a> {
    focus_styling: &'a FocusStyling,
    filtered_identifiers: Vec<QualifiedIdentifier>,
}

impl<'a> IdentifierList<'a> {
    pub fn new(
        focus_styling: &'a FocusStyling,
        filtered_identifiers: impl Iterator<Item = QualifiedIdentifier>,
    ) -> Self {
        let mut filtered_identifiers = Vec::from_iter(filtered_identifiers);
        filtered_identifiers.sort();

        Self {
            focus_styling,
            filtered_identifiers,
        }
    }
}

impl StatefulWidget for IdentifierList<'_> {
    type State = IdentifierListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let list = List::new(
            self.filtered_identifiers
                .iter()
                .map(|s| Text::from(s.to_meta_string())),
        )
        .block(self.focus_styling.block(&state.focus_flag))
        .highlight_style(Style::default().bg(Color::LightBlue))
        .highlight_symbol("» ")
        .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

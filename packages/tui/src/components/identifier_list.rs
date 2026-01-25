use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::StatefulWidget};

use crate::ratatui::focus_block;

#[derive(Debug, Default)]
pub struct IdentifierListState {
    focus_flag: FocusFlag,
    area: Rect,
}

impl HasFocus for IdentifierListState {
    fn build(&self, builder: &mut rat_focus::FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.focus_flag.clone()
    }

    fn area(&self) -> rat_focus::ratatui::layout::Rect {
        self.area
    }
}

impl HandleEvent<Event, Regular, Outcome> for IdentifierListState {
    fn handle(&mut self, _event: &Event, _qualifier: Regular) -> Outcome {
        Outcome::Continue // TODO:
    }
}

pub struct IdentifierList;

impl StatefulWidget for IdentifierList {
    type State = IdentifierListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        focus_block(&state.focus_flag).render(area, buf);
    }
}

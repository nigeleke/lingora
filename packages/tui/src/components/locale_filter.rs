use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::text_input::*;
use ratatui::prelude::*;

use crate::ratatui::focus_block;

#[derive(Debug, Default)]
pub struct LocaleFilterState {
    focus_flag: FocusFlag,
    input_state: TextInputState,
    area: Rect,
}

impl HasFocus for LocaleFilterState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.focus_flag.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }
}

impl HandleEvent<Event, Regular, Outcome> for LocaleFilterState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        if self.focus_flag.is_focused() {
            self.input_state.handle(event, qualifier).into()
        } else {
            Outcome::Continue
        }
    }
}

pub struct LocaleFilter;

impl StatefulWidget for LocaleFilter {
    type State = LocaleFilterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        TextInput::new()
            .block(focus_block(&state.focus_flag))
            .render(area, buf, &mut state.input_state);
    }
}

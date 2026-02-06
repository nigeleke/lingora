use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::{HasScreenCursor, text_input::*};
use ratatui::prelude::*;

use crate::ratatui::{Cursor, FocusStyling, TextStyling};

#[derive(Debug)]
pub struct LocaleFilterState {
    input_state: TextInputState,
    area: Rect,
}

impl LocaleFilterState {
    pub fn text(&self) -> &str {
        self.input_state.text()
    }
}

impl HasFocus for LocaleFilterState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.input_state.focus.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }
}

impl HasScreenCursor for LocaleFilterState {
    fn screen_cursor(&self) -> Cursor {
        self.input_state.screen_cursor()
    }
}

impl HandleEvent<Event, Regular, Outcome> for LocaleFilterState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        if self.input_state.is_focused() {
            self.input_state.handle(event, qualifier).into()
        } else {
            Outcome::Continue
        }
    }
}

impl Default for LocaleFilterState {
    fn default() -> Self {
        let input_state = TextInputState::default();
        input_state.focus.set(true);
        let area = Rect::default();

        Self { input_state, area }
    }
}

pub struct LocaleFilter<'a> {
    focus_styling: &'a FocusStyling,
    text_styling: &'a TextStyling,
}

impl<'a> LocaleFilter<'a> {
    pub fn new(focus_styling: &'a FocusStyling, text_styling: &'a TextStyling) -> Self {
        Self {
            focus_styling,
            text_styling,
        }
    }
}

impl StatefulWidget for LocaleFilter<'_> {
    type State = LocaleFilterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let block = self.focus_styling.block(&state.input_state.focus);

        let is_focused = state.input_state.focus.is_focused();
        let is_not_empty = !state.input_state.is_empty();

        if is_focused || is_not_empty {
            TextInput::new()
                .block(block)
                .render(area, buf, &mut state.input_state);
        } else {
            self.text_styling
                .placeholder("Filter locales…")
                .block(block)
                .render(area, buf);
        }
    }
}

use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::{HasScreenCursor, text_input::*};
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{components::Cursor, theme::LingoraTheme};

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
    theme: &'a LingoraTheme,
}

impl<'a> LocaleFilter<'a> {
    pub fn new(theme: &'a LingoraTheme) -> Self {
        Self { theme }
    }
}

impl StatefulWidget for LocaleFilter<'_> {
    type State = LocaleFilterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let block = self.theme.focus_block(&state.input_state.focus);

        let is_focused = state.input_state.focus.is_focused();
        let is_not_empty = !state.input_state.is_empty();

        if is_focused || is_not_empty {
            TextInput::new()
                .block(block)
                .render(area, buf, &mut state.input_state);
        } else {
            Paragraph::new("Filter locales…")
                .style(self.theme.placeholder())
                .block(block)
                .render(area, buf);
        }
    }
}

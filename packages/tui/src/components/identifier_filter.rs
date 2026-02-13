use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::{HasScreenCursor, text_input::*};
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{components::Cursor, theme::LingoraTheme};

#[derive(Debug, Default)]
pub struct IdentifierFilterState {
    input_state: TextInputState,
    area: Rect,
}

impl IdentifierFilterState {
    #[inline]
    pub fn text(&self) -> &str {
        self.input_state.text()
    }
}

impl HasFocus for IdentifierFilterState {
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

impl HasScreenCursor for IdentifierFilterState {
    fn screen_cursor(&self) -> Cursor {
        self.input_state.screen_cursor()
    }
}

impl HandleEvent<Event, Regular, Outcome> for IdentifierFilterState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        if self.input_state.focus.is_focused() {
            self.input_state.handle(event, qualifier).into()
        } else {
            Outcome::Continue
        }
    }
}

pub struct IdentifierFilter<'a> {
    theme: &'a LingoraTheme,
}

impl<'a> IdentifierFilter<'a> {
    pub fn new(theme: &'a LingoraTheme) -> Self {
        Self { theme }
    }
}

impl StatefulWidget for IdentifierFilter<'_> {
    type State = IdentifierFilterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let block = self.theme.focus_block(&state.input_state.focus);

        if state.input_state.focus.is_focused() || !state.input_state.is_empty() {
            TextInput::new()
                .block(block)
                .render(area, buf, &mut state.input_state);
        } else {
            Paragraph::new("Filter identifiers…")
                .style(self.theme.placeholder())
                .block(block)
                .render(area, buf);
        }
    }
}

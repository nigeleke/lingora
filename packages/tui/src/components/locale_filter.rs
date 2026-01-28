use crossterm::event::Event;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_text::{HasScreenCursor, text_input::*};
use ratatui::prelude::*;

use crate::ratatui::{Cursor, focus_block, placeholder_paragraph};

#[derive(Debug, Default)]
pub struct LocaleFilterState {
    input_state: TextInputState,
    area: Rect,
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

pub struct LocaleFilter;

impl StatefulWidget for LocaleFilter {
    type State = LocaleFilterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let block = focus_block(&state.input_state.focus);

        let is_focused = state.input_state.focus.is_focused();
        let is_not_empty = !state.input_state.is_empty();

        if is_focused || is_not_empty {
            TextInput::new()
                .block(block)
                .render(area, buf, &mut state.input_state);

            // if is_focused {
            //     // let cursor = state.input_state.cursor();
            //     // let scroll = state.input_state.set;
            //     // let cursor_x = area.x + 1 + cursor.saturating_sub(state.scroll()) as u16;
            //     // let cursor_y = area.y + 1;

            //     if let Some((cx, cy)) = state.input_state.screen_cursor() {
            //         //
            //     }
            //     state.input_state.set_screen_cursor(82, false);
            // }
        } else {
            placeholder_paragraph("Filter locales…")
                .block(block)
                .render(area, buf);
        }
    }
}

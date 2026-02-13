use crossterm::event::{Event, KeyCode, KeyEvent};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{List, ListState, StatefulWidget},
};

use crate::theme::LingoraTheme;

#[derive(Debug)]
pub struct LineNumberedTextViewState {
    focus_flag: FocusFlag,
    list_state: ListState,
    content: String,
    area: Rect,
}

impl LineNumberedTextViewState {
    pub fn new(content: String) -> Self {
        Self {
            focus_flag: FocusFlag::default(),
            list_state: ListState::default(),
            content,
            area: Rect::default(),
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match &event.code {
            KeyCode::Up => {
                self.list_state.select_previous();
                Outcome::Unchanged
            }
            KeyCode::Down => {
                self.list_state.select_next();
                Outcome::Unchanged
            }
            _ => Outcome::Continue,
        }
    }
}

impl HasFocus for LineNumberedTextViewState {
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

impl HandleEvent<Event, Regular, Outcome> for LineNumberedTextViewState {
    fn handle(&mut self, event: &Event, _qualifier: Regular) -> Outcome {
        match event {
            Event::Key(event) => self.handle_key_event(event),
            _ => Outcome::Continue,
        }
    }
}

pub struct LineNumberedTextView<'a> {
    theme: &'a LingoraTheme,
}

impl<'a> LineNumberedTextView<'a> {
    pub fn new(theme: &'a LingoraTheme) -> Self {
        Self { theme }
    }
}

impl StatefulWidget for LineNumberedTextView<'_> {
    type State = LineNumberedTextViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        let area = Rect::new(area.x, area.y + 1, area.width, area.height - 2);

        let rows = state
            .content
            .lines()
            .enumerate()
            .map(|(i, line)| {
                Line::from(vec![
                    Span::styled(format!("{i:>7}   "), self.theme.muted()),
                    Span::from(line),
                ])
            })
            .collect::<Vec<_>>();

        let list = List::new(rows).highlight_style(self.theme.selection());
        list.render(area, buf, &mut state.list_state);
    }
}

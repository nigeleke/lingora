use crossterm::event::{Event, KeyCode, KeyEvent};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Debug)]
pub struct LineNumberedTextViewState {
    focus_flag: FocusFlag,
    scroll_state: ScrollViewState,
    content: String,
    line_count: u16,
    line_numbers: String,
    area: Rect,
}

impl LineNumberedTextViewState {
    pub fn new(content: String) -> Self {
        let line_count = content.lines().count() as u16;
        let line_numbers = (1..=line_count)
            .map(|i| format!("{:>4} \n", i))
            .collect::<String>();

        Self {
            focus_flag: FocusFlag::default(),
            scroll_state: ScrollViewState::default(),
            content,
            line_count,
            line_numbers,
            area: Rect::default(),
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match &event.code {
            KeyCode::Up => {
                self.scroll_state.scroll_page_up();
                Outcome::Unchanged
            }
            KeyCode::Down => {
                self.scroll_state.scroll_page_down();
                Outcome::Unchanged
            }
            KeyCode::Right => {
                self.scroll_state.scroll_right();
                Outcome::Unchanged
            }
            KeyCode::Left => {
                self.scroll_state.scroll_left();
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

pub struct LineNumberedTextView;

impl StatefulWidget for LineNumberedTextView {
    type State = LineNumberedTextViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        let size = Size::new(area.width - 2, state.line_count - 2);

        let chunks =
            Layout::horizontal(vec![Constraint::Length(6), Constraint::Fill(1)]).split(area);

        let mut scroll_view = ScrollView::new(size);
        scroll_view.render_widget(
            Paragraph::new(state.line_numbers.as_str()).gray(),
            chunks[0],
        );
        scroll_view.render_widget(
            Paragraph::new(state.content.as_str()).wrap(Wrap::default()),
            chunks[1],
        );
        scroll_view.render(area, buf, &mut state.scroll_state);
    }
}

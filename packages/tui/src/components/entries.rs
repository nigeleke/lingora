use crossterm::event::{Event, KeyCode, KeyEvent};
use fluent4rs::ast::Entry;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};

use crate::ratatui::FocusStyling;

#[derive(Debug, Default)]
pub struct EntriesState {
    focus_flag: FocusFlag,
    list_state: ListState,
    area: Rect,
}

impl EntriesState {
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

impl HasFocus for EntriesState {
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

impl HandleEvent<Event, Regular, Outcome> for EntriesState {
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

pub struct Entries<'a> {
    focus_styling: &'a FocusStyling,
    entries: Vec<Entry>,
}

impl<'a> Entries<'a> {
    pub fn new(focus_styling: &'a FocusStyling, entries: impl Iterator<Item = &'a Entry>) -> Self {
        let entries = Vec::from_iter(entries.cloned());
        Self {
            focus_styling,
            entries,
        }
    }
}

impl StatefulWidget for &Entries<'_> {
    type State = EntriesState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let list = List::new(self.entries.iter().map(|e| Text::from(e.to_string())))
            .block(self.focus_styling.block(&state.focus_flag))
            .highlight_style(Style::default().bg(Color::LightBlue))
            .highlight_symbol("» ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

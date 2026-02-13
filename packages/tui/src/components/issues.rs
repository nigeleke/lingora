use crossterm::event::{Event, KeyCode, KeyEvent};
use lingora_core::prelude::AuditIssue;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};

use crate::theme::LingoraTheme;

#[derive(Default, Debug)]
pub struct IssuesState {
    focus_flag: FocusFlag,
    list_state: ListState,
    area: Rect,
}

impl IssuesState {
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

impl HasFocus for IssuesState {
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

impl HandleEvent<Event, Regular, Outcome> for IssuesState {
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

pub struct Issues<'a> {
    theme: &'a LingoraTheme,
    entries: Vec<AuditIssue>,
}

impl<'a> Issues<'a> {
    pub fn new(theme: &'a LingoraTheme, entries: Vec<AuditIssue>) -> Self {
        Self { theme, entries }
    }
}

impl StatefulWidget for &Issues<'_> {
    type State = IssuesState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        state.area = area;

        let list = List::new(self.entries.iter().map(|i| i.to_string()))
            .block(self.theme.focus_block(&state.focus_flag))
            .highlight_style(self.theme.selection())
            .highlight_symbol("» ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

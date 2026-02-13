use crossterm::event::Event;
use lingora_core::prelude::LingoraToml;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::Block};

use crate::{
    components::{LineNumberedTextView, LineNumberedTextViewState},
    theme::LingoraTheme,
};

#[derive(Debug)]
pub struct SettingsState {
    text_view_state: LineNumberedTextViewState,
    area: Rect,
}

impl SettingsState {
    pub fn new(settings: &LingoraToml) -> Self {
        let content = settings.to_string();
        let line_numbered_text_view_state = LineNumberedTextViewState::new(content);

        Self {
            text_view_state: line_numbered_text_view_state,
            area: Rect::default(),
        }
    }
}

impl HasFocus for SettingsState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.text_view_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HandleEvent<Event, Regular, Outcome> for SettingsState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.text_view_state.handle(event, qualifier)
    }
}

pub struct Settings<'a> {
    theme: &'a LingoraTheme,
}

impl<'a> Settings<'a> {
    pub fn new(theme: &'a LingoraTheme) -> Self {
        Self { theme }
    }
}

impl StatefulWidget for Settings<'_> {
    type State = SettingsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        Block::bordered()
            .title(Line::from(" Lingora.toml "))
            .render(area, buf);

        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        LineNumberedTextView::new(self.theme).render(area, buf, &mut state.text_view_state);
    }
}

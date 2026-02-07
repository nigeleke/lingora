use crossterm::event::Event;
use lingora_core::prelude::{DioxusI18nConfigRenderer, LingoraToml, Workspace};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::Block};

use crate::components::{LineNumberedTextView, LineNumberedTextViewState};

#[derive(Debug)]
pub struct DioxusI18nConfigState {
    text_view_state: LineNumberedTextViewState,
    area: Rect,
}

impl DioxusI18nConfigState {
    pub fn new(settings: &LingoraToml, workspace: &Workspace) -> Self {
        let mut cursor = std::io::Cursor::new(Vec::new());
        let renderer = DioxusI18nConfigRenderer::new(settings, workspace, None);
        let _ = renderer.render(&mut cursor);

        let content = String::from_utf8_lossy(&cursor.into_inner()).to_string();
        let text_view_state = LineNumberedTextViewState::new(content);

        Self {
            text_view_state,
            area: Rect::default(),
        }
    }
}

impl HasFocus for DioxusI18nConfigState {
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

impl HandleEvent<Event, Regular, Outcome> for DioxusI18nConfigState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.text_view_state.handle(event, qualifier)
    }
}

pub struct DioxusI18nConfig;

impl StatefulWidget for DioxusI18nConfig {
    type State = DioxusI18nConfigState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        Block::bordered()
            .title(Line::from(" dioxus-i18n: config.rs "))
            .render(area, buf);

        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        LineNumberedTextView.render(area, buf, &mut state.text_view_state);
    }
}

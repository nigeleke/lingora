use crossterm::event::Event;
use lingora_core::prelude::LingoraToml;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Debug, Default)]
pub struct SettingsState {
    focus_flag: FocusFlag,
    scroll_state: ScrollViewState,
    area: Rect,
}

impl HasFocus for SettingsState {
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

impl HandleEvent<Event, Regular, Outcome> for SettingsState {
    fn handle(&mut self, _event: &Event, _qualifier: Regular) -> Outcome {
        Outcome::Continue // TODO:
    }
}

pub struct Settings<'a> {
    settings: &'a LingoraToml,
}

impl<'a> Settings<'a> {
    pub fn new(settings: &'a LingoraToml) -> Self {
        Self { settings }
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

        let settings = self.settings.to_string();

        let line_count = settings.lines().count() as u16;

        let size = Size::new(area.width, line_count + 2);

        let line_numbers = (1..=line_count)
            .map(|i| format!("{:>4} \n", i))
            .collect::<String>();

        let chunks =
            Layout::horizontal(vec![Constraint::Length(6), Constraint::Fill(1)]).split(area);

        let mut scroll_view = ScrollView::new(size);
        scroll_view.render_widget(Paragraph::new(line_numbers).gray(), chunks[0]);
        scroll_view.render_widget(Paragraph::new(settings).wrap(Wrap::default()), chunks[1]);
        scroll_view.render(area, buf, &mut state.scroll_state);
    }
}

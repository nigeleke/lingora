use crossterm::event::Event;
use lingora_core::prelude::{DioxusI18nConfigRenderer, LingoraToml, Workspace};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Debug, Default)]
pub struct DioxusI18nConfigState {
    focus_flag: FocusFlag,
    scroll_state: ScrollViewState,
    area: Rect,
}

impl HasFocus for DioxusI18nConfigState {
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

impl HandleEvent<Event, Regular, Outcome> for DioxusI18nConfigState {
    fn handle(&mut self, _event: &Event, _qualifier: Regular) -> Outcome {
        Outcome::Continue // TODO:
    }
}

pub struct DioxusI18nConfig<'a> {
    settings: &'a LingoraToml,
    workspace: &'a Workspace,
}

impl<'a> DioxusI18nConfig<'a> {
    pub fn new(settings: &'a LingoraToml, workspace: &'a Workspace) -> Self {
        Self {
            settings,
            workspace,
        }
    }
}

impl StatefulWidget for DioxusI18nConfig<'_> {
    type State = DioxusI18nConfigState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        Block::bordered()
            .title(Line::from(" dioxus-i18n: config.rs "))
            .render(area, buf);
        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

        let mut cursor = std::io::Cursor::new(Vec::new());
        let renderer = DioxusI18nConfigRenderer::new(self.settings, self.workspace, None);
        let _ = renderer.render(&mut cursor);

        let content = String::from_utf8_lossy(&cursor.into_inner()).to_string();
        let line_count = content.lines().count() as u16;

        let size = Size::new(area.width, line_count + 2);

        let line_numbers = (1..=line_count)
            .map(|i| format!("{:>4} \n", i))
            .collect::<String>();

        let chunks =
            Layout::horizontal(vec![Constraint::Length(6), Constraint::Fill(1)]).split(area);

        let mut scroll_view = ScrollView::new(size);
        scroll_view.render_widget(Paragraph::new(line_numbers).gray(), chunks[0]);
        scroll_view.render_widget(Paragraph::new(content).wrap(Wrap::default()), chunks[1]);
        scroll_view.render(area, buf, &mut state.scroll_state);
    }
}

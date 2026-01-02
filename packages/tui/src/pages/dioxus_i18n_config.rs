use std::path::Path;

use lingora_common::DioxusI18nConfigRenderer;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{GlobalContext, state::UiState};

pub struct DioxusI18nConfig<'a> {
    context: &'a GlobalContext,
    ui_state: &'a UiState,
}

impl<'a> DioxusI18nConfig<'a> {
    pub fn new(context: &'a GlobalContext, ui_state: &'a UiState) -> Self {
        Self { context, ui_state }
    }
}

impl StatefulWidget for DioxusI18nConfig<'_> {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Block::bordered()
            .title(Line::from("dioxus-i18n: src/.../config.rs"))
            .render(area, buf);
        let area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

        let settings = self.context.settings.clone();
        let mut cursor = std::io::Cursor::new(Vec::new());
        let renderer = DioxusI18nConfigRenderer::new(settings, Some(Path::new(".")));
        let _ = renderer.render(&mut cursor);

        let content = String::from_utf8_lossy(&cursor.into_inner()).to_string();
        let line_count = content.lines().count() as u16;

        let size = Size::new(area.width, line_count + 1);

        let line_numbers = (1..=line_count)
            .map(|i| format!("{:>4} \n", i))
            .collect::<String>();

        let chunks =
            Layout::horizontal(vec![Constraint::Length(6), Constraint::Fill(1)]).split(area);

        let mut scroll_view = ScrollView::new(size);
        scroll_view.render_widget(Paragraph::new(line_numbers).light_red(), chunks[0]);
        scroll_view.render_widget(Paragraph::new(content).wrap(Wrap::default()), chunks[1]);
        scroll_view.render(area, buf, state);
    }
}

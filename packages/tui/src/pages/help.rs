use ratatui::{
    layout::{Constraint, Rect},
    prelude::*,
    widgets::{Block, Cell, Row, Table},
};

use crate::theme::LingoraTheme;

pub struct Help<'a> {
    theme: &'a LingoraTheme,
}

impl<'a> Help<'a> {
    pub fn new(theme: &'a LingoraTheme) -> Self {
        Self { theme }
    }
}

impl Widget for Help<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(Line::from(" Help "))
            .render(area, buf);

        let rows = vec![
            Row::new(vec![
                Cell::from(self.theme.accent_span("F1")),
                Cell::from(Span::from("This page")),
            ]),
            Row::new(vec![
                Cell::from(self.theme.accent_span("PgUp/PgDn")),
                Cell::from(Span::from("Page up/down")),
            ]),
            Row::new(vec![
                Cell::from(self.theme.accent_span("Tab/Shift+Tab")),
                Cell::from(Span::from("Change focus")),
            ]),
            Row::new(vec![
                Cell::from(self.theme.accent_span("<sp>")),
                Cell::from(Span::from("Set reference locale")),
            ]),
            Row::new(vec![
                Cell::from(self.theme.accent_span("↑/↓")),
                Cell::from(Span::from("Set target locale / scroll")),
            ]),
        ];

        let rows_len = rows.len() as u16;

        let table = Table::new(
            rows,
            [Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .header(
            Row::new(vec!["Key", "Description"]).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .row_highlight_style(self.theme.selection())
        .column_spacing(2);

        let area = Rect::new(
            area.x + 15,
            area.y + 5,
            std::cmp::min(50, area.width - 15),
            std::cmp::min(rows_len + 6, area.height - 5),
        );
        Widget::render(table, area, buf);
    }
}

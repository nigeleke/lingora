use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Cell, Row, Table},
};

pub struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(Line::from(" Help "))
            .render(area, buf);

        let rows = vec![
            Row::new(vec![
                Cell::from(Span::from("F1").blue()),
                Cell::from(Span::from("This page")),
            ]),
            Row::new(vec![
                Cell::from(Span::from("PgUp/PgDn").blue()),
                Cell::from(Span::from("Page up/down")),
            ]),
            Row::new(vec![
                Cell::from(Span::from("Tab/Shift+Tab").blue()),
                Cell::from(Span::from("Change focus")),
            ]),
            Row::new(vec![
                Cell::from(Span::from("<sp>").blue()),
                Cell::from(Span::from("Set reference")),
            ]),
            Row::new(vec![
                Cell::from(Span::from("↑/↓").blue()),
                Cell::from(Span::from("Select target")),
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
        .row_highlight_style(Style::default().bg(Color::LightBlue).fg(Color::White))
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

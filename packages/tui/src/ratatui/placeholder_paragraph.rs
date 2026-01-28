use ratatui::{prelude::*, widgets::Paragraph};

#[inline]
pub fn placeholder_paragraph<'a>(text: &'a str) -> Paragraph<'a> {
    Paragraph::new(Span::from(text).fg(Color::DarkGray).italic())
}

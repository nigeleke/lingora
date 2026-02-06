use ratatui::{prelude::*, widgets::Paragraph};

pub struct TextStyling {
    placeholder: Style,
}

impl Default for TextStyling {
    fn default() -> Self {
        Self {
            placeholder: Style::default().fg(Color::DarkGray).italic(),
        }
    }
}

impl TextStyling {
    #[inline]
    pub fn placeholder<'a>(&self, text: &'a str) -> Paragraph<'a> {
        Paragraph::new(Span::styled(text, self.placeholder))
    }
}

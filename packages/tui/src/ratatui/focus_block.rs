use rat_focus::{FocusFlag, HasFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub fn focus_block<'a>(focus: &FocusFlag) -> Block<'a> {
    Block::new()
        .borders(Borders::ALL)
        .border_style(if focus.is_focused() {
            Style::default().light_blue()
        } else {
            Style::default().dim()
        })
}

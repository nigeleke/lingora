use rat_focus::{FocusFlag, HasFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub struct FocusStyling {
    focused: Style,
    unfocused: Style,
}

impl Default for FocusStyling {
    fn default() -> Self {
        Self {
            focused: Style::default().light_blue(),
            unfocused: Style::default().dim(),
        }
    }
}

impl FocusStyling {
    pub fn block<'a>(&self, focus: &FocusFlag) -> Block<'a> {
        Block::new()
            .borders(Borders::ALL)
            .border_style(if focus.is_focused() {
                self.focused
            } else {
                self.unfocused
            })
    }
}

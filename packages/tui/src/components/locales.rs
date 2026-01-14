use std::collections::{HashMap, HashSet};

use lingora_core::prelude::{AuditReport, Locale};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    focus_border_type,
    state::{FocusableWidget, UiState},
};

pub struct Locales<'a> {
    report: &'a AuditReport,
    ui_state: &'a UiState,
}

impl<'a> Locales<'a> {
    pub fn new(report: &'a AuditReport, ui_state: &'a UiState) -> Self {
        Self { report, ui_state }
    }
}

impl Widget for &Locales<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ui_state = &self.ui_state;
        let canonical_locale = self.report.canonical_locale();

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);

        let b1 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleFilter));
        Paragraph::new("").block(b1).render(chunks[0], buf);

        let b2 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleTree));

        let tree = [TreeItem::new_leaf("identifier".into(), "text")];

        let mut state = TreeState::<String>::default();
        // for lang in languages.iter() {
        //     let lang_id = lang.to_string();
        //     state.open(vec![lang_id]);
        // }

        // if let Some(first_lang) = languages.first() {
        //     state.select(vec![first_lang.to_string()]);
        // }

        let tree = Tree::new(&tree)
            .expect("required unique language identifiers")
            .block(b2);

        StatefulWidget::render(tree, chunks[1], buf, &mut state);
    }
}

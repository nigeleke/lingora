use lingora_common::{Locale, ValidatedLanguage};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    GlobalContext, focus_border_type,
    state::{FocusableWidget, UiState},
};

pub struct Locales<'a> {
    context: &'a GlobalContext,
    ui_state: &'a UiState,
}

impl<'a> Locales<'a> {
    pub fn new(context: &'a GlobalContext, ui_state: &'a UiState) -> Self {
        Self { context, ui_state }
    }

    // let context = hooks.use_context_mut::<GlobalContext>();
    // let reference_locale = context
    //     .settings
    //     .reference_locale()
    //     .expect("reference_locale must be set")
    //     .to_string();

    // let mut languages = context
    //     .analysis
    //     .paths_by_locale_by_language()
    //     .keys()
    //     .collect::<Vec<_>>();
    // languages.sort();

    // let locales = languages.iter().fold(Vec::new(), |mut acc, language| {
    //     acc.push(language.to_string());
    //     let mut locales = context
    //         .analysis
    //         .paths_by_locale(&language)
    //         .keys()
    //         .collect::<Vec<_>>();
    //     locales.sort();
    //     locales
    //         .iter()
    //         .for_each(|locale| acc.push(format!("  {}", locale.to_string())));
    //     acc
    // });

    // element! {
    //     View(flex_direction: FlexDirection::Column, width: 100pct) {
    //         Input(
    //             value: "Locales filter...",
    //             focus_scope: TRANSLATIONS_FOCUS_SCOPE
    //         )
    //         View(left: 1, flex_direction: FlexDirection::Column, overflow: Overflow::Hidden) {
    //             #(locales.iter().map(|l| element!(Text(
    //                 content: l,
    //                 weight: if l.contains(&reference_locale) {Weight::Bold} else {Weight::Normal},
    //                 color: if l.contains(&reference_locale) {Color::DarkYellow} else {Color::Reset},
    //             ))))
    //         }
    //     }
    // }
}

impl Widget for &Locales<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ui_state = &self.ui_state;

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);

        let b1 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleFilter));
        Paragraph::new("filter").block(b1).render(chunks[0], buf);

        let b2 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleTree));
        Paragraph::new("tree").block(b2).render(chunks[1], buf);
    }
}

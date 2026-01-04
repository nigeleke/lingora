use std::collections::{HashMap, HashSet};

use lingora_common::ValidatedLocale;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

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
}

impl Widget for &Locales<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ui_state = &self.ui_state;
        let reference_locale = self
            .context
            .settings
            .reference_locale()
            .expect("reference_locale must be defined");

        let chunks = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]).split(area);

        let b1 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleFilter));
        Paragraph::new("").block(b1).render(chunks[0], buf);

        let b2 = Block::new()
            .borders(Borders::ALL)
            .border_type(focus_border_type!(ui_state, FocusableWidget::LocaleTree));

        let analysis = &self.context.analysis;
        let paths = analysis.paths();
        let language_locale_path_oks = paths
            .iter()
            .filter_map(|path| {
                // TODO: Filter...
                // let _stem = path.file_stem()?.to_str()?.to_lowercase();

                let ValidatedLocale::Valid(locale) =
                    ValidatedLocale::try_from(path.as_path()).ok()?
                else {
                    return None;
                };

                let language = locale.language;
                let valid = analysis.checks(path).is_empty();

                Some((language, locale, path, valid))
            })
            .collect::<Vec<_>>();

        let mut languages = Vec::from_iter(
            language_locale_path_oks
                .iter()
                .map(|llpo| llpo.0)
                .collect::<HashSet<_>>(),
        );
        languages.sort();

        let locales_by_lang = language_locale_path_oks.iter().fold(
            HashMap::new(),
            |mut acc, (lang, locale, path, ok)| {
                acc.entry(lang.clone()).or_insert_with(Vec::new).push((
                    locale.clone(),
                    **path,
                    *ok,
                ));
                acc
            },
        );

        let tree = languages
            .iter()
            .map(|l| {
                let children = locales_by_lang
                    .get(l)
                    .map(|entries| {
                        entries
                            .iter()
                            .map(|(locale, path, ok)| {
                                let style = Style::default()
                                    .fg((locale.to_string() == reference_locale.to_string())
                                        .then(|| Color::LightYellow)
                                        .unwrap_or_default())
                                    .bg(ok.then(|| Color::default()).unwrap_or(Color::LightRed));
                                TreeItem::new_leaf(
                                    path.display().to_string(),
                                    Span::from(locale.to_string()).style(style),
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                TreeItem::new(l.to_string(), l.to_string(), children)
                    .expect("required unique language identifiers")
            })
            .collect::<Vec<_>>();

        let mut state = TreeState::<String>::default();
        for lang in languages.iter() {
            let lang_id = lang.to_string();
            state.open(vec![lang_id]);
        }

        if let Some(first_lang) = languages.first() {
            state.select(vec![first_lang.to_string()]);
        }

        let tree = Tree::new(&tree)
            .expect("required unique language identifiers")
            .block(b2);

        StatefulWidget::render(tree, chunks[1], buf, &mut state);
    }
}

use std::{ffi::OsStr, path::PathBuf};

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use lingora_core::prelude::{LanguageRoot, Locale, QualifiedFluentFile, Workspace};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, text::*, widgets::*};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::ratatui::{focus_block, locale_span};

#[derive(Debug, Default)]
pub struct LocaleTreeState {
    pub focus_flag: FocusFlag,
    pub tree_state: TreeState<String>,
    pub area: Rect,
}

impl LocaleTreeState {
    pub fn new() -> Self {
        let focus_flag = FocusFlag::default();
        let tree_state = TreeState::<String>::default();
        let area = Rect::default();

        Self {
            focus_flag,
            tree_state,
            area,
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match &event.code {
            KeyCode::Up => {
                self.tree_state.key_up();
                Outcome::Unchanged
            }
            KeyCode::Down => {
                self.tree_state.key_down();
                Outcome::Unchanged
            }
            KeyCode::Right => {
                self.tree_state.key_right();
                Outcome::Unchanged
            }
            KeyCode::Left => {
                self.tree_state.key_left();
                Outcome::Unchanged
            }
            KeyCode::Char(' ') => {
                self.tree_state.toggle_selected();
                Outcome::Unchanged
            }
            _ => Outcome::Continue,
        }
    }

    fn handle_mouse_event(&mut self, _event: &MouseEvent) -> Outcome {
        Outcome::Continue
    }
}

impl HasFocus for LocaleTreeState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.focus_flag.clone()
    }

    fn area(&self) -> Rect {
        self.area
    }
}

impl HandleEvent<Event, Regular, Outcome> for LocaleTreeState {
    fn handle(&mut self, event: &Event, _qualifier: Regular) -> Outcome {
        if self.focus_flag.is_focused() {
            match event {
                Event::Key(event) => self.handle_key_event(event),
                Event::Mouse(event) => self.handle_mouse_event(event),
                _ => Outcome::Continue,
            }
        } else {
            Outcome::Continue
        }
    }
}

pub struct LocaleTree<'a> {
    workspace: &'a Workspace,
}

impl<'a> LocaleTree<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }
}

impl StatefulWidget for LocaleTree<'_> {
    type State = LocaleTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        let workspace = self.workspace;

        let common_prefix_len = |paths: &[QualifiedFluentFile]| {
            if paths.is_empty() {
                return 0;
            }

            let mut iters = paths
                .iter()
                .map(|p| p.path().components())
                .collect::<Vec<_>>();

            let mut len = 0;

            while let Some(first) = iters[0].next() {
                if iters.iter_mut().skip(1).all(|it| it.next() == Some(first)) {
                    len += 1;
                } else {
                    break;
                }
            }

            len
        };

        let strip_common_prefix = |paths: &[QualifiedFluentFile]| {
            let prefix_len = common_prefix_len(paths);

            paths
                .into_iter()
                .map(|p| p.path().components().skip(prefix_len).collect::<PathBuf>())
                .collect::<Vec<_>>()
        };

        let path_items = |files: &[QualifiedFluentFile], locale_idx: usize| {
            strip_common_prefix(files)
                .iter()
                .enumerate()
                .map(|(k, p)| {
                    let key = format!("path-{locale_idx}-{k}");
                    let prefix = p
                        .is_relative()
                        .then_some(format!("...{}", std::path::MAIN_SEPARATOR_STR))
                        .unwrap_or_default();
                    let text = format!("{prefix}{}", p.display());
                    TreeItem::new_leaf(key, text)
                })
                .collect::<Vec<_>>()
        };

        let locale_item = |locale_idx: usize, locale: &Locale, files: &[QualifiedFluentFile]| {
            let key = format!("locale-{locale_idx}-{}", files.len());

            let text = locale_span(locale, workspace);

            if files.len() > 1 {
                let paths = path_items(files, locale_idx);
                TreeItem::new(key, text, paths).ok()
            } else {
                Some(TreeItem::new_leaf(key, text))
            }
        };

        let language_item = |lang_idx: usize, root: LanguageRoot| {
            let key = format!("lang-{lang_idx}");
            let text = root.to_string();

            let locale_items = workspace
                .locales_by_language_root(&root)
                .enumerate()
                .filter_map(|(j, locale)| {
                    let files = workspace
                        .fluent_files_by_locale(locale)
                        .cloned()
                        .collect::<Vec<_>>();

                    locale_item(j, locale, &files)
                })
                .collect::<Vec<_>>();

            TreeItem::new(key, text, locale_items).ok()
        };

        let items = workspace
            .language_roots()
            .enumerate()
            .filter_map(|(i, root)| language_item(i, root))
            .collect::<Vec<_>>();

        let tree = Tree::new(&items)
            .expect("unique locale ids in tree")
            .block(focus_block(&state.focus_flag))
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            );

        StatefulWidget::render(tree, area, buf, &mut state.tree_state);
    }
}

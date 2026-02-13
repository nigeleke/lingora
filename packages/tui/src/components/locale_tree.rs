use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent};
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    projections::{
        FilteredLocalesHierarchy, HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind,
    },
    theme::LingoraTheme,
};

#[derive(Debug)]
pub struct LocaleTreeState {
    focus_flag: FocusFlag,
    tree_state: TreeState<LocaleNodeId>,
    reference: Option<LocaleNodeId>,
    target: Option<LocaleNodeId>,
    area: Rect,
}

impl LocaleTreeState {
    pub fn new(
        reference_node_id: Option<LocaleNodeId>,
        node_ids: impl IntoIterator<Item = LocaleNodeId>,
    ) -> Self {
        let focus_flag = FocusFlag::default();

        let mut tree_state = TreeState::default();
        node_ids.into_iter().for_each(|id| {
            tree_state.open(vec![id]);
        });

        let reference = reference_node_id;
        let target = None;

        let area = Rect::default();

        Self {
            focus_flag,
            tree_state,
            reference,
            target,
            area,
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match &event.code {
            KeyCode::Up => {
                self.tree_state.key_up();
                self.target = self.tree_state.selected().last().copied();
                Outcome::Unchanged
            }
            KeyCode::Down => {
                self.tree_state.key_down();
                self.target = self.tree_state.selected().last().copied();
                Outcome::Unchanged
            }
            KeyCode::Right => {
                self.tree_state.key_right();
                self.target = self.tree_state.selected().last().copied();
                Outcome::Unchanged
            }
            KeyCode::Left => {
                self.tree_state.key_left();
                self.target = self.tree_state.selected().last().copied();
                Outcome::Unchanged
            }
            KeyCode::Char(' ') => {
                self.tree_state.toggle_selected();
                self.reference = self.tree_state.selected().last().copied();
                Outcome::Unchanged
            }
            _ => Outcome::Continue,
        }
    }
}

impl HasSelectionPair for LocaleTreeState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<&Self::Item> {
        self.reference.as_ref()
    }

    fn target(&self) -> Option<&Self::Item> {
        self.target.as_ref()
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
                _ => Outcome::Continue,
            }
        } else {
            Outcome::Continue
        }
    }
}

pub struct LocaleTree<'a> {
    theme: &'a LingoraTheme,
    filtered_hierarchy: &'a FilteredLocalesHierarchy,
}

impl<'a> LocaleTree<'a> {
    pub fn new(theme: &'a LingoraTheme, filtered_hierarchy: &'a FilteredLocalesHierarchy) -> Self {
        Self {
            theme,
            filtered_hierarchy,
        }
    }

    fn to_tree_item(
        &self,
        id: &LocaleNodeId,
        nodes: &HashMap<LocaleNodeId, LocaleNode>,
    ) -> Option<TreeItem<'_, LocaleNodeId>> {
        let styled = |node: &LocaleNode| {
            let styled = match node.kind() {
                LocaleNodeKind::WorkspaceRoot => Span::from("workspace"),
                LocaleNodeKind::LanguageRoot { language } => {
                    self.theme.language_root_span(language)
                }
                LocaleNodeKind::Locale { locale } => self.theme.locale_span(locale),
            };

            if node.has_issues() {
                styled.patch_style(self.theme.error())
            } else {
                styled
            }
        };

        if let Some(node) = nodes.get(id) {
            if node.has_children() {
                let children = node
                    .children()
                    .filter_map(|id| self.to_tree_item(id, nodes))
                    .collect();
                TreeItem::new(*id, styled(node), children).ok()
            } else {
                Some(TreeItem::new_leaf(*id, styled(node)))
            }
        } else {
            None
        }
    }
}

impl StatefulWidget for LocaleTree<'_> {
    type State = LocaleTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        let roots = self
            .filtered_hierarchy
            .roots()
            .filter_map(|id| self.to_tree_item(id, self.filtered_hierarchy.nodes()))
            .collect::<Vec<_>>();

        let tree = Tree::new(&roots)
            .expect("unique locale ids in tree")
            .block(self.theme.focus_block(&state.focus_flag))
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(self.theme.selection());

        StatefulWidget::render(tree, area, buf, &mut state.tree_state);
    }
}

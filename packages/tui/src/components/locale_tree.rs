use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use lingora_core::prelude::Locale;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    projections::{Context, HasSelectionPair, LocaleNode, LocaleNodeId, LocaleNodeKind},
    ratatui::{focus_block, language_root_span, locale_span},
};

#[derive(Debug, Default)]
pub struct LocaleTreeState {
    initialized: bool,
    focus_flag: FocusFlag,
    tree_state: TreeState<LocaleNodeId>,
    reference: Option<LocaleNodeId>,
    target: Option<LocaleNodeId>,
    area: Rect,
}

impl LocaleTreeState {
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
                Outcome::Changed
            }
            _ => Outcome::Continue,
        }
    }

    fn handle_mouse_event(&mut self, _event: &MouseEvent) -> Outcome {
        Outcome::Continue
    }
}

impl HasSelectionPair for LocaleTreeState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<Self::Item> {
        self.reference
    }

    fn target(&self) -> Option<Self::Item> {
        self.target
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

pub struct LocaleTree {
    context: Context,
}

impl LocaleTree {
    fn initialize_state(&self, context: &Context, state: &mut LocaleTreeState) {
        if !state.initialized {
            self.context.locale_node_ids().for_each(|id| {
                state.tree_state.open(vec![*id]);
            });

            state.reference = context
                .node_id_for_locale(context.canonical_locale())
                .cloned();

            state.initialized = true;
        }
    }

    fn to_tree_item(&self, id: &LocaleNodeId) -> Option<TreeItem<'_, LocaleNodeId>> {
        let styled = |node: &LocaleNode| {
            let styled = match node.kind() {
                LocaleNodeKind::WorkspaceRoot => Span::from("workspace"),
                LocaleNodeKind::LanguageRoot { language } => {
                    language_root_span(language, &self.context)
                }
                LocaleNodeKind::Locale { locale } => locale_span(locale, &self.context),
            };

            if node.has_issues() {
                styled.light_red()
            } else {
                styled
            }
        };

        if let Some(node) = self.context.locale_node(id) {
            if node.has_children() {
                let children = node
                    .children()
                    .filter_map(|id| self.to_tree_item(id))
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

impl From<Context> for LocaleTree {
    fn from(context: Context) -> Self {
        Self { context }
    }
}

impl StatefulWidget for LocaleTree {
    type State = LocaleTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.initialize_state(&self.context, state);

        state.area = area;

        let roots = self
            .context
            .root_node_ids()
            .filter_map(|id| self.to_tree_item(id))
            .collect::<Vec<_>>();

        let tree = Tree::new(&roots)
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

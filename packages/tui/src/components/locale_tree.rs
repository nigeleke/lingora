use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use lingora_core::prelude::AuditResult;
use rat_event::{HandleEvent, Outcome, Regular};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use ratatui::{prelude::*, widgets::*};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    projections::{NodeId, TranslationsTree, TreeNode},
    ratatui::{focus_block, locale_span_str},
};

#[derive(Debug, Default)]
pub struct LocaleTreeState {
    focus_flag: FocusFlag,
    tree_state: TreeState<NodeId>,
    area: Rect,
}

impl LocaleTreeState {
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

pub struct LocaleTree {
    model: TranslationsTree,
    audit_result: Rc<AuditResult>,
}

impl LocaleTree {
    pub fn new(audit_result: Rc<AuditResult>) -> Self {
        let model = TranslationsTree::from(&*audit_result);
        Self {
            model,
            audit_result,
        }
    }

    fn to_tree_item(&self, id: &NodeId) -> Option<TreeItem<'_, NodeId>> {
        let styled = |node: &TreeNode| {
            let text = node.description();
            let styled = locale_span_str(&text, self.audit_result.workspace());
            if node.has_issues() {
                styled.light_red()
            } else {
                styled
            }
        };

        if let Some(node) = self.model.node(id) {
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

impl StatefulWidget for LocaleTree {
    type State = LocaleTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;

        let roots = self
            .model
            .roots()
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

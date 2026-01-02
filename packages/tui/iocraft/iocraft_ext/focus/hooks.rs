use iocraft::prelude::*;

use crate::{FocusScope, FocusState};

pub fn use_focus(hooks: &mut Hooks, scope: FocusScope) -> State<bool> {
    let mut state = hooks.use_context_mut::<FocusState>();

    let id = state.register(scope);
    let is_focused = state.is_focused(id);

    let is_focused_state = hooks.use_state(|| is_focused);

    {
        let is_focused_state = is_focused_state.clone();

        hooks.use_effect(
            {
                let state = state.clone();
                let mut is_focused_state = is_focused_state.clone();
                move || {
                    let focused = state.is_focused(id);
                    is_focused_state.set(focused);
                }
            },
            {},
        );
    }

    is_focused_state
}

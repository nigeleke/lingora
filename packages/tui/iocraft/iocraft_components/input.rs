use iocraft::prelude::*;

use crate::{FocusScope, FocusState, use_focus};

#[derive(Default, Props)]
pub struct InputProps {
    /// The color to make the text.
    pub color: Option<Color>,

    /// The current value.
    pub value: String,

    /// The handler to invoke when the value changes.
    pub on_change: HandlerMut<'static, String>,

    /// If true, the input will fill 100% of the height of its container and handle multiline input.
    pub multiline: bool,

    /// The color to make the cursor. Defaults to gray.
    pub cursor_color: Option<Color>,

    /// An optional handle which can be used for imperative control of the input.
    pub handle: Option<Ref<TextInputHandle>>,

    /// The focus scope for the control.
    pub focus_scope: FocusScope,
}

#[component]
pub fn Input(mut hooks: Hooks, props: &mut InputProps) -> impl Into<AnyElement<'static>> {
    let focus_state = hooks.use_context_mut::<FocusState>().clone();
    let mut on_change_handler = std::mem::take(&mut props.on_change);

    let has_focus = use_focus(&mut hooks, props.focus_scope);

    let mut on_tab = {
        let mut focus_state = focus_state.clone();
        move || {
            focus_state.focus_next();
        }
    };

    let mut on_back_tab = {
        let mut focus_state = focus_state.clone();
        move || {
            focus_state.focus_previous();
        }
    };

    hooks.use_terminal_events({
        move |event| {
            if has_focus.get() {
                match event {
                    TerminalEvent::Key(KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::Tab,
                        ..
                    }) => on_tab(),
                    TerminalEvent::Key(KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::BackTab,
                        ..
                    }) => on_back_tab(),
                    _ => {}
                }
            }
        }
    });

    let color = props.color.clone();
    let value = props.value.clone();
    let has_focus = has_focus.get();
    let on_change = move |s: String| on_change_handler(s);
    let multiline = props.multiline.clone();
    let cursor_color = props.cursor_color.clone();
    let handle = props.handle.clone();

    element! {
        View(
            border_style: if has_focus { BorderStyle::Bold } else { BorderStyle::Single },
            border_color: if has_focus { Color::White } else { Color::DarkGrey }
        ) {
            TextInput(color, value, has_focus, on_change, multiline, cursor_color, handle)
        }
    }
}

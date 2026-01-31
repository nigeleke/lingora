use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use rat_event::{ConsumedEvent, HandleEvent, Outcome, Regular};
use rat_focus::{Focus, FocusBuilder, FocusFlag, HasFocus};
use rat_text::HasScreenCursor;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    components::{Identifiers, IdentifiersState, Locales, LocalesState},
    projections::{Context, HasSelectionPair, LocaleNodeId},
    ratatui::Cursor,
};

#[derive(Debug, Default)]
pub struct TranslationsState {
    focus: Option<Focus>,
    locales_state: LocalesState,
    identifiers_state: IdentifiersState,
}

impl TranslationsState {
    pub fn rebuild_focus(&mut self) {
        let mut builder = FocusBuilder::new(self.focus.take());
        self.build(&mut builder);
        self.focus = Some(builder.build());
    }

    fn handle_key_event(&mut self, event: &KeyEvent) -> Outcome {
        match event.code {
            KeyCode::Tab => self.focus_next(),
            KeyCode::BackTab => self.focus_prev(),
            _ => Outcome::Continue,
        }
    }

    #[inline]
    fn focus_next(&mut self) -> Outcome {
        let focus = self.focus.as_mut().expect("focus");
        focus.next();
        Outcome::Unchanged
    }

    #[inline]
    fn focus_prev(&mut self) -> Outcome {
        let focus = self.focus.as_mut().expect("focus");
        focus.prev();
        Outcome::Unchanged
    }

    #[inline]
    fn handle_mouse_event(&mut self, _event: &MouseEvent) -> Outcome {
        Outcome::Continue
    }

    #[inline]
    pub fn locale_filter(&self) -> &str {
        self.locales_state.filter()
    }

    #[inline]
    pub fn identifier_filter(&self) -> &str {
        self.identifiers_state.filter()
    }
}

impl HasSelectionPair for TranslationsState {
    type Item = LocaleNodeId;

    fn reference(&self) -> Option<Self::Item> {
        self.locales_state.reference()
    }

    fn target(&self) -> Option<Self::Item> {
        self.locales_state.target()
    }
}

impl HasFocus for TranslationsState {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.widget(&self.locales_state);
        builder.widget(&self.identifiers_state);
    }

    fn focus(&self) -> FocusFlag {
        unreachable!()
    }

    fn area(&self) -> Rect {
        unreachable!()
    }
}

impl HasScreenCursor for TranslationsState {
    fn screen_cursor(&self) -> Cursor {
        self.locales_state
            .screen_cursor()
            .or_else(|| self.identifiers_state.screen_cursor())
    }
}

impl HandleEvent<Event, Regular, Outcome> for TranslationsState {
    fn handle(&mut self, event: &Event, qualifier: Regular) -> Outcome {
        self.rebuild_focus();

        match event {
            Event::Key(event) => self.handle_key_event(event),
            Event::Mouse(event) => self.handle_mouse_event(event),
            _ => Outcome::Continue,
        }
        .or_else(|| self.locales_state.handle(event, qualifier))
        .or_else(|| self.identifiers_state.handle(event, qualifier))
    }
}

pub struct Translations {
    context: Context,
}

impl From<Context> for Translations {
    fn from(context: Context) -> Self {
        Self { context }
    }
}

impl StatefulWidget for &Translations {
    type State = TranslationsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let chunks = Layout::horizontal(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(30),
            Constraint::Min(0),
        ])
        .split(area);

        Locales::from(self.context.clone()).render(chunks[0], buf, &mut state.locales_state);
        Identifiers::from(self.context.clone()).render(
            chunks[1],
            buf,
            &mut state.identifiers_state,
        );
        Paragraph::new("Entries").render(chunks[2], buf);
    }
}

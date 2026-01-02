use strum::VariantArray;

#[derive(Debug, Default)]
pub enum RunState {
    #[default]
    Running,
    Quit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, VariantArray)]
pub enum Page {
    #[default]
    Translations,
    DioxusI18nConfig,
    Settings,
}

impl Page {
    pub fn next(&self) -> Self {
        let index = Page::VARIANTS.iter().position(|x| x == self).unwrap();
        let index = (index + 1) % Page::VARIANTS.len();
        Page::VARIANTS[index]
    }

    pub fn previous(&self) -> Self {
        let index = Page::VARIANTS.iter().position(|x| x == self).unwrap();
        let index = (index + Page::VARIANTS.len() - 1) % Page::VARIANTS.len();
        Page::VARIANTS[index]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, VariantArray)]
pub enum FocusableWidget {
    #[default]
    LocaleFilter,
    LocaleTree,
    IdentifierFilter,
    IdentifierList,
}

impl FocusableWidget {
    pub fn next(&self) -> Self {
        let index = FocusableWidget::VARIANTS
            .iter()
            .position(|x| x == self)
            .unwrap();
        let index = (index + 1) % FocusableWidget::VARIANTS.len();
        FocusableWidget::VARIANTS[index]
    }

    pub fn previous(&self) -> Self {
        let index = FocusableWidget::VARIANTS
            .iter()
            .position(|x| x == self)
            .unwrap();
        let index = (index + FocusableWidget::VARIANTS.len() - 1) % FocusableWidget::VARIANTS.len();
        FocusableWidget::VARIANTS[index]
    }
}

#[macro_export]
macro_rules! focus_border_type {
    ($ui_state:expr, $widget:expr) => {
        if $ui_state.focused_widget == $widget {
            BorderType::Thick
        } else {
            BorderType::Plain
        }
    };
}

#[derive(Debug, Default)]
pub struct UiState {
    pub run_state: RunState,
    pub page: Page,
    pub focused_widget: FocusableWidget,
}

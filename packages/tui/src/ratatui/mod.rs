mod focus_styling;
mod locale_styling;
mod styling;
mod text_styling;

pub use focus_styling::FocusStyling;
pub use locale_styling::LocaleStyling;
pub use styling::Styling;
pub use text_styling::TextStyling;

pub type Cursor = Option<(u16, u16)>;

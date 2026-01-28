mod focus_block;
mod language_root_span;
mod locale_span;
mod placeholder_paragraph;

pub use focus_block::focus_block;
pub use language_root_span::language_root_span;
pub use locale_span::locale_span;
pub use placeholder_paragraph::placeholder_paragraph;

pub type Cursor = Option<(u16, u16)>;

//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("config/default_lingora.toml")]
//! ```

mod config;
mod domain;
mod error;
mod renderers;
mod utils;

pub use config::{AnalysisArgs, Settings, WithLocale};
pub use domain::{
    Analysis, IntegrityChecks, Locale, PathsByLocale, PathsByLocaleByLanguage, ValidatedLanguage,
    ValidatedLocale,
};
pub use error::LingoraError;
pub use renderers::{AnalysisRenderer, DioxusI18nConfigRenderer};

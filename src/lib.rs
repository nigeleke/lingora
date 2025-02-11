#![doc = include_str!("../README.md")]
//!
//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("config/default_lingora.toml")]
//! ```

// mod components;
mod app;
mod config;
mod domain;
mod output;

pub use app::{App, AppError};
pub use config::{Arguments, OutputMode};

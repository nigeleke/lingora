#![doc = include_str!("../README.md")]
//!
//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("config/default_lingora.toml")]
//! ```

mod app;
mod config;
mod domain;
mod gui;
mod output;
mod utils;

pub use app::{App, AppError};
pub use config::{Arguments, OutputMode};

#![doc = include_str!("../README.md")]
//!
//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("core/config/default_lingora.toml")]
//! ```

// mod components;
mod core;

pub use core::{App, AppError, OutputMode, Arguments};

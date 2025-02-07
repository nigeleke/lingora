#![doc = include_str!("../README.md")]
//!
//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("../examples/Lingora.toml")]
//! ```

// mod components;
mod core;

pub use core::{App, AppError, CommandLineArgs, OutputMode, ResolvedArgs, ResolvedArgsBuilder};

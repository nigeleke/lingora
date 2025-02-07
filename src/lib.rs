#![doc = include_str!("../README.md")]
//!
//! ## Lingora.toml
//!
//! ```toml
#![doc = include_str!("../examples/Lingora.toml")]
//! ```
//!
//! ## Flowcharts
//!
#![doc = include_str!("../docs/reference_file.md")]

// mod components;
mod core;

pub use core::{App, AppError, CommandLineArgs, OutputMode, ResolvedArgs, ResolvedArgsBuilder};

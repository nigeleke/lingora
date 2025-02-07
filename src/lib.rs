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
#![doc = mermaid!("../docs/reference_file.mmd")]

use simple_mermaid::*;

// mod components;
mod core;

pub use core::{App, AppError, CommandLineArgs, OutputMode, ResolvedArgs, ResolvedArgsBuilder};

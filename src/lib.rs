#![doc = include_str!("../README.md")]

// mod components;
mod core;

pub use core::{App, AppError, CommandLineArgs, OutputMode, ResolvedArgs, ResolvedArgsBuilder};

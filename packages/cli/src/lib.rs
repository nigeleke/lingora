#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod app;
mod args;
mod error;

pub use app::App;
pub use args::{CliArgs, OutputMode};
pub use error::CliError;

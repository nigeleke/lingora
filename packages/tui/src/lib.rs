#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod app;
mod args;
mod components;
mod error;
mod pages;
mod projections;
mod theme;

pub use app::App;
pub use args::TuiArgs;
pub use error::TuiError;

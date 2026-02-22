use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use lingora_core::prelude::CoreArgs;

/// Controls the level of output produced by `lingora-cli`.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputMode {
    /// Enables lingora to complete the error checks on the reference and target files, returning an
    /// error status if found. No detailed report will be produced.
    Silent,
    /// Output analysis report details to stdout.
    Standard,
}

/// Command-line arguments specific to the `lingora-cli` binary.
///
/// Extends the shared `CoreArgs` (from `lingora-core`) with CLI-only options:
/// - Output verbosity/behavior
/// - Optional generation of `dioxus_i18n::I18nConfig` Rust code
#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct CliArgs {
    #[command(flatten)]
    core_args: CoreArgs,

    #[arg(short, long = "output", value_enum, default_value_t = OutputMode::Standard)]
    output_mode: OutputMode,

    /// If provided, then the given file will be created, and will contain the
    /// function `pub fn config(initial_language) -> I18nConfig { ... }'.
    ///
    /// See <https://docs.rs/dioxus-i18n/latest/dioxus_i18n/>.
    #[arg(long)]
    dioxus_i18n_config_file: Option<PathBuf>,
}

impl CliArgs {
    /// Returns a reference to the shared core arguments.
    pub fn core_args(&self) -> &CoreArgs {
        &self.core_args
    }

    /// Returns the path where the generated Dioxus i18n config should be written.
    pub fn dioxus_i18n_config_file(&self) -> Option<&Path> {
        self.dioxus_i18n_config_file.as_deref()
    }

    /// Returns the selected output mode.
    pub fn output_mode(&self) -> &OutputMode {
        &self.output_mode
    }
}

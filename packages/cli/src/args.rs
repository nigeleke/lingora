use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use lingora_core::prelude::CoreArgs;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputMode {
    /// Enables lingora to complete the error checks on the reference and target files, returning an
    /// error status if found. No detailed report will be produced.
    Silent,
    /// Output analysis report details to stdout.
    Standard,
}

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
    pub fn core_args(&self) -> &CoreArgs {
        &self.core_args
    }

    pub fn dioxus_i18n_config_file(&self) -> Option<&Path> {
        self.dioxus_i18n_config_file.as_deref()
    }

    pub fn output_mode(&self) -> &OutputMode {
        &self.output_mode
    }
}

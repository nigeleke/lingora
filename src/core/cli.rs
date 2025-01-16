use clap::*;

use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(long)]
    config: Option<PathBuf>,
}

impl Cli {
    pub fn config(&self) -> Option<&PathBuf> {
        self.config.as_ref()
    }
}

#[cfg(test)]
impl From<&str> for Cli {
    fn from(value: &str) -> Self {
        let value = value.split_whitespace();
        Cli::try_parse_from(value).unwrap()
    }
}

use clap::Parser;
use lingora_core::prelude::CoreArgs;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct TuiArgs {
    #[command(flatten)]
    core_args: CoreArgs,
}

impl TuiArgs {
    pub fn core_args(&self) -> &CoreArgs {
        &self.core_args
    }
}

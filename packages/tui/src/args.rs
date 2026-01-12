use clap::Parser;
use lingora_common::AnalysisArgs;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct TuiArgs {
    #[command(flatten)]
    analysis_args: AnalysisArgs,
}

impl TuiArgs {
    pub fn analysis_args(&self) -> &AnalysisArgs {
        &self.analysis_args
    }
}

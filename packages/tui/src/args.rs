use clap::Parser;
use lingora_common::AnalysisArgs;

#[derive(Debug, Parser)]
pub struct TuiArgs {
    #[command(flatten)]
    analysis_args: AnalysisArgs,
}

impl TuiArgs {
    pub fn analysis_args(&self) -> &AnalysisArgs {
        &self.analysis_args
    }
}

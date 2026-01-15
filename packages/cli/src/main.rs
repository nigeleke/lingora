#![feature(coverage_attribute)]

use clap::Parser;
use lingora_cli::{App, CliArgs, CliError, OutputMode};

#[coverage(off)]
fn main() -> Result<(), CliError> {
    let args = CliArgs::parse();

    let app = App::try_from(&args)?;

    match args.output_mode() {
        OutputMode::Silent => app.exit_status(),

        OutputMode::Standard => {
            if let Some(path) = args.dioxus_i18n_config_file() {
                app.output_dioxus_i18n_config(path)?;
            }
            app.output_audit_report(&mut std::io::stdout())?;
            app.exit_status()
        }
    }
}

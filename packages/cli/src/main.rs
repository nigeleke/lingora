#![feature(coverage_attribute)]

use std::process::ExitCode;

use clap::Parser;
use lingora_cli::{App, CliArgs, CliError, OutputMode};

#[coverage(off)]
fn run() -> Result<(), CliError> {
    let args = CliArgs::parse();

    let app = App::try_from(&args)?;

    if let Some(path) = args.dioxus_i18n_config_file() {
        app.output_dioxus_i18n_config(path)?;
    }

    match args.output_mode() {
        OutputMode::Silent => app.exit_status(),

        OutputMode::Standard => {
            app.output_audit_report(&mut std::io::stdout())?;
            app.exit_status()
        }
    }
}

#[coverage(off)]
fn main() {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    };
}

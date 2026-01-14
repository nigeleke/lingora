#![feature(coverage_attribute)]

use clap::Parser;
use lingora_tui::{App, TuiArgs, TuiError};

#[coverage(off)]
fn main() -> Result<(), TuiError> {
    let args = TuiArgs::parse();

    let mut app = App::try_from(&args)?;
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}

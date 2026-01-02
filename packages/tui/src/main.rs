#![feature(coverage_attribute)]

use clap::Parser;
use lingora_common::{Analysis, IntegrityChecks, Locale, Settings};
use lingora_tui::{App, GlobalContext, TuiArgs, TuiError};

#[coverage(off)]
fn main() -> Result<(), TuiError> {
    let args = TuiArgs::parse();
    let settings = Settings::try_from_args(Locale::default(), args.analysis_args())?;
    let checks = IntegrityChecks::try_from(&settings)?;
    let analysis = Analysis::from(checks);
    let context = GlobalContext { settings, analysis };

    let mut app = App::new(context);
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}

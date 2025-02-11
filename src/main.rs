#![feature(coverage_attribute)]

use lingora::{App, AppError, Arguments, OutputMode};

use clap::Parser;

use std::cell::RefCell;
use std::rc::Rc;

#[coverage(off)]
fn main() -> Result<(), AppError> {
    let arguments = Arguments::parse();

    let app = App::try_from(&arguments)?;

    match arguments.output_mode() {
        OutputMode::Silent => app.exit_status(),

        OutputMode::Standard => {
            if let Some(path) = arguments.dioxus_i18n() {
                app.output_dioxus_i18n(path)?;
            }
            app.output_analysis(Rc::new(RefCell::new(std::io::stdout())))?;
            app.exit_status()
        }

        OutputMode::Gui => {
            app.show_gui();
            Ok(())
        }
    }
}

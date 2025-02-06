use lingora::{App, AppError, CommandLineArgs, OutputMode};

use clap::Parser;

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), AppError> {
    let args = CommandLineArgs::parse();

    let app = App::try_new(&args)?;

    match args.output_mode() {
        OutputMode::Silent => app.exit_status(),

        OutputMode::Standard => {
            if args.dioxus_i18n() {
                app.output_dioxus_i18n()?;
            }
            app.output_analysis(Rc::new(RefCell::new(std::io::stderr())))?;
            app.exit_status()
        }

        OutputMode::Gui => {
            app.show_gui();
            Ok(())
        }
    }
}

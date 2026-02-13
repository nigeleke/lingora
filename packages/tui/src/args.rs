use clap::Parser;
use lingora_core::prelude::CoreArgs;
use ratatui_themes::ThemeName;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct TuiArgs {
    #[command(flatten)]
    core_args: CoreArgs,

    /// UI Theme to be used - see https://github.com/ricardodantas/ratatui-themes?tab=readme-ov-file#-available-themes
    #[arg(long = "theme", value_enum, default_value = "Cyberpunk")]
    theme: ThemeName,
}

impl TuiArgs {
    pub fn core_args(&self) -> &CoreArgs {
        &self.core_args
    }

    pub fn theme(&self) -> ThemeName {
        self.theme
    }
}

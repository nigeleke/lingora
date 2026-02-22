use clap::Parser;
use lingora_core::prelude::CoreArgs;
use ratatui_themes::ThemeName;

/// Command-line arguments specific to the `lingora-tui` interactive terminal interface.
///
/// Extends the shared `CoreArgs` (from `lingora-core`) with TUI-specific options:
/// - UI theme selection (using the `ratatui-themes` crate)
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
    /// Core arguments shared across Lingora tools (config file, sources, locales, etc.).
    pub fn core_args(&self) -> &CoreArgs {
        &self.core_args
    }

    /// Visual theme for the terminal user interface.
    ///
    /// Selects one of the predefined themes from the `ratatui-themes` crate.
    ///
    /// Default: `Cyberpunk` (high-contrast, neon/cyber aesthetic)
    ///
    /// Preview all themes: https://github.com/ricardodantas/ratatui-themes?tab=readme-ov-file#-available-themes
    pub fn theme(&self) -> ThemeName {
        self.theme
    }
}

use clap::*;

use std::path::PathBuf;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputMode {
    /// Enables lingora to complete the error checks on the reference and target files, returning an
    /// error status if found. No detailed report will be produced.
    Silent,
    /// Output analysis report details to stderr, and [I18nConfig] (if --dioxus-i18n selected) to stdout.
    Standard,
    /// Open graphical application.
    Gui,
}

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct CommandLineArgs {
    /// Reference language or locale file.
    /// If not provided, and if any target is a folder and a file with the current
    /// default system locale is found, then that will be used as the reference.
    #[clap(short, long, default_value = None)]
    reference: Option<PathBuf>,

    /// Target language or locale files.
    /// One or more target language files to be compared against the reference language.
    /// If any target is a folder then all `ftl` files in the folder will be
    /// deemed a target (other than the reference).
    /// If the target is not provided then "./i18n/" folder will be used.
    #[arg(short, long)]
    target: Vec<PathBuf>,

    /// Select the output mode for the application.
    #[arg(short, long = "output", value_enum, default_value_t = OutputMode::Standard)]
    output_mode: OutputMode,

    /// If selected, then the output will be [I18nConfig::new](https://docs.rs/dioxus-i18n/0.4.1/dioxus_i18n/use_i18n/struct.I18nConfig.html).
    /// Either the analysis report will be output, or the [I18nConfig], but not both.
    #[arg(long)]
    dioxus_i18n: bool,
}

impl CommandLineArgs {
    pub fn reference(&self) -> Option<PathBuf> {
        self.reference.clone()
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        self.target.clone()
    }

    pub fn output_mode(&self) -> OutputMode {
        self.output_mode
    }

    pub fn dioxus_i18n(&self) -> bool {
        self.dioxus_i18n
    }
}

#[cfg(test)]
impl std::str::FromStr for CommandLineArgs {
    type Err = String; // Change if non-test version required

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split_whitespace();
        Ok(CommandLineArgs::try_parse_from(value).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn default_reference_locale_file_will_be_none() {
        let args = CommandLineArgs::from_str("").unwrap();
        assert_eq!(args.reference, None);
    }

    #[test]
    fn user_can_provide_reference_locale_file() {
        let args =
            CommandLineArgs::from_str("app_name --reference=tests/data/i18n/en/en-GB.ftl").unwrap();
        assert_eq!(
            args.reference,
            Some(PathBuf::from("tests/data/i18n/en/en-GB.ftl"))
        );
    }

    #[test]
    fn default_target_locales_will_be_empty() {
        let args = CommandLineArgs::from_str("").unwrap();
        assert_eq!(args.target, Vec::<PathBuf>::new())
    }

    #[test]
    fn user_can_provide_target_locale_file() {
        let args =
            CommandLineArgs::from_str("app_name --target=tests/data/i18n/it/it-IT.ftl").unwrap();
        assert_eq!(args.target, [PathBuf::from("tests/data/i18n/it/it-IT.ftl")])
    }

    #[test]
    fn user_can_provide_multiple_target_locale_files() {
        let args = CommandLineArgs::from_str("app_name --target=en-GB --target=en-US --target=it")
            .unwrap();
        assert_eq!(
            args.target,
            [
                PathBuf::from("en-GB"),
                PathBuf::from("en-US"),
                PathBuf::from("it")
            ]
        )
    }
}

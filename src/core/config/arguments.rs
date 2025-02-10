use clap::*;

use std::path::PathBuf;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputMode {
    /// Enables lingora to complete the error checks on the reference and target files, returning an
    /// error status if found. No detailed report will be produced.
    Silent,
    /// Output analysis report details to stdout.
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
pub struct Arguments {
    /// Config file.
    /// The config file contains attributes, most of which can also be overridden
    /// by command line arguments.
    /// If no config file is provided and `Lingora.toml` exists in the current
    /// working directory then that will be used. If `Lingora.toml` doesn't exist
    /// then sensible defaults will be used.
    #[clap(long, default_value = None)]
    config: Option<PathBuf>,

    /// Root path containing the translation files. This is used when no explicit
    /// targets are provided.
    #[clap(long, default_value = None)]
    root: Option<PathBuf>,

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

    /// If provided, then an the given file will be created, containing necessary code
    /// defining a [I18nConfig](https://docs.rs/dioxus-i18n/0.4.1/dioxus_i18n/use_i18n/struct.I18nConfig.html)
    /// struct.
    #[arg(long)]
    dioxus_i18n: Option<PathBuf>,
}

impl Arguments {
    pub fn config(&self) -> Option<&PathBuf> {
        self.config.as_ref()
    }

    pub fn root(&self) -> Option<&PathBuf> {
        self.root.as_ref()
    }

    pub fn reference(&self) -> Option<&PathBuf> {
        self.reference.as_ref()
    }

    pub fn targets(&self) -> Vec<&PathBuf> {
        Vec::from_iter(self.target.iter())
    }

    pub fn output_mode(&self) -> OutputMode {
        self.output_mode
    }

    pub fn dioxus_i18n(&self) -> Option<&PathBuf> {
        self.dioxus_i18n.as_ref()
    }
}

#[cfg(test)]
impl std::str::FromStr for Arguments {
    type Err = String; // Change if non-test version required

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split_whitespace();
        Ok(Arguments::try_parse_from(value).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn default_config_will_be_none() {
        let args = Arguments::from_str("").unwrap();
        assert_eq!(args.config, None);
    }

    #[test]
    fn user_can_provide_config() {
        let args = Arguments::from_str("app_name --config=tests/data/i18n/Lingora.toml").unwrap();
        assert_eq!(
            args.config,
            Some(PathBuf::from("tests/data/i18n/Lingora.toml"))
        );
    }

    #[test]
    fn default_root_will_be_none() {
        let args = Arguments::from_str("").unwrap();
        assert_eq!(args.root, None);
    }

    #[test]
    fn user_can_provide_root() {
        let args = Arguments::from_str("app_name --root=tests/data/i18n").unwrap();
        assert_eq!(args.root, Some(PathBuf::from("tests/data/i18n")));
    }

    #[test]
    fn default_reference_locale_file_will_be_none() {
        let args = Arguments::from_str("").unwrap();
        assert_eq!(args.reference, None);
    }

    #[test]
    fn user_can_provide_reference_locale_file() {
        let args =
            Arguments::from_str("app_name --reference=tests/data/i18n/en/en-GB.ftl").unwrap();
        assert_eq!(
            args.reference,
            Some(PathBuf::from("tests/data/i18n/en/en-GB.ftl"))
        );
    }

    #[test]
    fn default_target_locales_will_be_empty() {
        let args = Arguments::from_str("").unwrap();
        assert_eq!(args.target, Vec::<PathBuf>::new())
    }

    #[test]
    fn user_can_provide_target_locale_file() {
        let args = Arguments::from_str("app_name --target=tests/data/i18n/it/it-IT.ftl").unwrap();
        assert_eq!(args.target, [PathBuf::from("tests/data/i18n/it/it-IT.ftl")])
    }

    #[test]
    fn user_can_provide_multiple_target_locale_files() {
        let args =
            Arguments::from_str("app_name --target=en-GB --target=en-US --target=it").unwrap();
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

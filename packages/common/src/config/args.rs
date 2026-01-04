use std::path::{Path, PathBuf};

use clap::*;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct AnalysisArgs {
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
    /// If the target is not provided then the default root folder will be used.
    #[arg(short, long)]
    target: Vec<PathBuf>,

    /// The root source path rust source files that use dioxus_i18n macros.
    /// If provided Lingora will analyse the source files for usage of the
    /// `dioxus_i18n::t!`, `te!` and `tid!` macros. If parsable (const str)
    /// it check that the identifier exists in the reference file and produce
    /// a warning if it.
    #[clap(long, default_value = None)]
    rust_src: Option<PathBuf>,
}

impl AnalysisArgs {
    pub fn config(&self) -> Option<&Path> {
        self.config.as_deref()
    }

    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    pub fn reference(&self) -> Option<&Path> {
        self.reference.as_deref()
    }

    pub fn targets(&self) -> impl Iterator<Item = &Path> {
        self.target.iter().map(|p| p.as_path())
    }
}

#[cfg(test)]
impl std::str::FromStr for AnalysisArgs {
    type Err = String; // Change if non-test version required

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split_whitespace();
        Ok(AnalysisArgs::try_parse_from(value).unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_config_will_be_none() {
        let args = AnalysisArgs::from_str("").unwrap();
        assert_eq!(args.config, None);
    }

    #[test]
    fn user_can_provide_config() {
        let args =
            AnalysisArgs::from_str("app_name --config=tests/data/i18n/Lingora.toml").unwrap();
        assert_eq!(
            args.config,
            Some(PathBuf::from("tests/data/i18n/Lingora.toml"))
        );
    }

    #[test]
    fn default_root_will_be_none() {
        let args = AnalysisArgs::from_str("").unwrap();
        assert_eq!(args.root, None);
    }

    #[test]
    fn user_can_provide_root() {
        let args = AnalysisArgs::from_str("app_name --root=tests/data/i18n").unwrap();
        assert_eq!(args.root, Some(PathBuf::from("tests/data/i18n")));
    }

    #[test]
    fn default_reference_locale_file_will_be_none() {
        let args = AnalysisArgs::from_str("").unwrap();
        assert_eq!(args.reference, None);
    }

    #[test]
    fn user_can_provide_reference_locale_file() {
        let args =
            AnalysisArgs::from_str("app_name --reference=tests/data/i18n/en/en-GB.ftl").unwrap();
        assert_eq!(
            args.reference,
            Some(PathBuf::from("tests/data/i18n/en/en-GB.ftl"))
        );
    }

    #[test]
    fn default_target_locales_will_be_empty() {
        let args = AnalysisArgs::from_str("").unwrap();
        assert_eq!(args.target, Vec::<PathBuf>::new())
    }

    #[test]
    fn user_can_provide_target_locale_file() {
        let args =
            AnalysisArgs::from_str("app_name --target=tests/data/i18n/it/it-IT.ftl").unwrap();
        assert_eq!(args.target, [PathBuf::from("tests/data/i18n/it/it-IT.ftl")])
    }

    #[test]
    fn user_can_provide_multiple_target_locale_files() {
        let args =
            AnalysisArgs::from_str("app_name --target=en-GB --target=en-US --target=it").unwrap();
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

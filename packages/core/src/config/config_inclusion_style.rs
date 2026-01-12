use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, ValueEnum)]
#[clap(rename_all = "lowercase")]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum ConfigInclusionStyle {
    IncludeStr,
    PathBuf,
    Auto,
}

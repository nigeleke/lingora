use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[clap(rename_all = "lowercase")]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum ConfigInclusionStyle {
    IncludeStr,
    PathBuf,
    #[default]
    Auto,
}

use std::str::FromStr;

use crate::domain::{LanguageRoot, Locale};

pub fn locale(s: &str) -> Locale {
    Locale::from_str(s).expect("valid locale should be supplied")
}

pub fn root(s: &str) -> LanguageRoot {
    LanguageRoot::from(&locale(s))
}

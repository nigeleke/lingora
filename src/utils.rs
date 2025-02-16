use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

use std::{path::PathBuf, str::FromStr};

#[inline]
pub fn pb2id(path: &PathBuf) -> Result<LanguageIdentifier, LanguageIdentifierError> {
    let stem = path.file_stem().map_or("".into(), |s| s.to_string_lossy());
    LanguageIdentifier::from_str(&stem)
}

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{
    domain::{HasLocale, Locale},
    error::LingoraError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FluentFile {
    path: PathBuf,
    locale: Locale,
}

impl FluentFile {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl HasLocale for FluentFile {
    fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl TryFrom<&Path> for FluentFile {
    type Error = LingoraError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let is_fluent_ext = |path: &Path| path.extension() == Some(OsStr::new("ftl"));
        let is_fluent_file = |path: &Path| path.is_file() && is_fluent_ext(path);

        if is_fluent_file(path) {
            let locale = Locale::try_from(path)?;
            let path = path.to_path_buf();
            Ok(Self { path, locale })
        } else {
            Err(LingoraError::InvalidFluentPath(path.to_path_buf()))
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use tempfile::TempDir;

    use super::*;

    fn setup(folder: &str, filename: &str) -> PathBuf {
        let mut path = TempDir::new().expect("valid temp dir").path().to_path_buf();

        if !folder.trim().is_empty() {
            path = path.join(folder);
        };

        std::fs::create_dir_all(&path).expect("valid dir");

        path = path.join(filename);
        std::fs::write(&path, "").expect("valid write");

        path
    }

    #[test]
    fn is_fluent_file_if_locale_stem_and_ftl() {
        let locale = Locale::from_str("en-GB").expect("valid locale");
        let path = setup("", &format!("{locale}.ftl"));

        let file = FluentFile::try_from(path.as_path()).expect("valid file");
        assert_eq!(file.locale(), &locale);
    }

    #[test]
    fn is_fluent_file_if_locale_path_and_ftl() {
        let locale = Locale::from_str("en-AU").expect("valid locale");
        let path = setup(&format!("{locale}"), "module.ftl");

        let file = FluentFile::try_from(path.as_path()).expect("valid file");
        assert_eq!(file.locale(), &locale);
    }

    #[test]
    fn is_not_fluent_file_if_not_ftl() {
        let locale = Locale::from_str("en-US").expect("valid locale");
        let path = setup("", &format!("{locale}.txt"));

        let error = FluentFile::try_from(path.as_path()).expect_err("error");
        assert!(matches!(error, LingoraError::InvalidFluentPath(_)));
    }

    #[test]
    #[ignore = "temp root is /tmp/ and treated as valid locale"]
    fn is_not_fluent_file_if_not_locale_path() {
        let path = setup("en-NE", "module.ftl");

        let result = FluentFile::try_from(path.as_path());
        assert!(matches!(result, Err(LingoraError::InvalidFluentPath(_))));
    }
}

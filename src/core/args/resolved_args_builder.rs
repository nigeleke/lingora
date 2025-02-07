use super::{command_line_args::CommandLineArgs, resolved_args::ResolvedArgs};

use crate::core::domain::Locale;

use thiserror::Error;
use walkdir::WalkDir;

#[cfg(test)]
use fs_extra::*;

#[cfg(test)]
use tempfile::*;

use std::path::{Path, PathBuf};

#[derive(Debug, Error)]
pub enum ResolvedArgsBuilderError {
    #[error("no reference files found for default system locale: {0}")]
    NoReferenceFileFound(String),

    #[error("multiple reference files found for default system locale: {0}")]
    AmbiguousReferenceFiles(String),
}

type Result<T> = std::result::Result<T, ResolvedArgsBuilderError>;

#[derive(Debug)]
pub struct ResolvedArgsBuilder {
    default_i18n_path: PathBuf,
    default_sys_locale: Locale,
}

impl ResolvedArgsBuilder {
    #[cfg(test)]
    pub fn new(default_i18n_path: PathBuf, default_sys_locale: Locale) -> Self {
        Self {
            default_i18n_path,
            default_sys_locale,
        }
    }

    #[cfg(test)]
    pub fn create_using(default_i18n_root_source: PathBuf, default_sys_locale: Locale) -> Self {
        let default_i18n_path = tempdir().unwrap().into_path();
        dir::copy(
            &default_i18n_root_source,
            &default_i18n_path,
            &dir::CopyOptions::new(),
        )
        .expect("test data copied to test folder");

        Self {
            default_i18n_path,
            default_sys_locale,
        }
    }

    pub fn build(self, args: &CommandLineArgs) -> Result<ResolvedArgs> {
        let reference = self.validated_reference_file(args)?;
        let targets = self.collated_targets(args, &reference);

        let args = ResolvedArgs::new(reference, targets);

        Ok(args)
    }

    fn validated_reference_file(&self, args: &CommandLineArgs) -> Result<PathBuf> {
        if let Some(path) = args.reference() {
            Ok(path)
        } else {
            let paths = self.reference_file_search_paths(args);
            let ftl_files = self.find_sys_locale_files(paths.as_slice());

            type RB = ResolvedArgsBuilderError;
            match ftl_files.len() {
                0 => Err(RB::NoReferenceFileFound(Locale::default().to_string())),
                1 => Ok(ftl_files[0].clone()),
                _ => Err(RB::AmbiguousReferenceFiles(Locale::default().to_string())),
            }
        }
    }

    fn reference_file_search_paths(&self, args: &CommandLineArgs) -> Vec<PathBuf> {
        if args.targets().is_empty() {
            vec![self.default_i18n_path.clone()]
        } else {
            args.targets()
                .into_iter()
                .filter(|p| p.is_dir())
                .collect::<Vec<_>>()
        }
    }

    fn find_sys_locale_files(&self, paths: &[PathBuf]) -> Vec<PathBuf> {
        let locale = &self.default_sys_locale;
        let locale_filename = PathBuf::from(locale.to_string().as_str()).with_extension("ftl");

        paths.iter().fold(Vec::new(), |mut acc, p| {
            let files = deep_find_sys_locale_files(p, &locale_filename);
            acc.extend(files);
            acc
        })
    }

    fn collated_targets(&self, args: &CommandLineArgs, reference: &PathBuf) -> Vec<PathBuf> {
        if args.targets().is_empty() {
            deep_find_ftl_files(&self.default_i18n_path)
        } else {
            let ftl_extension = std::ffi::OsStr::new("ftl");
            args.targets()
                .into_iter()
                .fold(Vec::new(), |mut acc, path| {
                    if path.is_file() && path.extension() == Some(ftl_extension) {
                        acc.push(path);
                    } else {
                        acc.extend(deep_find_ftl_files(&path));
                    }
                    acc
                })
        }
        .into_iter()
        .filter(|path| path != reference)
        .collect::<Vec<_>>()
    }
}

impl Default for ResolvedArgsBuilder {
    fn default() -> Self {
        Self {
            default_i18n_path: std::env::current_dir().unwrap().join("i18n"),
            default_sys_locale: Locale::default(),
        }
    }
}

fn deep_find_sys_locale_files(path: &Path, locale_filename: &Path) -> Vec<PathBuf> {
    deep_find_ftl_files(path)
        .into_iter()
        .filter(|path| path.file_name() == locale_filename.file_name())
        .collect::<Vec<_>>()
}

fn deep_find_ftl_files(path: &Path) -> Vec<PathBuf> {
    let required_extension = std::ffi::OsStr::new("ftl");
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(required_extension))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use std::str::FromStr;

    #[test]
    fn builder_new_creates_builder() {
        let builder = ResolvedArgsBuilder::new(PathBuf::from("."), Locale::default());
        assert_eq!(builder.default_i18n_path, PathBuf::from("."));
        assert_eq!(builder.default_sys_locale, Locale::default());
    }

    #[test]
    fn default_builder_uses_current_working_directory_and_sys_locale() {
        let builder = ResolvedArgsBuilder::default();
        assert_eq!(
            builder.default_i18n_path,
            std::env::current_dir().unwrap().join("i18n")
        );
        assert_eq!(builder.default_sys_locale, Locale::default());
    }

    #[test]
    fn have_reference_file_if_provided_in_command_line_arguments() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_empty/"),
            Locale::default(),
        );
        let args = CommandLineArgs::from_str("app_name -r path/to/en.ftl").unwrap();
        let args = builder.build(&args).unwrap();
        assert_eq!(*args.reference(), PathBuf::from("path/to/en.ftl"));
    }

    #[test]
    fn have_reference_file_if_not_provided_but_sys_locale_file_is_in_target_folder() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n/"),
            Locale::from_str("en-GB").unwrap(),
        );
        let args = format!(
            "app_name -t {}",
            builder.default_i18n_path.to_string_lossy()
        );
        let args = CommandLineArgs::from_str(&args).unwrap();
        let args = builder.build(&args).unwrap();
        let expected_file_name = PathBuf::from("en-GB.ftl");
        assert_eq!(
            args.reference().file_name().unwrap(),
            expected_file_name.file_name().unwrap()
        );
    }

    #[test]
    fn fail_to_have_reference_file_if_not_provided_and_sys_locale_file_not_in_target_folder() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_empty/"),
            Locale::default(),
        );
        let args = format!(
            "app_name -t {}",
            builder.default_i18n_path.to_string_lossy()
        );
        let args = CommandLineArgs::from_str(&args).unwrap();
        let error = builder.build(&args).unwrap_err();
        assert!(matches!(
            error,
            ResolvedArgsBuilderError::NoReferenceFileFound(_)
        ));
    }

    #[test]
    fn fail_to_have_reference_file_if_not_provided_and_multiple_sys_locale_files_in_target_folder()
    {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_duplicates/"),
            Locale::from_str("en-GB").unwrap(),
        );
        let args = format!(
            "app_name -t {}",
            builder.default_i18n_path.to_string_lossy()
        );
        let args = CommandLineArgs::from_str(&args).unwrap();
        let error = builder.build(&args).unwrap_err();
        assert!(matches!(
            error,
            ResolvedArgsBuilderError::AmbiguousReferenceFiles(_)
        ));
    }

    fn count_files(file_name: &str, files: &[PathBuf]) -> usize {
        files
            .iter()
            .filter(|f| f.ends_with(PathBuf::from(file_name)))
            .count()
    }

    #[test]
    fn have_targets_when_explicitly_provided_command_line_arguments() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_empty"), // default unused
            Locale::default(),
        );
        let args = CommandLineArgs::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let args = builder.build(&args).unwrap();

        let targets = args.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", targets), 1);
    }

    #[test]
    fn not_have_reference_in_targets_even_when_requested() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_empty"), // default unused
            Locale::default(),
        );
        let args = CommandLineArgs::from_str("app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/en/en-AU.ftl -t tests/data/i18n/it/it-IT.ftl").unwrap();
        let args = builder.build(&args).unwrap();

        let targets = args.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", targets), 1);
    }

    #[test]
    fn have_targets_when_implicitly_provided_command_line_arguments() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n_empty"), // default unused
            Locale::default(),
        );
        let args = CommandLineArgs::from_str(
            "app_name -r tests/data/i18n/en/en-GB.ftl -t tests/data/i18n/",
        )
        .unwrap();
        let args = builder.build(&args).unwrap();

        let targets = args.targets();
        assert_eq!(count_files("tests/data/i18n/en/en.ftl", targets), 1);
        assert_eq!(count_files("tests/data/i18n/en/en-GB.ftl", targets), 0);
        assert_eq!(count_files("tests/data/i18n/en/en-AU.ftl", targets), 1);
        assert_eq!(count_files("tests/data/i18n/it/it-IT.ftl", targets), 1);
    }

    #[test]
    fn default_target_path_is_used_when_no_targets_provided() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n"),
            Locale::from_str("en-GB").unwrap(),
        );
        let args = CommandLineArgs::from_str("app_name").unwrap();
        let args = builder.build(&args).unwrap();

        let targets = args.targets();
        assert_eq!(count_files("i18n/en/en.ftl", targets), 1);
        assert_eq!(count_files("i18n/en/en-GB.ftl", targets), 0);
        assert_eq!(count_files("i18n/en/en-AU.ftl", targets), 1);
        assert_eq!(count_files("i18n/it/it-IT.ftl", targets), 1);
    }

    #[test]
    fn default_reference_path_is_used_when_no_targets_provided() {
        let builder = ResolvedArgsBuilder::create_using(
            PathBuf::from("tests/data/i18n"),
            Locale::from_str("en-GB").unwrap(),
        );
        let args = CommandLineArgs::from_str("app_name").unwrap();
        let args = builder.build(&args).unwrap();

        assert_eq!(
            args.reference().file_name(),
            PathBuf::from("en-GB.ftl").file_name()
        );
    }
}

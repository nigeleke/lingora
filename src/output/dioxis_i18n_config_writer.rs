use std::{
    collections::HashMap,
    path::{Component, PathBuf},
};

use super::error::{Result, WriterError};
use crate::{
    config::{Settings, WithLocale},
    domain::Locale,
};

pub struct DioxusI18nConfigWriter<'a> {
    settings: &'a Settings,
    path: &'a PathBuf,
}

impl<'a> DioxusI18nConfigWriter<'a> {
    pub fn new(settings: &'a Settings, path: &'a PathBuf) -> Self {
        Self { settings, path }
    }

    pub fn write(&self) -> Result<()> {
        let template = r#"use dioxus_i18n::{prelude::*, *};
use unic_langid::{langid, LanguageIdentifier};
/*** INCLUDE ***/

pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
    I18nConfig::new(initial_language)
/*** LOCALES ***//*** SHARES ***//*** FALLBACK ***/}
"#
        .replace("/*** INCLUDE ***/", &self.include())
        .replace("/*** LOCALES ***/", &self.locales())
        .replace("/*** SHARES ***/", &self.shares())
        .replace("/*** FALLBACK ***/", &self.fallback());

        let parent = self.path.parent().ok_or_else(|| {
            WriterError::WriteFailed(format!(
                "Cannot determine path for file: {}",
                self.path.to_string_lossy()
            ))
        })?;
        std::fs::create_dir_all(parent).map_err(|e| WriterError::WriteFailed(e.to_string()))?;
        std::fs::write(self.path, template).map_err(|e| WriterError::WriteFailed(e.to_string()))
    }

    fn include(&self) -> String {
        match self.settings.with_locale() {
            WithLocale::IncludeStr => "",
            _ => "use std::path::PathBuf;",
        }
        .into()
    }

    fn locales(&self) -> String {
        match self.settings.with_locale() {
            WithLocale::IncludeStr => self.locales_using_prefix("include_str!"),
            WithLocale::PathBuf => self.locales_using_prefix("PathBuf::from"),
            WithLocale::Auto => self.auto_locales(),
        }
    }

    fn locales_using_prefix(&self, prefix: &str) -> String {
        let mut ftl_files = self.settings.targets();
        ftl_files.push(self.settings.reference().clone());
        ftl_files.sort_by(|lhs, rhs| lhs.file_name().cmp(&rhs.file_name()));

        ftl_files.iter().fold(String::new(), |acc, p| {
            let locale = self.derived_locale_using_prefix(prefix, p);
            format!("{}{}", acc, locale)
        })
    }

    fn derived_locale_using_prefix(&self, prefix: &str, path: &PathBuf) -> String {
        Self::locale(
            &path.file_stem().unwrap().to_string_lossy(),
            prefix,
            &Self::relative_path_string(self.path, path),
        )
    }

    fn relative_path_string(from: &PathBuf, to_maybe_relative: &PathBuf) -> String {
        let from = Self::absolute_path(from);
        let from = from.parent().unwrap();
        let to = Self::absolute_path(to_maybe_relative);

        let from_components = from.components().collect::<Vec<_>>();
        let to_components = to.components().collect::<Vec<_>>();

        let common_prefix_len = from_components
            .iter()
            .zip(&to_components)
            .skip_while(|(a, b)| matches!(a, Component::RootDir) && matches!(b, Component::RootDir))
            .take_while(|(a, b)| a == b)
            .count();

        if common_prefix_len == 0 {
            to_maybe_relative.to_string_lossy().to_string()
        } else {
            let mut result = PathBuf::new();
            std::iter::repeat_n("..", from_components[common_prefix_len..].len());
            result.extend(&to_components[common_prefix_len..]);
            result.to_string_lossy().to_string()
        }
    }

    fn absolute_path(path: &PathBuf) -> PathBuf {
        if path.is_absolute() {
            path.clone()
        } else {
            std::env::current_dir().unwrap().join(path)
        }
    }

    fn locale(langid: &str, prefix: &str, path_str: &str) -> String {
        format!(
            r#"        .with_locale((
            langid!("{}"),
            {}
        ))
"#,
            langid,
            Self::locale_prefix_pathstr(prefix, path_str)
        )
    }

    fn locale_prefix_pathstr(prefix: &str, path_str: &str) -> String {
        format!("{}(\"{}\")", prefix, path_str)
    }

    fn auto_locales(&self) -> String {
        format!(
            r#"        .with_auto_locales(PathBuf::from("{}"))
"#,
            Self::relative_path_string(self.path, self.settings.root())
        )
    }

    fn shares(&self) -> String {
        match self.settings.with_locale() {
            WithLocale::IncludeStr => self.shares_using_prefix("include_str!"),
            WithLocale::PathBuf => self.shares_using_prefix("PathBuf::from"),
            WithLocale::Auto => self.shares_using_prefix("PathBuf::from"),
        }
    }

    fn shares_using_prefix(&self, prefix: &str) -> String {
        let mut ftl_files = self.settings.targets();
        ftl_files.push(self.settings.reference().clone());
        let ftl_files = ftl_files.iter().fold(HashMap::new(), |mut acc, f| {
            if let Ok(locale) = Locale::try_from(f) {
                acc.insert(locale, f);
            }
            acc
        });

        let shares = self.settings.shares();
        let targets = shares.iter().fold(HashMap::new(), |mut acc, (_, target)| {
            if let Some(path) = ftl_files.get(target) {
                acc.insert(target, *path);
            }
            acc
        });

        shares.iter().fold(String::new(), |acc, (source, target)| {
            let locale = Self::locale(
                &source.to_string(),
                prefix,
                &Self::relative_path_string(self.path, targets[target]),
            );
            format!("{}{}", acc, locale)
        })
    }

    fn fallback(&self) -> String {
        format!(
            r#"        .with_fallback(langid!("{}"))
"#,
            self.settings.fallback()
        )
    }
}

use std::{
    collections::HashSet,
    io,
    path::{Path, PathBuf},
};

use crate::{
    audit::Workspace,
    config::{ConfigInclusionStyle, LingoraToml},
    domain::LanguageRoot,
    error::LingoraError,
};

/// Renderer that generates Rust source code for `dioxus_i18n::I18nConfig`.
///
/// Produces a complete `config.rs`-style file that configures:
/// - All available locales (`.with_locale(...)` or `.with_auto_locales(...)`)
/// - Shared base documents (`.share(...)` for primaries/canonical)
/// - Fallback chain (`.with_fallback(...)`)
///
/// The output respects the `config_inclusion` style from `Lingora.toml`:
/// - `IncludeStr` → uses `include_str!()` for compile-time embedding
/// - `PathBuf`   → uses `PathBuf::from(...)` for runtime loading
/// - `Auto`      → uses `.with_auto_locales(...)` for directory-based discovery
///
/// Relative paths are computed from the target source directory (usually `src/`)
/// so the generated code works correctly regardless of where `config.rs` is placed.
pub struct DioxusI18nConfigRenderer {
    settings: LingoraToml,
    workspace: Workspace,
    target_source_path: PathBuf,
}

impl DioxusI18nConfigRenderer {
    /// Creates a new renderer.
    ///
    /// - `settings`: deserialized `Lingora.toml`
    /// - `workspace`: discovered Fluent files, canonical, primaries, etc.
    /// - `target_source_path`: path to the Rust source root (for relative path calculation).
    ///   Defaults to current directory (`.`) if `None`.
    pub fn new(
        settings: &LingoraToml,
        workspace: &Workspace,
        target_source_path: Option<&Path>,
    ) -> Self {
        let settings = settings.clone();
        let workspace = workspace.clone();
        let target_source_path = target_source_path
            .map(|p| p.to_path_buf())
            .unwrap_or(PathBuf::from("."));

        Self {
            settings,
            workspace,
            target_source_path,
        }
    }

    /// Renders the complete `I18nConfig` builder code to the given writer.
    ///
    /// Output is Rust source code that can be written to `src/config.rs` (or similar).
    /// The template uses placeholders that are replaced with:
    /// - `include` statements (if needed)
    /// - Locale registrations
    /// - Shared base documents
    /// - Fallback configuration
    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
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

        write!(out, "{template}")?;
        out.flush()?;

        Ok(())
    }

    fn include(&self) -> String {
        match self.settings.dioxus_i18n.config_inclusion {
            ConfigInclusionStyle::IncludeStr => "",
            _ => "use std::path::PathBuf;",
        }
        .into()
    }

    fn locales(&self) -> String {
        match self.settings.dioxus_i18n.config_inclusion {
            ConfigInclusionStyle::IncludeStr => self.locales_using_prefix("include_str!"),
            ConfigInclusionStyle::PathBuf => self.locales_using_prefix("PathBuf::from"),
            ConfigInclusionStyle::Auto => self.auto_locales(),
        }
    }

    fn locales_using_prefix(&self, prefix: &str) -> String {
        let mut fluent_files = Vec::from_iter(self.workspace.fluent_files());
        fluent_files.sort_by_key(|f| f.locale());
        fluent_files.iter().fold(String::new(), |acc, p| {
            let locale = self.derived_locale_using_prefix(prefix, p.path());
            format!("{}{}", acc, locale)
        })
    }

    fn derived_locale_using_prefix(&self, prefix: &str, path: &Path) -> String {
        Self::locale(
            &path.file_stem().unwrap().to_string_lossy(),
            prefix,
            &self.relative_path_string(path),
        )
    }

    fn relative_path_string(&self, to_maybe_relative: &Path) -> String {
        let from = Self::to_absolute_path(&self.target_source_path);
        let to = Self::to_absolute_path(to_maybe_relative);

        let from_components = from.components().collect::<Vec<_>>();
        let to_components = to.components().collect::<Vec<_>>();

        let common_prefix_len = from_components
            .iter()
            .zip(&to_components)
            .take_while(|(a, b)| a == b)
            .count();

        if common_prefix_len == 0 {
            to_maybe_relative.to_string_lossy().to_string()
        } else {
            let mut result = PathBuf::new();
            result.extend(&mut Vec::from_iter(std::iter::repeat_n(
                "..",
                from_components[common_prefix_len..].len(),
            )));
            result.extend(&to_components[common_prefix_len..]);
            result.to_string_lossy().to_string()
        }
    }

    fn to_absolute_path(path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
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
        self.settings
            .lingora
            .fluent_sources
            .iter()
            .filter(|source| source.is_dir())
            .fold(String::new(), |acc, path| {
                format!(
                    r#"{}        .with_auto_locales(PathBuf::from("{}"))
"#,
                    acc,
                    self.relative_path_string(path)
                )
            })
    }

    fn shares(&self) -> String {
        match self.settings.dioxus_i18n.config_inclusion {
            ConfigInclusionStyle::IncludeStr => self.shares_using_prefix("include_str!"),
            ConfigInclusionStyle::PathBuf => self.shares_using_prefix("PathBuf::from"),
            ConfigInclusionStyle::Auto => self.shares_using_prefix("PathBuf::from"),
        }
    }

    fn shares_using_prefix(&self, prefix: &str) -> String {
        let base_locales = self.workspace.base_locales().collect::<HashSet<_>>();
        let language_root_only = base_locales
            .iter()
            .filter(|l| l.region().is_none() && !l.has_variants())
            .collect::<HashSet<_>>();
        let unrooted_base_files = self.workspace.fluent_files().iter().filter(|f| {
            let locale = f.locale();
            base_locales.contains(locale) && !language_root_only.contains(&locale)
        });

        unrooted_base_files.fold(String::new(), |acc, file| {
            let locale = file.locale();
            format!(
                r#"{}        .share(langid!("{}"), {}("{}"))
            "#,
                acc,
                LanguageRoot::from(locale),
                prefix,
                self.relative_path_string(file.path())
            )
        })
    }

    fn fallback(&self) -> String {
        format!(
            r#"        .with_fallback(langid!("{}"))
"#,
            self.settings.lingora.canonical
        )
    }
}

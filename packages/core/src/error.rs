use thiserror::*;

/// Error type for the `lingora-core` crate.
#[derive(Debug, Error)]
pub enum LingoraError {
    /// An I/O error occurred while reading/writing files (Fluent files, config, Rust sources, etc)
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Failed to deserialize `Lingora.toml` configuration file.
    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),

    /// Clap failed to parse command-line arguments.
    #[error(transparent)]
    Args(#[from] clap::Error),

    /// Error originating from the `fluent4rs` parser while parsing `.ftl` files.
    #[error(transparent)]
    Fluent(#[from] fluent4rs::prelude::Fluent4rsError),

    /// `syn` failed to parse Rust source code (used when scanning for `t!`, `te!`, `tid!` macros).
    #[error(transparent)]
    Syn(#[from] syn::Error),

    /// The provided locale string is not a valid BCP 47 language tag.
    /// Example: `"french"` instead of `"fr"`, or `"en_US"` (underscore instead of hyphen).
    #[error("invalid locale: {0}")]
    InvalidLocale(String),

    /// A file path was given as a Fluent translation file, but it doesn't match expected
    /// naming conventions or location rules (e.g. wrong extension, not under a locale dir).
    #[error("invalid fluent file path: {0}")]
    InvalidFluentPath(std::path::PathBuf),

    /// A path was provided as a Rust source file to scan, but it cannot be processed
    /// (wrong extension, not readable, outside project root, etc)
    #[error("invalid rust file path: {0}")]
    InvalidRustPath(std::path::PathBuf),

    /// Multiple locale files resolve to the same primary language root, making it
    /// impossible to determine a clean fallback chain (e.g. two files both treated as "en").
    /// This usually indicates misconfiguration in `Lingora.toml` or ambiguous file naming.
    #[error(
        "multiple locales resolve to the same language root(s) making graceful fallback impossible for {0}"
    )]
    AmbiguousLanguageRoots(String),

    /// The configuration or CLI arguments require translation files for certain locales,
    /// but none were found on disk.
    #[error("no translation file(s) for required locales: {0}")]
    MissingTranslationFiles(String),

    /// The workspace or configuration specifies locales, but none of them qualify as
    /// *primary* locales (i.e. no non-variant/base-language documents were located).
    #[error("no primary locales found for provided locales: {0}")]
    MissingPrimaryLocales(String),

    /// A string literal found in Rust code (inside a `t!`, `te!` or `tid!` macro)
    /// does not conform to the expected Fluent identifier syntax.
    ///
    /// Examples of malformed literals:
    /// - contains invalid characters
    /// - starts with a digit
    /// - has invalid dot/hyphen placement
    #[error("malformed identifier literal {0}")]
    MalformedIdentifierLiteral(String),
}

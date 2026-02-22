#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod audit;
mod config;
mod domain;
mod error;
mod fluent;
mod renderers;
mod rust;
#[cfg(test)]
mod test_support;

/// Prelude module for convenient imports of the most commonly used types and traits
/// in the `lingora-core` crate.
///
/// ```rust
/// use lingora_core::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        audit::{
            AuditEngine, AuditIssue, AuditResult, AuditedDocument, DocumentRole, Kind, Subject,
            Workspace,
        },
        config::{CoreArgs, LingoraToml},
        domain::{LanguageRoot, Locale},
        error::LingoraError,
        fluent::{FluentDocument, QualifiedIdentifier},
        renderers::{AnalysisRenderer, DioxusI18nConfigRenderer},
    };
}

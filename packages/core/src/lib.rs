mod audit;
mod config;
mod domain;
mod error;
mod fluent;
mod renderers;
mod rust;
#[cfg(test)]
mod test_support;

pub mod prelude {
    pub use super::{
        audit::{AuditEngine, AuditIssue, AuditResult, AuditedDocument, Workspace},
        config::{CoreArgs, LingoraToml},
        domain::{LanguageRoot, Locale},
        error::LingoraError,
        fluent::{FluentDocument, QualifiedIdentifier},
        renderers::{AnalysisRenderer, DioxusI18nConfigRenderer},
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditIssue {
    InvalidSyntax(String),
    DuplicateDefinition(String),
    InvalidReference(String),
    MissingTranslation(String),
    RedundantTranslation(String),
    SignatureMismatch(String),
}

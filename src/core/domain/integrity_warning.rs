#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegrityWarning {
    IdentifierConflict(String),
    MessageTermConflict(String),
    InvalidMessageReference(String),
    InvalidTermReference(String),
    MissingTranslation(String),
    SuperfluousTranslation(String),
}

impl IntegrityWarning {
    pub fn category_str(&self) -> &str {
        match self {
            Self::IdentifierConflict(_) => "Identifier conflict",
            Self::MessageTermConflict(_) => "Message / term conflict",
            Self::InvalidMessageReference(_) => "Invalid message reference",
            Self::InvalidTermReference(_) => "Invalid term reference",
            Self::MissingTranslation(_) => "Missing translation",
            Self::SuperfluousTranslation(_) => "Superfluous translation",
        }
    }

    pub fn value_str(&self) -> &str {
        match self {
            Self::IdentifierConflict(s)
            | Self::MessageTermConflict(s)
            | Self::InvalidMessageReference(s)
            | Self::InvalidTermReference(s)
            | Self::MissingTranslation(s)
            | Self::SuperfluousTranslation(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for IntegrityWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.category_str(), self.value_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Warning {
    IdentifierConflict(String),
    MessageTermConflict(String),
    InvalidMessageReference(String),
    InvalidTermReference(String),
    MissingTranslation(String),
    SuperfluousTranslation(String),
}

impl Warning {
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

    pub fn is_error(&self) -> bool {
        match self {
            Self::IdentifierConflict(_)
            | Self::MessageTermConflict(_)
            | Self::InvalidMessageReference(_)
            | Self::InvalidTermReference(_)
            | Self::MissingTranslation(_) => true,
            Self::SuperfluousTranslation(_) => false,
        }
    }

    pub fn is_warning(&self) -> bool {
        !self.is_error()
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.category_str(), self.value_str())
    }
}

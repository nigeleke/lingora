use fluent4rs::ast::{Message, MessageReference, Term, TermReference};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn name(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Message> for Identifier {
    fn from(value: &Message) -> Self {
        Self(value.identifier_name())
    }
}

impl From<&MessageReference> for Identifier {
    fn from(value: &MessageReference) -> Self {
        Self(value.identifier_name())
    }
}

impl From<&Term> for Identifier {
    fn from(value: &Term) -> Self {
        Self(value.identifier_name())
    }
}

impl From<&TermReference> for Identifier {
    fn from(value: &TermReference) -> Self {
        Self(value.identifier_name())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

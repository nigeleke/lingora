use fluent4rs::ast::{Attribute, Message, MessageReference, Term, TermReference};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    parent_part: String,
    child_part: Option<String>,
}

impl Identifier {
    pub fn new(parent_part: String, child_part: Option<String>) -> Self {
        Self {
            parent_part,
            child_part,
        }
    }
}

impl Identifier {
    pub fn from_parent_and_attribute(identifier: &Identifier, attribute: &Attribute) -> Self {
        Self::new(identifier.to_string(), Some(attribute.identifier_name()))
    }

    pub fn name(&self) -> String {
        format!(
            "{}{}",
            self.parent_part,
            self.child_part.as_deref().unwrap_or_default()
        )
    }
}

impl From<&Message> for Identifier {
    fn from(value: &Message) -> Self {
        Self::new(value.identifier_name(), None)
    }
}

impl From<&MessageReference> for Identifier {
    fn from(value: &MessageReference) -> Self {
        Self::new(
            value.identifier_name(),
            value.attribute_accessor().map(|a| a.identifier_name()),
        )
    }
}

impl From<&Term> for Identifier {
    fn from(value: &Term) -> Self {
        Self::new(value.identifier_name(), None)
    }
}

impl From<&TermReference> for Identifier {
    fn from(value: &TermReference) -> Self {
        Self::new(
            value.identifier_name(),
            value.attribute_accessor().map(|a| a.identifier_name()),
        )
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

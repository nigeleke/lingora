use super::identifier::Identifier;

use fluent4rs::prelude::{Message, Parser, Resource, Term, Visitor, Walker};
use thiserror::*;

use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum FluentFileError {
    #[error("fluent file read failed - reason: {0}")]
    UnableToRead(String),

    #[error("fluent file parse failed - reason: {0}")]
    UnableToParse(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FluentFile(Resource);

impl FluentFile {
    pub fn resource(&self) -> &Resource {
        &self.0
    }

    pub fn identifiers(&self) -> HashSet<Identifier> {
        let mut visitor = IdentifierVisitor(HashSet::new());
        Walker::walk(&self.0, &mut visitor);
        visitor.0
    }
}

struct IdentifierVisitor(HashSet<Identifier>);

impl Visitor for IdentifierVisitor {
    fn visit_message(&mut self, message: &Message) {
        self.0.insert(Identifier::from(message));
    }

    fn visit_message_reference(&mut self, reference: &fluent4rs::prelude::MessageReference) {
        self.0.insert(Identifier::from(reference));
    }

    fn visit_term(&mut self, term: &Term) {
        self.0.insert(Identifier::from(term));
    }

    fn visit_term_reference(&mut self, reference: &fluent4rs::prelude::TermReference) {
        self.0.insert(Identifier::from(reference));
    }
}

impl TryFrom<&PathBuf> for FluentFile {
    type Error = FluentFileError;

    fn try_from(value: &PathBuf) -> std::result::Result<Self, Self::Error> {
        let content = std::fs::read_to_string(value)
            .map_err(|e| FluentFileError::UnableToRead(e.to_string()))?;

        let resource = Parser::parse(content.as_str())
            .map_err(|e| FluentFileError::UnableToParse(e.to_string()))?;

        Ok(Self(resource))
    }
}

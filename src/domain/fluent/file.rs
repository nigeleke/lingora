use std::{collections::HashSet, path::PathBuf};

use fluent4rs::prelude::{Parser, Resource, Walker};
use thiserror::*;

use super::identifier_visitor::IdentifierVisitor;
use crate::domain::identifier::Identifier;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("fluent file read failed - reason: {0}")]
    UnableToRead(String),

    #[error("fluent file parse failed - reason: {0}")]
    UnableToParse(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File(Resource);

impl File {
    pub fn resource(&self) -> &Resource {
        &self.0
    }

    pub fn identifiers(&self) -> HashSet<Identifier> {
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&self.0, &mut visitor);
        visitor.identifiers()
    }
}

impl TryFrom<&PathBuf> for File {
    type Error = FileError;

    fn try_from(value: &PathBuf) -> std::result::Result<Self, Self::Error> {
        let content =
            std::fs::read_to_string(value).map_err(|e| FileError::UnableToRead(e.to_string()))?;

        let resource =
            Parser::parse(content.as_str()).map_err(|e| FileError::UnableToParse(e.to_string()))?;

        Ok(Self(resource))
    }
}
